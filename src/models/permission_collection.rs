/// PermissionCollection : A collect of permissions that the user has to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionCollection {
    #[serde(rename = "userId")]
    pub user_id: String,
    /// A list of the permissions
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::PermissionObject>,
}

impl PermissionCollection {
    /// A collect of permissions that the user has to a resource.
    pub fn new(user_id: String, permissions: Vec<crate::models::PermissionObject>) -> PermissionCollection {
        PermissionCollection {
            user_id,
            permissions,
        }
    }
}


