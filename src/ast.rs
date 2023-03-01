use crate::context::Context;
use crate::define::*;
use crate::error::Error;
use crate::function::InnerFunctionManager;
use crate::operator::{BinaryOpFuncManager, UnaryOpFuncManager};
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::value::Value;
use rust_decimal::prelude::*;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub enum ExprAST {
    Number(Decimal),
    Bool(bool),
    String(String),
    Unary(String, Arc<ExprAST>),
    Binary(String, Arc<ExprAST>, Arc<ExprAST>),
    Ternary(Arc<ExprAST>, Arc<ExprAST>, Arc<ExprAST>),
    Reference(String),
    Function(String, Vec<ExprAST>),
    List(Vec<ExprAST>),
    Map(Vec<(ExprAST, ExprAST)>),
    Chain(Vec<ExprAST>),
    None,
}

impl fmt::Display for ExprAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(val) => write!(f, "Number AST: {}", val.clone()),
            Self::Bool(val) => write!(f, "Bool AST: {}", val.clone()),
            Self::String(val) => write!(f, "String AST: {}", val.clone()),
            Self::Unary(op, rhs) => {
                write!(f, "Unary AST: Op: {}, Rhs: {}", op.clone(), rhs.clone())
            }
            Self::Binary(op, lhs, rhs) => write!(
                f,
                "Binary AST: Op: {}, Lhs: {}, Rhs: {}",
                op.clone(),
                lhs.clone(),
                rhs.clone()
            ),
            Self::Ternary(condition, lhs, rhs) => write!(
                f,
                "Ternary AST: Condition: {}, Lhs: {}, Rhs: {}",
                condition.clone(),
                lhs.clone(),
                rhs.clone()
            ),
            Self::Reference(name) => write!(f, "Reference AST: reference: {}", name.clone()),
            Self::Function(name, params) => {
                let mut s = "[".to_string();
                for param in params.into_iter() {
                    s.push_str(format!("{},", param.clone()).as_str());
                }
                s.push(']');
                write!(f, "Function AST: name: {}, params: {}", name.clone(), s)
            }
            Self::List(params) => {
                let mut s = "[".to_string();
                for param in params.into_iter() {
                    s.push_str(format!("{},", param.clone()).as_str());
                }
                s.push(']');
                write!(f, "List AST: params: {}", s)
            }
            Self::Map(m) => {
                let mut s = String::new();
                for (k, v) in m {
                    s.push_str(format!("({} {}), ", k.clone(), v.clone()).as_str());
                }
                write!(f, "Map AST: {}", s)
            }
            Self::Chain(exprs) => {
                let mut s = String::new();
                for expr in exprs {
                    s.push_str(format!("{};", expr.clone()).as_str());
                }
                write!(f, "Chain AST: {}", s)
            }
            Self::None => write!(f, "None"),
        }
    }
}

impl ExprAST {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<Value> {
        match self {
            Self::Bool(val) => self.exec_bool(val.clone()),
            Self::Number(val) => self.exec_number(val.clone()),
            Self::String(val) => self.exec_string(val.clone()),
            Self::Reference(name) => self.exec_reference(name, ctx.clone()),
            Self::Function(name, exprs) => self.exec_function(name, exprs.clone(), ctx.clone()),
            Self::Unary(op, rhs) => self.exec_unary(op.clone(), rhs.clone(), ctx.clone()),
            Self::Binary(op, lhs, rhs) => {
                self.exec_binary(op.clone(), lhs.clone(), rhs.clone(), ctx.clone())
            }
            Self::Ternary(condition, lhs, rhs) => {
                self.exec_ternary(condition.clone(), lhs.clone(), rhs.clone(), ctx.clone())
            }
            Self::List(params) => self.exec_list(params.clone(), ctx.clone()),
            Self::Chain(exprs) => self.exec_list(exprs.clone(), ctx.clone()),
            Self::Map(m) => self.exec_map(m.clone(), ctx.clone()),
            Self::None => Ok(Value::None),
        }
    }

    fn exec_bool(&self, val: bool) -> Result<Value> {
        Ok(Value::Bool(val))
    }

    fn exec_number(&self, val: Decimal) -> Result<Value> {
        Ok(Value::Number(val))
    }

    fn exec_string(&self, val: String) -> Result<Value> {
        Ok(Value::String(val))
    }

