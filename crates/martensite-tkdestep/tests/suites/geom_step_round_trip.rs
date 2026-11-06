//! Geometry written to STEP entities and read back must describe the same geometry: the two
//! conversions are independent, so agreement checks both the scaling and the entity fields.
use martensite_tkdestep::geom_to_step::{
    GeomToStepMakeBoundedCurve, GeomToStepMakeCartesianPoint, GeomToStepMakeCurve,
    GeomToStepMakeDirection, GeomToStepMakeSurface,
};
use martensite_tkdestep::step_data::StepDataFactors;
use martensite_tkdestep::step_geom::StepGeomBSplineCurve;
use martensite_tkdestep::step_to_geom::StepToGeom;
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBoundedCurve, GeomCircle, GeomCurve, GeomCylindricalSurface, GeomLine,
    GeomPlane, GeomPoint, GeomSurface,
};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};

/// The factors of a file in millimetres read in metres, the case where scaling errors show.
fn length_factors(length: f64) -> StepDataFactors {
    let mut factors = StepDataFactors::new();
    factors.initialize_factors(length, 1.0, 1.0);
    factors
}

/// A frame that no default direction can mistake for another.
fn tilted_frame() -> Axis2 {
    let main = Direction3::new(0.0, 0.6, 0.8).unwrap();
    Axis2::with_x_direction(Point3::new(1.0, -2.0, 3.0), main, Direction3::X).unwrap()
}

/// A 1-based array of the given values.
fn array_of<T: Copy + Default>(values: &[T]) -> Array1<'static, T> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    array.as_mut_slice().copy_from_slice(values);
    array
}

/// A shared 1-based array of the given values.
fn shared<T: Copy + Default + Send + Sync + 'static>(values: &[T]) -> Handle<HArray1<T>> {
    Handle::new(HArray1::from_owned(array_of(values)))
}

fn assert_near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn point_is_divided_on_write_and_multiplied_on_read() {
    let factors = length_factors(1000.0);
    let written =
        GeomToStepMakeCartesianPoint::from_pnt(&Point3::new(1000.0, 2000.0, 3000.0), 1000.0);
    let entity = written.value().unwrap();
    assert_eq!(entity.coordinates_value(1).unwrap(), 1.0);
    let read = StepToGeom::make_cartesian_point(&entity, &factors).unwrap();
    assert_eq!(
        read.get().unwrap().pnt(),
        Point3::new(1000.0, 2000.0, 3000.0)
    );
}

#[test]
fn null_or_two_dimensional_point_gives_no_three_dimensional_point() {
    let factors = StepDataFactors::new();
    assert!(
        StepToGeom::make_cartesian_point(&Handle::default(), &factors)
            .unwrap()
            .is_null()
    );
    let planar = GeomToStepMakeCartesianPoint::from_pnt2d(&Default::default(), 1.0)
        .value()
        .unwrap();
    assert!(
        StepToGeom::make_cartesian_point(&planar, &factors)
            .unwrap()
            .is_null()
    );
}

#[test]
fn direction_with_null_magnitude_is_refused_on_read() {
    let direction = GeomToStepMakeDirection::from_dir(&Direction3::Z)
        .value()
        .unwrap();
    assert!(StepToGeom::make_direction(&direction).unwrap().is_some());
    direction
        .set_direction_ratios(&shared(&[0.0, 0.0, 0.0]))
        .unwrap();
    assert!(StepToGeom::make_direction(&direction).unwrap().is_null());
}

#[test]
fn circle_keeps_its_frame_and_radius_through_scaled_files() {
    let factors = length_factors(25.4);
    let circle: Handle<dyn GeomCurve> =
        Handle::new(GeomCircle::new(tilted_frame(), 50.8).unwrap()).cast();
    let entity = GeomToStepMakeCurve::from_geom(&circle, &factors)
        .unwrap()
        .value()
        .unwrap();
    let read = StepToGeom::make_curve(&entity, &factors).unwrap();
    let round: Handle<GeomCircle> = read.cast();
    let live = round.get().unwrap();
    assert_near(live.radius(), 50.8);
    let frame = live.circ().unwrap().position();
    assert_near(frame.location().y(), -2.0);
    assert_near(frame.direction().z(), 0.8);
    assert_near(frame.x_direction().x(), 1.0);
}

