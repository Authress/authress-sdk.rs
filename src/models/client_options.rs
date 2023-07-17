/// ClientOptions : A set of client specific options



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientOptions {
    /// Grant the client access to verify authorization on behalf of any user.
    #[serde(rename = "grantUserPermissionsAccess", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub grant_user_permissions_access: Option<Option<bool>>,
    /// Grant the client access to generate oauth tokens on behalf of the Authress account. **Security Warning**: This means that this client can impersonate any user, and should only be used when connecting an existing custom Authorization Server to Authress, when that server does not support a standard OAuth connection.
    #[serde(rename = "grantTokenGeneration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub grant_token_generation: Option<Option<bool>>,
}

impl ClientOptions {
    /// A set of client specific options
    pub fn new() -> ClientOptions {
        ClientOptions {
            grant_user_permissions_access: None,
            grant_token_generation: None,
        }
    }
}


