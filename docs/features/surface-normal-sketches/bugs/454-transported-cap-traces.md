# Transported cap-trace certificates (#454)

Canonical cap storage bounds each full-interval trace residual by the minimum
of its retained edge and face tolerances. A placed result must preserve that
individual limit. Checking only the operation's world epsilon leaves a proof
gap when the stored affine map has upper stretch greater than one. This is a
missing inequality check; no measured invalid production body is asserted.

For canonical curve and cap points p and q, the common actual stored map is
T(x) = A x + b. Its exact difference is A(p-q), so the same-frame trace bound
does not include ideal-frame deviation or ideal-origin displacement. Given
an upper stretch M, canonical residual bound r, and the existing certified
point-domain affine evaluation bound E for each operand, independent rounded
evaluations satisfy the conservative bound M*r + 2*E. The common translation
cancels in exact arithmetic, while its rounding contribution remains in E.

The placed trace check computes that expression with existing outward
nonnegative arithmetic for both caps of every retained span. It refuses when
any bound exceeds that cap certificate's original requested limit. The
reported world trace bound is the maximum achieved bound over those caps;
publication also checks the original operation epsilon. Native sampled checks
remain separate from this full-interval evidence. Existing span-inventory
coverage and point-domain certificates remain premises of the publication
pipeline.

Controls cover an at-limit near-unit stretch and an exact dyadic stretch,
exact identity/equality, independent evaluation errors, gradual underflow,
overflow and invalid arithmetic, ideal-origin cancellation, and nonzero
rounding from a large common translation. The same-frame scope is explicit:
this bound does not replace the ideal-frame placement bounds for other
reported quantities. No tolerance, default, fit limit or version is widened.
