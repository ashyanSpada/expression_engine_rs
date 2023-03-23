use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type BinaryOpFunc = dyn Fn(Value, Value) -> Result<Value> + Send + Sync + 'static;

type UnaryOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

#[derive(Clone)]
pub enum BinOpType {
    CALC,
    SETTER,
}

pub struct BinaryOpFuncManager {
    store: &'static Mutex<HashMap<String, (i32, BinOpType, Arc<BinaryOpFunc>)>>,
}

pub struct UnaryOpFuncManager {
    store: &'static Mutex<HashMap<String, Arc<UnaryOpFunc>>>,
}

impl BinaryOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, (i32, BinOpType, Arc<BinaryOpFunc>)>>> =
            OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(Self::internal_register(HashMap::new())));
        BinaryOpFuncManager { store: store }
    }

    fn internal_register(
        mut m: HashMap<String, (i32, BinOpType, Arc<BinaryOpFunc>)>,
    ) -> HashMap<String, (i32, BinOpType, Arc<BinaryOpFunc>)> {
        use BinOpType::*;
        m.insert(
            "=".to_string(),
            (20, SETTER, Arc::new(|left, right| Ok(right))),
        );

        for op in vec!["+=", "-=", "*=", "/=", "%="] {
            m.insert(
                op.to_string(),
                (
                    20,
                    SETTER,
                    Arc::new(move |left, right| {
                        let (mut a, b) = (left.decimal()?, right.decimal()?);
                        match op {
                            "+=" => a += b,
                            "-=" => a -= b,
                            "*=" => a *= b,
                            "/=" => a /= b,
                            "%=" => a %= b,
                            _ => (),
                        }
                        Ok(Value::Number(a))
                    }),
                ),
            );
        }

        for op in vec!["<<=", ">>=", "&=", "^=", "|="] {
            m.insert(
                op.to_string(),
                (
                    20,
                    SETTER,
                    Arc::new(move |left, right| {
                        let (mut a, b) = (left.integer()?, right.integer()?);
                        match op {
                            "<<=" => a <<= b,
                            ">>=" => a >>= b,
                            "&=" => a &= b,
                            "^=" => a ^= b,
                            "|=" => a |= b,
                            _ => (),
                        }
                        Ok(Value::from(a))
                    }),
                ),
            );
        }

        for (op, precedence) in vec![("||", 40), ("&&", 50)] {
            m.insert(
                op.to_string(),
                (
                    precedence,
                    CALC,
                    Arc::new(move |left, right| {
                        let (mut a, b) = (left.bool()?, right.bool()?);
                        match op {
                            "||" => a = a || b,
                            "&&" => a = a && b,
                            _ => (),
                        }
                        Ok(Value::from(a))
                    }),
                ),
            );
        }

        for op in vec!["<", "<=", ">", ">="] {
            m.insert(
                op.to_string(),
                (
                    60,
                    CALC,
                    Arc::new(move |left, right| {
                        let (a, b) = (left.decimal()?, right.decimal()?);
                        let mut value = false;
                        match op {
                            "<" => value = a < b,
                            "<=" => value = a <= b,
                            ">" => value = a > b,
                            ">=" => value = a >= b,
                            _ => (),
                        }
                        Ok(Value::from(value))
                    }),
                ),
            );
        }

        for op in vec!["==", "!="] {
            m.insert(
                op.to_string(),
                (
                    60,
                    CALC,
                    Arc::new(move |left, right| {
                        let mut value = false;
                        match op {
                            "==" => value = left == right,
                            "!=" => value = left != right,
                            _ => (),
                        }
                        Ok(Value::from(value))
                    }),
                ),
            );
        }

        for (op, precedence) in vec![("|", 70), ("^", 80), ("&", 90), ("<<", 100), (">>", 100)] {
            m.insert(
                op.to_string(),
                (
                    precedence,
                    CALC,
                    Arc::new(move |left, right| {
                        let (mut a, b) = (left.integer()?, right.integer()?);
                        match op {
                            "|" => a |= b,
                            "^" => a ^= b,
                            "&" => a &= b,
                            "<<" => a <<= b,
                            ">>" => a >>= b,
                            _ => (),
                        }
                        Ok(Value::from(a))
                    }),
                ),
            );
        }

        for (op, precedence) in vec![("+", 110), ("-", 110), ("*", 120), ("/", 120), ("%", 120)] {
            m.insert(
                op.to_string(),
                (
                    precedence,
                    CALC,
                    Arc::new(move |left, right| {
                        let (mut a, b) = (left.decimal()?, right.decimal()?);
                        match op {
                            "+" => a += b,
                            "-" => a -= b,
                            "*" => a *= b,
                            "/" => a /= b,
                            "%" => a %= b,
                            _ => (),
                        }
                        Ok(Value::from(a))
                    }),
                ),
            );
        }

        m.insert(
            "beginWith".to_string(),
            (
                200,
                BinOpType::CALC,
                Arc::new(|left, right| {
                    let (a, b) = (left.string()?, right.string()?);
                    Ok(Value::from(a.starts_with(&b)))
                }),
            ),
        );

        m.insert(
            "endWith".to_string(),
            (
                200,
                BinOpType::CALC,
                Arc::new(|left, right| {
                    let (a, b) = (left.string()?, right.string()?);
                    Ok(Value::from(a.ends_with(&b)))
                }),
            ),
        );

        m
    }

    pub fn register(
        &mut self,
        op: &str,
        op_type: BinOpType,
        precidence: i32,
        f: Arc<BinaryOpFunc>,
    ) {
        self.store
            .lock()
            .unwrap()
            .insert(op.to_string(), (precidence, op_type, f));
    }

    pub fn get(&self, op: &str) -> Result<Arc<BinaryOpFunc>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::BinaryOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().2.clone())
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

    pub fn get_op_type(&self, op: &str) -> Result<BinOpType> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::BinaryOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().1.clone())
    }

    pub fn operators(&self) -> Vec<(String, i32)> {
        let mut ans = vec![];
        let binding = self.store.lock().unwrap();
        for (op, (precedence, _, _)) in binding.iter() {
            ans.push((op.clone(), precedence.clone()));
        }
        ans.sort_by(|a, b| a.1.cmp(&b.1));
        ans
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

#[test]
fn test_operators() {
    let result = BinaryOpFuncManager::new().operators();
    for (op, precedence) in result {
        println!("|{}| {}||", op, precedence)
    }
}
