/// Invite : The user invite used to invite users to your application or to Authress as an admin.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invite {
    /// The unique identifier for the invite. Use this ID to accept the invite.
    #[serde(rename = "inviteId")]
    pub invite_id: String,
    /// A list of statements which match roles to resources. The invited user will all statements apply to them
    #[serde(rename = "statements")]
    pub statements: Vec<crate::models::Statement>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl Invite {
    /// The user invite used to invite users to your application or to Authress as an admin.
    pub fn new(invite_id: String, statements: Vec<crate::models::Statement>, links: crate::models::Links) -> Invite {
        Invite {
            invite_id,
            statements,
            links: Box::new(links),
        }
    }
}


