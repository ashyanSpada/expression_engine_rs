use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidNumber(String),
    UnexpectedEOF(usize),
    UnterminatedString(usize),
    InvalidBool(usize),
    NotSupportedChar(usize, char),
    ReferenceNotExist(String),
    FunctionNotExist(String),
    NotSupportedOp(String),
    BinaryOpNotRegistered(String),
    UnaryOpNotRegistered(String),
    InnerFunctionNotRegistered(String),
    ShouldBeNumber(),
    ShouldBeBool(),
    ParamInvalid(),
    ShouldBeString(),
    InvalidTernaryExprNeedColon(),
    ExpectedOpNotExist(String),
    WrongContextValueType(),
    UnexpectedToken(),
    NotReferenceExpr,
    NoOpenDelim,
    NoCloseDelim,
    InvalidOp(String),
    InvalidInteger,
    InvalidFloat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidNumber(s) => write!(f, "invalid number: {}", s),
            UnexpectedEOF(start) => write!(f, "unexpected eof: {}", start),
            UnterminatedString(start) => write!(f, "unterminated string: {}", start),
            InvalidBool(start) => write!(f, "invalid bool: {}", start),
            NotSupportedChar(start, ch) => write!(f, "not supported char: {}, {}", start, ch),
            ReferenceNotExist(name) => write!(f, "reference not exist: {}", name),
            FunctionNotExist(name) => write!(f, "function not exist: {}", name),
            NotSupportedOp(op) => write!(f, "not supported op: {}", op),
            BinaryOpNotRegistered(op) => write!(f, "binary op not registered: {}", op),
            UnaryOpNotRegistered(op) => write!(f, "unary op not registered: {}", op),
            InnerFunctionNotRegistered(name) => {
                write!(f, "inner function not registered: {}", name)
            }
            ShouldBeNumber() => write!(f, "should be number"),
            ShouldBeBool() => write!(f, "should be bool"),
            InvalidTernaryExprNeedColon() => write!(f, "invalid ternary expr needs colon"),
            ExpectedOpNotExist(op) => write!(f, "expected op:{} not exist", op.clone()),
            ParamInvalid() => write!(f, "param invalid"),
            ShouldBeString() => write!(f, "should be string"),
            WrongContextValueType() => write!(f, "wrong context value type"),
            UnexpectedToken() => write!(f, "unexpected token"),
            NotReferenceExpr => write!(f, "not reference expr"),
            NoOpenDelim => write!(f, "no open delim"),
            NoCloseDelim => write!(f, "no close delim"),
            InvalidOp(op) => write!(f, "invalid op {}", op),
            InvalidInteger => write!(f, "invalid integer"),
            InvalidFloat => write!(f, "invalid float"),
        }
    }
}
