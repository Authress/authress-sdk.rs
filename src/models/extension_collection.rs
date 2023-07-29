/// ExtensionCollection : A collection of platform extensions.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionCollection {
    #[serde(rename = "extensions")]
    pub extensions: Vec<crate::models::Extension>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
}

impl ExtensionCollection {
    /// A collection of platform extensions.
    pub fn new(extensions: Vec<crate::models::Extension>) -> ExtensionCollection {
        ExtensionCollection {
            extensions,
            pagination: None,
        }
    }
}


