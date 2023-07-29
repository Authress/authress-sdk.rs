/// UserIdentityCollection : A collection of user identities



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityCollection {
    /// A list of users
    #[serde(rename = "users")]
    pub users: Vec<crate::models::UserIdentity>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl UserIdentityCollection {
    /// A collection of user identities
    pub fn new(users: Vec<crate::models::UserIdentity>) -> UserIdentityCollection {
        UserIdentityCollection {
            users,
            pagination: None,
            links: None,
        }
    }
}


