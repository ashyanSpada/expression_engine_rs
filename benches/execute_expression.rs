use criterion::{criterion_group, criterion_main, Criterion};
use expression_engine::{create_context, execute, parse_expression, Value};

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

criterion_group!(benches, bench_execute_expression, bench_parse_expression);
criterion_main!(benches);
