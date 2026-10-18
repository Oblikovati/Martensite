//! Approx behaviour: the error contracts, the defaults, the refinement scripts of the
//! Approx line members and the preserved retained quirks. The package has no defined
//! Numeric assertion suite; bit-exact parity with the retained kernel is the numeric expectation's
//! job (tools/verify_tkgeombase_numeric.py), these tests pin the documented behaviour.
mod curves;
mod drivers;
mod fakes;
mod sweep;
