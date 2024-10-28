use kuchikiki::NodeRef;

pub trait NodeRefExt {
    /// Creates a new HTML element with the given name and attributes.
    fn new_html_element(name: &str, attributes: Vec<(&str, &str)>) -> NodeRef;
}
impl NodeRefExt for NodeRef {
    fn new_html_element(name: &str, attributes: Vec<(&str, &str)>) -> NodeRef {
        use kuchikiki::{Attribute, ExpandedName};
        use markup5ever::{namespace_url, ns, LocalName, QualName};

        NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from(name)),
            attributes.into_iter().map(|(n, v)| {
                (
                    ExpandedName::new(ns!(), n),
                    Attribute {
                        prefix: None,
                        value: v.to_string(),
                    },
                )
            }),
        )
    }
}
