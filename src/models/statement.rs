


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::Resource>,
    /// The list of users this statement applies to. Users and groups can be set at either the statement level or the record level, but not both.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    /// The list of groups this statement applies to. Users in these groups will be receive access to the resources listed. Users and groups can be set at either the statement level or the record level, but not both.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::LinkedGroup>>,
}

impl Statement {
    pub fn new(roles: Vec<String>, resources: Vec<crate::models::Resource>) -> Statement {
        Statement {
            roles,
            resources,
            users: None,
            groups: None,
        }
    }
}


