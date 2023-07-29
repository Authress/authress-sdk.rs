/// UserResources : A collect of permissions that the user has to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResources {
    #[serde(rename = "userId")]
    pub user_id: String,
    /// A list of the resources the user has some permission to.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::Resource>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl UserResources {
    /// A collect of permissions that the user has to a resource.
    pub fn new(user_id: String) -> UserResources {
        UserResources {
            user_id,
            resources: None,
            pagination: None,
            links: None,
        }
    }
}


