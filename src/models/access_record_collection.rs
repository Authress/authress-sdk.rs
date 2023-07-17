/// AccessRecordCollection : A collection of access records



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRecordCollection {
    /// A list of access records
    #[serde(rename = "records")]
    pub records: Vec<crate::models::AccessRecord>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl AccessRecordCollection {
    /// A collection of access records
    pub fn new(records: Vec<crate::models::AccessRecord>, links: crate::models::CollectionLinks) -> AccessRecordCollection {
        AccessRecordCollection {
            records,
            pagination: None,
            links: Box::new(links),
        }
    }
}


