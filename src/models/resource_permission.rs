


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "CLAIM")]
    Claim,
    #[serde(rename = "PUBLIC")]
    Public,
}

impl Default for Action {
    fn default() -> Action {
        Self::Claim
    }
}

