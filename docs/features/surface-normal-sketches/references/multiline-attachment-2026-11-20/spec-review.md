# Spec review: #425 multiline attachment controls

Reviewed only the two-path test delta against B1 checkpoint `da08ca1a`, with main merge-base `b61a031d0bde0e21d283e0f94975f79b9ae15574`. Both files match `/tmp/martensite-b1-current-cylinder-text-20261120/multiline-attachment-source.json`, SHA256 `2ca08114fb53baccd6886b51a97cbf0e63ec485f1411c3afb8bdfda0a5a910a1`, before and after inspection.

Verdict: **one coverage blocker; no production change reviewed.**

The requirement is that “every authored region LastDepthFace survives by semantic key + Boolean history/image.” In `body_controls/multiline.rs::retained_cap_has_output_image`, the acceptance predicate allows any generated/modified image present in the output. It never requires that image to be a face. Consequently, a lost cap with a surviving generated edge/vertex could satisfy the test. Require the selected source to be Face and require either that face itself or a Face history image in the output. Preserve the existing all-image membership/history checks. This is a test-proof gap, not evidence that production currently loses caps.

The remaining scope matches the request: complete 239-curve/18-region text, existing outward Fuse and inward Cut APIs, explicit original 8192 work, semantic Region/LastDepthFace lookup for every selection, full ordered body validation and one solid, all tool entity history, non-destructive input/tool snapshots, immutable encoded result and request equality. Neither production code/defaults nor acceptance tolerances change.

I independently recomputed the exact Green integral directly from the 239 recorded polynomial curves, scaled by 1/1024 in both axes: area `42000445/6291456`. Multiplying by the respective radial-depth integrals `81/320` and `79/320` gives exactly `226802403/134217728` and `663607031/402653184`. Thus both authored-volume constants and the 79/81 ratio are independent and correct. The attached-body volume check establishes direction only; it is not presented as an exact Boolean-volume oracle.

The two new tests are still running; no pass, timing, memory or completed-capability claim follows from this source review. After correcting the face-survival predicate, freeze and rerun both controls. Preserve first-run results as evidence for the weaker candidate, not qualification of the corrected source.

## Corrected freeze review

Final source/spec verdict: **ACCEPT; original coverage finding resolved.** Both paths match refreshed manifest SHA256 `9131610a7461d192b01cf1b4fe565182e1616ccc088beda607bc24881e796a78`. The guard now asserts source Face and accepts only a Face present in output, including modified/generated images. Existing all-image history checks remain. The independent volume review is unchanged. The first weaker pair passed, but corrected tests are rerunning; no corrected-source pass or performance acceptance is claimed.

Final typed-source verdict: **ACCEPT**, freeze `5b0fe8f18066e30d4e04574067c6de75ffd0acd8c1a160db2c0bae99247c1605`. Prior face-guard candidate failed compilation: shape_type returns Result. Corrected unwrap/Ok(Face) handling preserves face-only acceptance. Attempt4 records both tests passing; 207.16 seconds is execution duration, not performance acceptance. Independent numerical proof remains valid.
