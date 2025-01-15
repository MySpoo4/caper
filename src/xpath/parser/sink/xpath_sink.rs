use std::cell::{Cell, RefCell};

use crate::xpath::{
    error::{XPathError, XPathResult},
    parser::interface::{Token, TokenSinkResult, XPathStep},
    XPath,
};

pub struct XPathSink {
    steps: RefCell<Vec<XPathStep>>,
    error: Cell<Option<XPathError>>,
}

impl XPathSink {
    pub fn new() -> Self {
        XPathSink {
            steps: Vec::new().into(),
            error: None.into(),
        }
    }

    pub fn process_token(&self, token: Token) -> TokenSinkResult {
        match token {
            Token::XPathStep(xpath_step) => self.add_step(xpath_step),
            Token::EndOfInput => TokenSinkResult::Suspend,
            Token::InvalidChar(c) => {
                self.error.set(Some(XPathError::ParseError {
                    exp: format!("Invalid char '{}'", c),
                }));
                TokenSinkResult::Suspend
            }
        }
    }

    fn add_step(&self, xpath_step: XPathStep) -> TokenSinkResult {
        self.steps.borrow_mut().push(xpath_step);
        TokenSinkResult::Continue
    }

    pub fn end(self) -> XPathResult<XPath> {
        match self.error.take() {
            Some(e) => Err(e),
            None => Ok(XPath {
                steps: self.steps.take(),
            }),
        }
    }
}
