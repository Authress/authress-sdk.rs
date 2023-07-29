/// ExtensionClient : The extension's client configuration.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionClient {
    /// The unique ID of the client.
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::Links>>>,
}

impl ExtensionClient {
    /// The extension's client configuration.
    pub fn new(client_id: String) -> ExtensionClient {
        ExtensionClient {
            client_id,
            links: None,
        }
    }
}


