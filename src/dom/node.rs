use crate::{
    utils::{LazyStr, SharedStr},
    xpath::{filter::XPathFilter, XPath},
};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: SharedStr,
    pub value: AttributeValue,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Exists,
    Literal(String),
}

#[derive(Debug)]
pub struct DomNode {
    pub tag: SharedStr,
    pub attributes: Vec<Attribute>,
    pub text_content: LazyStr,
    pub children: Vec<DomNode>,
}

impl DomNode {
    pub fn new(tag: SharedStr) -> Self {
        DomNode {
            tag,
            attributes: Vec::new(),
            text_content: LazyStr::default(),
            children: Vec::new(),
        }
    }

    pub fn query<'a>(&'a self, xpath: &'a XPath) -> XPathFilter<'a> {
        XPathFilter::new_with_node(xpath, self)
    }
}
