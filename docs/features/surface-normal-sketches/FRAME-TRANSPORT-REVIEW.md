# Frame transport helper review

The review covered the staged coefficient-only helper against pinned main
merge-base `fb1217550ed9277f697f14a96a53cb19b8823770`. The public contract and
`OBLIQUE-FRAMES.md` supplied the scope; native admission, translation, domain
extent and placed-body publication were explicitly excluded.

The independent Spec review found no correctness findings. It checked exact
reference construction, denominator positivity, symmetric Gram bounds,
determinant sign, outward rounding and Frobenius endpoint error. Jacobian
independently confirmed the dyadic fixture determinant `63/64`.

The Standards review found one documented gap: compile-time Send/Sync controls
were missing for `FrameTransportInput`, `FrameTransportField` and
`FrameTransportQuantity`. All three were added to the existing control and
verified by all-target clippy. The optional suggestion to name the private
three-item qualification tuple was not adopted: it has one producer and one
consumer, with named destructuring and independent quantity checks in tests.

Sixteen focused numerical tests passed. Strict all-target TKMath and sketch
clippy passed for the composition; final TKMath clippy also compiled the added
public-type assertions. Both reviews were read-only and ran no Cargo commands.
This helper review does not qualify the complete oblique operation.

## Point-domain spatial budget

The next review covered the uncommitted spatial-budget extension above local
checkpoint `7f87285c`, with main merge-base
`77053b4cd0f6e223d94f20b30929df15466dfc09`. Its scope was the point-domain
allocation in `OBLIQUE-FRAMES.md`; physical body placement remained excluded.

Spec review found no correctness issue in translation, affine evaluation,
downward allocation or ownership. It verified the evaluation order against
`gp::dot`, `Matrix3::apply` and `Transform3::apply_coordinates` at unit scale.
Another scale or evaluation path needs separate correspondence evidence.

Standards review found that spatial-input diagnostics omitted the required
scalar ranges and that the combined origin/translation admission test exceeded
the function-size limit. The diagnostics now state those ranges and the
positive-allocation requirement; the admission tests are separate. The source
metrics script reports no function above 20 lines and no file above 500 lines
in the scoped files. These measurements use the repository's approximate
brace-counting script.

All 32 focused frame tests passed with no failures or ignored tests. The suite
includes the original 16 frame controls, exact translation and permutation
oracles, 125-point exact evaluation-error comparisons, 125 checked native rigid
transform evaluations, subnormal and overflow controls, large-origin refusal,
owned input and concurrent equality. The new public types have compile-time
Send/Sync assertions. The native control uses the coefficient-preserving
constructor from #451 and confirms unit scale before comparing evaluation bits.
Strict all-target TKMath clippy passed. Standards review verified both
corrections and found no new blocker. Documentation received the required
humanizer pass.

The conditional point-domain premise is not a completed kernel acceptance
claim. The operation must establish the domain for its ideal and retained
geometry, bind the frozen decoded coefficients, transport reports and validate
the placed body. Oblique frames remain refused at the operation entry point.
