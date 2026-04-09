use crate::context::Context;
use crate::define::Result;
use crate::error::Error;
use crate::function::InnerFunctionManager;
use crate::operator::{InfixOpManager, InfixOpType, PostfixOpManager, PrefixOpManager};
use crate::parser::{ExprAST, Literal};
use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    PushConst(usize),
    LoadReference(String),
    CallFunction {
        name: String,
        arg_count: usize,
    },
    ApplyPrefix(String),
    ApplyPostfix(String),
    ApplyInfix {
        op: String,
        setter_target: Option<String>,
    },
    BuildList(usize),
    BuildMap(usize),
    JumpIfFalse(usize),
    Jump(usize),
    Pop,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
}

impl Program {
    fn validate(&self) -> Result<()> {
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::JumpIfFalse(target) | Instruction::Jump(target) => {
                    if *target > self.instructions.len() {
                        return Err(Error::UnexpectedToken());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub fn compile_expression(ast: &ExprAST<'_>) -> Result<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)?;
    compiler.program.validate()?;
    Ok(compiler.program)
}

pub fn execute_program(program: &Program, ctx: &mut Context) -> Result<Value> {
    VirtualMachine::new(program).run(ctx)
}

struct Compiler {
    program: Program,
}

impl Compiler {
    fn new() -> Self {
        Self {
            program: Program {
                instructions: Vec::new(),
                constants: Vec::new(),
            },
        }
    }

    fn compile(&mut self, ast: &ExprAST<'_>) -> Result<()> {
        self.compile_expr(ast)
    }

    fn compile_expr(&mut self, ast: &ExprAST<'_>) -> Result<()> {
        match ast {
            ExprAST::Literal(literal) => {
                let value = match literal {
                    Literal::Number(v) => Value::from(*v),
                    Literal::Bool(v) => Value::from(*v),
                    Literal::String(v) => Value::from(*v),
                };
                let idx = self.push_const(value);
                self.emit(Instruction::PushConst(idx));
            }
            ExprAST::Reference(name) => {
                self.emit(Instruction::LoadReference((*name).to_string()));
            }
            ExprAST::Function(name, args) => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.emit(Instruction::CallFunction {
                    name: (*name).to_string(),
                    arg_count: args.len(),
                });
            }
            ExprAST::Unary(op, rhs) => {
                self.compile_expr(rhs)?;
                self.emit(Instruction::ApplyPrefix((*op).to_string()));
            }
            ExprAST::Binary(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let setter_target = match lhs.as_ref() {
                    ExprAST::Reference(name) => Some((*name).to_string()),
                    _ => None,
                };
                self.emit(Instruction::ApplyInfix {
                    op: (*op).to_string(),
                    setter_target,
                });
            }
            ExprAST::Postfix(lhs, op) => {
                self.compile_expr(lhs)?;
                self.emit(Instruction::ApplyPostfix(op.clone()));
            }
            ExprAST::Ternary(condition, lhs, rhs) => {
                self.compile_expr(condition)?;
                let jump_if_false = self.emit_placeholder_jump_if_false();
                self.compile_expr(lhs)?;
                let jump_end = self.emit_placeholder_jump();
                let rhs_start = self.program.instructions.len();
                self.patch_jump_if_false(jump_if_false, rhs_start);
                self.compile_expr(rhs)?;
                let end = self.program.instructions.len();
                self.patch_jump(jump_end, end);
            }
            ExprAST::List(values) => {
                for value in values {
                    self.compile_expr(value)?;
                }
                self.emit(Instruction::BuildList(values.len()));
            }
            ExprAST::Map(entries) => {
                for (k, v) in entries {
                    self.compile_expr(k)?;
                    self.compile_expr(v)?;
                }
                self.emit(Instruction::BuildMap(entries.len()));
            }
            ExprAST::Stmt(exprs) => {
                if exprs.is_empty() {
                    let idx = self.push_const(Value::None);
                    self.emit(Instruction::PushConst(idx));
                    return Ok(());
                }
                for (index, expr) in exprs.iter().enumerate() {
                    self.compile_expr(expr)?;
                    if index < exprs.len() - 1 {
                        self.emit(Instruction::Pop);
                    }
                }
            }
            ExprAST::None => {
                let idx = self.push_const(Value::None);
                self.emit(Instruction::PushConst(idx));
            }
        }
        Ok(())
    }

    fn emit(&mut self, instruction: Instruction) {
        self.program.instructions.push(instruction);
    }

    fn push_const(&mut self, value: Value) -> usize {
        self.program.constants.push(value);
        self.program.constants.len() - 1
    }

    fn emit_placeholder_jump_if_false(&mut self) -> usize {
        let index = self.program.instructions.len();
        self.emit(Instruction::JumpIfFalse(0));
        index
    }

    fn emit_placeholder_jump(&mut self) -> usize {
        let index = self.program.instructions.len();
        self.emit(Instruction::Jump(0));
        index
    }

    fn patch_jump_if_false(&mut self, index: usize, target: usize) {
        self.program.instructions[index] = Instruction::JumpIfFalse(target);
    }

    fn patch_jump(&mut self, index: usize, target: usize) {
        self.program.instructions[index] = Instruction::Jump(target);
    }
}

struct VirtualMachine<'a> {
    program: &'a Program,
    stack: Vec<Value>,
    ip: usize,
}

impl<'a> VirtualMachine<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            program,
            stack: Vec::new(),
            ip: 0,
        }
    }

    fn run(mut self, ctx: &mut Context) -> Result<Value> {
        while self.ip < self.program.instructions.len() {
            match &self.program.instructions[self.ip] {
                Instruction::PushConst(index) => {
                    let value = self
                        .program
                        .constants
                        .get(*index)
                        .ok_or_else(Error::UnexpectedToken)?
                        .clone();
                    self.stack.push(value);
                }
                Instruction::LoadReference(name) => {
                    self.stack.push(ctx.value(name)?);
                }
                Instruction::CallFunction { name, arg_count } => {
                    let mut args = Vec::with_capacity(*arg_count);
                    for _ in 0..*arg_count {
                        args.push(self.pop()?);
                    }
                    args.reverse();
                    let result = match ctx.get_func(name) {
                        Some(func) => func(args),
                        None => InnerFunctionManager::new().get(name)?(args),
                    }?;
                    self.stack.push(result);
                }
                Instruction::ApplyPrefix(op) => {
                    let rhs = self.pop()?;
                    let value = PrefixOpManager::new().get(op)?(rhs)?;
                    self.stack.push(value);
                }
                Instruction::ApplyPostfix(op) => {
                    let lhs = self.pop()?;
                    let value = PostfixOpManager::new().get(op)?(lhs)?;
                    self.stack.push(value);
                }
                Instruction::ApplyInfix { op, setter_target } => {
                    let rhs = self.pop()?;
                    let lhs = self.pop()?;
                    match InfixOpManager::new().get_op_type(op)? {
                        InfixOpType::CALC => {
                            let value = InfixOpManager::new().get_handler(op)?(lhs, rhs)?;
                            self.stack.push(value);
                        }
                        InfixOpType::SETTER => {
                            let target = setter_target.as_ref().ok_or(Error::NotReferenceExpr)?;
                            let value = InfixOpManager::new().get_handler(op)?(lhs, rhs)?;
                            ctx.set_variable(target, value);
                            self.stack.push(Value::None);
                        }
                    }
                }
                Instruction::BuildList(size) => {
                    let mut values = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        values.push(self.pop()?);
                    }
                    values.reverse();
                    self.stack.push(Value::List(values));
                }
                Instruction::BuildMap(size) => {
                    let mut values = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        let value = self.pop()?;
                        let key = self.pop()?;
                        values.push((key, value));
                    }
                    values.reverse();
                    self.stack.push(Value::Map(values));
                }
                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop()?;
                    match condition {
                        Value::Bool(false) => {
                            self.ip = *target;
                            continue;
                        }
                        Value::Bool(true) => {}
                        _ => return Err(Error::ShouldBeBool()),
                    }
                }
                Instruction::Jump(target) => {
                    self.ip = *target;
                    continue;
                }
                Instruction::Pop => {
                    self.pop()?;
                }
            }
            self.ip += 1;
        }

        if self.stack.is_empty() {
            return Ok(Value::None);
        }
        Ok(self.stack.pop().unwrap())
    }

    fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or_else(Error::UnexpectedToken)
    }
}

