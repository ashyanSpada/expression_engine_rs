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
| short_expression | 597.55 ns | 594.13 ns | -0.57% |
| long_expression | 2.5106 µs | 2.5698 µs | +2.36% |
| function_call | 695.27 ns | 655.10 ns | -5.78% |
| list_map_mix | 864.45 ns | 679.70 ns | -21.37% |
| ternary_chain | 493.75 ns | 494.15 ns | +0.08% |

## Takeaways

- At pure execution level, bytecode is **not uniformly slower**.
- Bytecode is close to parity for short-expression and ternary-chain cases in this run.
- Bytecode is slower in the long arithmetic-heavy case in this run.
- Bytecode is faster in function-call and list/map construction cases in this run.
- The previous end-to-end slowdown mostly comes from the extra **compile stage per call** in `execute()` (parse + compile + run).
