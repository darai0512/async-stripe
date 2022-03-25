use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    fs::write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use heck::SnakeCase;
use itertools::Itertools;

use crate::{
    codegen::{
        gen_emitted_structs, gen_enums, gen_generated_schemas, gen_impl_requests,
        gen_inferred_params, gen_objects, gen_prelude, gen_struct, gen_unions,
    },
    crate_generator::CrateGenerator,
    metadata::Metadata,
    types::{
        CopyOrClone, InferredEnum, InferredObject, InferredParams, InferredStruct, InferredUnion,
    },
    url_finder::UrlFinder,
};

///
#[derive(Default, Debug)]
pub struct FileGenerator {
    pub name: String,
    pub imported: Imported,
    pub inferred: Inferred,
    pub generated: Generated,
}

#[derive(Default, Debug)]
pub struct Imported {
    /// The ids that must be imported in this file.
    pub ids: BTreeSet<(&'static str, String)>,
    /// The config that must be imported in this file.
    pub client: BTreeSet<(&'static str, String)>,
    /// The params that must be imported in this file.
    pub params: BTreeSet<(&'static str, String)>,
    /// The resources that must be imported in this file.
    pub resources: BTreeSet<(&'static str, String)>,
}

impl Imported {
    pub fn gen_imports(
        &self,
        file_state: &FileGenerator,
        crate_state: &CrateGenerator,
        meta: &Metadata,
    ) -> String {
        let a_orig = &crate_state.crate_name;

        let imports = [(&self.client, "client"), (&self.ids, "ids"), (&self.params, "params")];
        let imports = imports
            .iter()
            .flat_map(|(set, name)| set.iter().map(move |(crate_, type_)| (crate_, name, type_)));

        let resources = self
            .resources
            .iter()
            .filter(|&(_, x)| {
                file_state.generated.schemas.keys().all(|sch| x != &meta.schema_to_rust_type(sch))
                    && !file_state.inferred.parameters.contains_key(x)
                    && !file_state.inferred.structs.contains_key(x)
                    && !file_state.inferred.unions.contains_key(x)
                    && !file_state.inferred.enums.contains_key(x)
                    && x != &meta.schema_to_rust_type(&file_state.name)
            })
            .map(|(crate_, type_)| (crate_, &"resources", type_));

        imports
            .chain(resources)
            .map(|(a, b, c)| format!("use {}::{b}::{c};", if a.eq(&a_orig) { "crate" } else { a }))
            .join("\n")
    }
}

#[derive(Default, Debug)]
pub struct Inferred {
    /// Extra (simple) enums that were / will be generated in this file.
    pub enums: BTreeMap<String, InferredEnum>,
    /// Extra (complex) enums that were / will be generated in this file.
    pub unions: BTreeMap<String, InferredUnion>,
    /// Extra structs that were / will be generated in this file.
    pub structs: BTreeMap<String, InferredStruct>,
    /// The request parameter structs that were / will be generated in this file.
    pub parameters: BTreeMap<String, InferredParams>,
}

#[derive(Default, Debug)]
pub struct Generated {
    /// The schemas that were / will be generated in this file.
    pub schemas: BTreeMap<String, bool>,
    /// New experimental struct thatclear
    ///  will eventually do most of the general work
    pub objects: BTreeMap<String, InferredObject>,
}

impl FileGenerator {
    pub fn new(object_name: String) -> Self {
        Self { name: object_name, ..Default::default() }
    }

    fn get_path(&self) -> String {
        self.get_module() + ".rs"
    }

    fn get_module(&self) -> String {
        self.name.replace('.', "_").to_snake_case()
    }

    /// Generates this file to the given Path, returning a set
    /// of FileGenerators for the files this one depends on.
    #[tracing::instrument(skip(self, meta, crate_state, url_finder))]
    pub fn write<T>(
        &mut self,
        base: T,
        meta: &Metadata,
        crate_state: &CrateGenerator,
        url_finder: &UrlFinder,
    ) -> Result<(String, BTreeSet<FileGenerator>)>
    where
        T: AsRef<Path> + std::fmt::Debug,
    {
        let path = self.get_path();
        let (out, additional) = self.generate(meta, crate_state, url_finder)?;
        let pathbuf = base.as_ref().join(path);
        log::debug!("writing object {} to {:?}", self.name, pathbuf);
        write(&pathbuf, out.as_bytes())?;
        Ok((self.get_module(), additional))
    }

    /// Generates this file, returning a set of FileGenerators
    /// for the files this one depends on.
    #[tracing::instrument(skip(self, meta, crate_state, url_finder))]
    pub fn generate(
        &mut self,
        meta: &Metadata,
        crate_state: &CrateGenerator,
        url_finder: &UrlFinder,
    ) -> Result<(String, BTreeSet<FileGenerator>)> {
        let mut out = String::new();
        let mut shared_objects = BTreeSet::<FileGenerator>::new();

        let id_type = meta.schema_to_id_type(&self.name);
        let struct_name = meta.schema_to_rust_type(&self.name);

        let fields = meta.spec["components"]["schemas"][&self.name]["properties"]
            .as_object()
            .ok_or(anyhow!("no properties"))?;

        gen_struct(&mut out, self, meta, &mut shared_objects, url_finder);

        if let Some(object_literal) = fields.get("object").and_then(|o| o["enum"][0].as_str()) {
            self.gen_object_trait(meta, id_type, &mut out, struct_name, object_literal);
        }

        gen_generated_schemas(&mut out, self, meta, &mut shared_objects);

        gen_inferred_params(&mut out, self, meta, &mut shared_objects);

        gen_emitted_structs(&mut out, self, meta, &mut shared_objects);

        gen_unions(&mut out, self, meta);

        gen_enums(&mut out, self, meta);

        gen_objects(&mut out, self);

        Ok((gen_prelude(self, crate_state, meta) + &out, shared_objects))
    }

    fn gen_object_trait(
        &mut self,
        meta: &Metadata,
        id_type: Option<(String, CopyOrClone)>,
        out: &mut String,
        struct_name: String,
        object_literal: &str,
    ) {
        if let Some(impls) =
            gen_impl_requests(self, meta, id_type.as_ref().map(|(x, _)| x.as_str()))
        {
            out.push('\n');
            out.push_str(&impls);
        }
        self.imported.params.insert(("stripe", "Object".to_string()));
        out.push('\n');
        out.push_str("impl Object for ");
        out.push_str(&struct_name);
        out.push_str(" {\n");
        out.push_str("    type Id = ");
        if let Some((id_type, _)) = &id_type {
            out.push_str(id_type);
            out.push_str(";\n");
            out.push_str("    fn id(&self) -> Self::Id {\n        self.id.clone()\n    }\n");
        } else {
            out.push_str("();\n");
            out.push_str("    fn id(&self) -> Self::Id {}\n");
        }
        out.push_str("    fn object(&self) -> &'static str {\n        \"");
        out.push_str(object_literal);
        out.push_str("\"\n    }\n");
        out.push_str("}\n");
    }

    pub fn insert_enum(&mut self, name: impl Into<String>, enum_: InferredEnum) {
        if let Err(other) = self.try_insert_enum(name, enum_.clone()) {
            panic!("conflicting enums are not compatible:\n\t{:?}\n\t!=\n\t{:?}", enum_, other);
        }
    }

    pub fn try_insert_enum(
        &mut self,
        name: impl Into<String>,
        enum_: InferredEnum,
    ) -> Result<(), &InferredEnum> {
        let name = name.into();
        let mut enum_ = enum_;
        enum_.options.sort();
        if let std::collections::btree_map::Entry::Vacant(e) =
            self.inferred.enums.entry(name.clone())
        {
            e.insert(enum_);
            return Ok(());
        }
        if let Some(other) = self.inferred.enums.get(&name) {
            if enum_.options != other.options {
                return Err(other);
            }
        }
        Ok(())
    }

    pub fn insert_struct(&mut self, name: impl Into<String>, struct_: InferredStruct) {
        if let Err(other) = self.try_insert_struct(name, struct_.clone()) {
            panic!("conflicting structs are not compatible:\n\t{:?}\n\t!=\n\t{:?}", struct_, other);
        }
    }

    pub fn try_insert_struct(
        &mut self,
        name: impl Into<String>,
        struct_: InferredStruct,
    ) -> Result<(), &InferredStruct> {
        let name = name.into();
        if let Entry::Vacant(e) = self.inferred.structs.entry(name.clone()) {
            e.insert(struct_);
            return Ok(());
        }
        if let Some(other) = self.inferred.structs.get(&name) {
            let mut self_schema = struct_.schema;
            let mut other_schema = other.schema.clone();
            if let Some(x) = self_schema.as_object_mut() {
                x.remove("description");
                x.remove("title");
            }
            if let Some(x) = other_schema.as_object_mut() {
                x.remove("description");
                x.remove("title");
            }
            if self_schema != other_schema {
                return Err(other);
            }
        }
        Ok(())
    }
}

impl PartialEq for FileGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for FileGenerator {}

impl PartialOrd for FileGenerator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for FileGenerator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
