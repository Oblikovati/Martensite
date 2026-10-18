//! GeomTools behaviour: the text layouts, the reader failure contract, the sets, the
//! undefined-type handler and the preserved retained quirks. The package has no defined
//! Numeric assertion suite; byte-exact parity with the retained kernel is the numeric expectation's
//! job (tools/verify_tkgeombase_numeric.py), these tests pin the documented behaviour.
mod fakes;
mod handler;
mod reads;
mod sets;
mod texts;
