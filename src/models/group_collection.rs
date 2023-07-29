/// GroupCollection : A collection of groups



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCollection {
    /// A list of groups
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::Group>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl GroupCollection {
    /// A collection of groups
    pub fn new(groups: Vec<crate::models::Group>) -> GroupCollection {
        GroupCollection {
            groups,
            pagination: None,
            links: None,
        }
    }
}


