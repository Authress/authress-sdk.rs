/// ClientAccessKey : A client access key configuration. The configuration contains information about the key. On first creation the configuration also contains the raw `clientSecret` and `accessKey` for use with OAuth and the Authress SDKs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientAccessKey {
    /// The unique ID of the client.
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// The unique ID of the client.
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "generationDate", skip_serializing_if = "Option::is_none")]
    pub generation_date: Option<String>,
    /// The unencoded OAuth client secret used with the OAuth endpoints to request a JWT using the `client_credentials` grant type. Pass the clientId and the clientSecret to the documented /tokens endpoint.
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// An encoded access key which contains identifying information for client access token creation. For direct use with the Authress SDKs. This private access key must be saved on first creation as it is discarded afterwards. Authress only saves the corresponding public key to verify the private access key.
    #[serde(rename = "accessKey", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
}

impl ClientAccessKey {
    /// A client access key configuration. The configuration contains information about the key. On first creation the configuration also contains the raw `clientSecret` and `accessKey` for use with OAuth and the Authress SDKs.
    pub fn new(client_id: String) -> ClientAccessKey {
        ClientAccessKey {
            key_id: None,
            client_id,
            generation_date: None,
            client_secret: None,
            access_key: None,
        }
    }
}


