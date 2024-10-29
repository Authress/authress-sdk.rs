


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteStatement {
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::Resource>
}

impl InviteStatement {
    pub fn new(roles: Vec<String>, resources: Vec<crate::models::Resource>) -> InviteStatement {
        InviteStatement {
            roles,
            resources
        }
    }
}


