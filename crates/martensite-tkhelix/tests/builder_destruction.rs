//! Dynamic builder destruction releases its owned curve sequence.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkhelix::helix_geom::{
    HelixGeomBuilderApproxCurve, HelixGeomBuilderHelixCoil, HelixGeomBuilderHelixGen,
};
use martensite_tkmath::gp::Axis1;

fn builder_with_curve() -> (HelixGeomBuilderHelixCoil, Handle<dyn GeomCurve>) {
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(Axis1::default())).cast();
    let mut builder = HelixGeomBuilderHelixCoil::new();
    builder.approx_curve_base_mut().curves.append(curve.clone());
    (builder, curve)
}

#[test]
fn destroying_approximation_trait_object_releases_curve_owner() {
    let (builder, curve) = builder_with_curve();
    let erased: Box<dyn HelixGeomBuilderApproxCurve> = Box::new(builder);
    assert_eq!(curve.ref_count(), 2);
    drop(erased);
    assert_eq!(curve.ref_count(), 1);
}

#[test]
fn destroying_parameter_trait_object_releases_curve_owner() {
    let (builder, curve) = builder_with_curve();
    let erased: Box<dyn HelixGeomBuilderHelixGen> = Box::new(builder);
    assert_eq!(curve.ref_count(), 2);
    drop(erased);
    assert_eq!(curve.ref_count(), 1);
}

#[test]
fn parameter_trait_final_owner_frees_curve() {
    let (builder, curve) = builder_with_curve();
    let weak = curve.downgrade();
    let erased: Box<dyn HelixGeomBuilderHelixGen> = Box::new(builder);
    drop(curve);
    assert!(!weak.upgrade().is_null());
    drop(erased);
    assert!(weak.upgrade().is_null());
}

#[test]
fn approximation_trait_unwind_frees_curve() {
    let (builder, curve) = builder_with_curve();
    let weak = curve.downgrade();
    drop(curve);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _erased: Box<dyn HelixGeomBuilderApproxCurve> = Box::new(builder);
        panic!("exercise local builder unwind");
    }));
    assert!(result.is_err());
    assert!(weak.upgrade().is_null());
}

#[test]
fn cloned_builder_retains_curve_until_both_owners_drop() {
    let (builder, curve) = builder_with_curve();
    let weak = curve.downgrade();
    let clone = builder.clone();
    drop(curve);
    drop(Box::new(builder) as Box<dyn HelixGeomBuilderApproxCurve>);
    assert!(!weak.upgrade().is_null());
    drop(Box::new(clone) as Box<dyn HelixGeomBuilderHelixGen>);
    assert!(weak.upgrade().is_null());
}