#[cfg(test)]
mod tests {
    use super::{compile_expression, execute_program, Instruction};
    use crate::{create_context, parse_expression, Value};
    fn run_ast(expr: &str) -> crate::Result<Value> {
        let ast = parse_expression(expr)?;
        let mut ctx = create_context!(
            "d" => 3,
            "f" => Arc::new(|_| Ok(Value::from(3)))
        );
        ast.exec(&mut ctx)
    }

    fn run_vm(expr: &str) -> crate::Result<Value> {
        let ast = parse_expression(expr)?;
        let program = compile_expression(&ast)?;
        let mut ctx = create_context!(
            "d" => 3,
            "f" => Arc::new(|_| Ok(Value::from(3)))
        );
        execute_program(&program, &mut ctx)
    }

    #[test]
    fn test_compile_ternary_has_jumps() {
        let ast = parse_expression("true ? 1 : 2").unwrap();
        let program = compile_expression(&ast).unwrap();
        assert!(program
            .instructions
            .iter()
            .any(|ins| matches!(ins, Instruction::JumpIfFalse(_))));
        assert!(program
            .instructions
            .iter()
            .any(|ins| matches!(ins, Instruction::Jump(_))));
    }

    #[test]
    fn test_vm_parity_success_cases() {
        let cases = [
            "2+3*5-2/2+6*(2+4)-20",
            "d+=3;d",
            "d-=2;d*5",
            "d*=0.1;d+1.5",
            "d/=2;d==1.5",
            "d<<=2;d",
            "2<=3?'haha':false",
            "2++ *3",
            "'hahhadf' beginWith \"hahha\"",
            "2 not in ['a', false, true, 1+2]",
            "[2>3,1+5,true]",
            "{'haha':2, 1+2:2>3}",
            "f(3)",
        ];

        for case in cases {
            let ast_result = run_ast(case).unwrap();
            let vm_result = run_vm(case).unwrap();
            assert_eq!(ast_result, vm_result, "expr: {}", case);
        }
    }

    #[test]
    fn test_vm_parity_error_cases() {
        let cases = ["5 / 0", "5 % 0", "+true", "true ? haha :", "2 = 3"];

        for case in cases {
            let ast_err = run_ast(case).unwrap_err().to_string();
            let vm_err = run_vm(case).unwrap_err().to_string();
            assert_eq!(ast_err, vm_err, "expr: {}", case);
        }
    }
}
