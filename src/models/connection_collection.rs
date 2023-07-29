/// ConnectionCollection : A collection of connections.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionCollection {
    #[serde(rename = "connections")]
    pub connections: Vec<crate::models::Connection>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
}

impl ConnectionCollection {
    /// A collection of connections.
    pub fn new(connections: Vec<crate::models::Connection>) -> ConnectionCollection {
        ConnectionCollection {
            connections,
            pagination: None,
        }
    }
}


