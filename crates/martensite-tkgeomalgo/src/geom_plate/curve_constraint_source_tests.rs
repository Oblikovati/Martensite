use super::{
    GeomPlateBuildPlateSurface, GeomPlateCurveConstraint, GeomPlateCurveConstraintSource,
    GeomPlateCurveConstraintState,
};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurveOnSurface, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3};

struct DerivedPlateConstraint(GeomPlateCurveConstraintState);

impl_transient!(
    DerivedPlateConstraint,
    "Test_DerivedPlateConstraint",
    dyn GeomPlateCurveConstraintSource,
    interfaces: [dyn GeomPlateCurveConstraintSource]
);

impl GeomPlateCurveConstraintSource for DerivedPlateConstraint {
    fn curve_constraint_state(&self) -> &GeomPlateCurveConstraintState {
        &self.0
    }
}

fn on_plane() -> Result<DerivedPlateConstraint, Failure> {
    let normal = Direction3::new(0., 0., 1.)?;
    let plane: Handle<dyn GeomSurface> =
        Handle::new(GeomPlane::from_point(Point3::default(), normal)?).into_upcast();
    let surface: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&plane)?).into_upcast();

    let direction = Direction2::new(1., 0.)?;
    let line = Handle::new(Geom2dLine::from_point(Point2::default(), direction)).into_upcast();
    let curve: Handle<dyn Adaptor2dCurve2d> =
        Handle::new(Geom2dAdaptorCurve::with_range(&line, 0., 2.)?).into_upcast();
    let boundary = Handle::new(Adaptor3dCurveOnSurface::with_curve(&curve, &surface)?);
    let state = GeomPlateCurveConstraintState::from_derived_state(
        &boundary,
        1,
        12,
        [1e-4, 0.01, 0.1],
        &plane,
    )?;
    Ok(DerivedPlateConstraint(state))
}

fn plate_builder() -> Result<GeomPlateBuildPlateSurface, Failure> {
    GeomPlateBuildPlateSurface::new(3, 10, 3, 1e-5, 1e-4, 0.01, 0.1, false)
}

#[test]
fn plate_builder_keeps_the_derived_curve_constraint_owner_and_descriptor() -> Result<(), Failure> {
    let derived = Handle::new(on_plane()?);
    let mut builder = plate_builder()?;
    builder.add_curve(&derived);
    assert_eq!(derived.ref_count(), 2);
    let stored = builder.curve_constraint(1)?;
    assert_eq!(stored.object_address(), derived.object_address());
    assert_eq!(stored.downcast::<DerivedPlateConstraint>(), derived);
    let native = <dyn GeomPlateCurveConstraintSource>::static_type();
    assert!(std::ptr::eq(
        native,
        GeomPlateCurveConstraint::static_type()
    ));
    assert!(std::ptr::eq(
        stored.dynamic_type().unwrap().parent().unwrap(),
        native
    ));
    assert!(stored.is_kind_name("GeomPlate_CurveConstraint"));
    Ok(())
}

#[test]
fn plate_builder_shares_both_constraint_locks_with_the_derived_owner() -> Result<(), Failure> {
    let derived = Handle::new(on_plane()?);
    let interface: Handle<dyn GeomPlateCurveConstraintSource> = derived.upcast();
    let mut builder = plate_builder()?;
    builder.add_curve(&interface);
    let stored = builder.curve_constraint(1)?;
    let state = &derived.get().unwrap().0;
    assert!(std::ptr::eq(stored.curve_constraint_state(), state));

    stored.set_nb_points(18);
    assert_eq!(state.nb_points(), 18);
    assert_eq!(stored.g1_criterion(0.5)?, 0.01);
    let point = stored.lprop_surf(0.5)?.value();
    assert_eq!(point, stored.d0(0.5)?);
    assert_eq!(state.lprop.read().value(), point);
    Ok(())
}

#[test]
fn plate_builder_still_accepts_the_concrete_constraint_handle() -> Result<(), Failure> {
    let concrete = Handle::new(GeomPlateCurveConstraint::empty()?);
    let mut builder = plate_builder()?;
    builder.add_curve(&concrete);
    assert_eq!(
        builder
            .curve_constraint(1)?
            .downcast::<GeomPlateCurveConstraint>(),
        concrete
    );
    Ok(())
}
