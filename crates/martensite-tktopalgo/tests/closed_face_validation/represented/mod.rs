//! Located strip inputs bind every field and alias before numerical validation.
mod binding;
mod binding_geometry;
mod binding_representations;
mod binding_topology;
mod data;
mod field_words;
mod geometry;
mod locations;
mod registry;
mod representations;
mod spline_binding;
mod splines;
mod topology;
mod types;
use martensite_tkbrep::{
    brep::face_input_capture::{FaceCaptureLimits, capture_closed_extrusion_face},
    topods::face,
};
use martensite_tktopalgo::face_validation::{ClosedFacePolicy, certify_closed_extrusion_face};
fn input_face(input: &types::GraphInput) -> martensite_tkbrep::topods::Face {
    let mut owners = registry::FixtureOwners::new(input);
    geometry::construct(input, &mut owners);
    representations::construct(input, &mut owners);
    topology::construct(input, &mut owners);
    face(&owners.occurrence(input.root)).unwrap()
}
#[test]
fn complete_located_input_graphs_bind_before_self_exclusion() {
    let results: Vec<_> = data::cases()
        .iter()
        .enumerate()
        .map(|(index, input)| check_input(index, input))
        .collect();
    assert!(
        results.iter().all(|result| *result),
        "certified={results:?}"
    );
}
fn check_input(index: usize, input: &types::GraphInput) -> bool {
    let source = input_face(input);
    let owned =
        capture_closed_extrusion_face(&source, FaceCaptureLimits::bounded(), || false).unwrap();
    binding::assert_graph(input, &owned);
    println!(
        "{{\"input\":{index},\"field_words\":{:?}}}",
        field_words::field_words(input)
    );
    let report =
        certify_closed_extrusion_face(owned, ClosedFacePolicy::piecewise_boundary(), || false);
    let certified = report.certificate().is_some();
    println!(
        "{{\"input\":{index},\"bound\":true,\"certified\":{certified},\"completion\":{:?},\"obligations\":{},\"finished\":{},\"queries\":{}}}",
        format!("{:?}", report.completion()),
        report.work().support_obligations,
        report.work().support_finished,
        report.work().queries
    );
    certified
}
