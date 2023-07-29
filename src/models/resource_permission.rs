


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePermission {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "allow")]
    pub allow: bool,
}

impl ResourcePermission {
    pub fn new(action: Action, allow: bool) -> ResourcePermission {
        ResourcePermission {
            action,
            allow,
        }
    }
}

/// 
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[default]
    #[serde(rename = "CLAIM")]
    Claim,
    #[serde(rename = "PUBLIC")]
    Public,
}

