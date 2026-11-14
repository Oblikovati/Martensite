//! Refuse detached native mutations before an old boundary certificate can be reused.
use super::super::{native as construction, subdivision, tests::request};
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomBezierSurface;
use martensite_tkmath::gp::Point3;

pub(super) fn frozen() -> (Request, Request, Vec<(BindingKey, Shape)>) {
    let original = request();
    let prepared = subdivision::prepare(&original).unwrap();
    let built = construction::construct(&prepared).unwrap();
    let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
    let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
    let (_, bindings) = super::super::pipeline::decode(&bytes, &keys).unwrap();
    (original, prepared, bindings)
}

#[test]
fn changed_side_control_net_refuses_the_stale_boundary_binding() {
    let (original, request, bindings) = frozen();
    assert_eq!(certify(&original, &request, &bindings).unwrap().len(), 19);
    let key = key(request.loops[0][0].span, HistoryRole::SideFace);
    change_side(key, &bindings);
    let error = certify(&original, &request, &bindings).err().unwrap();
    assert_eq!(error.key, key);
    assert!(matches!(
        error.cause,
        StorageCause::Mismatch {
            field: "ruled control-net row",
            ..
        }
    ));
}

fn change_side(key: BindingKey, bindings: &[(BindingKey, Shape)]) {
    let indexed = index(bindings).unwrap();
    let face = topods::face(lookup(key, &indexed).unwrap()).unwrap();
    let surface: Handle<GeomBezierSurface> = BRepTool::surface(&face).unwrap().cast();
    let surface = surface.get().unwrap();
    let p = surface.pole(2, 1).unwrap();
    surface
        .set_pole(2, 1, Point3::new(p.x(), p.y(), p.z() + 0.01))
        .unwrap();
}

#[test]
fn duplicate_semantic_bindings_refuse_instead_of_using_order() {
    let (original, request, mut bindings) = frozen();
    let cap = bindings
        .iter()
        .find(|(key, _)| key.role == HistoryRole::FirstDepthFace)
        .unwrap()
        .clone();
    bindings.push(cap);
    let error = certify(&original, &request, &bindings).err().unwrap();
    assert!(matches!(
        error.cause,
        StorageCause::Mismatch {
            field: "history membership",
            ..
        }
    ));
}

#[test]
fn missing_semantic_binding_refuses_instead_of_using_order() {
    let (original, request, mut bindings) = frozen();
    bindings.retain(|(key, _)| key.role != HistoryRole::FirstDepthFace);
    let error = certify(&original, &request, &bindings).err().unwrap();
    assert!(matches!(
        error.cause,
        StorageCause::Mismatch {
            field: "history membership",
            ..
        }
    ));
}

#[test]
fn storage_allocation_rounds_down_including_subnormal_requests() {
    for requested in [1e-5, 1., 1000., f64::MIN_POSITIVE] {
        assert!(storage_share(requested) < requested / 1000.);
    }
    assert_eq!(storage_share(f64::from_bits(1)), 0.);
}

#[test]
fn coherent_native_row_mutation_still_requires_a_new_storage_certificate() {
    let (original, request, bindings) = frozen();
    let span = request.loops[0][0].span;
    change_boundary_and_row(span, &bindings);
    let error = certify(&original, &request, &bindings).err().unwrap();
    assert_eq!(error.key, key(span, HistoryRole::FirstDepthEdge));
    assert!(matches!(error.cause, StorageCause::Certificate(_)));
    assert!(error.to_string().contains("Uncertified"));
}

fn change_boundary_and_row(span: CurveSpan, bindings: &[(BindingKey, Shape)]) {
    use martensite_tkg3d::geom::GeomBezierCurve;
    let indexed = index(bindings).unwrap();
    let edge =
        topods::edge(lookup(key(span, HistoryRole::FirstDepthEdge), &indexed).unwrap()).unwrap();
    let curve: Handle<GeomBezierCurve> = BRepTool::curve_with_location(&edge).unwrap().curve.cast();
    let face = topods::face(lookup(key(span, HistoryRole::SideFace), &indexed).unwrap()).unwrap();
    let surface: Handle<GeomBezierSurface> = BRepTool::surface(&face).unwrap().cast();
    let p = curve.get().unwrap().pole(2).unwrap();
    let point = Point3::new(p.x() + 0.001, p.y(), p.z());
    curve.get().unwrap().set_pole(2, point).unwrap();
    surface.get().unwrap().set_pole(2, 1, point).unwrap();
}

