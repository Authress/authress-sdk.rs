


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantConnection {
    #[serde(rename = "connectionId", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

impl TenantConnection {
    pub fn new() -> TenantConnection {
        TenantConnection {
            connection_id: None,
        }
    }
}


