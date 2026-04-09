# Execution-only benchmark: AST vs Bytecode VM

Command:

```bash
cargo bench --bench execute_expression
```

Method:

- Parse each expression once to AST.
- Compile the AST once to bytecode `Program`.
- Benchmark **execution only**:
  - `ast.exec(&mut ctx)`
  - `bytecode::execute_program(&program, &mut ctx)`
- Create a fresh context per iteration for both paths.

## Results (Criterion, median)

| Scenario | AST exec | Bytecode exec | Delta (Bytecode vs AST) |
| --- | ---: | ---: | ---: |
| short_expression | 598.27 ns | 608.04 ns | +1.63% |
| long_expression | 2.5331 µs | 2.5760 µs | +1.69% |
| function_call | 719.02 ns | 663.25 ns | -7.76% |
| list_map_mix | 846.15 ns | 715.54 ns | -15.43% |
| ternary_chain | 490.74 ns | 518.99 ns | +5.76% |

## Takeaways

- At pure execution level, bytecode is **not uniformly slower**.
- Bytecode is slightly slower in arithmetic-heavy and ternary cases in this run.
- Bytecode is faster in function-call and list/map construction cases in this run.
- The previous end-to-end slowdown mostly comes from the extra **compile stage per call** in `execute()` (parse + compile + run).
