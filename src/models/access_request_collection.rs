/// AccessRequestCollection : A collection of access requests



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRequestCollection {
    /// A list of access requests
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::AccessRequest>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl AccessRequestCollection {
    /// A collection of access requests
    pub fn new(links: crate::models::CollectionLinks) -> AccessRequestCollection {
        AccessRequestCollection {
            records: None,
            pagination: None,
            links: Box::new(links),
        }
    }
}