    fn exec_reference(&self, name: &str, ctx: Arc<Context>) -> Result<Value> {
        match ctx.get_variable(name) {
            Some(value) => Ok(Value::Pair(name.to_string(), Box::new(value))),
            None => Err(Error::WrongContextValueType()),
        }
    }

    fn exec_function(
        &self,
        name: &String,
        exprs: Vec<ExprAST>,
        ctx: Arc<Context>,
    ) -> Result<Value> {
        let mut params: Vec<Value> = Vec::new();
        for expr in exprs.into_iter() {
            params.push(expr.exec(ctx.clone())?)
        }
        match ctx.get_func(name) {
            Some(func) => func(params),
            None => self.redirect_inner_function(name, params),
        }
    }

    fn redirect_inner_function(&self, name: &str, params: Vec<Value>) -> Result<Value> {
        InnerFunctionManager::new().lock().unwrap().get(name)?(params)
    }

    fn exec_unary(&self, op: String, rhs: Arc<ExprAST>, ctx: Arc<Context>) -> Result<Value> {
        UnaryOpFuncManager::new().get(op)?(rhs.exec(ctx)?)
    }

    fn exec_binary(
        &self,
        op: String,
        lhs: Arc<ExprAST>,
        rhs: Arc<ExprAST>,
        ctx: Arc<Context>,
    ) -> Result<Value> {
        BinaryOpFuncManager::new().get(op)?(lhs.exec(ctx.clone())?, rhs.exec(ctx.clone())?)
    }

    fn exec_ternary(
        &self,
        condition: Arc<ExprAST>,
        lhs: Arc<ExprAST>,
        rhs: Arc<ExprAST>,
        ctx: Arc<Context>,
    ) -> Result<Value> {
        match condition.exec(ctx.clone())? {
            Value::Bool(val) => {
                if val {
                    return lhs.exec(ctx.clone());
                }
                rhs.exec(ctx.clone())
            }
            _ => Err(Error::ShouldBeBool()),
        }
    }

    fn exec_list(&self, params: Vec<ExprAST>, ctx: Arc<Context>) -> Result<Value> {
        let mut ans = Vec::new();
        for expr in params {
            ans.push(expr.exec(ctx.clone())?);
        }
        Ok(Value::List(ans))
    }

    fn exec_map(&self, m: Vec<(ExprAST, ExprAST)>, ctx: Arc<Context>) -> Result<Value> {
        let mut ans = Vec::new();
        for (k, v) in m {
            ans.push((k.exec(ctx.clone())?, v.exec(ctx.clone())?));
        }
        Ok(Value::Map(ans))
    }

    pub fn expr(&self) -> String {
        match self {
            Self::Bool(val) => {
                if val.clone() {
                    return "true".to_string();
                }
                return "false".to_string();
            }
            Self::Number(val) => self.number_expr(val.clone()),
            Self::String(val) => self.string_expr(val.clone()),
            Self::Reference(name) => self.reference_expr(name.clone()),
            Self::Function(name, exprs) => self.function_expr(name.clone(), exprs.clone()),
            Self::Unary(op, rhs) => self.unary_expr(op, rhs.clone()),
            Self::Binary(op, lhs, rhs) => self.binary_expr(op, lhs.clone(), rhs.clone()),
            Self::Ternary(condition, lhs, rhs) => {
                self.ternary_expr(condition.clone(), lhs.clone(), rhs.clone())
            }
            Self::List(params) => self.list_expr(params.clone()),
            Self::Map(m) => self.map_expr(m.clone()),
            Self::Chain(exprs) => self.chain_expr(exprs.clone()),
            Self::None => "".to_string(),
        }
    }

