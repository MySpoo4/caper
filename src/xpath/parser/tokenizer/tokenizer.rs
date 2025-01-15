use std::sync::Arc;

use crate::{
    utils::{
        parser::{error::PError, traits::Parser},
        CharQueue, ParseQueue,
    },
    xpath::parser::{
        interface::{Token, TokenSinkResult},
        sink::xpath_sink::XPathSink,
    },
};

use super::parsers::parse_xpath_step;

enum ProcessResult {
    Continue,
    Suspend,
}

pub struct Tokenizer {
    sink: Arc<XPathSink>,
}

impl Tokenizer {
    pub fn new(sink: Arc<XPathSink>) -> Self {
        Tokenizer { sink }
    }

    pub fn feed(&mut self, input: CharQueue) {
        if input.is_empty() {
            return;
        }

        self.run(input);
    }

    fn run(&mut self, mut input: CharQueue) {
        let mut input = ParseQueue::new(&mut input);
        loop {
            match self.step(&mut input) {
                ProcessResult::Continue => (),
                ProcessResult::Suspend => break,
            }
        }
    }

    fn step(&mut self, input: &mut ParseQueue) -> ProcessResult {
        let xpath_step = parse_xpath_step().parse(input);
        match xpath_step {
            Ok(xpath_step) => {
                input.update();
                self.emit_token(Token::XPathStep(xpath_step))
            }
            Err(err) => self.handle_err(err),
        }
    }

    fn handle_err(&self, err: PError) -> ProcessResult {
        match err {
            PError::InvalidChar(c) => self.emit_token(Token::InvalidChar(c)),
            PError::EndOfInput => self.emit_token(Token::EndOfInput),
        }
    }

    fn emit_token(&self, token: Token) -> ProcessResult {
        match self.sink.process_token(token) {
            TokenSinkResult::Continue => ProcessResult::Continue,
            TokenSinkResult::Suspend => ProcessResult::Suspend,
        }
    }
}
