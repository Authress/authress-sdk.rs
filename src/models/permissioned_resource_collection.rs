/// PermissionedResourceCollection : A collection of resource permissions that have been defined.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionedResourceCollection {
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::PermissionedResource>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl PermissionedResourceCollection {
    /// A collection of resource permissions that have been defined.
    pub fn new(resources: Vec<crate::models::PermissionedResource>, links: crate::models::CollectionLinks) -> PermissionedResourceCollection {
        PermissionedResourceCollection {
            resources,
            pagination: None,
            links: Box::new(links),
        }
    }
}


