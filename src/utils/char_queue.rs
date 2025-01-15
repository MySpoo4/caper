use std::collections::VecDeque;

#[derive(Debug)]
pub struct CharQueue {
    queue: VecDeque<char>,
}

impl CharQueue {
    pub fn from_str<S: AsRef<str>>(input: S) -> Self {
        CharQueue {
            queue: VecDeque::from(input.as_ref().chars().collect::<Vec<_>>()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<char> {
        self.queue.front().map(|c| c.clone())
    }

    pub fn dequeue(&mut self) -> Option<char> {
        self.queue.pop_front()
    }

    pub fn push_front(&mut self, c: char) {
        self.queue.push_front(c);
    }

    pub fn remove_white(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' {
                self.dequeue();
            } else {
                break;
            }
        }
    }

    pub fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if condition(c) {
                result.push(self.dequeue().unwrap());
            } else {
                break;
            }
        }
        result
    }

    pub fn eat(&mut self, word: &str) -> bool {
        let word_len = word.len();

        if self.queue.len() < word_len {
            return false;
        }

        if self
            .queue
            .iter()
            .take(word_len)
            .zip(word.bytes())
            .all(|(&q, w)| {
                let q = q as u8;
                q.to_ascii_lowercase() == w.to_ascii_lowercase()
            })
        {
            // Consume the characters if there's a match
            self.queue.drain(0..word_len);
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) {
        self.dequeue();
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

#[derive(Debug)]
pub struct ParseQueue<'queue> {
    char_queue: &'queue mut CharQueue,
    save_states: Vec<usize>,
    position: usize,
}

impl<'queue> ParseQueue<'queue> {
    pub fn new(char_queue: &'queue mut CharQueue) -> Self {
        ParseQueue {
            char_queue,
            save_states: Vec::new(),
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.char_queue.queue.get(self.position).map(|c| c.clone())
    }

    pub fn real_dequeue(&mut self) -> Option<char> {
        self.char_queue.dequeue()
    }

    pub fn dequeue(&mut self) -> Option<char> {
        let out = self.char_queue.queue.get(self.position).map(|c| c.clone());
        self.position += 1;
        out
    }

    pub fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if condition(c) {
                result.push(self.dequeue().unwrap());
            } else {
                break;
            }
        }
        result
    }

    pub fn consume_till<F>(&mut self, mut condition: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if !condition(c) {
                result.push(self.dequeue().unwrap());
            } else {
                break;
            }
        }
        result
    }

    pub fn update(&mut self) {
        self.char_queue.queue.drain(0..self.position);
        self.save_states.clear();
        self.position = 0;
    }

    pub fn save(&mut self) {
        self.save_states.push(self.position);
    }

    pub fn remove_save(&mut self) {
        self.save_states.pop();
    }

    pub fn revert(&mut self) {
        self.position = self.save_states.pop().unwrap_or(0);
    }

    pub fn len(&self) -> usize {
        self.char_queue.len()
    }
}
