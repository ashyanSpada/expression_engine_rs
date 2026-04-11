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

#[derive(Clone)]
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
        let binding = self.0.lock().unwrap();
        match binding.get(name)? {
            ContextValue::Function(func) => Some(func.clone()),
            ContextValue::Variable(_) => None,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        let binding = self.0.lock().unwrap();
        match binding.get(name)? {
            ContextValue::Variable(v) => Some(v.clone()),
            ContextValue::Function(_) => None,
        }
    }

    pub fn get(&self, name: &str) -> Option<ContextValue> {
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn value(&self, name: &str) -> Result<Value> {
        let func = {
            let binding = self.0.lock().unwrap();
            match binding.get(name) {
                Some(ContextValue::Variable(v)) => return Ok(v.clone()),
                Some(ContextValue::Function(func)) => func.clone(),
                None => return Ok(Value::None),
            }
        };
        func(Vec::new())
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
    use crate::context::Context;
    use crate::value::Value;
    use std::sync::Arc;

    #[test]
    fn test_context() {
        let mut ctx = Context::new();

        ctx.set_variable("a", Value::from(10));
        ctx.set_func("f", Arc::new(|_| Ok(Value::from(20))));

        assert_eq!(ctx.get_variable("a").unwrap(), Value::from(10));
        assert!(ctx.get_variable("f").is_none());
        assert!(ctx.get_variable("missing").is_none());

        assert!(ctx.get_func("f").is_some());
        assert!(ctx.get_func("a").is_none());
        assert!(ctx.get_func("missing").is_none());

        assert_eq!(ctx.value("a").unwrap(), Value::from(10));
        assert_eq!(ctx.value("f").unwrap(), Value::from(20));
        assert_eq!(ctx.value("missing").unwrap(), Value::None);
    }
}
