use criterion::{criterion_group, criterion_main, Criterion, black_box};
use expression_engine::{parse_expression, ExprAST};

fn bench_display_expression(c: &mut Criterion) {
    // A complex expression that involves Function, List, Map, and Stmt (chain)
    let input = "
        a = 1;
        b = 2;
        func_call(
            [a, b, 3, 4],
            {
                'key1': a + b,
                'key2': [1, 2, 3]
            },
            nested_func(a, b)
        );
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ";

    // Parse it once to get the AST
    let ast = parse_expression(input).expect("Failed to parse expression");

    c.bench_function("display_expression", |b| {
        b.iter(|| {
            // Measure the time it takes to format the AST
            format!("{}", black_box(&ast))
        })
    });
}

criterion_group!(benches, bench_display_expression);
criterion_main!(benches);
