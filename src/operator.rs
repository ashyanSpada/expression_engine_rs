use crate::define::{Param, Result};
use crate::error::Error;
use std::collections::HashMap;
use std::sync::Arc;

// static BINARY_OP_FUNC_MANAGER: Mutex<Option<BinaryOpFuncManager>> = Mutex::new(None);
// static UNARY_OP_FUNC_MANAGER: Mutex<Option<UnaryOpFuncManager>> = Mutex::new(None);


type BinaryOpFunc = dyn Fn(Param, Param) -> Result<Param> + Send + 'static;

type UnaryOpFunc = dyn Fn(Param) -> Result<Param> + Send + 'static;

pub struct BinaryOpFuncManager {
    store: HashMap<String, (i32, Arc<BinaryOpFunc>)>
}

pub struct UnaryOpFuncManager {
    store: HashMap<String, Arc<UnaryOpFunc>>
}

impl BinaryOpFuncManager {
    pub fn new() -> Arc<Self> {
        static mut BINARY_OP_FUNC_MANAGER: Option<Arc<BinaryOpFuncManager>> = None;
        unsafe {
            match &BINARY_OP_FUNC_MANAGER {
                Some(m) => m.clone(), 
                None => {
                    BINARY_OP_FUNC_MANAGER.get_or_insert(Arc::new(BinaryOpFuncManager { store: Self::internal_register(HashMap::new()) })).clone()
                }
            }
        }
    }
    
    fn internal_register(mut m: HashMap<String, (i32, Arc<BinaryOpFunc>)>) -> HashMap<String, (i32, Arc<BinaryOpFunc>)> {
        m.insert("+".to_string(), (60, Arc::new(|left, right| {
            let a = match left {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Literal(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a + b))
        })));

        m.insert("-".to_string(), (60, Arc::new(|left, right| {
            let a = match left {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Literal(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a - b))
        })));

        m.insert("*".to_string(), (80, Arc::new(|left, right| {
            let a = match left {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Literal(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a * b))
        })));

        m.insert("/".to_string(), (80, Arc::new(|left, right| {
            let a = match left {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Literal(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a / b))
        })));

        m.insert("%".to_string(), (80, Arc::new(|left, right| {
            let a = match left {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Literal(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a + b))
        })));

        m.insert("&&".to_string(), (40, Arc::new(|left, right| {
            let a = match left {
                Param::Bool(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Bool(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Bool(a && b))
        })));

        m.insert("||".to_string(), (40, Arc::new(|left, right| {
            let a = match left {
                Param::Bool(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            let b = match right {
                Param::Bool(b) => b,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Bool(a || b))
        })));

        m.insert("==".to_string(), (20, Arc::new(|left, right| {
            Ok(Param::Bool(left == right))
        })));

        m.insert("!=".to_string(), (20, Arc::new(|left, right| {
            Ok(Param::Bool(left != right))
        })));

        m
    }

    pub fn register(&mut self, op: String, precidence: i32, f: Arc<BinaryOpFunc>) {
        self.store.insert(op, (precidence, f));
    }

    pub fn get(&self, op: String) -> Result<Arc<BinaryOpFunc>> {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return Err(Error::BinaryOpNotRegistered(op))
        }
        Ok(ans.unwrap().1.clone())
    }

    pub fn get_precidence(&self, op: String) -> i32 {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return -1
        }
        ans.unwrap().0
    }
    
}

impl UnaryOpFuncManager {
    pub fn new() -> Arc<Self> {
        static mut UNARY_OP_FUNC_MANAGER: Option<Arc<UnaryOpFuncManager>> = None;
        unsafe {
            match &UNARY_OP_FUNC_MANAGER {
                Some(m) => m.clone(), 
                None => {
                    UNARY_OP_FUNC_MANAGER.get_or_insert(Arc::new(UnaryOpFuncManager { store: Self::internal_register(HashMap::new()) })).clone()
                }
            }
        }
    }

    fn internal_register(mut m: HashMap<String, Arc<UnaryOpFunc>>) -> HashMap<String, Arc<UnaryOpFunc>> {
        m.insert("-".to_string(), Arc::new(|param| {
            let a = match param {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(-a))
        }));

        m.insert("+".to_string(), Arc::new(|param| {
            let a = match param {
                Param::Literal(a) => a,
                _ => {
                    return Err(Error::ShouldBeNumber())
                }
            };
            Ok(Param::Literal(a))
        }));

        m.insert("!".to_string(), Arc::new(|param| {
            let a = match param {
                Param::Bool(a) => a,
                _ => {
                    return Err(Error::ShouldBeBool())
                }
            };
            Ok(Param::Bool(a))
        }));

        m
    }

    pub fn register(&mut self, op: String, f: Arc<UnaryOpFunc>) {
        self.store.insert(op,  f);
    }

    pub fn get(&self, op: String) -> Result<Arc<UnaryOpFunc>> {
        let ans = self.store.get(&op);
        if ans.is_none() {
            return Err(Error::UnaryOpNotRegistered(op))
        }
        Ok(ans.unwrap().clone())
    }
}