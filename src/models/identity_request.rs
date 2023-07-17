/// IdentityRequest : Request to link an identity provider's audience and your app's audience with Authress.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRequest {
    /// A valid JWT OIDC compliant token which will still pass authentication requests to the identity provider. Must contain a unique audience and issuer.
    #[serde(rename = "jwt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<Option<String>>,
    /// The issuer of the OAuth OIDC provider's JWTs. This value should match the `iss` claim in the provided tokens exactly.
    #[serde(rename = "issuer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<Option<String>>,
    /// If the `jwt` token contains more than one valid audience, then the single audience that should associated with Authress. If more than one audience is preferred, repeat this call with each one.
    #[serde(rename = "preferredAudience", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preferred_audience: Option<Option<String>>,
}

impl IdentityRequest {
    /// Request to link an identity provider's audience and your app's audience with Authress.
    pub fn new() -> IdentityRequest {
        IdentityRequest {
            jwt: None,
            issuer: None,
            preferred_audience: None,
        }
    }
}


