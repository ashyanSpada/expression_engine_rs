use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type BinaryOpFunc = dyn Fn(Value, Value) -> Result<Value> + Send + Sync + 'static;

type UnaryOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

pub struct BinaryOpFuncManager {
    store: &'static Mutex<HashMap<String, (i32, Arc<BinaryOpFunc>)>>,
}

pub struct UnaryOpFuncManager {
    store: &'static Mutex<HashMap<String, Arc<UnaryOpFunc>>>,
}

impl BinaryOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, (i32, Arc<BinaryOpFunc>)>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(Self::internal_register(HashMap::new())));
        BinaryOpFuncManager { store: store }
    }

    fn internal_register(
        mut m: HashMap<String, (i32, Arc<BinaryOpFunc>)>,
    ) -> HashMap<String, (i32, Arc<BinaryOpFunc>)> {
        m.insert(
            "+".to_string(),
            (
                60,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Number(a + b))
                }),
            ),
        );

        m.insert(
            "-".to_string(),
            (
                60,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Number(a - b))
                }),
            ),
        );

        m.insert(
            "in".to_string(),
            (
                100,
                Arc::new(|left, right| match right {
                    Value::List(params) => {
                        for target in params {
                            if target == left {
                                return Ok(Value::Bool(true));
                            }
                        }
                        return Ok(Value::Bool(false));
                    }
                    Value::Map(params) => {
                        for (target, _) in params {
                            if target == left {
                                return Ok(Value::Bool(true));
                            }
                        }
                        return Ok(Value::Bool(false));
                    }
                    _ => Err(Error::ParamInvalid()),
                }),
            ),
        );

        m.insert(
            "beginWith".to_string(),
            (
                120,
                Arc::new(|left, right| match left {
                    Value::String(s) => match right {
                        Value::String(t) => {
                            if s.starts_with(&t) {
                                return Ok(Value::Bool(true));
                            }
                            return Ok(Value::Bool(false));
                        }
                        _ => Err(Error::ShouldBeString()),
                    },
                    _ => Err(Error::ShouldBeString()),
                }),
            ),
        );

        m.insert(
            "endWith".to_string(),
            (
                120,
                Arc::new(|left, right| match left {
                    Value::String(s) => match right {
                        Value::String(t) => {
                            if s.ends_with(&t) {
                                return Ok(Value::Bool(true));
                            }
                            return Ok(Value::Bool(false));
                        }
                        _ => Err(Error::ShouldBeString()),
                    },
                    _ => Err(Error::ShouldBeString()),
                }),
            ),
        );

        m.insert(
            "*".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Number(a * b))
                }),
            ),
        );

        m.insert(
            "/".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Number(a / b))
                }),
            ),
        );

        m.insert(
            "%".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Number(a + b))
                }),
            ),
        );

        m.insert(
            "&&".to_string(),
            (
                40,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Bool(a) => a,
                        _ => return Err(Error::ShouldBeBool()),
                    };
                    let b = match right {
                        Value::Bool(b) => b,
                        _ => return Err(Error::ShouldBeBool()),
                    };
                    Ok(Value::Bool(a && b))
                }),
            ),
        );

        m.insert(
            "||".to_string(),
            (
                40,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Bool(a) => a,
                        _ => return Err(Error::ShouldBeBool()),
                    };
                    let b = match right {
                        Value::Bool(b) => b,
                        _ => return Err(Error::ShouldBeBool()),
                    };
                    Ok(Value::Bool(a || b))
                }),
            ),
        );

        m.insert(
            "==".to_string(),
            (20, Arc::new(|left, right| Ok(Value::Bool(left == right)))),
        );

        m.insert(
            "!=".to_string(),
            (20, Arc::new(|left, right| Ok(Value::Bool(left != right)))),
        );

        m.insert(
            ">".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Bool(a > b))
                }),
            ),
        );

        m.insert(
            ">=".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Bool(a >= b))
                }),
            ),
        );

        m.insert(
            "<".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Bool(a < b))
                }),
            ),
        );

        m.insert(
            "<=".to_string(),
            (
                80,
                Arc::new(|left, right| {
                    let a = match left {
                        Value::Number(a) => a,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    let b = match right {
                        Value::Number(b) => b,
                        _ => return Err(Error::ShouldBeNumber()),
                    };
                    Ok(Value::Bool(a <= b))
                }),
            ),
        );

        m
    }

    pub fn register(&mut self, op: String, precidence: i32, f: Arc<BinaryOpFunc>) {
        self.store.lock().unwrap().insert(op, (precidence, f));
    }

    pub fn get(&self, op: &str) -> Result<Arc<BinaryOpFunc>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::BinaryOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().1.clone())
    }

    pub fn get_precidence(&self, op: &str) -> i32 {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return -1;
        }
        ans.unwrap().0
    }

    pub fn redirect(&mut self, source: &str, target: &str) {
        let func = self.store.lock().unwrap().get(target).unwrap().clone();
        let mut binding = self.store.lock().unwrap();
        binding.insert(source.to_string(), func);
    }
}

impl UnaryOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<UnaryOpFunc>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(Self::internal_register(HashMap::new())));
        UnaryOpFuncManager { store: store }
    }

    fn internal_register(
        mut m: HashMap<String, Arc<UnaryOpFunc>>,
    ) -> HashMap<String, Arc<UnaryOpFunc>> {
        m.insert(
            "-".to_string(),
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a,
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(-a))
            }),
        );

        m.insert(
            "+".to_string(),
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a,
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(a))
            }),
        );

        m.insert(
            "!".to_string(),
            Arc::new(|param| {
                let a = match param {
                    Value::Bool(value) => !value,
                    _ => return Err(Error::ShouldBeBool()),
                };
                Ok(Value::Bool(a))
            }),
        );

        m
    }

    pub fn register(&mut self, op: &str, f: Arc<UnaryOpFunc>) {
        self.store.lock().unwrap().insert(op.to_string(), f);
    }

    pub fn get(&self, op: &str) -> Result<Arc<UnaryOpFunc>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::UnaryOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().clone())
    }
}