#[test]
fn original_authored_profile_cannot_be_replaced_by_rounded_fit_points() {
    let (mut original, prepared, bindings) = frozen();
    for pole in original
        .loops
        .iter_mut()
        .flatten()
        .flat_map(|piece| &mut piece.poles)
    {
        pole[1] += 0.01;
    }
    let error = certify(&original, &prepared, &bindings).err().unwrap();
    assert!(matches!(error.cause, StorageCause::Profile(_)));
    assert!(error.to_string().contains("Uncertified"));
}

#[test]
fn same_point_with_different_vertex_identity_refuses() {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let (original, request, mut bindings) = frozen();
    let entry = bindings
        .iter_mut()
        .find(|(key, _)| key.role == HistoryRole::FirstDepthVertex)
        .unwrap();
    let old = topods::vertex(&entry.1).unwrap();
    let replacement = BRepBuilderAPIMakeVertex::new(BRepTool::pnt(&old).unwrap())
        .unwrap()
        .vertex()
        .unwrap();
    entry.1 = replacement.into_shape();
    assert_incidence_failure(&original, &request, &bindings, "shared endpoint identity");
}

#[test]
fn moved_shared_vertex_refuses_even_when_its_identity_is_preserved() {
    use martensite_tkbrep::brep::BRepBuilder;
    let (original, request, bindings) = frozen();
    let entry = bindings
        .iter()
        .find(|(key, _)| key.role == HistoryRole::FirstDepthVertex)
        .unwrap();
    let vertex = topods::vertex(&entry.1).unwrap();
    let p = BRepTool::pnt(&vertex).unwrap();
    BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(p.x(), p.y(), p.z() + 0.001), 0.)
        .unwrap();
    assert_incidence_failure(
        &original,
        &request,
        &bindings,
        "shared endpoint coordinates",
    );
}

fn assert_incidence_failure(
    original: &Request,
    request: &Request,
    bindings: &[(BindingKey, Shape)],
    expected: &'static str,
) {
    let error = certify(original, request, bindings).err().unwrap();
    assert!(matches!(error.cause, StorageCause::Mismatch { field, .. } if field == expected));
}

#[test]
fn tiny_nonzero_span_is_certified_after_native_weight_demotion() {
    let authored = [[0., 0.], [1e-8, 1.]];
    let coefficients = super::super::angular::coefficients(&authored, 10., 5e-10);
    let poles = super::super::angular::at_radius(&coefficients, 9.);
    let curve = super::super::boundaries::curve3(&poles).unwrap();
    assert!(!curve.get().unwrap().is_rational());
    let retained = frozen_demoted_curve(&curve, &poles.points);
    assert!(retained.weights.iter().all(|weight| *weight == 1.));
    let span = CurveSpan {
        curve: SurfaceCurveId(777),
        level: 0,
        index: 0,
    };
    let proof = prove_profile(&request(), span, &retained, &authored).unwrap();
    assert!(proof.developed_position_bound() <= request().tolerance / 4.);
}

fn frozen_demoted_curve(
    curve: &Handle<martensite_tkg3d::geom::GeomBezierCurve>,
    poles: &[[f64; 3]],
) -> OwnedCurve {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let ends = [poles[0], poles[poles.len() - 1]].map(|[x, y, z]| {
        BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))
            .unwrap()
            .vertex()
            .unwrap()
    });
    let edge = super::super::boundaries::edge(curve, [&ends[0], &ends[1]])
        .unwrap()
        .into_shape();
    let span = CurveSpan {
        curve: SurfaceCurveId(777),
        level: 0,
        index: 0,
    };
    let key = key(span, HistoryRole::FirstDepthEdge);
    let bytes = crate::surface_sketch::encoding::encode(&edge, &[(key, edge.clone())]).unwrap();
    let (_, bindings) = super::super::pipeline::decode(&bytes, &[key]).unwrap();
    capture::curve(key, &index(&bindings).unwrap()).unwrap()
}

#[test]
fn curved_edge_cannot_replace_a_linear_normal_connector() {
    let (original, request, mut bindings) = frozen();
    let curved = bindings
        .iter()
        .find(|(key, _)| key.role == HistoryRole::FirstDepthEdge)
        .unwrap()
        .1
        .clone();
    let connector = bindings
        .iter_mut()
        .find(|(key, _)| key.role == HistoryRole::NormalEdge)
        .unwrap();
    connector.1 = curved;
    assert_incidence_failure(&original, &request, &bindings, "linear normal connector");
}
