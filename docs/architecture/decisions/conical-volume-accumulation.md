# Bounded conical volume accumulation

Status: approved for implementation under #570 on 2026-11-19; runtime qualification remains pending.

The operand-size diagnostic recorded O. member with 137 spans reference-volume sum operands reaching
7500/7493 numerator/denominator bits before reduction and 7115/7108 afterward.
Its term-input maxima were 219/209 bits. These measurements establish growth in
the running sum; they do not isolate its runtime cost. The retained evidence is
[the operand-size receipt](../../features/surface-normal-sketches/references/volume-bits-attribution-2026-11-19/summary.json).

Conical reference-volume and analytic-cone flux now admit parent integration
behavior version 2. Version 1 remains selectable and is the default. Standalone
boundary/face integrals, integral programs and cylindrical flux retain their
existing version 1 admission and exact sums. The existing shared `BoundsSum`
implementation is unchanged.

Parent version 2 selects a private conical sum adapter. Each exact contribution is already
weighted and oriented in volume units when it reaches the adapter. For precision
`p`, its lower endpoint is rounded down and its upper endpoint up onto the grid
`2^-p`. Same-grid addition is exact. After `N` contributions, each endpoint has
at most `N*2^-p` extra allowance; the possible width increase is at most twice
that. The unchanged term integrator still produces each original contribution
interval and enforces its original width share.

The adapter reuses bounded dyadic arithmetic: two integer captures and one
division per endpoint, followed by one interval addition. Those seven actual
calls per contribution use the operation's remaining arithmetic grant. Exhausted
work, oversized integers and cancellation return the original typed cause. The
adapter does not increase precision or retry with an exact sum after refusal.
The final outward publication and the operation's volume gate still enforce
the requested caps. An error radius, an emitted interval width and a term's
integration width retain their separate meanings.

The parent receipt keeps requested and effective version 2. Its scalar quotient
integrator is explicitly algorithm 1; local conical routing constructs that child
request without changing the shared cylinder engine. Source binding includes
the parent selection and precision. Analytic-cone version 1 keeps its existing
geometry-only matcher across valid resource policies; version 2 also binds every
resource-policy bit, including signed zero.

`AnalyticConicalFluxError` was introduced in unreleased capability commit
`7a2aa081`; it is absent from main and published `v0.13.1`. Before first delivery,
its contract now retains actual arithmetic operations and examined cells on
failure, plus a typed arithmetic cause with span/cap context. Sketch charges
that actual work once and preserves the original leaf error if bookkeeping also
fails. Version 1 error Debug retains its earlier field layout. No published
error schema or toolkit API is migrated by this change.

Acceptance requires independent signed rational sums and containment, cap/work/
bit refusals, source-policy mutation and cancellation controls, unchanged
version 1 values/work/callbacks, and the complete O. operation in both senses
under the original one-million-unit grant and original limits. Matched timing
and memory measurements follow correctness qualification. No speedup is claimed
by this decision.
