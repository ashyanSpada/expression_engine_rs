use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type InfixOpFunc = dyn Fn(Value, Value) -> Result<Value> + Send + Sync + 'static;

pub type PrefixOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

pub type PostfixOpFunc = dyn Fn(Value) -> Result<Value> + Send + Sync + 'static;

#[derive(Clone)]
pub enum InfixOpType {
    CALC,
    SETTER,
}

#[derive(Clone, PartialEq)]
pub enum InfixOpAssociativity {
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct InfixOpConfig(
    pub i32,
    pub InfixOpType,
    pub InfixOpAssociativity,
    pub Arc<InfixOpFunc>,
);

pub struct InfixOpManager {
    store: &'static Mutex<HashMap<String, InfixOpConfig>>,
}

pub struct PrefixOpManager {
    store: &'static Mutex<HashMap<String, Arc<PrefixOpFunc>>>,
}

pub struct PostfixOpManager {
    store: &'static Mutex<HashMap<String, Arc<PostfixOpFunc>>>,
}

impl InfixOpManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, InfixOpConfig>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        InfixOpManager { store: store }
    }

    pub fn init(&mut self) {
        use InfixOpAssociativity::*;
        use InfixOpType::*;
        self.register("=", 20, SETTER, RIGHT, Arc::new(|_, right| Ok(right)));

        for op in vec!["+=", "-=", "*=", "/=", "%="] {
            self.register(
                op,
                20,
                SETTER,
                RIGHT,
                Arc::new(move |left, right| {
                    let (mut a, b) = (left.decimal()?, right.decimal()?);
                    match op {
                        "+=" => a += b,
                        "-=" => a -= b,
                        "*=" => a *= b,
                        "/=" => {
                            if b.is_zero() {
                                return Err(Error::DivByZero);
                            }
                            a /= b
                        }
                        "%=" => {
                            if b.is_zero() {
                                return Err(Error::DivByZero);
                            }
                            a %= b
                        }
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
                RIGHT,
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
                LEFT,
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
                LEFT,
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
                LEFT,
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
                LEFT,
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
                LEFT,
                Arc::new(move |left, right| {
                    let (mut a, b) = (left.decimal()?, right.decimal()?);
                    match op {
                        "+" => a += b,
                        "-" => a -= b,
                        "*" => a *= b,
                        "/" => {
                            if b.is_zero() {
                                return Err(Error::DivByZero);
                            }
                            a /= b
                        }
                        "%" => {
                            if b.is_zero() {
                                return Err(Error::DivByZero);
                            }
                            a %= b
                        }
                        _ => (),
                    }
                    Ok(Value::from(a))
                }),
            );
        }

        self.register(
            "beginWith",
            200,
            CALC,
            LEFT,
            Arc::new(|left, right| {
                let (a, b) = (left.string()?, right.string()?);
                Ok(Value::from(a.starts_with(&b)))
            }),
        );

        self.register(
            "endWith",
            200,
            CALC,
            LEFT,
            Arc::new(|left, right| {
                let (a, b) = (left.string()?, right.string()?);
                Ok(Value::from(a.ends_with(&b)))
            }),
        );

        self.register(
            "in",
            200,
            InfixOpType::CALC,
            InfixOpAssociativity::LEFT,
            Arc::new(|left, right| {
                let list = right.list()?;
                for item in list {
                    if item == left {
                        return Ok(true.into());
                    }
                }
                Ok(false.into())
            }),
        );
    }

    pub fn register(
        &mut self,
        op: &str,
        precidence: i32,
        op_type: InfixOpType,
        op_associativity: InfixOpAssociativity,
        f: Arc<InfixOpFunc>,
    ) {
        self.store.lock().unwrap().insert(
            op.to_string(),
            InfixOpConfig(precidence, op_type, op_associativity, f),
        );
    }

    pub fn get_handler(&self, op: &str) -> Result<Arc<InfixOpFunc>> {
        Ok(self.get(op)?.3)
    }

    pub fn get_precidence(&self, op: &str) -> (i32, i32) {
        let ans = self.get(op);
        if ans.is_err() {
            return (-1, -1);
        }
        let config = ans.unwrap();
        let l_bp = config.0;
        let mut r_bp = 0;
        if config.2 == InfixOpAssociativity::LEFT {
            r_bp = l_bp + 1;
        } else if config.2 == InfixOpAssociativity::RIGHT {
            r_bp = l_bp - 1;
        }
        (l_bp, r_bp)
    }

    pub fn get_op_type(&self, op: &str) -> Result<InfixOpType> {
        Ok(self.get(op)?.1)
    }

    pub fn get(&self, op: &str) -> Result<InfixOpConfig> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::InfixOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().clone())
    }

    pub fn operators(&self) -> Vec<(String, i32)> {
        let mut ans = vec![];
        let binding = self.store.lock().unwrap();
        for (op, InfixOpConfig(precedence, _, _, _)) in binding.iter() {
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

impl PrefixOpManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<PrefixOpFunc>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        PrefixOpManager { store: store }
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

    pub fn register(&mut self, op: &str, f: Arc<PrefixOpFunc>) {
        self.store.lock().unwrap().insert(op.to_string(), f);
    }

    pub fn get(&self, op: &str) -> Result<Arc<PrefixOpFunc>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(op);
        if ans.is_none() {
            return Err(Error::PrefixOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().clone())
    }

    pub fn exist(&self, op: &str) -> bool {
        let binding = self.store.lock().unwrap();
        binding.get(op).is_some()
    }
}

impl PostfixOpManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<PrefixOpFunc>>>> = OnceCell::new();
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
            return Err(Error::PrefixOpNotRegistered(op.to_string()));
        }
        Ok(ans.unwrap().clone())
    }

    pub fn exist(&self, op: &str) -> bool {
        let binding = self.store.lock().unwrap();
        binding.get(op).is_some()
    }
}

#[cfg(test)]
mod tetst {
    use crate::operator::InfixOpManager;
    #[test]
    fn test_operators() {
        let result = InfixOpManager::new().operators();
        for (op, precedence) in result {
            println!("|{}| {}||", op, precedence)
        }
    }
}
