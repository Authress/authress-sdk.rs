


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLinks {
    #[serde(rename = "self", deserialize_with = "Option::deserialize")]
    pub param_self: Option<Box<crate::models::Link>>,
}

impl AccountLinks {
    pub fn new(param_self: Option<crate::models::Link>) -> AccountLinks {
        AccountLinks {
            param_self: if let Some(x) = param_self {Some(Box::new(x))} else {None},
        }
    }
}


