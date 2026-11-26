# QA impact investigation (#492)

The existing planner selected Vulkan and eleven dependents for #471's test-control
ownership relocation on main `b61a031d`. The relocation changes no published
Vulkan implementation, but the planner has no proof of QA-only input ownership.
#471 remains pending a scope fix; those eleven unaffected suites were not run.

A temporary traversal compared the frozen #471 v5 inputs with main using cached
`git cat-file --batch` snapshots. Its first pass visited 583 lexical file contexts
and 204 literal resources per revision in about 1.086 seconds, with five unknowns
from unsupported cfg fields and statements. Retaining that content conservatively
increased traversal to 599 contexts and kept 204 resources. The remaining token
footprints matched; the two revisions took about 0.935 seconds together. These
are diagnostic planner observations, not kernel performance measurements.

Reusing the existing macro input-extension analysis classified 45 shader input
expressions as disjoint from the changed Rust, TOML and Markdown paths. That pass
took about 2.169 seconds. Imported and aliased macro authority remained unresolved,
including `retained_view_methods!`, `impl_transient!` and optional `serde_json::json!`.
Cross-crate QA input ownership was not established. Matching footprints therefore
did not establish Vulkan-only selection. Opaque/generated inputs retain the
existing conservative propagation; no macro-name waiver was introduced.

The protected #413 work was inspected read-only. Its snapshot/context-key design
was useful background; its unfinished parser dependency and macro allowlists were
not copied. Main already supplies proc-macro2 as a normal build-driver dependency.
No dependencies, production code or selector behavior changed.

The logs and hash receipts here preserve the diagnostic result and its limits.
The temporary traversal source and executables were removed after verified copies
were retained. The clean #492 worktree had no commits or changes and was removed.
[Bug #492](https://github.com/Oblikovati/Martensite/issues/492) remains open, related
to #413 and #471.
