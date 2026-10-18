//! ProjLib behaviour: the contracts of the projectors and the approximations and the
//! preserved retained quirks (docs/tkgeombase-progress.md, Preserved retained behaviour).
//! Bit-exact parity with the retained kernel is the numeric expectation's job
//! (tools/verify_tkgeombase_numeric.py); these tests pin the documented behaviour.
mod approximations;
mod degenerate_ranges;
mod elementary;
mod support;
