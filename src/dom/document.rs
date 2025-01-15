use std::sync::Arc;

use crate::{
    dom::DomNode,
    utils::LazyBase,
    xpath::{filter::XPathFilter, XPath},
};

#[derive(Debug)]
pub struct Document {
    pub lazy_base: Arc<LazyBase>,
    pub root: DomNode,
}

impl Document {
    pub fn query<'a>(&'a self, xpath: &'a XPath) -> XPathFilter<'a> {
        XPathFilter::new_with_node(xpath, &self.root)
    }
}
