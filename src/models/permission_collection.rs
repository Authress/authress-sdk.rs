/// PermissionCollection : A collect of permissions that the user has to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionCollection {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<crate::models::PermissionCollectionAccount>>,
    #[serde(rename = "userId")]
    pub user_id: crate::models::UserId,
    /// A list of the permissions
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::PermissionObject>,
}

impl PermissionCollection {
    /// A collect of permissions that the user has to a resource.
    pub fn new(user_id: crate::models::UserId, permissions: Vec<crate::models::PermissionObject>) -> PermissionCollection {
        PermissionCollection {
            account: None,
            user_id,
            permissions,
        }
    }
}


