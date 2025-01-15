use std::sync::Arc;

use crate::{dom::node::Attribute, utils::SharedStr};

#[derive(Debug, Clone)]
pub enum TagKind {
    StartTag,
    EndTag,
    EmptyTag,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub kind: TagKind,
    pub name: SharedStr,
    pub attrs: Vec<Attribute>,
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            kind: TagKind::StartTag,
            name: Arc::from(""),
            attrs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Tag(Tag),
    Text(String),
    Doctype(String),
    Comment(String),

    // Errors
    InvalidChar(char),
    EndOfInput,
}

// #[derive(Debug, Clone)]
// pub enum Token<'a> {
//     DoctypeToken(&'a str),
//     CharacterToken(char),
//     TagToken(Tag),
//     CommentToken(&'a str),
//     ParseError(&'a str),
// }

pub enum TokenSinkResult {
    Continue,
    Special(SharedStr),
    Suspend,
}
