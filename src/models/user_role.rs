/// UserRole : A role with associated role data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(rename = "roleId")]
    pub role_id: crate::models::RoleId,
}

impl UserRole {
    /// A role with associated role data.
    pub fn new(role_id: crate::models::RoleId) -> UserRole {
        UserRole {
            role_id,
        }
    }
}


