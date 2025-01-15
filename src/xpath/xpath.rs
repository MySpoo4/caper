use super::parser::interface::XPathStep;

#[derive(Debug)]
pub struct XPath {
    pub steps: Vec<XPathStep>,
}
