/// Pagination : Details containing how to pagination through the collection. Consists of an optional *next* property that may contain a cursor. Pagination is mutable and the list can change between requests.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<Box<crate::models::PaginationCursor>>>,
}

impl Pagination {
    /// Details containing how to pagination through the collection. Consists of an optional *next* property that may contain a cursor. Pagination is mutable and the list can change between requests.
    pub fn new() -> Pagination {
        Pagination {
            next: None,
        }
    }
}


