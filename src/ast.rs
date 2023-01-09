use rust_decimal::prelude::*;
use std::fmt;
use std::collections::HashMap;
use std::sync::Arc;
use crate::error::Error;
use crate::tokenizer::Tokenizer;
use crate::token::Token;
use crate::define::*;
use crate::operator::{BinaryOpFuncManager, UnaryOpFuncManager};
use crate::function::{InnerFunction, InnerFunctionManager};


#[derive(Clone)]
pub enum ExprAST {
    Literal(Decimal),
    Bool(bool),
    String(String),
    UnaryExprAST(String, Arc<ExprAST>),
    BinaryExprAST(String, Arc<ExprAST>, Arc<ExprAST>),
    TernaryExprAST(Arc<ExprAST>, Arc<ExprAST>, Arc<ExprAST>),
    Reference(String),
    Function(String, Vec<ExprAST>),
}

impl fmt::Display for ExprAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(val) => write!(f, "Literal AST: {}", val.clone()),
            Self::Bool(val) => write!(f, "Bool AST: {}", val.clone()),
            Self::String(val) => write!(f, "String AST: {}", val.clone()),
            Self::UnaryExprAST(op, rhs) => write!(f, "Unary AST: Op: {}, Rhs: {}", op.clone(), rhs.clone()),
            Self::BinaryExprAST(op, lhs, rhs) => write!(f, "Binary AST: Op: {}, Lhs: {}, Rhs: {}", op.clone(), lhs.clone(), rhs.clone()),
            Self::TernaryExprAST(condition, lhs, rhs) => write!(f, "Ternary AST: Condition: {}, Lhs: {}, Rhs: {}", condition.clone(), lhs.clone(), rhs.clone()),
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
    pub fn exec(&self, funcs: &HashMap<String, Arc<InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        match self {
            ExprAST::Bool(val) => self.exec_bool(val.clone()),
            ExprAST::Literal(val) => self.exec_literal(val.clone()),
            ExprAST::String(val) => self.exec_string(val.clone()),
            ExprAST::Reference(name) => self.exec_reference(name, &vars),
            ExprAST::Function(name, exprs) => self.exec_function(name, exprs.clone(), &funcs, &vars),
            ExprAST::UnaryExprAST(op, rhs) => self.exec_unary(op.clone(), rhs.clone(), funcs, vars),
            ExprAST::BinaryExprAST(op, lhs, rhs) => self.exec_binary(op.clone(), lhs.clone(), rhs.clone(), funcs, vars),
            ExprAST::TernaryExprAST(condition, lhs, rhs) => self.exec_ternary(condition.clone(), lhs.clone(), rhs.clone(), funcs, vars),
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

    fn exec_function(&self, name: &String, exprs: Vec<ExprAST>, funcs: &HashMap<String, Arc<InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        let mut params: Vec<Param> = Vec::new();
        for expr in exprs.into_iter() {
            params.push(expr.exec(funcs, vars)?)
        }
        let func = funcs.get(name);
        if func.is_some() {
            return func.unwrap().clone()(params);
        }
        let m = InnerFunctionManager::new();
        println!("expty check: {}", m.store.is_empty());
        m.get(name.clone())?(params)
    }

    fn exec_unary(&self, op: String, rhs: Arc<ExprAST>, funcs: &HashMap<String, Arc<InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        UnaryOpFuncManager::new().get(op)?(rhs.exec(funcs, vars)?)
    }

    fn exec_binary(&self, op: String, lhs: Arc<ExprAST>, rhs: Arc<ExprAST>, funcs: &HashMap<String, Arc<InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        BinaryOpFuncManager::new().get(op)?(lhs.exec(funcs, vars)?, rhs.exec(funcs, vars)?)
    }

    fn exec_ternary(&self, condition: Arc<ExprAST>, lhs: Arc<ExprAST>, rhs: Arc<ExprAST>, funcs: &HashMap<String, Arc<InnerFunction>>, vars: &HashMap<String, Param>) -> Result<Param> {
        match condition.exec(funcs, vars)? {
            Param::Bool(val) => {
                if val {
                    return lhs.exec(funcs, vars);
                }
                rhs.exec(funcs, vars)
            },
            _ => Err(Error::ShouldBeBool())
        }
    }

    pub fn expr(&self) -> String {
        match self {
            ExprAST::Bool(val) => {
                if val.clone() {
                    return "true".to_string();
                }
                return "false".to_string();
            },
            ExprAST::Literal(val) => self.literal_expr(val.clone()),
            ExprAST::String(val) => self.string_expr(val.clone()),
            ExprAST::Reference(name) => self.reference_expr(name.clone()),
            ExprAST::Function(name, exprs) => self.function_expr(name.clone(), exprs.clone()),
            ExprAST::UnaryExprAST(op, rhs) => self.unary_expr(op, rhs.clone()),
            ExprAST::BinaryExprAST(op, lhs, rhs) => self.binary_expr(op, lhs.clone(), rhs.clone()),
            ExprAST::TernaryExprAST(condition, lhs, rhs) => self.ternary_expr(condition.clone(), lhs.clone(), rhs.clone()),
        }
    }

    fn literal_expr(&self, val: Decimal) -> String {
        val.to_string()
    }

    fn string_expr(&self, val: String) -> String {
        val
    }

    fn reference_expr(&self, val: String) -> String {
        val
    }

    fn function_expr(&self, mut name: String, exprs: Vec<ExprAST>) -> String {
        name.push('(');  
        for i in 0..exprs.len() {
            name.push_str(&exprs[i].expr());
            if i < exprs.len() - 1 {
                name.push(',');
            }
        }
        name.push(')');
        name
    }

    fn unary_expr(&self, op: &String, rhs: Arc<ExprAST>) -> String {
        let mut ans = op.clone();
        ans.push_str(&rhs.expr());
        ans
    }

    fn binary_expr(&self, op: &String, lhs: Arc<ExprAST>, rhs: Arc<ExprAST>) -> String {
        let mut left = {
            let (is, precidence) = lhs.get_precidence();
            let mut tmp: String = lhs.expr();
            if is && precidence < BinaryOpFuncManager::new().get_precidence(op.clone()) {
                tmp = "(".to_string() + &lhs.expr() + &")".to_string();
            }
            tmp
        };
        let right = {
            let (is, precidence) = rhs.get_precidence();
            let mut tmp = rhs.expr();
            if is && precidence < BinaryOpFuncManager::new().get_precidence(op.clone()) {
                tmp = "(".to_string() + &rhs.expr() + &")".to_string();
            }
            tmp
        };
        left + " " + op + " " + &right
    }

    fn ternary_expr(&self, condition: Arc<ExprAST>, lhs: Arc<ExprAST>, rhs: Arc<ExprAST>) -> String {
        let condition_expr = match condition.as_ref() {
            ExprAST::BinaryExprAST(_, _, _) | ExprAST::TernaryExprAST(_, _, _) => "(".to_string() + &condition.expr() + ")",
            _ => condition.expr(),
        };

        let left_expr = match lhs.as_ref() {
            ExprAST::BinaryExprAST(_, _, _) | ExprAST::TernaryExprAST(_, _, _) => "(".to_string() + &lhs.expr() + ")",
            _ => lhs.expr(),
        };

        let right_expr = match rhs.as_ref() {
            ExprAST::BinaryExprAST(_, _, _) | ExprAST::TernaryExprAST(_, _, _) => "(".to_string() + &rhs.expr() + ")",
            _ => rhs.expr(),
        };
        condition_expr + " ? " + &left_expr + " : " + &right_expr
    }

    fn get_precidence(&self) -> (bool, i32) {
        match self {
            ExprAST::BinaryExprAST(op, _, _) => (true, BinaryOpFuncManager::new().get_precidence(op.clone())),
            _ => (false, 0),
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
            Token::Brace(brace, _) => self.parse_brace(brace),
        }
    }

    fn parse_expression(&mut self) -> Result<ExprAST> {
        let lhs = self.parse_primary()?;
        let expr = self.parse_op(lhs)?;
        Ok(expr)
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

    fn parse_op(&mut self, lhs: ExprAST) -> Result<ExprAST> {
        println!("curTok is {}", self.cur_tok);
        if self.cur_tok.is_question_mark() {
            return self.parse_terop(lhs)
        }
        self.parse_binop(0, lhs)
    }

    fn parse_binop(&mut self, exec_prec: i32, mut lhs: ExprAST) -> Result<ExprAST> {
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
                rhs = self.parse_binop(tok_prec+1, rhs)?;
            }
            lhs = ExprAST::BinaryExprAST(op, Arc::new(lhs), Arc::new(rhs))
        }
    }

    fn parse_terop(&mut self, condition: ExprAST) -> Result<ExprAST> {
        self.next()?;
        let lhs = self.parse_primary()?;
        if !self.cur_tok.is_colon() {
            return Err(Error::InvalidTernaryExprNeedColon());
        }
        self.next()?;
        let rhs = self.parse_primary()?;
        Ok(ExprAST::TernaryExprAST(Arc::new(condition), Arc::new(lhs), Arc::new(rhs)))
    }

    fn get_token_precidence(&self) -> i32 {
        match &self.cur_tok {
            Token::Operator(op, _) => BinaryOpFuncManager::new().get_precidence(op.clone()),
            _ => -1
        }
    }

    fn parse_brace(&mut self, brace: String) -> Result<ExprAST> {
        if brace == "(" {
            self.next()?;
            let expr = self.parse_expression()?;
            if !self.cur_tok.is_right_brace() {
                return Err(Error::NoLeftBrace(0))
            }
            self.next()?;
            return Ok(expr);
        }
        Err(Error::NoLeftBrace(0))
    }

    fn parse_operator(&mut self, op: String) -> Result<ExprAST> {
        self.next()?;
        Ok(ExprAST::UnaryExprAST(op, Arc::new(self.parse_primary()?)))
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
    let input = "func(1+2+mm, 2, true, func(1, 2, 3))";
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
    let input = "(5 < 3) ? 3 : false";
    let ast = AST::new(input);
    let funcs = HashMap::new();
    let mut vars = HashMap::new();
    vars.insert("mm".to_string(), Param::Literal(Decimal::new(12, 0)));
    match ast {
        Ok(mut a) => {
            let expr = a.parse_expression().unwrap();
            println!("expr is {}", expr);
            println!("string is {}", expr.expr());
            let ans = expr.exec(&funcs, &vars).unwrap();
            println!("ans is {}", ans);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
