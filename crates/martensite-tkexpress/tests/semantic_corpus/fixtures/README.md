# Expression semantics corpus, behavior version 1

This corpus has 7,340 complete request/result pairs:

| Request family | Cases |
| --- | ---: |
| Interpreter text and stateful sequences | 3,318 |
| Unary functions | 2,200 |
| Binary operators | 1,800 |
| Relations | 13 |
| Direct expression operations | 9 |

The text family includes 3,000 token-fuzz requests with seed 20261008 and explicit
parser stack boundaries. Text payloads use hexadecimal bytes prefixed by `x`;
floating values use unsigned decimal bit patterns. The request files are inputs,
never executable scripts.

Tests invoke the Rust implementation directly. Finite numbers, infinities,
signed zeros, expression bytes and exception categories compare exactly. Only
NaN payload and sign are normalized, preserving the corpus comparison policy.
The raw expected payloads remain recorded.

These are frozen semantic expectations. Independent mathematical checks live in
`independent_math.rs`; a symbolic text baseline alone is not a numerical proof.
Baseline updates require review stating the reason and behavior version.
