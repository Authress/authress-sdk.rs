/// UserRoleCollection : A collect of roles that the user has to a resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRoleCollection {
    #[serde(rename = "userId")]
    pub user_id: crate::models::UserId,
    /// A list of the roles
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::UserRole>,
}

impl UserRoleCollection {
    /// A collect of roles that the user has to a resource.
    pub fn new(user_id: crate::models::UserId, roles: Vec<crate::models::UserRole>) -> UserRoleCollection {
        UserRoleCollection {
            user_id,
            roles,
        }
    }
}


