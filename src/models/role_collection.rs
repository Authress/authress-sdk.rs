/// RoleCollection : A collection of roles



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCollection {
    /// A list of roles
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::Role>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl RoleCollection {
    /// A collection of roles
    pub fn new(roles: Vec<crate::models::Role>, links: crate::models::CollectionLinks) -> RoleCollection {
        RoleCollection {
            roles,
            pagination: None,
            links: Box::new(links),
        }
    }
}


