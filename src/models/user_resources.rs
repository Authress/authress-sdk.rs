/// UserResources : A collect of permissions that the user has to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResources {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<crate::models::PermissionCollectionAccount>>,
    #[serde(rename = "userId")]
    pub user_id: crate::models::UserId,
    /// A list of the resources the user has some permission to.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::Resource>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl UserResources {
    /// A collect of permissions that the user has to a resource.
    pub fn new(user_id: crate::models::UserId, links: crate::models::CollectionLinks) -> UserResources {
        UserResources {
            account: None,
            user_id,
            resources: None,
            pagination: None,
            links: Box::new(links),
        }
    }
}


