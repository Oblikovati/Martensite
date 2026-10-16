# martensite-tkmath test fixtures

These files are reviewed baselines and inputs. The tests read them with
`include_str!` (or from `CARGO_MANIFEST_DIR`) and compare the crate's output
with them. No test reads a file outside this crate.

## Contents

- `expected/*.expected`: the expected output of one case each, named after the
  test that reads it.
- `certified_bezier_pairs/recorded.tsv`: ordinary floating-point pair
  intersections. They supplement the exact certificates and never define them.
- `prism_complete_measures/`: exact area and sweep expectations of a closed
  cubic boundary; `generate.py` recomputes them with Python `Fraction`
  arithmetic.
- `extended_sqrt_x87.txt` and `extended_complex_x87.txt`: x87 extended
  precision results of `sqrtl` and of complex arithmetic, as GCC and glibc
  compute them on x86-64.
- `mathutils_random.txt`: the integer and floating-point stream of the seeded
  random generator.
- `multiple_partial_output_trace.txt`: the callback trace of the multiple root
  search.
- `scalar-triple-product-exact.json`: exact signs of scalar triple products.
- `bezier_wire.rs`: shared wire inputs of the Bezier line tests.

Other test directories keep their own corpora next to the tests that read
them, with a README where a generator exists: `exact_comparisons/`,
`exact_orientation/`, `exact_roots/bezier_line/incidence_assets/` and
`sketch_line_incidences/`. Files named `oracle*` come from independent
generators (exact rational arithmetic or Jacobian); files named
`recorded-ordinary*` are reviewed baselines of ordinary floating-point
results.

## Changing a baseline

A baseline changes only through a reviewed update that states the reason and
the behavior version (AGENTS.md, "Verification and completion"). Do not
regenerate a file to make a test pass.