#[test]
fn line_is_rebuilt_from_its_vector() {
    let factors = StepDataFactors::new();
    let line = GeomLine::from_point(Point3::new(1.0, 2.0, 3.0), Direction3::Y);
    let curve: Handle<dyn GeomCurve> = Handle::new(line).cast();
    let entity = GeomToStepMakeCurve::from_geom(&curve, &factors)
        .unwrap()
        .value()
        .unwrap();
    let round: Handle<GeomLine> = StepToGeom::make_curve(&entity, &factors).unwrap().cast();
    let axis = round.get().unwrap().lin();
    assert_eq!(axis.location(), Point3::new(1.0, 2.0, 3.0));
    assert_near(axis.direction().y(), 1.0);
}

/// A clamped cubic with an interior knot: poles and knots stand out by their values.
fn cubic_spline() -> Handle<GeomBSplineCurve> {
    let mut poles = Array1::with_bounds(1, 5).unwrap();
    for (index, slot) in poles.as_mut_slice().iter_mut().enumerate() {
        *slot = Point3::new(index as f64, (index * index) as f64, 1.0);
    }
    let knots = array_of(&[0.0, 0.5, 1.0]);
    let multiplicities = array_of(&[4, 1, 4]);
    Handle::new(GeomBSplineCurve::new(&poles, &knots, &multiplicities, 3, false).unwrap())
}

#[test]
fn spline_keeps_poles_knots_and_evaluation() {
    let factors = length_factors(10.0);
    let spline = cubic_spline();
    let bounded: Handle<dyn GeomBoundedCurve> = spline.clone().cast();
    let entity = GeomToStepMakeBoundedCurve::from_geom(&bounded, &factors)
        .unwrap()
        .value()
        .unwrap();
    let as_spline: Handle<StepGeomBSplineCurve> = entity.cast();
    assert_eq!(as_spline.get().unwrap().degree(), 3);
    let round = StepToGeom::make_bspline_curve(&as_spline, &factors).unwrap();
    let live = round.get().unwrap();
    assert_eq!(live.nb_knots(), 3);
    assert!(!live.is_rational());
    for parameter in [0.0, 0.2, 0.5, 0.9, 1.0] {
        let expected = spline.get().unwrap().value(parameter).unwrap();
        assert!(live.value(parameter).unwrap().distance(&expected) < 1e-12);
    }
}

#[test]
fn repeated_knots_are_merged_and_their_multiplicities_added() {
    let factors = StepDataFactors::new();
    let spline = cubic_spline();
    let bounded: Handle<dyn GeomBoundedCurve> = spline.cast();
    let entity = GeomToStepMakeBoundedCurve::from_geom(&bounded, &factors)
        .unwrap()
        .value()
        .unwrap();
    let with_knots: Handle<martensite_tkdestep::step_geom::StepGeomBSplineCurveWithKnots> =
        entity.cast();
    let live = with_knots.get().unwrap();
    live.set_knots(&shared(&[0.0, 0.0, 0.5, 1.0]));
    live.set_knot_multiplicities(&shared(&[3, 1, 1, 4]));
    let round = StepToGeom::make_bspline_curve(&with_knots.cast(), &factors).unwrap();
    assert_eq!(round.get().unwrap().multiplicity(1).unwrap(), 4);
    assert_eq!(round.get().unwrap().nb_knots(), 3);
}

#[test]
fn plane_and_cylinder_round_trip_through_the_surface_dispatch() {
    let factors = length_factors(2.0);
    let frame = Axis3::from_axis2(tilted_frame());
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    let entity = GeomToStepMakeSurface::from_geom(&plane, &factors)
        .unwrap()
        .value()
        .unwrap();
    let round: Handle<GeomPlane> = StepToGeom::make_surface(&entity, &factors).unwrap().cast();
    assert_near(round.get().unwrap().pln().position().location().x(), 1.0);

    let cylinder = GeomCylindricalSurface::new(frame, 8.0).unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(cylinder).cast();
    let entity = GeomToStepMakeSurface::from_geom(&surface, &factors)
        .unwrap()
        .value()
        .unwrap();
    let round: Handle<GeomCylindricalSurface> =
        StepToGeom::make_surface(&entity, &factors).unwrap().cast();
    assert_near(round.get().unwrap().radius(), 8.0);
}
