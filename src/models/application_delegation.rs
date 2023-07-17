/// ApplicationDelegation : The delegation response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDelegation {
    /// Redirect the user to this url to automatically log them into a third-party application.
    #[serde(rename = "authenticationUrl")]
    pub authentication_url: String,
}

impl ApplicationDelegation {
    /// The delegation response.
    pub fn new(authentication_url: String) -> ApplicationDelegation {
        ApplicationDelegation {
            authentication_url,
        }
    }
}


