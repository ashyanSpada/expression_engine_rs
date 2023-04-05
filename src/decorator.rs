use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Hash, Eq, PartialEq)]
enum DecoratorKey {
    UNARY(String),
    BINARY(String),
    TERNARY,
    FUNCTION(String),
    REFERENCE(String),
    LIST,
    MAP,
    CHAIN,
}

#[derive(Clone)]
enum Decorator {
    UNARY(Arc<UnaryDecorator>),
    BINARY(Arc<BinaryDecorator>),
    TERNARY(Arc<TernaryDecorator>),
    FUNCTION(Arc<FunctionDecorator>),
    REFERENCE(Arc<ReferenceDecorator>),
    LIST(Arc<ListDecorator>),
    MAP(Arc<MapDecorator>),
    CHAIN(Arc<ChainDecorator>),
}

type UnaryDecorator = dyn Fn(String, String) -> String + Send + Sync + 'static;
type BinaryDecorator = dyn Fn(String, String, String) -> String + Send + Sync + 'static;
type TernaryDecorator = dyn Fn(String, String, String) -> String + Send + Sync + 'static;
type FunctionDecorator = dyn Fn(String, Vec<String>) -> String + Send + Sync + 'static;
type ReferenceDecorator = dyn Fn(String) -> String + Send + Sync + 'static;
type ListDecorator = dyn Fn(Vec<String>) -> String + Send + Sync + 'static;
type MapDecorator = dyn Fn(HashMap<String, String>) -> String + Send + Sync + 'static;
type ChainDecorator = dyn Fn(Vec<String>) -> String + Send + Sync + 'static;

pub struct DecoratorManager {
    store: &'static Mutex<HashMap<DecoratorKey, Decorator>>,
}

impl DecoratorManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<DecoratorKey, Decorator>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        DecoratorManager { store }
    }

    fn set(&mut self, key: DecoratorKey, value: Decorator) {
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    fn get(&self, key: DecoratorKey) -> Option<Decorator> {
        let binding = self.store.lock().unwrap();
        let value = binding.get(&key);
        if value.is_none() {
            return None;
        }
        Some(value.unwrap().clone())
    }

    pub fn set_unary_decorator(&mut self, op: String, decorator: Arc<UnaryDecorator>) {
        let key = DecoratorKey::UNARY(op);
        let value = Decorator::UNARY(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_unary_decorator(&self, op: String) -> Arc<UnaryDecorator> {
        let key = DecoratorKey::UNARY(op);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_unary_decorator);
        }
        match v.unwrap() {
            Decorator::UNARY(f) => f.clone(),
            _ => Arc::new(default_unary_decorator),
        }
    }

    pub fn set_binary_decorator(&mut self, op: String, decorator: Arc<BinaryDecorator>) {
        let key = DecoratorKey::BINARY(op);
        let value = Decorator::BINARY(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_binary_decorator(&self, op: String) -> Arc<BinaryDecorator> {
        let key = DecoratorKey::UNARY(op);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_binary_decorator);
        }
        match v.unwrap() {
            Decorator::BINARY(f) => f.clone(),
            _ => Arc::new(default_binary_decorator),
        }
    }

    pub fn set_ternary_decorator(&mut self, decorator: Arc<TernaryDecorator>) {
        let key = DecoratorKey::TERNARY;
        let value = Decorator::TERNARY(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_ternary_decorator(&self) -> Arc<TernaryDecorator> {
        let key = DecoratorKey::TERNARY;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_ternary_decorator);
        }
        match v.unwrap() {
            Decorator::TERNARY(f) => f.clone(),
            _ => Arc::new(default_binary_decorator),
        }
    }

    pub fn set_function_decorator(&mut self, name: String, decorator: Arc<FunctionDecorator>) {
        let key = DecoratorKey::FUNCTION(name);
        let value = Decorator::FUNCTION(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_function_decorator(&self, name: String) -> Arc<FunctionDecorator> {
        let key = DecoratorKey::FUNCTION(name);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_function_decorator);
        }
        match v.unwrap() {
            Decorator::FUNCTION(f) => f.clone(),
            _ => Arc::new(default_function_decorator),
        }
    }

    pub fn set_reference_decorator(&mut self, name: String, decorator: Arc<ReferenceDecorator>) {
        let key = DecoratorKey::REFERENCE(name);
        let value = Decorator::REFERENCE(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_reference_decorator(&self, name: String) -> Arc<ReferenceDecorator> {
        let key = DecoratorKey::REFERENCE(name);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_reference_decorator);
        }
        match v.unwrap() {
            Decorator::REFERENCE(f) => f.clone(),
            _ => Arc::new(default_reference_decorator),
        }
    }

    pub fn set_list_decorator(&mut self, decorator: Arc<ListDecorator>) {
        let key = DecoratorKey::LIST;
        let value = Decorator::LIST(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_list_decorator(&self) -> Arc<ListDecorator> {
        let key = DecoratorKey::LIST;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_list_decorator);
        }
        match v.unwrap() {
            Decorator::LIST(f) => f.clone(),
            _ => Arc::new(default_list_decorator),
        }
    }

    pub fn set_map_decorator(&mut self, decorator: Arc<MapDecorator>) {
        let key = DecoratorKey::MAP;
        let value = Decorator::MAP(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_map_decorator(&self) -> Arc<MapDecorator> {
        let key = DecoratorKey::MAP;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_map_decorator);
        }
        match v.unwrap() {
            Decorator::MAP(f) => f.clone(),
            _ => Arc::new(default_map_decorator),
        }
    }

    pub fn set_chain_decorator(&mut self, decorator: Arc<ChainDecorator>) {
        let key = DecoratorKey::CHAIN;
        let value = Decorator::CHAIN(decorator);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_chain_decorator(&self) -> Arc<ChainDecorator> {
        let key = DecoratorKey::CHAIN;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_chain_decorator);
        }
        match v.unwrap() {
            Decorator::CHAIN(f) => f.clone(),
            _ => Arc::new(default_chain_decorator),
        }
    }
}

fn default_unary_decorator(op: String, rhs: String) -> String {
    op + &rhs
}

fn default_binary_decorator(op: String, lhs: String, rhs: String) -> String {
    lhs + &op + &rhs
}

fn default_ternary_decorator(condition: String, lhs: String, rhs: String) -> String {
    condition + "?" + &lhs + ":" + &rhs
}

fn default_function_decorator(name: String, params: Vec<String>) -> String {
    name + "(" + &params.join(",") + ")"
}

fn default_reference_decorator(name: String) -> String {
    name
}

fn default_list_decorator(params: Vec<String>) -> String {
    "[".to_string() + &params.join(",") + "]"
}

fn default_map_decorator(m: HashMap<String, String>) -> String {
    let mut tmp = Vec::new();
    for (k, v) in m {
        tmp.push(k + ":" + &v)
    }
    "{".to_string() + &tmp.join(",") + "}"
}

fn default_chain_decorator(params: Vec<String>) -> String {
    params.join(";")
}
