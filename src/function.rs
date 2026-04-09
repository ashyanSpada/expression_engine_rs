use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type InnerFunction = dyn Fn(Vec<Value>) -> Result<Value> + Send + Sync + 'static;

pub struct InnerFunctionManager {
    pub store: &'static Mutex<HashMap<String, Arc<InnerFunction>>>,
}

impl InnerFunctionManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<InnerFunction>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        InnerFunctionManager { store: store }
    }

    pub fn init(&mut self) {
        self.register(
            "min",
            Arc::new(|params| {
                let mut min = None;
                for param in params.into_iter() {
                    let num = param.decimal()?;
                    if min.is_none() || num < min.unwrap() {
                        min = Some(num);
                    }
                }
                if let Some(min) = min {
                    Ok(Value::Number(min))
                } else {
                    Err(Error::ParamEmpty("min".to_string()))
                }
            }),
        );

        self.register(
            "max",
            Arc::new(|params| {
                let mut max = None;
                for param in params.into_iter() {
                    let num = param.decimal()?;
                    if max.is_none() || num > max.unwrap() {
                        max = Some(num);
                    }
                }
                if let Some(max) = max {
                    Ok(Value::Number(max))
                } else {
                    Err(Error::ParamEmpty("max".to_string()))
                }
            }),
        );

        self.register(
            "sum",
            Arc::new(|params| {
                let mut ans = Decimal::ZERO;
                for param in params.into_iter() {
                    ans += param.decimal()?;
                }
                Ok(Value::Number(ans))
            }),
        );

        self.register(
            "mul",
            Arc::new(|params| {
                let mut ans = Decimal::ONE;
                for param in params.into_iter() {
                    ans *= param.decimal()?;
                }
                Ok(Value::Number(ans))
            }),
        );

        self.register(
            "abs",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("abs".to_string()));
                }
                let val = params.into_iter().next().unwrap().decimal()?;
                Ok(Value::Number(val.abs()))
            }),
        );

        self.register(
            "floor",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("floor".to_string()));
                }
                let val = params.into_iter().next().unwrap().decimal()?;
                Ok(Value::Number(val.floor()))
            }),
        );

        self.register(
            "ceil",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("ceil".to_string()));
                }
                let val = params.into_iter().next().unwrap().decimal()?;
                Ok(Value::Number(val.ceil()))
            }),
        );

        self.register(
            "round",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("round".to_string()));
                }
                let val = params.into_iter().next().unwrap().decimal()?;
                Ok(Value::Number(val.round()))
            }),
        );

        self.register(
            "avg",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("avg".to_string()));
                }
                let count = Decimal::from(params.len() as u64);
                let mut sum = Decimal::ZERO;
                for param in params.into_iter() {
                    sum += param.decimal()?;
                }
                Ok(Value::Number(sum / count))
            }),
        );

        self.register(
            "pow",
            Arc::new(|params| {
                if params.len() < 2 {
                    return Err(Error::ParamEmpty("pow".to_string()));
                }
                let mut iter = params.into_iter();
                let base = iter.next().unwrap().float()?;
                let exp = iter.next().unwrap().float()?;
                Ok(Value::from(base.powf(exp)))
            }),
        );

        self.register(
            "sqrt",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("sqrt".to_string()));
                }
                let val = params.into_iter().next().unwrap().float()?;
                Ok(Value::from(val.sqrt()))
            }),
        );

        self.register(
            "len",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("len".to_string()));
                }
                let length = match &params[0] {
                    Value::String(s) => s.len(),
                    Value::List(l) => l.len(),
                    Value::Map(m) => m.len(),
                    _ => return Err(Error::ParamInvalid()),
                };
                Ok(Value::from(length as i64))
            }),
        );

        self.register(
            "upper",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("upper".to_string()));
                }
                let s = params.into_iter().next().unwrap().string()?;
                Ok(Value::String(s.to_uppercase()))
            }),
        );

        self.register(
            "lower",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("lower".to_string()));
                }
                let s = params.into_iter().next().unwrap().string()?;
                Ok(Value::String(s.to_lowercase()))
            }),
        );

        self.register(
            "trim",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("trim".to_string()));
                }
                let s = params.into_iter().next().unwrap().string()?;
                Ok(Value::String(s.trim().to_string()))
            }),
        );

        self.register(
            "concat",
            Arc::new(|params| {
                let mut result = String::new();
                for param in params.into_iter() {
                    result.push_str(&param.string()?);
                }
                Ok(Value::String(result))
            }),
        );

        self.register(
            "str",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("str".to_string()));
                }
                let s = match params.into_iter().next().unwrap() {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::None => "None".to_string(),
                    _ => return Err(Error::ParamInvalid()),
                };
                Ok(Value::String(s))
            }),
        );

        self.register(
            "num",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("num".to_string()));
                }
                match params.into_iter().next().unwrap() {
                    Value::Number(n) => Ok(Value::Number(n)),
                    Value::String(s) => Decimal::from_str(&s)
                        .map(Value::Number)
                        .map_err(|_| Error::InvalidNumber(s)),
                    Value::Bool(b) => {
                        Ok(Value::Number(if b { Decimal::ONE } else { Decimal::ZERO }))
                    }
                    _ => Err(Error::ParamInvalid()),
                }
            }),
        );

        self.register(
            "first",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("first".to_string()));
                }
                let list = params.into_iter().next().unwrap().list()?;
                list.into_iter()
                    .next()
                    .ok_or_else(|| Error::ParamEmpty("first".to_string()))
            }),
        );

        self.register(
            "last",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("last".to_string()));
                }
                let list = params.into_iter().next().unwrap().list()?;
                list.into_iter()
                    .last()
                    .ok_or_else(|| Error::ParamEmpty("last".to_string()))
            }),
        );

        self.register(
            "keys",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("keys".to_string()));
                }
                let m = params.into_iter().next().unwrap().map()?;
                Ok(Value::List(m.into_iter().map(|(k, _)| k).collect()))
            }),
        );

        self.register(
            "values",
            Arc::new(|params| {
                if params.is_empty() {
                    return Err(Error::ParamEmpty("values".to_string()));
                }
                let m = params.into_iter().next().unwrap().map()?;
                Ok(Value::List(m.into_iter().map(|(_, v)| v).collect()))
            }),
        );
    }

    pub fn register(&mut self, name: &str, f: Arc<InnerFunction>) {
        self.store.lock().unwrap().insert(name.to_string(), f);
    }

    pub fn get(&self, name: &str) -> Result<Arc<InnerFunction>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(name);
        if ans.is_none() {
            return Err(Error::InnerFunctionNotRegistered(String::from(name)));
        }
        Ok(ans.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_context, execute, Value};

    fn run(expr: &str) -> Value {
        execute(expr, create_context!()).expect(expr)
    }

    fn run_err(expr: &str) {
        assert!(execute(expr, create_context!()).is_err(), "{}", expr);
    }

    // --- math functions ---

    #[test]
    fn test_abs() {
        assert_eq!(run("abs(-5)"), Value::from(5i32));
        assert_eq!(run("abs(3)"), Value::from(3i32));
        assert_eq!(run("abs(-3.7)"), Value::from(3.7f64));
        run_err("abs()");
    }

    #[test]
    fn test_floor() {
        assert_eq!(run("floor(3.7)"), Value::from(3i32));
        assert_eq!(run("floor(-1.2)"), Value::from(-2i32));
        run_err("floor()");
    }

    #[test]
    fn test_ceil() {
        assert_eq!(run("ceil(3.1)"), Value::from(4i32));
        assert_eq!(run("ceil(-1.9)"), Value::from(-1i32));
        run_err("ceil()");
    }

    #[test]
    fn test_round() {
        assert_eq!(run("round(3.5)"), Value::from(4i32));
        assert_eq!(run("round(3.4)"), Value::from(3i32));
        assert_eq!(run("round(-2.5)"), Value::from(-2i32));
        run_err("round()");
    }

    #[test]
    fn test_avg() {
        assert_eq!(run("avg(1, 2, 3)"), Value::from(2i32));
        assert_eq!(run("avg(10)"), Value::from(10i32));
        run_err("avg()");
    }

    #[test]
    fn test_pow() {
        assert_eq!(run("pow(2, 10)"), Value::from(1024.0f64));
        assert_eq!(run("pow(9, 0.5)"), Value::from(3.0f64));
        run_err("pow(2)");
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(run("sqrt(4)"), Value::from(2.0f64));
        assert_eq!(run("sqrt(9)"), Value::from(3.0f64));
        run_err("sqrt()");
    }

    // --- string functions ---

    #[test]
    fn test_len_string() {
        assert_eq!(run("len('hello')"), Value::from(5i64));
        assert_eq!(run("len('')"), Value::from(0i64));
        run_err("len()");
    }

    #[test]
    fn test_len_list() {
        assert_eq!(run("len([1, 2, 3])"), Value::from(3i64));
        assert_eq!(run("len([])"), Value::from(0i64));
    }

    #[test]
    fn test_len_map() {
        assert_eq!(run("len({'a': 1, 'b': 2})"), Value::from(2i64));
        assert_eq!(run("len({})"), Value::from(0i64));
    }

    #[test]
    fn test_upper() {
        assert_eq!(run("upper('hello')"), Value::from("HELLO"));
        assert_eq!(run("upper('Hello World')"), Value::from("HELLO WORLD"));
        run_err("upper()");
    }

    #[test]
    fn test_lower() {
        assert_eq!(run("lower('HELLO')"), Value::from("hello"));
        assert_eq!(run("lower('Hello World')"), Value::from("hello world"));
        run_err("lower()");
    }

    #[test]
    fn test_trim() {
        assert_eq!(run("trim('  hello  ')"), Value::from("hello"));
        assert_eq!(run("trim('no spaces')"), Value::from("no spaces"));
        run_err("trim()");
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            run("concat('hello', ' ', 'world')"),
            Value::from("hello world")
        );
        assert_eq!(run("concat()"), Value::from(""));
        assert_eq!(run("concat('only')"), Value::from("only"));
    }

    #[test]
    fn test_str() {
        assert_eq!(run("str(42)"), Value::from("42"));
        assert_eq!(run("str(true)"), Value::from("true"));
        assert_eq!(run("str('already')"), Value::from("already"));
        assert_eq!(run("str(None)"), Value::from("None"));
        run_err("str()");
    }

    #[test]
    fn test_num() {
        assert_eq!(run("num('3.14')"), Value::from(3.14f64));
        assert_eq!(run("num(42)"), Value::from(42i32));
        assert_eq!(run("num(true)"), Value::from(1i32));
        assert_eq!(run("num(false)"), Value::from(0i32));
        run_err("num('not_a_number')");
        run_err("num()");
    }

    // --- list functions ---

    #[test]
    fn test_first() {
        assert_eq!(run("first([10, 20, 30])"), Value::from(10i32));
        assert_eq!(run("first(['a', 'b'])"), Value::from("a"));
        run_err("first([])");
        run_err("first()");
    }

    #[test]
    fn test_last() {
        assert_eq!(run("last([10, 20, 30])"), Value::from(30i32));
        assert_eq!(run("last(['a', 'b'])"), Value::from("b"));
        run_err("last([])");
        run_err("last()");
    }

    // --- map functions ---

    #[test]
    fn test_keys() {
        let result = run("keys({'a': 1, 'b': 2})");
        match result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert!(items.contains(&Value::from("a")));
                assert!(items.contains(&Value::from("b")));
            }
            _ => panic!("expected list"),
        }
        run_err("keys()");
    }

    #[test]
    fn test_values() {
        let result = run("values({'a': 1, 'b': 2})");
        match result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert!(items.contains(&Value::from(1i32)));
                assert!(items.contains(&Value::from(2i32)));
            }
            _ => panic!("expected list"),
        }
        run_err("values()");
    }
}
