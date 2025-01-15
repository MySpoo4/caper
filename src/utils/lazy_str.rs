use std::{cell::RefCell, sync::Arc, usize};

use super::substring_finder::StrFinder;

#[derive(Debug)]
pub struct LazyBase {
    base: RefCell<String>,
    str_finder: RefCell<StrFinder>,
}

impl LazyBase {
    pub fn init(base: String) -> Self {
        LazyBase {
            base: base.into(),
            str_finder: StrFinder::default().into(),
        }
    }

    pub fn append(&self, str: &str) {
        let mut base_borrow = self.base.borrow_mut(); // Borrow mutably first

        // Check if the last character is a space or whitespace
        if let Some(last_char) = base_borrow.chars().last() {
            if !matches!(last_char, ' ' | '\n' | '\r' | '\t') {
                base_borrow.push(' ');
            }
        }

        // Now append the new string
        base_borrow.push_str(str);
    }

    pub fn finalize(&self) {
        self.str_finder.borrow_mut().change(&self.base.borrow());
    }

    pub fn len(&self) -> usize {
        self.base.borrow().len()
    }

    pub fn find_all(&self, needle: &str) -> Vec<usize> {
        self.str_finder
            .borrow()
            .find_all(&self.base.borrow(), needle)
    }

    pub fn contains(&self, needle: &str) -> bool {
        !self
            .str_finder
            .borrow()
            .find_all(&self.base.borrow(), needle)
            .is_empty()
    }
}

impl Default for LazyBase {
    fn default() -> Self {
        LazyBase {
            base: String::new().into(),
            str_finder: StrFinder::default().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LazyStr {
    base: Arc<LazyBase>,
    start: usize,
    end: usize,
}

impl LazyStr {
    pub fn build(base: Arc<LazyBase>, sub: &str) -> Option<Self> {
        // Get raw pointers to the start of the string and the substring
        // let base_str = &base.as_ref().borrow().base;
        let orig_ptr = base.as_ref().base.borrow().as_ptr();
        let sub_ptr = sub.as_ptr();

        // Ensure the substring is inside the string (the substring must be from the string)
        if orig_ptr <= sub_ptr
            && sub_ptr < unsafe { orig_ptr.add(base.as_ref().base.borrow().len()) }
        {
            // Calculate the offset of the substring within the string
            let offset = sub_ptr as usize - orig_ptr as usize;
            Some(LazyStr {
                base,
                start: offset,
                end: offset + sub.len(),
            })
        } else {
            None
        }
    }

    pub fn init(base: Arc<LazyBase>) -> Self {
        let len = base.as_ref().len();
        LazyStr {
            base,
            start: len,
            end: len,
        }
    }

    pub fn finalize(&mut self) {
        self.end = self.base.as_ref().len();
    }

    // assumes correct lazybase or will break
    pub fn as_str(&self) -> String {
        self.base.as_ref().base.borrow()[self.start..self.end].to_string()
    }

    pub fn contains(&self, base: &LazyBase, needle: &str) -> bool {
        base.find_all(needle)
            .into_iter()
            .any(|idx| idx >= self.start && idx < self.end)
    }
}

impl Default for LazyStr {
    fn default() -> Self {
        LazyStr {
            base: Arc::new(LazyBase::default()),
            start: usize::default(),
            end: usize::default(),
        }
    }
}
