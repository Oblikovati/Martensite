# Frozen numeric behavior cases, behavior version 1

The four input and four expectation chunks retain **1,787** requests in their
original order. Concatenate numbered chunks to reconstruct the original streams.
Every chunk has fewer than 500 lines; splitting changes no input or output byte.

Input SHA-256:
`c7c714afd10911d613f1bff23eb29c053928261d3098d8500351c2b6a1fd3200`

Expected stream SHA-256:
`c2f8783dde028cac7c168f95d162511192ac824da73986ebc677b931155bbfda`

These expectations are a reviewed frozen behavior baseline, **not independent
mathematical results**. The ordinary Rust target parses these checked-in requests, calls
public Rust geometry APIs in process, and compares the result directly; it does
not invoke an interpreter, compiler, subprocess or reference kernel.

The comparison preserves finite values, infinities and signed-zero bits exactly,
normalizes only complete 16–20 digit decimal words that encode NaNs, and retains
exception names, JSON text, order and whitespace. There is no numerical tolerance
for baseline comparisons. Four separately named closed-form tests derive their
line, circle, parabola and quadratic Bernstein expectations analytically; they do
not read these fixture values.

Changing this baseline requires a reviewed reason and an explicit behavior
change; do not regenerate expected values to accommodate a failing replay.

Trailing spaces in the four expected streams are significant comparison bytes.
The local Git attribute disables only end-of-line blank lint for those files;
replay still checks all text and whitespace exactly. Do not trim the fixtures.
