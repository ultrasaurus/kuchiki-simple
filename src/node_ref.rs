use kuchikiki::NodeRef;
use markup5ever::{interface::QualName, namespace_url, ns, LocalName};

pub trait NodeRefExt {
    /// returns the `head` html element, if present  as child of `html`
    fn head(self) -> Option<NodeRef>;
    /// finds first match
    fn find_html_child_element(self, name: &str) -> Option<NodeRef>;
    /// Creates a new HTML element with the given name and attributes.
    fn new_html_element(name: &str, attributes: Vec<(&str, &str)>) -> NodeRef;
}
impl NodeRefExt for NodeRef {

    fn head(self) -> Option<NodeRef> {
        let maybe_html: Option<_> = self.find_html_child_element("html");
        if let Some(html) = maybe_html {
            if let Some(head) = html.find_html_child_element("head") {
                return Some(head)
            }
        }
        None
    }


    fn find_html_child_element(self, name: &str) -> Option<NodeRef> {
        let html_element: markup5ever::QualName = QualName::new(None, ns!(html),LocalName::from(name));
        self.children().find(|node| {
                if let Some(data) = node.as_element() {

                    if data.name == html_element {
                        return true
                    }
                }
            false
            })
    }


    // thanks critter-rs for this utility function
    fn new_html_element(name: &str, attributes: Vec<(&str, &str)>) -> NodeRef {
        use kuchikiki::{Attribute, ExpandedName};

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

#[cfg(test)]
mod tests {
    use markup5ever::LocalName;

    use super::*;

    #[test]
    fn test_head() {
        use kuchikiki::traits::*;
        let html: &str = r"
            <!DOCTYPE html>
            <html>
            <head></head>
            <body>
                <h1>Hello World!</h1>
            </body>
            </html>
        ";

        let document = kuchikiki::parse_html().one(html);
        let node_ref = document.head().unwrap();
        let e = node_ref.as_element().unwrap();
        assert_eq!(e.name.local, LocalName::from("head"));
    }



    #[test]
    fn test_new_html_element_no_attrs() {
        let node_ref = NodeRef::new_html_element("head", Vec::new());
        assert!(node_ref.as_element().is_some());
        let e = node_ref.as_element().unwrap();
        assert_eq!(e.name.local, LocalName::from("head"));
        assert!(e.attributes.borrow().map.is_empty());
    }
}