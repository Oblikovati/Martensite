# Shape healing

`martensite-tkshhealing` analyzes, repairs, converts and subdivides boundary shapes (#215).

## Verification

Run `cargo test -p martensite-tkshhealing`. The suite covers wire and face repair, shell sewing, surface and curve conversion, subdivision policies and dataset-backed regression cases.

The dataset fixture helpers accept viewing commands without rendering them. Those
runs verify geometry and log assertions, and do not qualify visual output. Three
additional regression cases require Boolean construction and remain unregistered
until that dependency is available. These checks remain outside executed coverage
(#213); a passing registered suite does not establish complete shape-healing
acceptance.
