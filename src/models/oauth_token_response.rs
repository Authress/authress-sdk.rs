


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    /// An expiring access token that can be used to access either Authress or any platform service.
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl OAuthTokenResponse {
    pub fn new(access_token: String) -> OAuthTokenResponse {
        OAuthTokenResponse {
            access_token,
        }
    }
}


