//! Persistent curve-pair owners retain calculated state and independent value copies.
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve, GeomLine};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::extrema::ExtCC;
use martensite_tkmath::gp::{Direction3, Point3};

fn line_owner(origin: Point3, direction: [f64; 3]) -> Handle<dyn Adaptor3dCurve> {
    let direction = Direction3::new(direction[0], direction[1], direction[2]).unwrap();
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(origin, direction)).cast();
    Handle::new(GeomAdaptorCurve::with_curve(&curve).unwrap()).cast()
}

fn upper_line(z: f64) -> Handle<dyn Adaptor3dCurve> {
    line_owner(Point3::new(1.0, -1.0, z), [0.0, 1.0, 0.0])
}

fn retained_crossed_lines() -> ExtCC<'static> {
    let first = line_owner(Point3::default(), [1.0, 0.0, 0.0]);
    let second = upper_line(3.0);
    let mut extrema = ExtCC::new(1e-10, 1e-10);
    extrema.set_curve_range_shared(1, &first, 0.0, 2.0).unwrap();
    extrema
        .set_curve_range_shared(2, &second, 0.0, 2.0)
        .unwrap();
    extrema
}

#[test]
fn owned_adaptors_survive_constructor_scope_and_match_closed_form_distances() {
    let mut extrema = retained_crossed_lines();
    extrema.perform().unwrap();
    assert!(extrema.is_done());
    assert!(!extrema.is_parallel().unwrap());
    assert_eq!(extrema.nb_ext().unwrap(), 1);
    assert_eq!(extrema.square_distance(1).unwrap(), 9.0);
    assert_eq!(extrema.trimmed_square_distances().0, [11.0; 4]);
}

#[test]
fn replacing_owned_curve_keeps_results_until_the_next_perform() {
    let mut extrema = retained_crossed_lines();
    extrema.perform().unwrap();
    let before = extrema.trimmed_square_distances();
    extrema.set_curve_shared(2, &upper_line(5.0)).unwrap();
    assert!(extrema.is_done());
    assert_eq!(extrema.square_distance(1).unwrap(), 9.0);
    assert_eq!(extrema.trimmed_square_distances(), before);
    extrema.perform().unwrap();
    assert_eq!(extrema.square_distance(1).unwrap(), 25.0);
    assert_eq!(extrema.trimmed_square_distances().0, [27.0; 4]);
}

#[test]
fn value_copies_keep_completion_and_results_independent() {
    let mut original = retained_crossed_lines();
    original.perform().unwrap();
    let mut cloned = original.clone();
    cloned.set_curve_shared(2, &upper_line(5.0)).unwrap();
    cloned.perform().unwrap();
    assert_eq!(original.square_distance(1).unwrap(), 9.0);
    assert_eq!(cloned.square_distance(1).unwrap(), 25.0);
    assert_eq!(original.trimmed_square_distances().0, [11.0; 4]);
    assert_eq!(cloned.trimmed_square_distances().0, [27.0; 4]);
}

#[test]
fn invalid_rank_and_null_owners_leave_existing_calculation_available() {
    let mut extrema = retained_crossed_lines();
    extrema.perform().unwrap();
    let failure = extrema.set_curve_shared(3, &Handle::default()).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    let failure = extrema.set_curve_shared(2, &Handle::default()).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    extrema.perform().unwrap();
    assert_eq!(extrema.square_distance(1).unwrap(), 9.0);
}

fn quadratic_line_owner() -> Handle<dyn Adaptor3dCurve> {
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    for index in 1..=3 {
        poles[index] = Point3::new(f64::from(index - 1), 0.0, 0.0);
    }
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomBezierCurve::new(&poles).unwrap()).cast();
    Handle::new(GeomAdaptorCurve::with_curve(&curve).unwrap()).cast()
}

#[test]
fn general_curve_value_copy_preserves_accumulated_parameter_state() {
    let mut original = retained_crossed_lines();
    original
        .set_curve_range_shared(1, &quadratic_line_owner(), 0.0, 1.0)
        .unwrap();
    original.perform().unwrap();
    let count = original.nb_ext().unwrap();
    assert_eq!(count, 1);
    assert_eq!(original.square_distance(1).unwrap(), 9.0);
    let mut cloned = original.clone();
    cloned.set_curve_shared(2, &upper_line(5.0)).unwrap();
    cloned.perform().unwrap();
    assert_eq!(original.nb_ext().unwrap(), count);
    assert_eq!(original.square_distance(1).unwrap(), 9.0);
    assert_eq!(cloned.nb_ext().unwrap(), 2 * count);
    assert_eq!(cloned.square_distance(1).unwrap(), 25.0);
    assert_eq!(cloned.square_distance(2).unwrap(), 25.0);
}

#[test]
fn borrowed_and_owned_curve_pairs_keep_the_same_closed_form_contract() {
    let first = line_owner(Point3::default(), [1.0, 0.0, 0.0]);
    let second = upper_line(3.0);
    let mut borrowed = ExtCC::new(1e-10, 1e-10);
    borrowed
        .set_curve_range(1, first.get().unwrap(), 0.0, 2.0)
        .unwrap();
    borrowed
        .set_curve_range(2, second.get().unwrap(), 0.0, 2.0)
        .unwrap();
    borrowed.perform().unwrap();
    let mut owned = retained_crossed_lines();
    owned.perform().unwrap();
    assert_eq!(borrowed.square_distance(1).unwrap(), 9.0);
    assert_eq!(
        owned.trimmed_square_distances(),
        borrowed.trimmed_square_distances()
    );
}
