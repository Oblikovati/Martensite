//! Coverage uses actual decoded incidence, independently of native traversal order.
use super::super::storage::tests::frozen;
use super::*;
fn fixture() -> (Request, Shape, Vec<(BindingKey, Shape)>) {
    let (_, request, bindings) = frozen();
    let key = expected::region(&request, HistoryRole::Solid);
    let root = bindings
        .iter()
        .find(|(actual, _)| *actual == key)
        .unwrap()
        .1
        .clone();
    (request, root, bindings)
}
#[test]
fn decoded_mixed_profile_has_complete_declared_coverage() {
    let (request, root, bindings) = fixture();
    let report = certify(&request, &root, &bindings).unwrap();
    assert_eq!(
        (report.faces, report.wires, report.edges, report.vertices),
        (21, 23, 57, 38)
    );
    assert_eq!(report.edge_uses, 114);
    assert_eq!(report.binding_count, 117);
    assert_eq!(report.euler_characteristic, 0);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CoverageEvidence>();
}

#[test]
fn missing_binding_is_refused() {
    let (request, root, mut bindings) = fixture();
    bindings.pop();
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Count {
            role: "semantic bindings",
            ..
        }
    ));
}
#[test]
fn duplicate_binding_is_refused() {
    let (request, root, mut bindings) = fixture();
    bindings.push(bindings[0].clone());
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::DuplicateBinding { .. }
    ));
}
#[test]
fn independent_decode_cannot_supply_the_bound_root() {
    let (request, _, bindings) = fixture();
    let (_, other_root, _) = fixture();
    assert!(matches!(
        certify(&request, &other_root, &bindings).unwrap_err().cause,
        CoverageCause::RootBinding
    ));
}
#[test]
fn reversed_root_is_refused() {
    let (request, root, bindings) = fixture();
    let reversed = root.oriented(Orientation::Reversed);
    assert!(matches!(
        certify(&request, &reversed, &bindings).unwrap_err().cause,
        CoverageCause::RootBinding
    ));
}
#[test]
fn binding_order_does_not_change_coverage() {
    let (request, root, mut bindings) = fixture();
    let expected = certify(&request, &root, &bindings).unwrap();
    bindings.reverse();
    assert_eq!(certify(&request, &root, &bindings).unwrap(), expected);
}

fn direct(parent: &Shape) -> Vec<Shape> {
    let mut iterator = martensite_tkbrep::topods::ShapeIterator::new(parent, true, true);
    let mut shapes = Vec::new();
    while iterator.more() {
        shapes.push(iterator.value().unwrap().clone());
        iterator.next();
    }
    shapes
}
fn replace(parent: &mut Shape, old: &Shape, new: &Shape) {
    let builder = martensite_tkbrep::topods::Builder;
    parent.set_free(true).unwrap();
    builder.remove(parent, old).unwrap();
    builder.add(parent, new).unwrap();
}
#[test]
fn reversed_side_use_is_refused() {
    let (request, root, bindings) = fixture();
    let mut shell = direct(&root).remove(0);
    let side = direct(&shell)
        .into_iter()
        .find(|s| s.orientation() == Orientation::Forward)
        .unwrap();
    replace(&mut shell, &side, &side.oriented(Orientation::Reversed));
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Uses { .. }
    ));
}
#[test]
fn duplicated_side_boundary_is_refused() {
    let (request, root, bindings) = fixture();
    let key = expected::span(request.loops[0][0].span, HistoryRole::SideFace);
    let face = &bindings.iter().find(|(k, _)| *k == key).unwrap().1;
    let mut wire = direct(face).remove(0);
    let edges = direct(&wire);
    replace(
        &mut wire,
        &edges[2],
        &edges[0].oriented(Orientation::Reversed),
    );
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Contour { .. }
    ));
}
#[test]
fn aliased_semantic_faces_are_refused() {
    let (request, root, mut bindings) = fixture();
    let first = bindings
        .iter()
        .position(|(k, _)| k.role == HistoryRole::FirstDepthFace)
        .unwrap();
    let last = bindings
        .iter()
        .position(|(k, _)| k.role == HistoryRole::LastDepthFace)
        .unwrap();
    bindings[last].1 = bindings[first].1.clone();
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::AliasedBindings { .. }
    ));
}
#[test]
fn extra_curve_representation_is_refused() {
    use martensite_tkbrep::brep::access::edge_tshape;
    let (request, root, bindings) = fixture();
    let (_, edge) = bindings
        .iter()
        .find(|(k, _)| k.role == HistoryRole::FirstDepthEdge)
        .unwrap();
    let stored = edge_tshape(edge, "coverage control").unwrap();
    let extra = stored.curves()[0].clone();
    stored.change_curves(|curves| curves.push(extra));
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Count {
            role: "edge curve representations",
            ..
        }
    ));
}
#[test]
fn cap_wire_order_does_not_change_coverage() {
    let (request, root, bindings) = fixture();
    let expected = certify(&request, &root, &bindings).unwrap();
    let key = expected::region(&request, HistoryRole::FirstDepthFace);
    let mut face = bindings.iter().find(|(k, _)| *k == key).unwrap().1.clone();
    let wires = direct(&face);
    replace(&mut face, &wires[0], &wires[0]);
    assert_eq!(certify(&request, &root, &bindings).unwrap(), expected);
}

