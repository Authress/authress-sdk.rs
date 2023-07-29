/// PermissionedResourceCollection : A collection of resource permissions that have been defined.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionedResourceCollection {
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::PermissionedResource>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl PermissionedResourceCollection {
    /// A collection of resource permissions that have been defined.
    pub fn new(resources: Vec<crate::models::PermissionedResource>) -> PermissionedResourceCollection {
        PermissionedResourceCollection {
            resources,
            pagination: None,
            links: None,
        }
    }
}


