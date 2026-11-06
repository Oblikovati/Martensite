//! Independent segment fixtures pin the endpoint membership rule.
use super::same_domain::restrictions_overlap;
use martensite_tkbrep::topods::Edge;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

fn segment(first: (f64, f64), last: (f64, f64)) -> Edge {
    BRepBuilderAPIMakeEdge::from_points(
        Point3::new(first.0, first.1, 0.),
        Point3::new(last.0, last.1, 0.),
    )
    .unwrap()
    .edge()
    .unwrap()
}

#[test]
fn endpoint_membership_keeps_touching_and_contained_segments() {
    let long = segment((0., 0.), (2., 0.));
    let inside = segment((0.5, 0.), (1., 0.));
    let touching = segment((2., 0.), (3., 0.));
    assert!(restrictions_overlap(&long, &inside).unwrap());
    assert!(restrictions_overlap(&inside, &long).unwrap());
    assert!(restrictions_overlap(&long, &touching).unwrap());
}

#[test]
fn interior_crossings_and_separated_endpoints_do_not_form_domains() {
    let horizontal = segment((-1., 0.), (1., 0.));
    let crossing = segment((0., -1.), (0., 1.));
    let separated = segment((2., 0.), (3., 0.));
    assert!(!restrictions_overlap(&horizontal, &crossing).unwrap());
    assert!(!restrictions_overlap(&horizontal, &separated).unwrap());
}
