


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClaimRequest {
    /// The parent resource to add a sub-resource to. The resource must have a resource configuration that add the permission CLAIM for this to work.
    #[serde(rename = "collectionResource")]
    pub collection_resource: String,
    /// The sub-resource the user is requesting Admin ownership over.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

impl ClaimRequest {
    pub fn new(collection_resource: String, resource_id: String) -> ClaimRequest {
        ClaimRequest {
            collection_resource,
            resource_id,
        }
    }
}


