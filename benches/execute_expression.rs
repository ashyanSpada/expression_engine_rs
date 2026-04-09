use criterion::{black_box, criterion_group, criterion_main, Criterion};
use expression_engine::{bytecode, create_context, execute, parse_expression, Value};
use std::sync::Arc;

fn bench_execute_expression(c: &mut Criterion) {
    let input = "c = 5+3; c+=10+f; c";
    c.bench_function("execute_expression", |b| {
        b.iter(|| {
            execute(
                input,
                create_context!(
                "d" => 2,
                "b" => true,
                "f" => Arc::new(|_| Ok(Value::from(3)))
                ),
            )
        })
    });
}

fn bench_parse_expression(c: &mut Criterion) {
    let input = "c = 5+3; c+=10+f; c";
    c.bench_function("parse_expression", |b| b.iter(|| parse_expression(input)));
}

fn create_bench_context(with_func: bool) -> expression_engine::Context {
    if with_func {
        create_context!(
            "d" => 2,
            "b" => true,
            "f" => Arc::new(|_| Ok(Value::from(3)))
        )
    } else {
        create_context!(
            "d" => 2,
            "b" => true
        )
    }
}

fn bench_execution_only_ast_vs_bytecode(c: &mut Criterion) {
    let scenarios = [
        ("short_expression", "1+2*3-4", false),
        (
            "long_expression",
            "2+3*5-2/2+6*(2+4)-20+1+2+3+4+5+6+7+8+9+10",
            false,
        ),
        ("function_call", "f(3)+2*f(4)", true),
        ("list_map_mix", "{'a':1+2, 'b':[1,2,3,4], 5: 6>2}", false),
        ("ternary_chain", "d > 1 ? (2 < 3 ? 11 : 12) : 13", false),
    ];

    for (name, expr, with_func) in scenarios {
        let ast = parse_expression(expr).unwrap();
        let program = bytecode::compile_expression(&ast).unwrap();
        let mut group = c.benchmark_group(format!("execution_only/{}", name));

        group.bench_function("ast_exec", |b| {
            b.iter(|| {
                let mut ctx = create_bench_context(with_func);
                black_box(ast.exec(&mut ctx).unwrap())
            })
        });

        group.bench_function("bytecode_exec", |b| {
            b.iter(|| {
                let mut ctx = create_bench_context(with_func);
                black_box(bytecode::execute_program(&program, &mut ctx).unwrap())
            })
        });
        group.finish();
    }
}

criterion_group!(
    benches,
    bench_execute_expression,
    bench_parse_expression,
    bench_execution_only_ast_vs_bytecode
);
criterion_main!(benches);
