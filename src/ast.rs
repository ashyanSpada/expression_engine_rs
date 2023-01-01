use rust_decimal::prelude::*;
use std::fmt;
use std::collections::HashMap;
use crate::error::Error;
use crate::tokenizer::Tokenizer;
use crate::token::Token;
use crate::define::*;


#[derive(Clone)]
pub 
enum ExprAST {
    Literal(Decimal),
    Bool(bool),
    String(String),
    BinaryExprAST(String, Box<ExprAST>, Box<ExprAST>),
    Reference(String),
    Function(String, Vec<ExprAST>)
}

impl fmt::Display for ExprAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(val) => write!(f, "Literal AST: {}", val.clone()),
            Self::Bool(val) => write!(f, "Bool AST: {}", val.clone()),
            Self::String(val) => write!(f, "String AST: {}", val.clone()),
            Self::BinaryExprAST(op, left, right) => write!(f, "Binary AST: Op: {}, Left: {}, Right: {}", op.clone(), left.clone(), right.clone()),
            Self::Reference(name) => write!(f, "Reference AST: reference: {}", name.clone()),
            Self::Function(name, params,) => {
                let mut s = "[".to_string();
                for param in params.into_iter() {
                    s.push_str(format!("{},", param.clone()).as_str());
                }
                s.push(']');
                write!(f, "Function AST: name: {}, params: {}", name.clone(), s)
            }
        }
    }
}

impl ExprAST {
    pub fn exec(&self, funcs: &HashMap<String, Box<dyn InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        match self {
            ExprAST::Bool(val) => self.exec_bool(val.clone()),
            ExprAST::Literal(val) => self.exec_literal(val.clone()),
            ExprAST::String(val) => self.exec_string(val.clone()),
            ExprAST::Reference(name) => self.exec_reference(name, &vars),
            ExprAST::Function(name, exprs) => self.exec_function(name, exprs.clone(), &funcs, &vars),
            ExprAST::BinaryExprAST(op, lhs, rhs) => self.exec_binary(op.clone(), lhs, rhs, funcs, vars)
        }
    }

    fn exec_bool(&self, val: bool) -> Result<Param> {
        Ok(Param::Bool(val))
    }

    fn exec_literal(&self, val: Decimal) -> Result<Param> {
        Ok(Param::Literal(val))
    }

    fn exec_string(&self, val: String) -> Result<Param> {
        Ok(Param::String(val))
    }

    fn exec_reference(&self, name: &String, vars: &HashMap<String, Param>) -> Result<Param> {
        if vars.get(name).is_none() {
            return Err(Error::ReferenceNotExist(name.clone()))
        }
        let v = vars.get(name).unwrap();
        Ok(v.clone())
    }

    fn exec_function(&self, name: &String, exprs: Vec<ExprAST>, funcs: &HashMap<String, Box<dyn InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        if funcs.get(name).is_none() {
            return Err(Error::FunctionNotExist(name.clone()))
        }
        let mut params: Vec<Param> = Vec::new();
        for expr in exprs.into_iter() {
            params.push(expr.exec(funcs, vars)?)
        }
        let func = funcs.get(name).unwrap();
        func.call(params)
    }

    fn exec_binary(&self, op: String, lhs: &Box<ExprAST>, rhs: &Box<ExprAST>, funcs: &HashMap<String, Box<dyn InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        let left = match lhs.exec(funcs, vars)? {
            Param::Literal(val) => val,
            _ => Decimal::ZERO,
        };
        let right = match rhs.exec(funcs, vars)? {
            Param::Literal(val) => val,
            _ => Decimal::ZERO,
        };
        match op.as_str() {
            "+" => Ok(Param::Literal(left+right)),
            "-" => Ok(Param::Literal(left-right)),
            "*" => Ok(Param::Literal(left*right)),
            "/" => Ok(Param::Literal(left/right)),
            "%" => Ok(Param::Literal(left%right)),
            _ => Err(Error::NotSupportedOp(op))
        }
    }
}

pub struct AST<'a> {
    tokenizer: Tokenizer<'a>,
    cur_tok: Token,
}

