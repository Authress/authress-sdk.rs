


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionLinks {
    #[serde(rename = "self", deserialize_with = "Option::deserialize")]
    pub self_link: Option<Box<crate::models::Link>>,
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<Box<crate::models::Link>>>,
}

impl CollectionLinks {
    pub fn new(self_link: Option<crate::models::Link>) -> CollectionLinks {
        CollectionLinks {
            self_link: if let Some(x) = self_link {Some(Box::new(x))} else {None},
            next: None,
        }
    }
}


