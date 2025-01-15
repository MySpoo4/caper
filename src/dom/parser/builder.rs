use std::{cell::RefCell, sync::Arc};

use crate::{
    dom::{
        error::{DomError, DomResult},
        parser::{sink::DomSink, tokenizer::Tokenizer},
        Document,
    },
    utils::CharQueue,
};

pub struct DomBuilder {
    tokenizer: RefCell<Tokenizer>,
    sink: Arc<DomSink>,
}

impl DomBuilder {
    pub fn parse(input: &str) -> DomResult<Document> {
        let builder = Self::new();
        builder.feed(input);
        builder.finalize()
    }

    pub fn new() -> Self {
        let sink = Arc::new(DomSink::new());
        let tokenizer = RefCell::new(Tokenizer::new(sink.clone()));
        Self { tokenizer, sink }
    }

    pub fn feed(&self, input: &str) {
        self.tokenizer.borrow_mut().feed(CharQueue::from_str(input));
    }

    pub fn finalize(self) -> DomResult<Document> {
        std::mem::drop(self.tokenizer);
        match Arc::try_unwrap(self.sink) {
            Ok(sink) => Ok(sink.end()?),
            Err(_) => Err(DomError::Error {
                msg: "More references to sink exists".to_string(),
            }),
        }
    }
}
