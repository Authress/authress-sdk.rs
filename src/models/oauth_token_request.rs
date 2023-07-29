/// OAuthTokenRequest : The OAuth 2.1 token request that follows [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749). The properties in the request must be snake_case to follow the standard.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthTokenRequest {
    /// The client identifier to constrain the token to.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The secret associated with the client that authorizes the generation of token it's behalf. (Either the `client_secret` or the `code_verifier` is required)
    #[serde(rename = "client_secret", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<Option<String>>,
    /// The code verifier is the the value used in the generation of the OAuth authorization request `code_challenge` property. (Either the `client_secret` or the `code_verifier` is required)
    #[serde(rename = "code_verifier", skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// A suggestion to the token generation which type of credentials are being provided.
    #[serde(rename = "grant_type", skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<GrantType>,
    /// When using the user password grant_type, specify the username. Authress recommends this should always be an email address.
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
    /// When using the user password grant_type, specify the user's password.
    #[serde(rename = "password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    /// Enables additional configuration of the grant_type. In the case of user password grant_type, set this to **signup**, to enable the creation of users. Set this to **update**, force update the password associated with the user.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Type>>,
}

impl OAuthTokenRequest {
    /// The OAuth 2.1 token request that follows [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749). The properties in the request must be snake_case to follow the standard.
    pub fn new(client_id: String) -> OAuthTokenRequest {
        OAuthTokenRequest {
            client_id,
            client_secret: None,
            code_verifier: None,
            grant_type: None,
            username: None,
            password: None,
            r#type: None,
        }
    }
}

/// A suggestion to the token generation which type of credentials are being provided.
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GrantType {
    #[default]
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "password")]
    Password,
}

/// Enables additional configuration of the grant_type. In the case of user password grant_type, set this to **signup**, to enable the creation of users. Set this to **update**, force update the password associated with the user.
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[default]
    #[serde(rename = "")]
    Default,
    #[serde(rename = "signup")]
    Signup,
    #[serde(rename = "update")]
    Update,
}
