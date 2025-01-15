use std::sync::Arc;

use crate::{
    dom::parser::{
        interface::{Token, TokenSinkResult},
        sink::dom_sink::DomSink,
    },
    utils::{
        parser::{error::PError, traits::Parser},
        CharQueue, ParseQueue, SharedStr,
    },
};

use super::parsers::{parse_special, parse_token};

enum ProcessResult {
    Continue,
    Suspend,
}

pub enum State {
    Base,
    Special(SharedStr),
}

pub struct Tokenizer {
    sink: Arc<DomSink>,
    state: State,
}

impl Tokenizer {
    pub fn new(sink: Arc<DomSink>) -> Self {
        Tokenizer {
            sink,
            state: State::Base,
        }
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
        let token = match &self.state {
            State::Base => parse_token().parse(input),
            State::Special(sp) => {
                let token = parse_special(sp.as_ref()).parse(input);
                if let Ok(Token::Tag(_)) = token {
                    self.state = State::Base;
                }
                token
            }
        };

        match token {
            Ok(token) => {
                input.update();
                self.emit_token(token)
            }
            Err(err) => self.handle_err(err),
        }
    }

    fn emit_token(&mut self, token: Token) -> ProcessResult {
        match self.sink.process_token(token) {
            TokenSinkResult::Continue => ProcessResult::Continue,
            TokenSinkResult::Special(sp) => {
                self.state = State::Special(sp);
                ProcessResult::Continue
            }
            TokenSinkResult::Suspend => ProcessResult::Suspend,
        }
    }

    fn handle_err(&mut self, err: PError) -> ProcessResult {
        match err {
            PError::InvalidChar(c) => self.emit_token(Token::InvalidChar(c)),
            PError::EndOfInput => self.emit_token(Token::EndOfInput),
        }
    }
}
