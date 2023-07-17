


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionLinks {
    #[serde(rename = "self", deserialize_with = "Option::deserialize")]
    pub param_self: Option<Box<crate::models::Link>>,
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<Box<crate::models::Link>>>,
}

impl CollectionLinks {
    pub fn new(param_self: Option<crate::models::Link>) -> CollectionLinks {
        CollectionLinks {
            param_self: if let Some(x) = param_self {Some(Box::new(x))} else {None},
            next: None,
        }
    }
}


