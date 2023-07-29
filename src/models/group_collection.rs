/// GroupCollection : A collection of groups



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCollection {
    /// A list of groups
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::Group>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl GroupCollection {
    /// A collection of groups
    pub fn new(groups: Vec<crate::models::Group>, links: crate::models::CollectionLinks) -> GroupCollection {
        GroupCollection {
            groups,
            pagination: None,
            links: Box::new(links),
        }
    }
}


