


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenRequest {
    /// A list of statements which match roles to resources. The token will have all statements apply to it.
    #[serde(rename = "statements")]
    pub statements: Vec<crate::models::Statement>,
    /// The ISO8601 datetime when the token will expire. Default is 24 hours from now.
    #[serde(rename = "expires")]
    pub expires: String,
}

impl TokenRequest {
    pub fn new(statements: Vec<crate::models::Statement>, expires: String) -> TokenRequest {
        TokenRequest {
            statements,
            expires,
        }
    }
}


