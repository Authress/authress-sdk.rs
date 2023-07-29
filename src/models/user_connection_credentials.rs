/// UserConnectionCredentials : The user credentials for this connection which can be used to access the connection provider APIs.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConnectionCredentials {
    /// The access token.
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

impl UserConnectionCredentials {
    /// The user credentials for this connection which can be used to access the connection provider APIs.
    pub fn new(access_token: String) -> UserConnectionCredentials {
        UserConnectionCredentials {
            access_token,
        }
    }
}