#[test]
fn added_vertex_use_is_refused() {
    let (request, root, bindings) = fixture();
    let mut edge = first_role(&bindings, HistoryRole::FirstDepthEdge);
    let vertex = direct(&edge).remove(0).oriented(Orientation::Internal);
    edge.set_free(true).unwrap();
    martensite_tkbrep::topods::Builder
        .add(&mut edge, &vertex)
        .unwrap();
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Count {
            role: "direct children",
            actual: 3,
            expected: 2
        }
    ));
}
#[test]
fn duplicate_face_pcurve_is_refused_even_with_correct_count() {
    let (request, root, bindings) = fixture();
    let edge = first_role(&bindings, HistoryRole::FirstDepthEdge);
    repeat_surface_pcurve(&edge);
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Uses { .. }
    ));
}
#[test]
fn extra_face_use_is_refused() {
    let (request, root, bindings) = fixture();
    let mut shell = direct(&root).remove(0);
    let face = direct(&shell).remove(0);
    shell.set_free(true).unwrap();
    martensite_tkbrep::topods::Builder
        .add(&mut shell, &face)
        .unwrap();
    assert!(matches!(
        certify(&request, &root, &bindings).unwrap_err().cause,
        CoverageCause::Count {
            role: "direct children",
            actual: 22,
            expected: 21
        }
    ));
}
#[test]
fn false_wire_closed_flag_is_refused() {
    let (request, root, bindings) = fixture();
    let key = expected::region(&request, HistoryRole::FirstDepthFace);
    let face = &bindings.iter().find(|(k, _)| *k == key).unwrap().1;
    direct(face)[0].set_closed(false).unwrap();
    let refusal = certify(&request, &root, &bindings).unwrap_err();
    assert_eq!(refusal.owner, key);
    assert!(matches!(refusal.cause, CoverageCause::ClosedFlag));
}
#[test]
fn side_edge_order_does_not_change_coverage() {
    let (request, root, bindings) = fixture();
    let expected = certify(&request, &root, &bindings).unwrap();
    let key = expected::span(request.loops[0][0].span, HistoryRole::SideFace);
    let face = &bindings.iter().find(|(k, _)| *k == key).unwrap().1;
    let mut wire = direct(face).remove(0);
    let edge = direct(&wire).remove(0);
    replace(&mut wire, &edge, &edge);
    assert_eq!(certify(&request, &root, &bindings).unwrap(), expected);
}

fn first_role(bindings: &[(BindingKey, Shape)], role: HistoryRole) -> Shape {
    bindings
        .iter()
        .find(|(key, _)| key.role == role)
        .unwrap()
        .1
        .clone()
}
fn repeat_surface_pcurve(edge: &Shape) {
    use martensite_tkbrep::brep::access::edge_tshape;
    let stored = edge_tshape(edge, "coverage control").unwrap();
    let pcurve = stored
        .curves()
        .into_iter()
        .find(|r| r.get().unwrap().is_curve_on_surface())
        .unwrap();
    stored.change_curves(|curves| {
        curves.retain(|r| r.get().unwrap().is_curve3d());
        curves.extend([pcurve.clone(), pcurve]);
    });
}
