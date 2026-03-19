use crate::define::Result;
use crate::function::InnerFunction;
use crate::value::Value;
use core::clone::Clone;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum ContextValue {
    Variable(Value),
    Function(Arc<InnerFunction>),
}

pub struct Context(pub Arc<Mutex<HashMap<String, ContextValue>>>);

impl Context {
    pub fn new() -> Self {
        Context(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn set_func(&mut self, name: &str, func: Arc<InnerFunction>) {
        self.set(name, ContextValue::Function(func.clone()));
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.set(name, ContextValue::Variable(value));
    }

    pub fn set(&mut self, name: &str, v: ContextValue) {
        self.0.lock().unwrap().insert(name.to_string(), v);
    }

    pub fn get_func(&self, name: &str) -> Option<Arc<InnerFunction>> {
        let value = self.get(name)?;
        match value {
            ContextValue::Function(func) => Some(func),
            ContextValue::Variable(_) => None,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        let value = self.get(name)?;
        match value {
            ContextValue::Variable(v) => Some(v),
            ContextValue::Function(_) => None,
        }
    }

    pub fn get(&self, name: &str) -> Option<ContextValue> {
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn value(&self, name: &str) -> Result<Value> {
        match self.get(name) {
            Some(ContextValue::Variable(v)) => Ok(v),
            Some(ContextValue::Function(func)) => func(Vec::new()),
            None => Ok(Value::None),
        }
    }
}

///
///```rust
/// use expression_engine::create_context;
/// use expression_engine::Value;
/// let a = create_context!("d" => 3.5, "c" => Arc::new(|params| {
///    Ok(Value::from(3))
/// }));
///```
///
///
#[macro_export]
macro_rules! create_context {
    (($ctx:expr) $k:expr => Arc::new($($v:tt)*), $($tt:tt)*) => {{
        $ctx.set_func($k, Arc::new($($v)*));
        $crate::create_context!(($ctx) $($tt)*);
    }};

    (($ctx:expr) $k:expr => $v:expr, $($tt:tt)*) => {{
        $ctx.set_variable($k, Value::from($v));
        $crate::create_context!(($ctx) $($tt)*);
    }};

    (($ctx:expr) $k:expr => Arc::new($($v:tt)*)) => {{
        $ctx.set_func($k, Arc::new($($v)*));
    }};

    (($ctx:expr) $k:expr => $v:expr) => {{
        $ctx.set_variable($k, Value::from($v));
    }};

    (($ctx:expr)) => {};

    ($($tt:tt)*) => {{
        use std::sync::Arc;
        let mut ctx = $crate::Context::new();
        $crate::create_context!((&mut ctx) $($tt)*);
        ctx
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_context_methods() {
        let mut ctx = Context::new();
        ctx.set_variable("var1", Value::from(42));
        ctx.set_func("func1", Arc::new(|_| Ok(Value::from(100))));

        // Test get_variable
        assert_eq!(ctx.get_variable("var1").unwrap(), Value::from(42));
        assert!(ctx.get_variable("func1").is_none());
        assert!(ctx.get_variable("not_found").is_none());

        // Test get_func
        assert!(ctx.get_func("func1").is_some());
        assert!(ctx.get_func("var1").is_none());
        assert!(ctx.get_func("not_found").is_none());

        // Test value
        assert_eq!(ctx.value("var1").unwrap(), Value::from(42));
        assert_eq!(ctx.value("func1").unwrap(), Value::from(100));
        assert_eq!(ctx.value("not_found").unwrap(), Value::None);
    }
}
