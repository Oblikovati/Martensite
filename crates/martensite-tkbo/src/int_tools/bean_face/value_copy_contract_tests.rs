//! Value assignment retains warm cache owners; it does not isolate mutable geometry.
use super::*;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomBezierSurface, GeomCurve, GeomSurface};
use martensite_tkmath::gp::Point3;
mod fixtures;
use fixtures::*;

#[test]
fn adaptor_initialization_retains_warm_surface_cache_before_source_mutation() {
    for mode in 0..4 {
        let (_, curve) = cached_curve();
        let (geometry, surface) = cached_surface();
        let bean = constructed(mode, &curve, &surface);
        raise_patch(&geometry);
        assert_eq!(surface.value(0.25, 0.5).unwrap(), Point3::new(0.5, 1., 0.));
        assert_eq!(
            bean.surface.value(0.25, 0.5).unwrap(),
            Point3::new(0.5, 1., 0.)
        );
    }
}

#[test]
fn adaptor_initialization_retains_warm_curve_cache_before_source_mutation() {
    for mode in 0..4 {
        let (geometry, curve) = cached_curve();
        let (_, surface) = cached_surface();
        let bean = constructed(mode, &curve, &surface);
        raise_curve(&geometry);
        assert_eq!(curve.value(0.25).unwrap(), Point3::new(0.5, 0., 0.));
        assert_eq!(bean.curve.value(0.25).unwrap(), Point3::new(0.5, 0., 0.));
    }
}

#[test]
fn topology_initialization_retains_the_warm_context_surface_value() {
    let (curve_geometry, curve) = cached_curve();
    let (geometry, surface) = cached_surface();
    let context = warm_context(&surface.face());
    let mut bean = BeanFaceIntersector::new();
    bean.set_context(&context);
    bean.init_shapes(&curve.edge(), &surface.face()).unwrap();
    raise_patch(&geometry);
    raise_curve(&curve_geometry);
    assert_eq!(
        bean.surface.value(0.25, 0.5).unwrap(),
        Point3::new(0.5, 1., 0.)
    );
    assert_eq!(bean.curve.value(0.25).unwrap(), Point3::new(0.5, 0., 4.));
}

fn warm_context(face: &Face) -> Handle<Context> {
    let context = Handle::new(Context::default());
    let cached = context.surface_adaptor(face).unwrap();
    assert_eq!(
        cached.read().value(0.25, 0.5).unwrap(),
        Point3::new(0.5, 1., 0.)
    );
    context
}
