/// AccessRequestResponse : A dynamic body to support request PATCH operations



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRequestResponse {
    /// New result, either approve or deny the request
    #[serde(rename = "status")]
    pub status: Status,
}

impl AccessRequestResponse {
    /// A dynamic body to support request PATCH operations
    pub fn new(status: Status) -> AccessRequestResponse {
        AccessRequestResponse {
            status,
        }
    }
}

/// New result, either approve or deny the request
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "APPROVED")]
    Approved,
    #[default]
    #[serde(rename = "DENIED")]
    Denied,
}