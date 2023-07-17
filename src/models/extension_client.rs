/// ExtensionClient : The extension's client configuration.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionClient {
    /// The unique ID of the client.
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl ExtensionClient {
    /// The extension's client configuration.
    pub fn new(client_id: String, links: crate::models::Links) -> ExtensionClient {
        ExtensionClient {
            client_id,
            links: Box::new(links),
        }
    }
}


