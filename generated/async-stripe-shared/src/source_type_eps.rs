#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeEps {
    pub reference: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeEpsBuilder {
    reference: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeEps {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeEps>,
        builder: SourceTypeEpsBuilder,
    }

    impl Visitor for Place<SourceTypeEps> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeEpsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeEpsBuilder {
        type Out = SourceTypeEps;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.reference),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                reference: self.reference.take()?,
                statement_descriptor: self.statement_descriptor.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceTypeEps {
        type Builder = SourceTypeEpsBuilder;
    }

    impl FromValueOpt for SourceTypeEps {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeEpsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => {
                        b.statement_descriptor = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};