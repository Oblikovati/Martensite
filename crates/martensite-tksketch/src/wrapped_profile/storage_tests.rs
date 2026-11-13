//! Refuse detached native mutations before an old boundary certificate can be reused.
use super::super::{native as construction, subdivision, tests::request};
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomBezierSurface;
use martensite_tkmath::gp::Point3;

fn frozen() -> (Request, Vec<(BindingKey, Shape)>) {
    let prepared = subdivision::prepare(&request());
    let built = construction::construct(&prepared).unwrap();
    let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
    let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
    let (_, bindings) = super::super::pipeline::decode(&bytes, &keys).unwrap();
    (prepared, bindings)
}

#[test]
fn changed_side_control_net_refuses_the_stale_boundary_binding() {
    let (request, bindings) = frozen();
    assert_eq!(certify(&request, &bindings).unwrap().len(), 32);
    let key = key(request.loops[0][0].span, HistoryRole::SideFace);
    change_side(key, &bindings);
    let error = certify(&request, &bindings).err().unwrap();
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
    let (request, mut bindings) = frozen();
    let cap = bindings
        .iter()
        .find(|(key, _)| key.role == HistoryRole::FirstDepthFace)
        .unwrap()
        .clone();
    bindings.push(cap);
    let error = certify(&request, &bindings).err().unwrap();
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
    let (request, mut bindings) = frozen();
    bindings.retain(|(key, _)| key.role != HistoryRole::FirstDepthFace);
    let error = certify(&request, &bindings).err().unwrap();
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
    let (request, bindings) = frozen();
    let span = request.loops[0][0].span;
    change_boundary_and_row(span, &bindings);
    let error = certify(&request, &bindings).err().unwrap();
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
