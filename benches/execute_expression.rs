use criterion::{criterion_group, criterion_main, Criterion};
use expression_engine::{create_context, execute, Value};

fn criterion_benchmark(c: &mut Criterion) {
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
