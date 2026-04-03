use criterion::{black_box, criterion_group, criterion_main, Criterion};
use expression_engine::Value;
use rust_decimal::Decimal;

fn value_display_benchmark(c: &mut Criterion) {
    let mut map_vec = Vec::new();
    for i in 0..10 {
        map_vec.push((
            Value::String(format!("key{}", i)),
            Value::Number(Decimal::from(i)),
        ));
    }
    let val = Value::Map(map_vec);

    c.bench_function("value_display", |b| {
        b.iter(|| {
            let _ = format!("{}", black_box(&val));
        })
    });
}

criterion_group!(benches, value_display_benchmark);
criterion_main!(benches);
