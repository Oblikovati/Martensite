#[path = "../boolean_section_endpoints.rs"]
mod boolean_section_endpoints;
use boolean_section_endpoints::alone_vertices;
#[path = "../boolean_length_assertion.rs"]
mod boolean_length_assertion;
use boolean_length_assertion::check_length;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

pub(super) fn check_result(result: &Shape, case: usize) {
    let children = ShapeIterator::new(result, true, true).count();
    if case == 23985 {
        assert!(children > 0, "bug23985: expected nonempty explode result");
        return;
    }
    assert_eq!(
        children,
        if case == 24496 { 2 } else { 1 },
        "bug{case}: explode result"
    );
    check_length(result, if case == 24496 { 11.7031 } else { 3.83537 });
    check_section(result, case);
}

fn check_section(result: &Shape, case: usize) {
    let alone = alone_vertices(result);
    if case == 24519 {
        assert_eq!(alone, 2, "bug24519: checksection -r 2");
        return;
    }
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap(),
        "bug24496: checkshape"
    );
    // checksection without -r reports the count; its independently captured value is four.
    assert_eq!(alone, 4, "bug24496: native checksection observation");
}
