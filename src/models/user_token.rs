/// UserToken : A JWT token with represents the user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserToken {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<crate::models::PermissionCollectionAccount>>,
    #[serde(rename = "userId")]
    pub user_id: crate::models::UserId,
    /// The unique identifier for the token
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// The access token
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::AccountLinks>>,
}

impl UserToken {
    /// A JWT token with represents the user.
    pub fn new(user_id: crate::models::UserId, token_id: String, token: String) -> UserToken {
        UserToken {
            account: None,
            user_id,
            token_id,
            token,
            links: None,
        }
    }
}


