//! The presentation attribute of a label and its mode state.
#![allow(non_snake_case)]

use martensite_tkcaf::tdata_xtd::TDataXtdConstraint;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkvcaf::tprs_std::TPrsStdAISPresentation;

// The case tests/bugs/caf/buc60910 checks presentation mode state only; it does not create a
// viewer or otherwise exercise visualization output.
#[test]
fn TPrsStdAISPresentationTest__CafBug_60910_ModeState() {
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("BinOcaf")).unwrap();
    let label = document.main().unwrap().find_child(2, true).unwrap();
    let presentation = TPrsStdAISPresentation::set(&label, &TDataXtdConstraint::get_id()).unwrap();
    assert!(!presentation.is_null());

    assert!(!presentation.has_own_mode().unwrap());
    let display_mode = 3;
    presentation.set_mode(display_mode).unwrap();
    assert_eq!(presentation.mode().unwrap(), display_mode);
    assert!(presentation.has_own_mode().unwrap());
    presentation.unset_mode().unwrap();
    assert!(!presentation.has_own_mode().unwrap());
}
