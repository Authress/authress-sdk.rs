/// PaginationCursor : A reference to the next page in the collection, pass the cursor as a query parameter in the subsequent request to get the next page.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationCursor {
    #[serde(rename = "cursor")]
    pub cursor: String,
}

impl PaginationCursor {
    /// A reference to the next page in the collection, pass the cursor as a query parameter in the subsequent request to get the next page.
    pub fn new(cursor: String) -> PaginationCursor {
        PaginationCursor {
            cursor,
        }
    }
}


