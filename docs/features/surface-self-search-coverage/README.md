# Surface self-search coverage

Surface self-search operations retain an owned `SurfaceSelfSearchReceipt` after success, error or cancellation. This adds the validation-coverage part of feature #213 for this operation; the broader feature remains open.

The receipt records the receiver's invocation number, candidate-discovery completion, and scheduled, visited, finished, pruned, failed and cancelled calls. Section lines and tangent zones are counted when discovery returns them. Start refinement and marching calls are counted where they run. Marching steps and existing limit events are retained even when the numerical method returns normally.

`Finished` means the observed scheduled work is accounted for. Candidate discovery uses a sampled surface polyhedron, so that status does not certify global absence of self-intersections. The existing analytic/extrusion shortcut is reported separately as `AnalyticPolicy`.

Nested numerical routines currently expose final flags without all attempted termination conditions. Calls through that boundary increment `calls_with_unobserved_inner_work` and leave the receipt `Incomplete`. A caller must retain that uncertainty. A normal return, an empty result or `is_done()` cannot erase it.

Existing entry points keep their geometric behavior. The additive `perform_self_with_progress` methods accept a progress range and poll it before discovery, before dispatched work and during marching. Cancellation retains the work already observed and returns an error. Results and counters belong to the operation instance; a later invocation replaces its receipt, while a cloned earlier receipt remains unchanged.

```rust,ignore
search.perform_self_with_progress(&surface, &domain, tolerances, &range)?;
let coverage = search.self_search_receipt().clone();
```

The front-end intersection object also retains the receipt when its worker returns an error. Receipt fields do not supply a numerical error bound or change search tolerances. Independent tests use a plane patch and a cubic patch with a closed-form interior crossing, plus explicit cancellation and unavailable-surface controls.
