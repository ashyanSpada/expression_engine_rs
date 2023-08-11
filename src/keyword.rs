use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq)]
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
        let store = STORE.get_or_init(|| Mutex::new(Self::internal_register(HashMap::new())));
        KeywordManager { store: store }
    }

    pub fn register(&mut self, keyword: String, typ: KeywordType) {
        self.store.lock().unwrap().insert(keyword, typ);
    }

    pub fn get_type(&self, keyword: &str) -> KeywordType {
        let store = self.store.lock().unwrap();

        let ans = store.get(keyword);
        if ans.is_none() {
            return KeywordType::Unknown;
        }
        ans.unwrap().clone()
    }

    fn internal_register(mut m: HashMap<String, KeywordType>) -> HashMap<String, KeywordType> {
        m.insert("beginWith".to_string(), KeywordType::Op);
        m.insert("endWith".to_string(), KeywordType::Op);
        m.insert("in".to_string(), KeywordType::Op);
        m
    }
}
