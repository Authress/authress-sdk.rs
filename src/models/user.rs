/// User : A user object that identifies a user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl User {
    /// A user object that identifies a user.
    pub fn new(user_id: String) -> User {
        User {
            user_id,
        }
    }
}


