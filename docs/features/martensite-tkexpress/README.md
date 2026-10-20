# Symbolic expressions

`martensite-tkexpress` provides shared expression trees, symbolic differentiation,
simplification, named functions, equations and relation systems. Its text interpreter
supports assignments, function definitions, derivatives, sums and products (#100).

```rust
use martensite_tkexpress::expr_intrp::ExprIntrpGenExp;
use martensite_tkernel::tcollection::AsciiString;

let interpreter = ExprIntrpGenExp::create();
interpreter.process(&AsciiString::new("2+3"));
assert!(interpreter.is_done());
assert_eq!(interpreter.expression()?.evaluate_numeric()?, 5.0);
# Ok::<(), martensite_tkernel::standard::Failure>(())
```

## Modules and ownership

- `expr` contains expression nodes, arithmetic constructors, functions and relations.
- `expr_intrp` contains scanning, parsing and invocation-local semantic state.

Handles share mutable expression nodes; cloning a handle preserves node identity.
An interpreter retains named expressions and functions between calls. Separate
interpreters have independent parser state. A successful assignment or definition
may have no expression result. Check `is_done()` before retrieving the result;
evaluating unresolved unknowns produces a typed failure. Scanner output defaults
to standard output and can be redirected with `set_scanner_output`.

This API does not publish immutable modelling results. Callers must account for
shared node mutation and must not assume that cloned handles isolate edits.

Message-only failures honor the foundation stack-capture setting (#141). An
explicit stack is retained exactly, including an explicit empty stack. The
stack-policy regression runs in a separate executable because the setting is
process-wide.

Relation-system `remove` always returns a failure and leaves the system
unchanged. Build a new system to select a subset of relations.

## Verification

Run `cargo test -p martensite-tkexpress`. The tests cover expression construction,
differentiation, simplification, interpreter boundaries and state isolation.

The frozen corpus (numerical behavior version 2, with correctly rounded
elementary functions, [#201](../portable-transcendentals/attribution/README.md))
retains 7,340 request/result pairs: 3,318
text sequences, 2,200 unary cases, 1,800 binary cases, 13 relation cases and nine
direct API scenarios. It includes 3,000 invalid-token requests with seed 20261008.
Replay runs in process using compiled fixtures. Finite values, infinity, signed
zero, expression bytes, whitespace and failure categories compare exactly; only
NaN sign and payload are normalized. Baseline updates require review stating
the reason and behavior version.

Independent checks cover a quartic and five derivatives at 25 dyadic points,
eight elementary-function values and derivatives, three integer/dyadic sums and
products, and interpreter assignment isolation. All numeric comparisons in these
checks have zero metric tolerance. Ten repeated literal parses are a parsing
smoke check, not proof of leak freedom.

The initial preparation has been executed on Linux x86_64. The final candidate
requires the full supported platform matrix before release.
