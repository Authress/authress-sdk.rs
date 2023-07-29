/// Role : The role which contains a list of permissions.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// Unique identifier for the role, can be specified on creation, and used by records to map to permissions.
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// A helpful name for this role
    #[serde(rename = "name")]
    pub name: String,
    /// A description for when to the user as well as additional information.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// A list of the permissions
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::PermissionObject>,
}

impl Role {
    /// The role which contains a list of permissions.
    pub fn new(role_id: String, name: String, permissions: Vec<crate::models::PermissionObject>) -> Role {
        Role {
            role_id,
            name,
            description: None,
            permissions,
        }
    }
}


