/// UserToken : A JWT token with represents the user.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserToken {
    #[serde(rename = "userId")]
    pub user_id: String,
    /// The unique identifier for the token
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// The access token
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::Links>>,
}

impl UserToken {
    /// A JWT token with represents the user.
    pub fn new(user_id: String, token_id: String, token: String) -> UserToken {
        UserToken {
            user_id,
            token_id,
            token,
            links: None,
        }
    }
}


