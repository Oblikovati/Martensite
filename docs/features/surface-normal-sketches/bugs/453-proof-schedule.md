# Wrapped proof schedule (#453)

The oblique process replay control failed with actual consumed pair work of
75 versus 57 for forward and reversed selections. All other captured geometry,
history and report fields matched. A planar-only reduction removed placement,
body construction, the hole and export: two disjoint three-piece contours
returned 19 versus zero bisections in 0.07 seconds. Three pieces per contour
are the minimum admitted by this wrapped operation. Their independent areas
are 2/3 and 1/2.

The wrapped bridge appended curves in selection order. TKBO deliberately
preserves its caller's curve order when assigning proof edges; coupled pair
queries can spend different work when their parameter axes exchange. The
receipts recorded real solver work. The correction belongs at the wrapped
bridge, which now orders its completed proof input by authored curve and
vertex IDs before calling TKBO. No counters are rewritten and no limits change.

Original-order admission and conversion finish before ordering begins.
Their failures therefore keep their original precedence. Cancellation polling
is unchanged. Already ordered vectors skip stable sorting and its possible
scratch allocation; this matters for large profiles on Rust 1.99, whose
stable sort reserves scratch storage before its run processing.

Regression controls compare full actual receipts, unchanged authored input
under reordered selections and rotated loop starts, concurrent calls,
zero-cap root refusals with their authored pair, and original first-invalid
source precedence. The original independent-process replay passed after the
correction. Runtime and full-sketch qualification remain with the coordinator;
this fix makes no new performance claim.
