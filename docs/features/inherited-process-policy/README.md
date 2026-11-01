# Inherited process policies

`CaseSpec::inherit_process_policy` runs an isolated case while preserving the
process policy it inherits. Existing finite OS limits stay in effect. The runner
leaves the CPU limit, signal handling and elapsed watchdog untouched (#351).

Use the exact registered test identity, as with a finite case:

```rust,no_run
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};

#[test]
fn inherited_case() {
    let root = std::env::temp_dir().join("owned-case-records");
    let spec = CaseSpec::inherit_process_policy(
        "inherited_case",
        FactorProfile::RequireUnset,
    );
    CaseRunner::new(&root, None).run(spec, || {
        assert_eq!(2 + 2, 4);
        Ok(())
    }).unwrap();
}
```

The same specification works with `run_prepared`. Preparation runs before the
policy boundary; the body receives its owned result once. Inheritance leaves
any process settings made by that preparation in place.

`FactorProfile::RequireUnset` keeps its environment contract: a present
`CSF_CPULIMIT_FACTOR`, including an empty value, causes refusal. The inherited
mode performs no factor calculation.

The caller owns any surrounding supervisor deadline. Inheritance records no
requested CPU or elapsed duration.

## Retained records

Finite cases retain their version 1 request and receipt bytes. Inherited cases
use the `MARTENSITE_EXACT_CASE_V2` request header with
`process_policy=inherit`. Their schema 2 receipt has `budget_seconds: null`,
`process_policy: "inherit"` and `runner_policy_installed: false`.

The `policy` word in the lifecycle identifies the boundary between preparation
and the body. The receipt states which action the runner took there. Entry,
preparation and completion witnesses still bind the exact request, executable
and invocation directory. The parent finalizes logs after the child exits.

The application limit report starts at 0 in a fresh process and becomes -1 after
an explicit removal. Query the OS to measure inherited resource limits; the
application report records only requests made through Martensite. A body can
still make its own explicit process-policy request.

[Verification](verification.md) describes the controls and their platform scope.
