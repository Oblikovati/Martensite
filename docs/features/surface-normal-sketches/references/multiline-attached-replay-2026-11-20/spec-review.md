# Focused spec review: cylindrical attachment chart and multiline replay

Reviewed the six frozen paths against B1 head `8b5e6e2f4848adab9ffb0ecee505e0b3e61dbf03`. All hashes match `/tmp/martensite-multiline-attached-replay-20261120/source-r2.json`, SHA256 `b0f910c4fcbab2419b58bbe4fd845d5fcc19c0adcc55b39300aa7bca460d1dbd`, before and after review.

Verdict: **source/spec ACCEPT; no findings.**

The new support accessor checks the caller revision before seam validation or decoding, then reads the explicitly frozen selected face. BRepAdaptorSurface applies the face location before returning its cylinder. The accessor retains the actual cylinder origin, axis, radial direction and radius; indirect frames refuse. Face orientation is intentionally not substituted for the caller’s documented radial normal sense. Seam is a native angular datum, consistent with the existing developed coordinate `s=radius*(u-seam)`, and remains unreduced. Returning this analytic chart is not trim classification or acceptance of a downstream modelling request.

Reattachment continues through exact frozen-source comparison, distinct revision and unique explicit history image; no proximity matching or new logical identity is introduced. The replay fixture edits radius, authored placement and extrusion depth, preserves attachment ID/authored region and vertex keys, rejects the old revision and restores the new record.

Both original and edited bodies use unchanged ordered validation and one-solid checks. All tool history images remain output-bound, every selected LastDepthFace must survive as a face, and the independent area/radial-volume oracle remains in place. The old attachment bytes and original body snapshot are checked after recomputation. Fresh native exports and request mutation establish ownership isolation.

The child invocation uses the exact ignored worker name; absence of actual worker output fails rather than accepting zero selected tests. Its signature compares complete attached-body bytes, wrapped-tool bytes and authored tool history across processes. This does not claim byte-identical Boolean history or general trim-policy coverage; each operation separately checks Boolean history membership.

The 27 attachment tests are reported passing; the complete edited multiline boss replay remains running. No final replay pass, runtime/RSS acceptance or full support-trim capability is inferred. No builds or edits were performed.
