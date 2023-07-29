


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthAuthorizeResponse {
    /// The authorization code to be used with the /tokens endpoint to retrieve an access_token.
    #[serde(rename = "code")]
    pub code: String,
}

impl OAuthAuthorizeResponse {
    pub fn new(code: String) -> OAuthAuthorizeResponse {
        OAuthAuthorizeResponse {
            code,
        }
    }
}


