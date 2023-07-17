


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// A resource path which can be top level, fully qualified, or end with a *. Parent resources imply permissions to sub-resources.
    #[serde(rename = "resourceUri")]
    pub resource_uri: String,
}

impl Resource {
    pub fn new(resource_uri: String) -> Resource {
        Resource {
            resource_uri,
        }
    }
}


