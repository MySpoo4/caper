use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    collections::HashSet,
    sync::{Arc, OnceLock},
};

use crate::{
    dom::{
        error::{DomError, DomResult},
        node::DomNode,
        parser::interface::{Tag, TagKind, Token, TokenSinkResult},
        Document,
    },
    utils::{LazyBase, LazyStr, SharedPool},
};

impl DomNode {
    pub fn init(tag: Tag, lazy_base: Arc<LazyBase>) -> Self {
        DomNode {
            tag: tag.name,
            attributes: tag.attrs,
            text_content: LazyStr::init(lazy_base),
            children: Vec::new(),
        }
    }

    pub fn finalize(&mut self) {
        self.text_content.finalize();
    }
}

pub struct DomSink {
    lazy_base: Arc<LazyBase>,
    open_stack: RefCell<Vec<DomNode>>,
    text_content: RefCell<String>,
    error: Cell<Option<DomError>>,
}

impl DomSink {
    fn special_tags() -> &'static HashSet<&'static str> {
        static SPECIAL_TAGS: OnceLock<HashSet<&'static str>> = OnceLock::new();

        SPECIAL_TAGS.get_or_init(|| HashSet::from(["script", "style"]))
    }

    pub fn new() -> Self {
        let open_stack = vec![DomNode::new(SharedPool::get_or_intern("root"))];

        DomSink {
            lazy_base: Arc::new(LazyBase::default()),
            open_stack: open_stack.into(),
            text_content: String::new().into(),
            error: None.into(),
        }
    }

    pub fn process_token(&self, token: Token) -> TokenSinkResult {
        match token {
            Token::Tag(tag) => match tag.kind {
                TagKind::StartTag => self.handle_start(tag),
                TagKind::EndTag => self.handle_end(tag),
                TagKind::EmptyTag => self.handle_empty(tag),
            },
            Token::Text(text) => self.handle_text(text),
            Token::EndOfInput => TokenSinkResult::Suspend,
            Token::InvalidChar(c) => {
                self.error.set(Some(DomError::ParseError {
                    exp: format!("Invalid char '{}'", c),
                }));
                TokenSinkResult::Suspend
            }
            _ => TokenSinkResult::Continue,
        }
    }

    fn handle_start(&self, tag: Tag) -> TokenSinkResult {
        self.finalize_text();

        // handles special tags differently (e.g. script, style)
        match DomSink::special_tags().contains(tag.name.as_ref()) {
            true => {
                let name = tag.name.clone();
                self.add_node(tag);
                TokenSinkResult::Special(name)
            }
            false => self.add_node(tag),
        }
    }

    fn handle_end(&self, tag: Tag) -> TokenSinkResult {
        self.finalize_text();

        let matches;
        if self.open_stack.borrow().len() == 1 {
            return self.revert_stack();
        } else {
            matches = match self.open_stack.borrow().last() {
                Some(top) => tag.name == top.tag,
                _ => return TokenSinkResult::Suspend,
            };
        }

        match matches {
            true => self.finalize_node(),
            false => {
                self.finalize_node();
                self.handle_end(tag)
            }
        }
    }

    fn handle_empty(&self, tag: Tag) -> TokenSinkResult {
        self.finalize_text();

        self.add_node(tag);
        self.finalize_node()
    }

    fn handle_text(&self, str: String) -> TokenSinkResult {
        let mut text_content = self.text_content.borrow_mut();
        match text_content.borrow().len() {
            0 => {
                for (i, char) in str.chars().enumerate() {
                    if !matches!(char, ' ' | '\n' | '\r' | '\t') {
                        text_content.borrow_mut().push_str(&str[i..str.len()]);
                        break;
                    }
                }
            }
            _ => text_content.borrow_mut().push_str(&str),
        }
        TokenSinkResult::Continue
    }

    fn add_node(&self, tag: Tag) -> TokenSinkResult {
        self.open_stack
            .borrow_mut()
            .push(DomNode::init(tag, self.lazy_base.clone()));
        TokenSinkResult::Continue
    }

    fn finalize_node(&self) -> TokenSinkResult {
        let mut open_stack = self.open_stack.borrow_mut();
        let last = open_stack.pop();
        let parent = open_stack.last_mut();
        match (last, parent) {
            (Some(mut last), Some(parent)) => {
                last.finalize();
                parent.children.push(last);
                TokenSinkResult::Continue
            }
            _ => TokenSinkResult::Suspend,
        }
    }

    fn revert_stack(&self) -> TokenSinkResult {
        let mut stack = self.open_stack.borrow_mut();
        while let Some(node) = stack.last_mut() {
            match node.children.pop() {
                Some(child) => stack.push(child),
                None => break,
            }
        }
        TokenSinkResult::Continue
    }

    fn finalize_text(&self) {
        {
            let mut text_content = self.text_content.borrow_mut();
            for (i, char) in text_content.chars().rev().enumerate() {
                if !matches!(char, ' ' | '\n' | '\r' | '\t') {
                    let len = text_content.len();
                    text_content.truncate(len - i);
                    break;
                }
            }
        }

        self.lazy_base.as_ref().append(&self.text_content.take());
    }

    pub fn end(self) -> DomResult<Document> {
        match self.error.take() {
            Some(e) => Err(e),
            None => {
                let mut root = self.open_stack.borrow_mut().pop().unwrap();
                let node = root.children.pop();
                match (self.open_stack.borrow().len(), root.children.len(), node) {
                    (0, 0, Some(root)) => {
                        let lazy_base = self.lazy_base;
                        lazy_base.as_ref().borrow_mut().finalize();

                        Ok(Document { root, lazy_base })
                    }
                    (_, _, Some(_)) => Err(DomError::Error {
                        msg: "Multiple root nodes".to_string(),
                    }),
                    (_, _, None) => Err(DomError::Error {
                        msg: "No root node exists".to_string(),
                    }),
                }
            }
        }
    }
}
