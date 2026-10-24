# Combined walking-point reads

Issue #220 concerned repeated snapshots during fitting. A combined
`ApproxIntMultiLine::value` call now obtains one point and uses it for the
spatial and parametric coordinates. Separate reads retain their behavior.
The regression counts calls through `ApproxIntLine` and checks the offsets
for either parameter curve, both curves, and a short output buffer.

On Linux x86_64 with Rust 1.99.0, five debug-profile batches of 200,000 combined
reads produced these milliseconds:

| State | Batches | Median |
| --- | --- | --- |
| Main `98d40844` | 229.647, 230.627, 238.225, 245.193, 242.590 | 238.225 |
| Single snapshot | 150.567, 148.433, 151.596, 122.750, 136.120 | 148.433 |

Both runs used the same point, offsets and two-curve layout. The host was
shared; these local timings do not predict release-profile or whole-operation
performance. CI checks the deterministic snapshot count without a timing limit.
Reproduce the timing from the repository root:

```sh
cargo test -p martensite-tkgeomalgo --test toolkit measure_combined_walking_reads -- --ignored --nocapture
```
