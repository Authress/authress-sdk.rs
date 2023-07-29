


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionedResource {
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::ResourcePermission>,
}

impl PermissionedResource {
    pub fn new(permissions: Vec<crate::models::ResourcePermission>) -> PermissionedResource {
        PermissionedResource {
            permissions,
        }
    }
}


