//! Real transfer settings initialize missing units while existing units remain untouched.
#[path = "support/shape_units.rs"]
mod shape_units;
use martensite_tkernel::{
    standard::Handle, tcollection::ExtendedString, units_methods::LengthUnit,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::{
    interface::InterfaceStatic,
    xs_algo::{XSAlgo, XSAlgoShapeProcessor, xs_algo_units},
};
use shape_units::ensure_shape_document_unit;

const TRANSFER_UNIT_STATIC: &str = "xstep.cascade.unit";
fn standalone_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap()
}
fn unit_in_meters(document: &Handle<TDocStdDocument>) -> f64 {
    let mut value = -1.;
    assert!(XCAFDocDocumentTool::get_length_unit(document, &mut value).unwrap());
    value
}
fn assert_transfer_unit(name: &str, expected_meters: f64) {
    assert!(InterfaceStatic::set_c_val(TRANSFER_UNIT_STATIC, name).unwrap());
    let document = standalone_document();
    ensure_shape_document_unit(&document);
    let measured = unit_in_meters(&document);
    assert!(
        (measured - expected_meters).abs() <= 4. * f64::EPSILON * expected_meters,
        "{name}: expected {expected_meters} meters, got {measured}"
    );
}
fn assert_existing_unit_skips_transfer_preparation() {
    let document = standalone_document();
    XCAFDocDocumentTool::set_length_unit(&document, 2.75).unwrap();
    xs_algo_units().set_cascade_length_unit_default(246.);
    assert!(InterfaceStatic::set_c_val(TRANSFER_UNIT_STATIC, "INCH").unwrap());
    ensure_shape_document_unit(&document);
    assert_eq!(unit_in_meters(&document), 2.75);
    assert_eq!(
        xs_algo_units().get_cascade_length_unit(LengthUnit::Millimeter),
        246.
    );
}
#[test]
fn missing_units_follow_real_settings_and_existing_units_skip_preparation() {
    XSAlgo::init().unwrap();
    for (name, meters) in [("MM", 0.001), ("INCH", 0.0254), ("M", 1.)] {
        assert_transfer_unit(name, meters);
    }
    assert_existing_unit_skips_transfer_preparation();
    assert!(InterfaceStatic::set_c_val(TRANSFER_UNIT_STATIC, "MM").unwrap());
    XSAlgoShapeProcessor::prepare_for_transfer();
}
