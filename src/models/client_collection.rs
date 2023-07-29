/// ClientCollection : The collection of a list of clients



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCollection {
    /// A list of clients
    #[serde(rename = "clients")]
    pub clients: Vec<crate::models::Client>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::CollectionLinks>,
}

impl ClientCollection {
    /// The collection of a list of clients
    pub fn new(clients: Vec<crate::models::Client>, links: crate::models::CollectionLinks) -> ClientCollection {
        ClientCollection {
            clients,
            pagination: None,
            links: Box::new(links),
        }
    }
}


