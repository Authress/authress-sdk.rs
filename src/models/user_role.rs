/// UserRole : A role with associated role data.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(rename = "roleId")]
    pub role_id: String,
}

impl UserRole {
    /// A role with associated role data.
    pub fn new(role_id: String) -> UserRole {
        UserRole {
            role_id,
        }
    }
}


