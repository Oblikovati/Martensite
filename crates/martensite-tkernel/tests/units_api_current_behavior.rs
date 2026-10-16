//! Composite units: keep the observed kernel failure, per the user's explicit decision.
//! This is a behavioral regression until composite units are supported.
#[path = "units_support/mod.rs"]
mod units_support;
use martensite_tkernel::units::UnitSentence;
use units_support::api;
#[test]
fn composite_prefix_failure_is_preserved() {
    let mut units = api();
    let mut sentence = UnitSentence::new(units.units(), b"sq.km").unwrap();
    assert!(sentence.is_done().unwrap());
    let failure = sentence.evaluate().err().unwrap();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(failure.message(), "NCollection_Sequence::Value");
    let failure = units.any_to_si(1.0, b"sq.km").err().unwrap();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(failure.message(), "NCollection_Sequence::Value");
}
