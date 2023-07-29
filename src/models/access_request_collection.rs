/// AccessRequestCollection : A collection of access requests



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRequestCollection {
    /// A list of access requests
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::AccessRequest>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl AccessRequestCollection {
    /// A collection of access requests
    pub fn new() -> AccessRequestCollection {
        AccessRequestCollection {
            records: None,
            pagination: None,
            links: None,
        }
    }
}


