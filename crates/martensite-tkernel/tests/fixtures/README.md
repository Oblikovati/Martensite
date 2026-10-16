# martensite-tkernel test fixtures

These files are reviewed baselines. The tests compare the crate's output with
them byte for byte. No test reads a file outside this crate.

## Contents

- `oracle/*.expected`: the expected output of each case, one file per probe
  test that reads it.
- The other files in this directory are expected outputs or inputs named in
  the test that reads them.
- `array1_mt19937_seed1.txt` and `parallel_mt19937_seed42.txt` are the C++
  `std::mt19937` sequences (seeds 1 and 42) that the tests draw their inputs
  from.
- `tkernel_hex_float.windows.expected` is the expected output with the MSVC C
  runtime.

## Changing a baseline

A baseline changes only through a reviewed update that states the reason and
the behavior version (AGENTS.md, "Verification and completion"). Do not
regenerate a file to make a test pass.
