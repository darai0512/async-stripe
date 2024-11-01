// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{InvoiceRenderingTemplateId};
use crate::params::{Expand, List, Metadata, Object, Paginable, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceRenderingTemplate".
///
/// For more details see <https://stripe.com/docs/api/invoice-rendering-template/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceRenderingTemplate {
    /// Unique identifier for the object.
    pub id: InvoiceRenderingTemplateId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// A brief description of the template, hidden from customers.
    pub nickname: Option<String>,

    /// The status of the template, one of `active` or `archived`.
    pub status: InvoiceRenderingTemplateStatus,

    /// Version of this template; version increases by one when an update on the template changes any field that controls invoice rendering.
    pub version: i64,
}

impl InvoiceRenderingTemplate {

    /// List all templates, ordered by creation date, with the most recently created template appearing first.
pub fn list(client: &Client, params: &ListInvoiceRenderingTemplates<'_>) -> Response<List<InvoiceRenderingTemplate>> {
   client.get_query("/invoice_rendering_templates", params)
}


    /// Retrieves an invoice rendering template with the given ID.
    ///
    /// It by default returns the latest version of the template.
    /// Optionally, specify a version to see previous versions.
    pub fn retrieve(client: &Client, id: &InvoiceRenderingTemplateId, expand: &[&str]) -> Response<InvoiceRenderingTemplate> {
        client.get_query(&format!("/invoice_rendering_templates/{}", id), Expand { expand })
    }

    /// Updates the status of an invoice rendering template to ‘archived’ so no new Stripe objects (customers, invoices, etc.) can reference it.
    ///
    /// The template can also no longer be updated.
    /// However, if the template is already set on a Stripe object, it will continue to be applied on invoices generated by it.
    pub fn update(client: &Client, id: &InvoiceRenderingTemplateId, params: UpdateInvoiceRenderingTemplate<'_>) -> Response<InvoiceRenderingTemplate> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/invoice_rendering_templates/{}", id), &params)
    }
}

impl Object for InvoiceRenderingTemplate {
    type Id = InvoiceRenderingTemplateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoice_rendering_template"
    }
}

/// The parameters for `InvoiceRenderingTemplate::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListInvoiceRenderingTemplates<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<InvoiceRenderingTemplateId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<InvoiceRenderingTemplateId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceRenderingTemplateStatus>,
}

impl<'a> ListInvoiceRenderingTemplates<'a> {
    pub fn new() -> Self {
        ListInvoiceRenderingTemplates {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
impl Paginable for ListInvoiceRenderingTemplates<'_> {
    type O = InvoiceRenderingTemplate;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// The parameters for `InvoiceRenderingTemplate::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateInvoiceRenderingTemplate<'a> {

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> UpdateInvoiceRenderingTemplate<'a> {
    pub fn new() -> Self {
        UpdateInvoiceRenderingTemplate {
            expand: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `InvoiceRenderingTemplate`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceRenderingTemplateStatus {
    Active,
    Archived,
}

impl InvoiceRenderingTemplateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceRenderingTemplateStatus::Active => "active",
            InvoiceRenderingTemplateStatus::Archived => "archived",
        }
    }
}

impl AsRef<str> for InvoiceRenderingTemplateStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceRenderingTemplateStatus {
    fn default() -> Self {
        Self::Active
    }
}