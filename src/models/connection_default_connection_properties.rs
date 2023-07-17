


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionDefaultConnectionProperties {
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Option<String>>,
}

impl ConnectionDefaultConnectionProperties {
    pub fn new() -> ConnectionDefaultConnectionProperties {
        ConnectionDefaultConnectionProperties {
            scope: None,
        }
    }
}


