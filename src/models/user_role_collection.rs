/// UserRoleCollection : A collect of roles that the user has to a resource.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRoleCollection {
    #[serde(rename = "userId")]
    pub user_id: String,
    /// A list of the roles
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::UserRole>,
}

impl UserRoleCollection {
    /// A collect of roles that the user has to a resource.
    pub fn new(user_id: String, roles: Vec<crate::models::UserRole>) -> UserRoleCollection {
        UserRoleCollection {
            user_id,
            roles,
        }
    }
}


