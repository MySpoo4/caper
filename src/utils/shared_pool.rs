use std::{
    collections::HashSet,
    sync::{Arc, Mutex, OnceLock},
};

pub type SharedStr = Arc<str>;

#[derive(Debug)]
pub struct SharedPool {
    pool: Mutex<HashSet<Arc<str>>>,
}

impl SharedPool {
    pub fn instance() -> &'static Self {
        static POOL: OnceLock<SharedPool> = OnceLock::new();

        POOL.get_or_init(|| SharedPool {
            pool: HashSet::new().into(),
        })
    }

    pub fn get_or_intern<S: AsRef<str>>(input: S) -> SharedStr {
        let input = input.as_ref();
        let mut pool = Self::instance().pool.lock().unwrap();
        if let Some(name) = pool.get(input) {
            name.clone()
        } else {
            let name: Arc<str> = Arc::from(input);
            pool.insert(name.clone());
            name
        }
    }
}
