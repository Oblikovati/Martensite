# Raw tolerance admission (#490)

## Reproduction

On main commit `d2de32ffd`, a radius-10, height-4 cylinder with a stored vertex
tolerance of NaN retained that NaN after the body's native topology and geometry
adapters. Running the native self-intersection adapter on a fresh copy of the same
fixture changed the stored value to `1e-7` and returned no findings. Each stage used
a separately constructed cylinder. The diagnostic used the actual private adapters
called by `body_check::validate_body`, including contextual analyzer queries.

The existing tolerance-value checker read `BRepTool`'s compatibility getter, which
floors NaN and negative stored values. Reading raw storage at the final stage alone
also misses values already overwritten by the self-checker. Both paths require
checked admission before native validation starts.

## Result contract

Admission reads the existing indexed vertex, edge and face records. Invalid raw
values produce `FindingKind::InvalidTolerance`, naming the exact indexed entity and
preserving the offending value. The tolerance verdict is `Failed` with `Exact` mode.
The three native stages stay `NotRun`; no earlier check is claimed to have passed.
An unreadable required record produces an inconclusive tolerance finding.

Finite nonnegative values retain the compatibility floor for metric comparisons.
The ordered topology, geometry, self-intersection and tolerance checks still run
for admitted bodies. Public signatures and `BRepTool` getter behavior are unchanged.
The admission guard does not alter the native checker or Boolean ownership rules.

## Regression scope

The focused controls cover vertex, edge and face raw values of NaN, negative one,
and both infinities under each cap policy. They compare the complete serialized
body and the raw tolerance bits before and after refusal. Separate controls retain
the effective floor for zero and subnormal values, cap comparisons, and tolerance
nesting. Diagnostic-only probe code is removed after investigation.
