/// ResourceUsersCollection : A collection of users with explicit permission to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUsersCollection {
    /// A list of users
    #[serde(rename = "users")]
    pub users: Vec<crate::models::UserRoleCollection>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl ResourceUsersCollection {
    /// A collection of users with explicit permission to a resource.
    pub fn new(users: Vec<crate::models::UserRoleCollection>) -> ResourceUsersCollection {
        ResourceUsersCollection {
            users,
            pagination: None,
            links: None,
        }
    }
}


