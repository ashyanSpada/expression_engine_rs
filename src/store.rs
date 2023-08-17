use std::collections::HashMap;
use std::sync::Arc;

pub struct Store<T> {
    data: HashMap<String, Arc<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<T>> {
        let value = self.get(key);
        if value.is_none() {
            return None;
        }
        Some(value.unwrap().clone())
    }

    pub fn set(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Arc::new(value));
    }
}
