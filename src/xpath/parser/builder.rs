use std::{cell::RefCell, sync::Arc};

use crate::{
    utils::CharQueue,
    xpath::{
        error::{XPathError, XPathResult},
        XPath,
    },
};

use super::{sink::XPathSink, tokenizer::Tokenizer};

pub struct XPathBuilder {
    tokenizer: RefCell<Tokenizer>,
    sink: Arc<XPathSink>,
}

impl XPathBuilder {
    pub fn parse(input: &str) -> XPathResult<XPath> {
        let builder = Self::new();
        builder.feed(input);
        builder.finalize()
    }

    pub fn new() -> Self {
        let sink = Arc::new(XPathSink::new());
        let tokenizer = RefCell::new(Tokenizer::new(sink.clone()));
        Self { tokenizer, sink }
    }

    pub fn feed(&self, input: &str) {
        self.tokenizer.borrow_mut().feed(CharQueue::from_str(input));
    }

    pub fn finalize(self) -> XPathResult<XPath> {
        std::mem::drop(self.tokenizer);
        match Arc::try_unwrap(self.sink) {
            Ok(sink) => Ok(sink.end()?),
            Err(_) => Err(XPathError::Error {
                msg: "More references to sink exists".to_string(),
            }),
        }
    }
}
