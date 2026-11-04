# Surface self-search coverage

Surface self-search operations retain an owned `SurfaceSelfSearchReceipt` after success, error or cancellation. This adds the validation-coverage part of feature #213 for this operation; the broader feature remains open.

The receipt records the receiver's invocation number, candidate-discovery completion, and scheduled, visited, finished, pruned, failed and cancelled calls. Section lines and tangent zones are counted when discovery returns them. Start refinement and marching calls are counted where they run. Marching steps and existing limit events are retained even when the numerical method returns normally.

`Finished` means the observed scheduled work is accounted for. Candidate discovery uses a sampled surface polyhedron, so that status does not certify global absence of self-intersections. The existing analytic/extrusion shortcut is reported separately as `AnalyticPolicy`.

The receipt counts the actual isoparametric numerical solves used by refinement and marching. It retains accepted solves, residual rejection, local decline, evaluation decline, divergence, exhausted iteration budgets and errors, together with the outer iterations entered. Vanishing-slope and repeated-progress stops remain local declines even when the numerical solver's existing result flag accepts them. Root-search declines, declined evaluations and exhausted required budgets leave the self-search receipt `Incomplete`. The `scalar_result_declined` counter records rejections by the optimization improvement filter. Those rejections do not, by themselves, make otherwise accounted work incomplete.

The separate tangency-neighbour extrema search still lacks complete termination observations. When that path is entered it increments `calls_with_unobserved_inner_work` and leaves the receipt `Incomplete`. A caller must retain that uncertainty. A normal return, an empty result or `is_done()` cannot erase it. The local solver's additive `search_observation()` accessor is available even when no root was accepted; it does not change root selection or certify that all roots were found.

Existing entry points keep their geometric behavior. The additive `perform_self_with_progress` methods accept a progress range and poll it before discovery, before dispatched work and during marching. Cancellation retains the work already observed and returns an error. Results and counters belong to the operation instance; a later self-search replaces its receipt, while a cloned earlier receipt remains unchanged. Read the receipt immediately after the intended self-search call. Pair-search entry points retain the last self-search receipt. The invocation number identifies work on that receiver, rather than providing a global identity.

```rust,ignore
search.perform_self_with_progress(&surface, &domain, tolerances, &range)?;
let coverage = search.self_search_receipt().clone();
```

The front-end intersection object also retains the receipt when its worker returns an error. Receipt fields do not supply a numerical error bound or change search tolerances. Independent tests use a plane patch and a cubic patch with a closed-form interior crossing, plus explicit cancellation and unavailable-surface controls.
