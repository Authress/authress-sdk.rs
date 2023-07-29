/// UserIdentity : The composite user identity stored in Authress sourced by the customer SSO/SAML/OAuth IdP.

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    /// The user identifier.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// The user's formatted display name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A url that resolves to a picture that can be rendered.
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    /// The user's verified email address sourced from their SSO IdP.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl UserIdentity {
    /// The composite user identity stored in Authress sourced by the customer SSO/SAML/OAuth IdP.
    pub fn new(user_id: String) -> UserIdentity {
        UserIdentity {
            user_id,
            name: None,
            picture: None,
            email: None,
        }
    }
}


