//! Supplemental closed forms: a unit-speed 3–4 direction and its plane embedding.
use super::super::construction_corpus::{common::Reader, geometry_adaptor_fixtures as fixtures};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeombase::approx::{
    Curve2d, Curve3d, CurveOnSurface, CurveOnSurfaceOptions, CurvilinearOptions,
    CurvilinearParameter, CurvlinFunc, SameParameter,
};
use martensite_tkmath::geom_abs::Shape;
const SEGMENT: &[f64] = &[1., 0., 5., 1., 0., 0., 0., 3., 4., 0.];
const PLANE: &[f64] = &[1., 0., 0., 0., 0., 0., 1., 1., 0., 0., 0.];
const PLANAR_LINE: &[f64] = &[8., 0., 5., 1., 1., 0., 0., 3., 4.];

#[test]
fn linear_approximation_preserves_independent_affine_coordinates() -> Result<(), Failure> {
    let line = fixtures::adapted_curve(&mut Reader::new(SEGMENT))?;
    let approximation = Curve3d::new(&line, 1e-7, Shape::C2, 10, 8)?;
    assert!(approximation.is_done() && approximation.has_result());
    let curve = approximation.curve();
    assert_affine_space_segment(curve.get().unwrap(), 5.)?;
    Ok(())
}

#[test]
fn planar_approximation_preserves_independent_affine_coordinates() -> Result<(), Failure> {
    let line = fixtures::adapted_curve2d(&mut Reader::new(PLANAR_LINE))?;
    let approximation = Curve2d::new(&line, 0., 5., 1e-7, 1e-7, Shape::C1, 8, 10)?;
    assert!(approximation.is_done() && approximation.has_result());
    let curve = approximation.curve();
    for i in 0..=20 {
        let u = f64::from(i) / 4.;
        let point = curve.get().unwrap().value(u)?;
        assert!((point.x() - 0.6 * u).abs() < 1e-7);
        assert!((point.y() - 0.8 * u).abs() < 1e-7);
    }
    Ok(())
}

#[test]
fn deferred_plane_approximation_publishes_both_coordinate_curves() -> Result<(), Failure> {
    let line = fixtures::adapted_curve2d(&mut Reader::new(PLANAR_LINE))?;
    let plane = fixtures::adapted_surface(&mut Reader::new(PLANE))?;
    let mut approximation = CurveOnSurface::without_perform(&line, &plane, 0., 5., 1e-7);
    assert!(!approximation.is_done() && !approximation.has_result());
    let options = CurveOnSurfaceOptions {
        max_segments: 10,
        max_degree: 8,
        continuity: Shape::C1,
        only_3d: false,
        only_2d: false,
    };
    approximation.perform(options)?;
    assert!(approximation.is_done() && approximation.has_result());
    assert_plane_midpoint(&approximation)?;
    Ok(())
}

#[test]
fn normalized_abscissa_has_closed_form_length_position_and_unsupported_order() -> Result<(), Failure>
{
    let line = fixtures::adapted_curve(&mut Reader::new(SEGMENT))?;
    let function = CurvlinFunc::from_curve(&line, 1e-7)?;
    assert!((function.get_length() - 5.).abs() < 1e-9);
    assert!((function.get_s_parameter(2.5)? - 0.5).abs() < 1e-9);
    let mut coordinates = Array1::with_bounds(0, 2).unwrap();
    coordinates.init(-7.);
    assert!(function.eval_case1(0.5, 0, &mut coordinates)?);
    assert!((coordinates[0] - 1.5).abs() < 1e-7);
    assert!((coordinates[1] - 2.).abs() < 1e-7 && coordinates[2].abs() < 1e-7);
    assert!(!function.eval_case1(0.5, 3, &mut coordinates)?);
    assert!(coordinates.iter().all(|coordinate| *coordinate == 0.));
    Ok(())
}

#[test]
fn curvilinear_reparametrization_preserves_unit_interval_segment() -> Result<(), Failure> {
    let line = fixtures::adapted_curve(&mut Reader::new(SEGMENT))?;
    let options = CurvilinearOptions {
        order: Shape::C1,
        max_degree: 8,
        max_segments: 10,
    };
    let approximation = CurvilinearParameter::from_curve(&line, 1e-7, options)?;
    assert!(approximation.is_done() && approximation.has_result());
    let curve = approximation.curve_3d();
    let curve = curve.get().unwrap();
    assert_eq!((curve.first_parameter(), curve.last_parameter()), (0., 1.));
    assert_affine_space_segment(curve, 1.)?;
    Ok(())
}

#[test]
fn matching_plane_parameterization_needs_no_replacement_pcurve() -> Result<(), Failure> {
    use super::super::construction_corpus::geometry_record_fixtures::{curve, curve2d, surface};
    let space = curve(&mut Reader::new(&[
        8., 0., 5., 1., 1., 0., 0., 0., 3., 4., 0.,
    ]))?;
    let plane = surface(&mut Reader::new(PLANE))?;
    let planar = curve2d(&mut Reader::new(PLANAR_LINE))?;
    let matching = SameParameter::from_geometry(&space, &planar, &plane, 1e-7)?;
    assert!(matching.is_done() && matching.is_same_parameter());
    assert!(matching.curve_2d().is_null());
    assert!(matching.tol_reached() <= 1e-7);
    Ok(())
}

fn assert_affine_space_segment(curve: &dyn GeomCurve, last: f64) -> Result<(), Failure> {
    for i in 0..=20 {
        let fraction = f64::from(i) / 20.;
        let point = curve.value(last * fraction)?;
        assert!((point.x() - 3. * fraction).abs() < 1e-7);
        assert!((point.y() - 4. * fraction).abs() < 1e-7);
        assert!(point.z().abs() < 1e-7);
    }
    Ok(())
}

fn assert_plane_midpoint(approximation: &CurveOnSurface) -> Result<(), Failure> {
    let space = approximation.curve_3d();
    let planar = approximation.curve_2d();
    let point = space.get().unwrap().value(2.5)?;
    let uv = planar.get().unwrap().value(2.5)?;
    assert!((point.x() - 1.5).abs() < 1e-7 && (point.y() - 2.).abs() < 1e-7);
    assert!(point.z().abs() < 1e-7);
    assert!((uv.x() - 1.5).abs() < 1e-7 && (uv.y() - 2.).abs() < 1e-7);
    Ok(())
}