impl <'a> AST<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let token = tokenizer.next()?;
        Ok(Self{
            tokenizer: tokenizer,
            cur_tok: token.unwrap(),
        })
    }

    pub fn next(&mut self) -> Result<Option<Token>> {
        let token = self.tokenizer.next()?;
        if token.is_some() {
            self.cur_tok = token.clone().unwrap();
        }
        Ok(token)
    }

    fn parse_token(&mut self) -> Result<ExprAST> {
        let token = self.cur_tok.clone();
        match token {
            Token::Literal(val, _) => Ok(ExprAST::Literal(val)),
            Token::Bool(val, _) => Ok(ExprAST::Bool(val)),
            Token::Comma(_, _) => {
                self.next()?;
                self.parse_token()
            },
            Token::String(val, _) => Ok(ExprAST::String(val)),
            Token::Reference(val, _) => Ok(ExprAST::Reference(val)),
            Token::Function(name, _) => self.parse_function(name),
            Token::Operator(op, _) => self.parse_operator(op),
        }
    }

    fn parse_primary(&mut self) -> Result<ExprAST> {
        let expr = self.parse_token()?;
        match expr {
            ExprAST::Literal(_) | ExprAST::String(_) | ExprAST::Bool(_) | ExprAST::Function(_, _) | ExprAST::Reference(_) => {
                self.next()?;
            },
            _ => {},
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<ExprAST> {
        let lhs = self.parse_primary()?;
        let expr = self.parse_binop_rhs(0, lhs)?;
        Ok(expr)
    }

    fn parse_binop_rhs(&mut self, exec_prec: i32, mut lhs: ExprAST) -> Result<ExprAST> {
        loop {
            let tok_prec = self.get_token_precidence();
            // println!("pre compare, {}, {}, {}", tok_prec, exec_prec, self.cur_tok);
            if tok_prec < exec_prec {
                return Ok(lhs);
            }
            let op = self.cur_tok.string();
            self.next()?;
            let mut rhs = self.parse_primary()?;
            let next_prec = self.get_token_precidence();
            if tok_prec < next_prec {
                rhs = self.parse_binop_rhs(tok_prec+1, rhs)?;
            }
            lhs = ExprAST::BinaryExprAST(op, Box::new(lhs), Box::new(rhs))
        }
    }

    fn get_token_precidence(&self) -> i32 {
        match &self.cur_tok {
            Token::Operator(op, _) => match op.as_str() {
                "+" | "-" => 20,
                "*" | "/" | "%" => 40,
                _ => -1,
            },
            _ => -1
        }
    }

    fn parse_operator(&mut self, op: String) -> Result<ExprAST> {
        if op == "(" {
            self.next()?;
            let expr = self.parse_expression()?;
            if !self.cur_tok.is_right_brace() {
                return Err(Error::NoLeftBrace(0))
            }
            self.next()?;
            return Ok(expr);
        } else if op == "-" {
            self.next()?;
            return Ok(ExprAST::BinaryExprAST(op, Box::new(ExprAST::Literal(Decimal::ZERO)), Box::new(self.parse_primary()?)));
        }
        Err(Error::InvalidBool(0))
    }

    fn parse_function(&mut self, name : String) -> Result<ExprAST> {
        let next = self.next()?;
        if next.is_none() || !next.unwrap().is_left_brace() {
            return Err(Error::NoLeftBrace(1))
        }
        let mut ans = Vec::new();
        let has_right_brace: bool;
        loop {
            if self.cur_tok.is_right_brace() {
                has_right_brace = true;
                break
            }
            match self.next()? {
                // Some(_) => ans.push(self.parse_token()),
                // Some(Token::Comma(_, _)) => continue,
                Some(_) => {
                    ans.push(self.parse_expression()?);
                },
                None => {}
            }
        }
        if !has_right_brace {
            return Err(Error::NoRightBrace(0))
        }
        Ok(ExprAST::Function(name, ans))
    }
}

#[test]
fn test() {
    let input = "$func(1+2+&mm, 2, true, $func(1, 2, 3))";
    let ast = AST::new(input);
    match ast {
        Ok(mut a) => {
            let expr = a.parse_expression().unwrap();
            println!("{}", expr)
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[test]
fn test_exec() {
    let input = "(1+2)*3+5/2+&mm";
    let ast = AST::new(input);
    let funcs = HashMap::new();
    let mut vars = HashMap::new();
    vars.insert("mm".to_string(), Param::Literal(Decimal::new(12, 0)));
    match ast {
        Ok(mut a) => {
            let expr = a.parse_expression().unwrap();
            let ans = expr.exec(&funcs, &vars).unwrap();
            println!("ans is {}", ans);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
