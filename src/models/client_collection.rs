/// ClientCollection : The collection of a list of clients



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCollection {
    /// A list of clients
    #[serde(rename = "clients")]
    pub clients: Vec<crate::models::Client>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::CollectionLinks>>>,
}

impl ClientCollection {
    /// The collection of a list of clients
    pub fn new(clients: Vec<crate::models::Client>) -> ClientCollection {
        ClientCollection {
            clients,
            pagination: None,
            links: None,
        }
    }
}


