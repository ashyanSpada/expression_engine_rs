use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Hash, Eq, PartialEq)]
enum DescriptorKey {
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
enum Descriptor {
    UNARY(Arc<UnaryDescriptor>),
    BINARY(Arc<BinaryDescriptor>),
    TERNARY(Arc<TernaryDescriptor>),
    FUNCTION(Arc<FunctionDescriptor>),
    REFERENCE(Arc<ReferenceDescriptor>),
    LIST(Arc<ListDescriptor>),
    MAP(Arc<MapDescriptor>),
    CHAIN(Arc<ChainDescriptor>),
}

type UnaryDescriptor = dyn Fn(String, String) -> String + Send + Sync + 'static;
type BinaryDescriptor = dyn Fn(String, String, String) -> String + Send + Sync + 'static;
type TernaryDescriptor = dyn Fn(String, String, String) -> String + Send + Sync + 'static;
type FunctionDescriptor = dyn Fn(String, Vec<String>) -> String + Send + Sync + 'static;
type ReferenceDescriptor = dyn Fn(String) -> String + Send + Sync + 'static;
type ListDescriptor = dyn Fn(Vec<String>) -> String + Send + Sync + 'static;
type MapDescriptor = dyn Fn(HashMap<String, String>) -> String + Send + Sync + 'static;
type ChainDescriptor = dyn Fn(Vec<String>) -> String + Send + Sync + 'static;

pub struct DescriptorManager {
    store: &'static Mutex<HashMap<DescriptorKey, Descriptor>>,
}

impl DescriptorManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<DescriptorKey, Descriptor>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        DescriptorManager { store }
    }

    fn set(&mut self, key: DescriptorKey, value: Descriptor) {
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    fn get(&self, key: DescriptorKey) -> Option<Descriptor> {
        let binding = self.store.lock().unwrap();
        let value = binding.get(&key);
        if value.is_none() {
            return None;
        }
        Some(value.unwrap().clone())
    }

    pub fn set_unary_descriptor(&mut self, op: String, descriptor: Arc<UnaryDescriptor>) {
        let key = DescriptorKey::UNARY(op);
        let value = Descriptor::UNARY(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_unary_descriptor(&self, op: String) -> Arc<UnaryDescriptor> {
        let key = DescriptorKey::UNARY(op);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_unary_descriptor);
        }
        match v.unwrap() {
            Descriptor::UNARY(f) => f.clone(),
            _ => Arc::new(default_unary_descriptor),
        }
    }

    pub fn set_binary_descriptor(&mut self, op: String, descriptor: Arc<BinaryDescriptor>) {
        let key = DescriptorKey::BINARY(op);
        let value = Descriptor::BINARY(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_binary_descriptor(&self, op: String) -> Arc<BinaryDescriptor> {
        let key = DescriptorKey::UNARY(op);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_binary_descriptor);
        }
        match v.unwrap() {
            Descriptor::BINARY(f) => f.clone(),
            _ => Arc::new(default_binary_descriptor),
        }
    }

    pub fn set_ternary_descriptor(&mut self, descriptor: Arc<TernaryDescriptor>) {
        let key = DescriptorKey::TERNARY;
        let value = Descriptor::TERNARY(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_ternary_descriptor(&self) -> Arc<TernaryDescriptor> {
        let key = DescriptorKey::TERNARY;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_ternary_descriptor);
        }
        match v.unwrap() {
            Descriptor::TERNARY(f) => f.clone(),
            _ => Arc::new(default_binary_descriptor),
        }
    }

    pub fn set_function_descriptor(&mut self, name: String, descriptor: Arc<FunctionDescriptor>) {
        let key = DescriptorKey::FUNCTION(name);
        let value = Descriptor::FUNCTION(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_function_descriptor(&self, name: String) -> Arc<FunctionDescriptor> {
        let key = DescriptorKey::FUNCTION(name);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_function_descriptor);
        }
        match v.unwrap() {
            Descriptor::FUNCTION(f) => f.clone(),
            _ => Arc::new(default_function_descriptor),
        }
    }

    pub fn set_reference_descriptor(&mut self, name: String, descriptor: Arc<ReferenceDescriptor>) {
        let key = DescriptorKey::REFERENCE(name);
        let value = Descriptor::REFERENCE(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_reference_descriptor(&self, name: String) -> Arc<ReferenceDescriptor> {
        let key = DescriptorKey::REFERENCE(name);
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_reference_descriptor);
        }
        match v.unwrap() {
            Descriptor::REFERENCE(f) => f.clone(),
            _ => Arc::new(default_reference_descriptor),
        }
    }

    pub fn set_list_descriptor(&mut self, descriptor: Arc<ListDescriptor>) {
        let key = DescriptorKey::LIST;
        let value = Descriptor::LIST(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_list_descriptor(&self) -> Arc<ListDescriptor> {
        let key = DescriptorKey::LIST;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_list_descriptor);
        }
        match v.unwrap() {
            Descriptor::LIST(f) => f.clone(),
            _ => Arc::new(default_list_descriptor),
        }
    }

    pub fn set_map_descriptor(&mut self, descriptor: Arc<MapDescriptor>) {
        let key = DescriptorKey::MAP;
        let value = Descriptor::MAP(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_map_descriptor(&self) -> Arc<MapDescriptor> {
        let key = DescriptorKey::MAP;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_map_descriptor);
        }
        match v.unwrap() {
            Descriptor::MAP(f) => f.clone(),
            _ => Arc::new(default_map_descriptor),
        }
    }

    pub fn set_chain_descriptor(&mut self, descriptor: Arc<ChainDescriptor>) {
        let key = DescriptorKey::CHAIN;
        let value = Descriptor::CHAIN(descriptor);
        let mut binding = self.store.lock().unwrap();
        binding.insert(key, value);
    }

    pub fn get_chain_descriptor(&self) -> Arc<ChainDescriptor> {
        let key = DescriptorKey::CHAIN;
        let v = self.get(key);
        if v.is_none() {
            return Arc::new(default_chain_descriptor);
        }
        match v.unwrap() {
            Descriptor::CHAIN(f) => f.clone(),
            _ => Arc::new(default_chain_descriptor),
        }
    }
}

fn default_unary_descriptor(op: String, rhs: String) -> String {
    op + &rhs
}

fn default_binary_descriptor(op: String, lhs: String, rhs: String) -> String {
    lhs + &op + &rhs
}

fn default_ternary_descriptor(condition: String, lhs: String, rhs: String) -> String {
    condition + "?" + &lhs + ":" + &rhs
}

fn default_function_descriptor(name: String, params: Vec<String>) -> String {
    name + "(" + &params.join(",") + ")"
}

fn default_reference_descriptor(name: String) -> String {
    name
}

fn default_list_descriptor(params: Vec<String>) -> String {
    "[".to_string() + &params.join(",") + "]"
}

fn default_map_descriptor(m: HashMap<String, String>) -> String {
    let mut tmp = Vec::new();
    for (k, v) in m {
        tmp.push(k + ":" + &v)
    }
    "{".to_string() + &tmp.join(",") + "}"
}

fn default_chain_descriptor(params: Vec<String>) -> String {
    params.join(";")
}
