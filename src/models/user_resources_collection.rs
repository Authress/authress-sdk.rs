/// UserResourcesCollection : A collect of permissions that the user has to a resource.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResourcesCollection {
    #[serde(rename = "userId")]
    pub user_id: String,
    /// A list of the resources the user has some permission to.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::Resource>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Option<Box<crate::models::CollectionLinks>>,
}

impl UserResourcesCollection {
    /// A collect of permissions that the user has to a resource.
    pub fn new(user_id: String) -> UserResourcesCollection {
        UserResourcesCollection {
            user_id,
            resources: None,
            pagination: None,
            links: None,
        }
    }
}


