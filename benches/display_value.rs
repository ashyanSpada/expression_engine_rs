use criterion::{black_box, criterion_group, criterion_main, Criterion};
use expression_engine::Value;

fn bench_display_value(c: &mut Criterion) {
    let mut list = Vec::new();
    for i in 0..10 {
        list.push(Value::from(i));
    }

    let mut map = Vec::new();
    for i in 0..10 {
        map.push((Value::from(format!("key{}", i)), Value::from(i)));
    }

    let val = Value::List(vec![
        Value::from("hello"),
        Value::from(42),
        Value::from(true),
        Value::List(list),
        Value::Map(map),
    ]);

    c.bench_function("display_value", |b| {
        b.iter(|| format!("{}", black_box(&val)))
    });
}

criterion_group!(benches, bench_display_value);
criterion_main!(benches);
