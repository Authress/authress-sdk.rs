


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self", deserialize_with = "Option::deserialize")]
    pub self_link: Option<Box<crate::models::Link>>,
}

impl Links {
    pub fn new(self_link: Option<crate::models::Link>) -> Links {
        Links {
            self_link: if let Some(x) = self_link {Some(Box::new(x))} else {None},
        }
    }
}


