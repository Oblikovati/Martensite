//! Material identity comes from the declared loops, never capture-local region indices.
use super::super::tests::request;
use super::*;

#[test]
fn authored_mixed_loop_and_hole_select_the_certified_ring() {
    let proof = certify(&request()).unwrap();
    assert!(proof.area.encloses(3.525));
    assert_eq!(proof.area.evidence(), Evidence::Estimated);
    assert_eq!(proof.identity.outer.len(), 4);
    assert_eq!(proof.identity.holes.len(), 1);
    assert_eq!(proof.certificate.components, 2);
    assert_eq!(proof.certificate.loops, 2);
}

#[test]
fn material_identity_is_stable_when_loop_start_changes() {
    let first = certify(&request()).unwrap();
    let mut moved = request();
    for contour in &mut moved.loops {
        contour.rotate_left(1);
    }
    let second = certify(&moved).unwrap();
    assert_eq!(first.identity, second.identity);
    assert!(second.area.encloses(3.525));
}

#[test]
fn a_disconnected_loop_cannot_be_silently_selected_as_a_hole() {
    let mut input = request();
    for piece in &mut input.loops[1] {
        for pole in &mut piece.poles {
            pole[0] += 5.;
        }
    }
    assert!(matches!(
        certify(&input).unwrap_err(),
        PlanarFailure::Selection { .. }
    ));
}

#[test]
fn prepared_fit_points_cannot_replace_original_authored_sources() {
    let prepared = super::super::subdivision::prepare(&request()).unwrap();
    assert!(matches!(
        certify(&prepared).unwrap_err(),
        PlanarFailure::NonAuthoredVertex(_) | PlanarFailure::SplitSource { .. }
    ));
}

#[test]
fn pipeline_refuses_inconsistent_material_before_native_construction() {
    let mut input = request();
    for pole in input.loops[1].iter_mut().flat_map(|piece| &mut piece.poles) {
        pole[0] += 5.;
    }
    let failure = super::super::pipeline::wrap_extrude(&input).err().unwrap();
    assert_eq!(failure.stage, Stage::Planar);
    assert!(matches!(failure.cause, RefusalCause::Planar(_)));
}

#[test]
fn undeclared_crossings_are_refused_by_the_existing_planar_certificate() {
    let mut input = request();
    input.loops[0][2].poles = vec![[2., 1.5], [1., -2.], [1., -2.], [0., 1.5]];
    assert!(matches!(
        certify(&input).unwrap_err(),
        PlanarFailure::Capture(_)
    ));
}

#[test]
fn tangent_return_at_an_authored_join_is_refused() {
    let mut input = request();
    input.loops[0][1].poles[1] = [1., 0.];
    assert!(matches!(
        certify(&input).unwrap_err(),
        PlanarFailure::Join(_)
    ));
}

#[test]
fn stationary_authored_endpoint_is_refused() {
    let mut input = request();
    input.loops[0][1].poles[1] = input.loops[0][1].poles[0];
    let error = certify(&input).unwrap_err();
    assert!(matches!(error, PlanarFailure::Join(_)));
    assert!(error.to_string().contains("nonzero endpoint derivative"));
}

#[test]
fn regular_straight_through_join_remains_admitted() {
    let mut input = request();
    input.loops[0][1].poles[1] = [3., 0.];
    certify(&input).unwrap();
}

#[test]
fn hole_order_does_not_change_authored_material_identity() {
    let mut input = request();
    input.loops[1] = rectangular_hole(30, [0.5, 0.75]);
    input.loops.push(rectangular_hole(40, [1.25, 1.5]));
    let first = certify(&input).unwrap();
    input.loops.swap(1, 2);
    let second = certify(&input).unwrap();
    assert_eq!(first.identity, second.identity);
    assert_eq!(second.identity.holes.len(), 2);
    assert!(second.area.encloses(3.525));
}

fn rectangular_hole(first: u64, [a, b]: [f64; 2]) -> Vec<AuthoredPiece> {
    super::super::tests::contour(
        first,
        &[
            &[[a, 0.5], [a, 1.]],
            &[[a, 1.], [b, 1.]],
            &[[b, 1.], [b, 0.5]],
            &[[b, 0.5], [a, 0.5]],
        ],
    )
}

#[test]
fn an_original_curve_label_cannot_hide_a_nonoriginal_interval() {
    let mut input = request();
    input.loops[0][0].span.level = 1;
    let error = certify(&input).unwrap_err();
    assert!(matches!(error, PlanarFailure::SplitSource { .. }));
    assert!(error.to_string().contains("expected original interval"));
}
