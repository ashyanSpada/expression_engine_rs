use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};

type BinaryOpFunc = dyn Fn(Value, Value) -> Result<Value> + Send + Sync + 'static;

type UnaryOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

type PostfixOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

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

pub struct PostfixOpFuncManager {
    store: &'static Mutex<HashMap<String, Arc<PostfixOpFunc>>>,
}

impl BinaryOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, (i32, BinOpType, Arc<BinaryOpFunc>)>>> =
            OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        BinaryOpFuncManager { store: store }
    }

    pub fn init(&mut self) {
        use BinOpType::*;
        self.register("=", 20, SETTER, Arc::new(|left, right| Ok(right)));

        for op in vec!["+=", "-=", "*=", "/=", "%="] {
            self.register(
                op,
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
            );
        }

        for op in vec!["<<=", ">>=", "&=", "^=", "|="] {
            self.register(
                op,
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
            );
        }

        for (op, precedence) in vec![("||", 40), ("&&", 50)] {
            self.register(
                op,
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
            );
        }

        for op in vec!["<", "<=", ">", ">="] {
            self.register(
                op,
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
            );
        }

        for op in vec!["==", "!="] {
            self.register(
                op,
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
            );
        }

        for (op, precedence) in vec![("|", 70), ("^", 80), ("&", 90), ("<<", 100), (">>", 100)] {
            self.register(
                op,
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
            );
        }

        for (op, precedence) in vec![("+", 110), ("-", 110), ("*", 120), ("/", 120), ("%", 120)] {
            self.register(
                op,
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
            );
        }

        self.register(
            "beginWith",
            200,
            BinOpType::CALC,
            Arc::new(|left, right| {
                let (a, b) = (left.string()?, right.string()?);
                Ok(Value::from(a.starts_with(&b)))
            }),
        );

        self.register(
            "endWith",
            200,
            BinOpType::CALC,
            Arc::new(|left, right| {
                let (a, b) = (left.string()?, right.string()?);
                Ok(Value::from(a.ends_with(&b)))
            }),
        );

        self.register(
            "in",
            200,
            BinOpType::CALC,
            Arc::new(|left, right| {
                let list = right.list()?;
                for item in list {
                    if item == left {
                        return Ok(true.into());
                    }
                }
                Ok(false.into())
            }),
        )
    }

    pub fn register(
        &mut self,
        op: &str,
        precidence: i32,
        op_type: BinOpType,
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

    pub fn exist(&self, op: &str) -> bool {
        let binding = self.store.lock().unwrap();
        binding.get(op).is_some()
    }
}

impl UnaryOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<UnaryOpFunc>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        UnaryOpFuncManager { store: store }
    }

    pub fn init(&mut self) {
        self.register(
            "-",
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a,
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(-a))
            }),
        );

        self.register(
            "+",
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a,
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(a))
            }),
        );

        self.register(
            "!",
            Arc::new(|param| {
                let a = match param {
                    Value::Bool(value) => !value,
                    _ => return Err(Error::ShouldBeBool()),
                };
                Ok(Value::Bool(a))
            }),
        );

        self.register(
            "not",
            Arc::new(|param| {
                let a = match param {
                    Value::Bool(value) => !value,
                    _ => return Err(Error::ShouldBeBool()),
                };
                Ok(Value::Bool(a))
            }),
        );

        self.register(
            "AND",
            Arc::new(|value| {
                let list = value.list()?;
                for value in list {
                    if !value.bool()? {
                        return Ok(false.into());
                    }
                }
                Ok(true.into())
            }),
        );

        self.register(
            "OR",
            Arc::new(|value| {
                let list = value.list()?;
                for value in list {
                    if value.bool()? {
                        return Ok(true.into());
                    }
                }
                Ok(false.into())
            }),
        );
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

    pub fn exist(&self, op: &str) -> bool {
        let binding = self.store.lock().unwrap();
        binding.get(op).is_some()
    }
}

impl PostfixOpFuncManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<UnaryOpFunc>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        Self { store: store }
    }

    pub fn init(&mut self) {
        self.register(
            "++",
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a + Decimal::from_i32(1).unwrap(),
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(a))
            }),
        );

        self.register(
            "--",
            Arc::new(|param| {
                let a = match param {
                    Value::Number(a) => a - Decimal::from_i32(1).unwrap(),
                    _ => return Err(Error::ShouldBeNumber()),
                };
                Ok(Value::Number(a))
            }),
        );
    }

    pub fn register(&mut self, op: &str, f: Arc<PostfixOpFunc>) {
        self.store.lock().unwrap().insert(op.to_string(), f);
    }

    pub fn get(&self, op: &str) -> Result<Arc<PostfixOpFunc>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::UnaryOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().clone())
    }

    pub fn exist(&self, op: &str) -> bool {
        let binding = self.store.lock().unwrap();
        binding.get(op).is_some()
    }
}

#[test]
fn test_operators() {
    let result = BinaryOpFuncManager::new().operators();
    for (op, precedence) in result {
        println!("|{}| {}||", op, precedence)
    }
}
