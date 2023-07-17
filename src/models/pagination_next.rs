/// PaginationNext : A reference to the next page in the collection, pass the cursor as a query parameter in the subsequent request to get the next page.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationNext {
    #[serde(rename = "cursor")]
    pub cursor: String,
}

impl PaginationNext {
    /// A reference to the next page in the collection, pass the cursor as a query parameter in the subsequent request to get the next page.
    pub fn new(cursor: String) -> PaginationNext {
        PaginationNext {
            cursor,
        }
    }
}


