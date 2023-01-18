use std::collections::HashMap;
use std::sync::Arc;

pub enum KeywordType {
    Unknown,
    Reference,
    Function,
    Op,
}

pub struct KeywordManager {
    store: HashMap<String, KeywordType>,
}

impl KeywordManager {
    pub fn new() -> Arc<Self> {
        static mut KEYWORD_MANAGER: Option<Arc<KeywordManager>> = None;
        unsafe {
            match &KEYWORD_MANAGER {
                Some(m) => m.clone(),
                None => KEYWORD_MANAGER
                    .get_or_insert(Arc::new(KeywordManager {
                        store: Self::internal_register(HashMap::new()),
                    }))
                    .clone(),
            }
        }
    }

    pub fn register(&mut self, keyword: String, typ: KeywordType) {
        self.store.insert(keyword, typ);
    }

    pub fn get_type(&self, keyword: &String) -> &KeywordType {
        self.store.get(keyword).get_or_insert(&KeywordType::Unknown)
    }

    fn internal_register(mut m: HashMap<String, KeywordType>) -> HashMap<String, KeywordType> {
        m.insert("beginWith".to_string(), KeywordType::Op);
        m.insert("endWith".to_string(), KeywordType::Op);
        m.insert("in".to_string(), KeywordType::Op);
        m
    }
}
