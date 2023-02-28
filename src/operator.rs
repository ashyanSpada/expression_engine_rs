use crate::define::Result;
use crate::error::Error;
use std::collections::HashMap;
use std::sync::Arc;
use crate::value::Value;

type BinaryOpFunc = dyn Fn(Value, Value) -> Result<Value> + Send + 'static;

type UnaryOpFunc = dyn Fn(Value) -> Result<Value> + Send + 'static;

pub struct BinaryOpFuncManager {
    store: HashMap<String, (i32, Arc<BinaryOpFunc>)>,
}

pub struct UnaryOpFuncManager {
    store: HashMap<String, Arc<UnaryOpFunc>>,
}

impl BinaryOpFuncManager {
    pub fn new() -> Arc<Self> {
        static mut BINARY_OP_FUNC_MANAGER: Option<Arc<BinaryOpFuncManager>> = None;
        unsafe {
            match &BINARY_OP_FUNC_MANAGER {
                Some(m) => m.clone(),
                None => BINARY_OP_FUNC_MANAGER
                    .get_or_insert(Arc::new(BinaryOpFuncManager {
                        store: Self::internal_register(HashMap::new()),
                    }))
                    .clone(),
            }
        }
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
        self.store.insert(op, (precidence, f));
    }

    pub fn get(&self, op: String) -> Result<Arc<BinaryOpFunc>> {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return Err(Error::BinaryOpNotRegistered(op));
        }
        Ok(ans.unwrap().1.clone())
    }

    pub fn get_precidence(&self, op: String) -> i32 {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return -1;
        }
        ans.unwrap().0
    }

    pub fn redirect(&mut self, source: String, target: String) {
        self.store
            .insert(source, self.store.get(&target).unwrap().clone());
    }
}

impl UnaryOpFuncManager {
    pub fn new() -> Arc<Self> {
        static mut UNARY_OP_FUNC_MANAGER: Option<Arc<UnaryOpFuncManager>> = None;
        unsafe {
            match &UNARY_OP_FUNC_MANAGER {
                Some(m) => m.clone(),
                None => UNARY_OP_FUNC_MANAGER
                    .get_or_insert(Arc::new(UnaryOpFuncManager {
                        store: Self::internal_register(HashMap::new()),
                    }))
                    .clone(),
            }
        }
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
                    Value::Bool(a) => a,
                    _ => return Err(Error::ShouldBeBool()),
                };
                Ok(Value::Bool(a))
            }),
        );

        m
    }

    pub fn register(&mut self, op: String, f: Arc<UnaryOpFunc>) {
        self.store.insert(op, f);
    }

    pub fn get(&self, op: String) -> Result<Arc<UnaryOpFunc>> {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return Err(Error::UnaryOpNotRegistered(op));
        }
        Ok(ans.unwrap().clone())
    }
}

#[test]
fn print_op() {
    let mut list = Vec::new();
    let m = BinaryOpFuncManager::new();
    for (key, (precedence, _)) in &m.store {
        list.push((key, precedence));
    }
    list.sort_by(|a, b| a.1.cmp(b.1));
    for (key, precedence) in list {
        print!("op:{}-precedence:{}\n", key, precedence);
    }
}
