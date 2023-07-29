/// Link : A url linking object that complies to application/links+json RFC. Either is an IANA approved link relation or has a custom rel specified.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// The absolute url pointing to the reference resource.
    #[serde(rename = "href")]
    pub href: String,
    /// Optional property indicating the type of link if it is not a default IANA approved global link relation.
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
}

impl Link {
    /// A url linking object that complies to application/links+json RFC. Either is an IANA approved link relation or has a custom rel specified.
    pub fn new(href: String) -> Link {
        Link {
            href,
            rel: None,
        }
    }
}


