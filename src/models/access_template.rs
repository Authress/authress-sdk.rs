/// AccessTemplate : A logical grouping of access related elements



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTemplate {
    /// The list of users the access applies to
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
    /// A list of statements which match roles to resources.
    #[serde(rename = "statements")]
    pub statements: Vec<crate::models::Statement>,
}

impl AccessTemplate {
    /// A logical grouping of access related elements
    pub fn new(users: Vec<crate::models::User>, statements: Vec<crate::models::Statement>) -> AccessTemplate {
        AccessTemplate {
            users,
            statements,
        }
    }
}


