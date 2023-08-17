use crate::store::Store;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KeywordType {
    Unknown,
    Reference,
    Function,
    Op,
}

impl KeywordType {
    pub fn is_op(&self) -> bool {
        self == &KeywordType::Op
    }
}

pub struct KeywordManager {
    store: &'static Mutex<HashMap<String, KeywordType>>,
}

impl KeywordManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, KeywordType>>> = OnceCell::new();
        let store = STORE.get_or_init(|| {
            let tmp = Mutex::new(HashMap::new());
            tmp
        });
        KeywordManager { store: store }
    }

    pub fn init(&mut self) {
        let list = vec!["?", ":", "not"];
        for i in list {
            self.register(i, KeywordType::Op);
        }
    }

    pub fn register(&mut self, keyword: &str, typ: KeywordType) {
        self.store.lock().unwrap().insert(keyword.to_string(), typ);
    }

    pub fn get_type(&self, keyword: &str) -> KeywordType {
        let store = self.store.lock().unwrap();

        let ans = store.get(keyword);
        if ans.is_none() {
            return KeywordType::Unknown;
        }
        ans.unwrap().clone()
    }

    pub fn list(&self) -> Vec<(String, KeywordType)> {
        let binding = self.store.lock().unwrap();
        binding
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }
}