    fn number_expr(&self, val: Decimal) -> String {
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
        let left = {
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

    fn ternary_expr(
        &self,
        condition: Arc<ExprAST>,
        lhs: Arc<ExprAST>,
        rhs: Arc<ExprAST>,
    ) -> String {
        let condition_expr = match condition.as_ref() {
            ExprAST::Binary(_, _, _) | ExprAST::Ternary(_, _, _) => {
                "(".to_string() + &condition.expr() + ")"
            }
            _ => condition.expr(),
        };

        let left_expr = match lhs.as_ref() {
            ExprAST::Binary(_, _, _) | ExprAST::Ternary(_, _, _) => {
                "(".to_string() + &lhs.expr() + ")"
            }
            _ => lhs.expr(),
        };

        let right_expr = match rhs.as_ref() {
            ExprAST::Binary(_, _, _) | ExprAST::Ternary(_, _, _) => {
                "(".to_string() + &rhs.expr() + ")"
            }
            _ => rhs.expr(),
        };
        condition_expr + " ? " + &left_expr + " : " + &right_expr
    }

    fn list_expr(&self, params: Vec<ExprAST>) -> String {
        let mut s = String::from("[");
        for i in 0..params.len() {
            s.push_str(params[i].expr().as_str());
            if i < params.len() - 1 {
                s.push_str(",");
            }
        }
        s.push_str("]");
        s
    }

    fn map_expr(&self, m: Vec<(ExprAST, ExprAST)>) -> String {
        let mut s = String::from("{");
        for (k, v) in m.into_iter() {
            s.push_str(k.expr().as_str());
            s.push_str(":");
            s.push_str(v.expr().as_str());
            s.push_str(", ");
        }
        s.push_str("}");
        s
    }

    fn chain_expr(&self, exprs: Vec<ExprAST>) -> String {
        let mut s = String::new();
        for i in 0..exprs.len() {
            s.push_str(exprs[i].expr().as_str());
            if i < exprs.len() - 1 {
                s.push_str(";");
            }
        }
        s
    }

    fn get_precidence(&self) -> (bool, i32) {
        match self {
            ExprAST::Binary(op, _, _) => {
                (true, BinaryOpFuncManager::new().get_precidence(op.clone()))
            }
            _ => (false, 0),
        }
    }
}

pub struct AST<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> AST<'a> {
    fn cur_tok(&self) -> Token {
        self.tokenizer.cur_token.clone()
    }

    fn prev_tok(&self) -> Token {
        self.tokenizer.prev_token.clone()
    }

    pub fn new(input: &'a str) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.next()?;
        Ok(Self {
            tokenizer: tokenizer,
        })
    }

    pub fn next(&mut self) -> Result<Token> {
        self.tokenizer.next()
    }

    fn peek(&self) -> Result<Token> {
        self.tokenizer.peek()
    }

    fn expect(&mut self, expected: &str) -> Result<()> {
        self.tokenizer.expect(expected)
    }

    fn parse_token(&mut self) -> Result<ExprAST> {
        let token = self.cur_tok();
        match token {
            Token::Number(val, _) => {
                self.next()?;
                Ok(ExprAST::Number(val))
            }
            Token::Bool(val, _) => {
                self.next()?;
                Ok(ExprAST::Bool(val))
            }
            Token::String(val, _) => {
                self.next()?;
                Ok(ExprAST::String(val))
            }
            Token::Reference(val, _) => {
                self.next()?;
                Ok(ExprAST::Reference(val))
            }
            Token::Function(name, _) => self.parse_function(name),
            Token::Operator(op, _) => self.parse_operator(op),
            Token::Bracket(_, _) => self.parse_bracket(),
            Token::EOF => Ok(ExprAST::None),
            _ => Err(Error::UnexpectedToken()),
        }
    }

    pub fn parse_expression(&mut self) -> Result<ExprAST> {
        let mut ans = Vec::new();
        loop {
            let lhs = self.parse_primary()?;
            if !self.cur_tok().is_op_token() {
                ans.push(lhs);
            } else {
                ans.push(self.parse_op(lhs)?);
            }
            if !self.cur_tok().is_semicolon() {
                println!("cur_tok is {}", self.cur_tok());
                break;
            }
            self.next()?;
        }
        if ans.len() == 1 {
            return Ok(ans[0].clone());
        }
        Ok(ExprAST::Chain(ans))
    }

    fn parse_primary(&mut self) -> Result<ExprAST> {
        Ok(self.parse_token()?)
    }

    fn parse_op(&mut self, lhs: ExprAST) -> Result<ExprAST> {
        if self.cur_tok().is_question_mark() {
            return self.parse_terop(lhs);
        }
        self.parse_binop(0, lhs)
    }

    fn parse_binop(&mut self, exec_prec: i32, mut lhs: ExprAST) -> Result<ExprAST> {
        loop {
            let tok_prec = self.get_token_precidence();
            if tok_prec < exec_prec {
                return Ok(lhs);
            }
            let op = self.cur_tok().string();
            self.next()?;
            let mut rhs = self.parse_primary()?;
            let next_prec = self.get_token_precidence();
            if tok_prec < next_prec {
                rhs = self.parse_binop(tok_prec + 1, rhs)?;
            }
            lhs = ExprAST::Binary(op, Arc::new(lhs), Arc::new(rhs))
        }
    }

    fn parse_terop(&mut self, condition: ExprAST) -> Result<ExprAST> {
        self.next()?;
        let lhs = self.parse_primary()?;
        if !self.cur_tok().is_colon() {
            return Err(Error::InvalidTernaryExprNeedColon());
        }
        self.next()?;
        let rhs = self.parse_primary()?;
        Ok(ExprAST::Ternary(
            Arc::new(condition),
            Arc::new(lhs),
            Arc::new(rhs),
        ))
    }

    fn get_token_precidence(&self) -> i32 {
        match &self.cur_tok() {
            Token::Operator(op, _) => BinaryOpFuncManager::new().get_precidence(op.clone()),
            _ => -1,
        }
    }

    fn parse_bracket(&mut self) -> Result<ExprAST> {
        if self.cur_tok().is_left_paren() {
            return self.parse_left_paren();
        } else if self.cur_tok().is_left_bracket() {
            return self.parse_left_bracket();
        } else if self.cur_tok().is_left_curly() {
            return self.parse_left_curly();
        }
        Err(Error::NoLeftBrace(0))
    }

    fn parse_left_paren(&mut self) -> Result<ExprAST> {
        self.next()?;
        let expr = self.parse_expression()?;
        if !self.cur_tok().is_right_paren() {
            return Err(Error::NoRightBrace(0));
        }
        self.next()?;
        return Ok(expr);
    }

    fn parse_left_curly(&mut self) -> Result<ExprAST> {
        self.next()?;
        let mut m = Vec::new();
        if self.cur_tok().is_right_curly() {
            return Ok(ExprAST::Map(m));
        }
        loop {
            let k = self.parse_primary()?;
            self.expect(":")?;
            let v = self.parse_expression()?;
            m.push((k, v));
            if self.cur_tok().is_right_curly() {
                self.next()?;
                break;
            }
            self.expect(",")?;
        }
        Ok(ExprAST::Map(m))
    }

    fn parse_left_bracket(&mut self) -> Result<ExprAST> {
        self.next()?;
        let mut exprs = Vec::new();
        if self.cur_tok().is_right_bracket() {
            self.next()?;
            return Ok(ExprAST::List(exprs));
        }
        loop {
            exprs.push(self.parse_expression()?);
            if self.cur_tok().is_right_bracket() {
                self.next()?;
                break;
            }
            self.expect(",")?;
        }
        Ok(ExprAST::List(exprs))
    }

    fn parse_operator(&mut self, op: String) -> Result<ExprAST> {
        self.next()?;
        Ok(ExprAST::Unary(op, Arc::new(self.parse_primary()?)))
    }

    fn parse_function(&mut self, name: String) -> Result<ExprAST> {
        self.next()?;
        self.expect("(")?;
        let mut ans = Vec::new();
        if self.cur_tok().is_right_paren() {
            self.next()?;
            return Ok(ExprAST::Function(name, ans));
        }
        let mut has_right_paren = false;
        loop {
            ans.push(self.parse_expression()?);
            if self.cur_tok().is_right_paren() {
                has_right_paren = true;
                self.next()?;
                break;
            }
            self.expect(",")?;
        }
        if !has_right_paren {
            return Err(Error::NoRightBrace(0));
        }
        Ok(ExprAST::Function(name, ans))
    }
}

#[test]
fn test() {
    let input = "{1:2+3*2};1+2*3";
    let ast = AST::new(input);
    match ast {
        Ok(mut a) => {
            let expr = a.parse_expression().unwrap();
            println!("{}", expr)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[test]
fn test_exec() {
    let input = "\"abcdsaf\" endWith \"acd\"";
    let ast = AST::new(input);
    let mut ctx = Context::new();
    ctx.set_variable(&"mm".to_string(), Value::from(12.0_f64));
    match ast {
        Ok(mut a) => {
            let expr = a.parse_expression().unwrap();
            println!("expr is {}", expr);
            println!("string is {}", expr.expr());
            let ans = expr.exec(Arc::new(ctx)).unwrap();
            println!("ans is {}", ans);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
