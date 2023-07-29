/// Identity : The identifying information which uniquely links a JWT OIDC token to an identity provider



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    /// The issuer of the JWT token. This can be any OIDC compliant provider.
    #[serde(rename = "issuer")]
    pub issuer: String,
    /// The audience of the JWT token. This can be either an audience for your entire app, or one particular audience for it. If there is more than one audience, multiple identities can be created.
    #[serde(rename = "audience")]
    pub audience: String,
}

impl Identity {
    /// The identifying information which uniquely links a JWT OIDC token to an identity provider
    pub fn new(issuer: String, audience: String) -> Identity {
        Identity {
            issuer,
            audience,
        }
    }
}


