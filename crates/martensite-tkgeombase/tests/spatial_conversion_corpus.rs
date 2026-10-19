//! Complete original spatial conversion/recognition families and independent geometry checks.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-00.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-01.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-02.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-03.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-04.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-05.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-06.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/spatial_conversion_corpus/inputs-07.txt"),
        include_str!("fixtures/spatial_conversion_corpus/expected-07.txt"),
    ),
];
#[test]
fn gcv_to_bspline_complete_family() {
    super::replay_family(CHUNKS, "gcv_to_bspline", 90);
}
#[test]
fn gcv_surf_to_bspline_complete_family() {
    super::replay_family(CHUNKS, "gcv_surf_to_bspline", 29);
}
#[test]
fn gcv_split_complete_family() {
    super::replay_family(CHUNKS, "gcv_split", 130);
}
#[test]
fn gcv_split_surface_complete_family() {
    super::replay_family(CHUNKS, "gcv_split_surface", 220);
}
#[test]
fn gcv_knot_split_complete_family() {
    super::replay_family(CHUNKS, "gcv_knot_split", 25);
}
#[test]
fn gcv_knot_split_surface_complete_family() {
    super::replay_family(CHUNKS, "gcv_knot_split_surface", 28);
}
#[test]
fn gcv_bezier_complete_family() {
    super::replay_family(CHUNKS, "gcv_bezier", 35);
}
#[test]
fn gcv_bezier_surface_complete_family() {
    super::replay_family(CHUNKS, "gcv_bezier_surface", 36);
}
#[test]
fn gcv_comp_complete_family() {
    super::replay_family(CHUNKS, "gcv_comp", 60);
}
#[test]
fn gcv_comp_bezier_complete_family() {
    super::replay_family(CHUNKS, "gcv_comp_bezier", 24);
}
#[test]
fn gcv_concat_complete_family() {
    super::replay_family(CHUNKS, "gcv_concat", 24);
}
#[test]
fn gcv_c0c1_complete_family() {
    super::replay_family(CHUNKS, "gcv_c0c1", 45);
}
#[test]
fn gcv_approx_curve_complete_family() {
    super::replay_family(CHUNKS, "gcv_approx_curve", 48);
}
#[test]
fn gcv_approx_surface_complete_family() {
    super::replay_family(CHUNKS, "gcv_approx_surface", 30);
}
#[test]
fn gcv_ana_curve_complete_family() {
    super::replay_family(CHUNKS, "gcv_ana_curve", 484);
}
#[test]
fn gcv_ana_empty_complete_family() {
    super::replay_family(CHUNKS, "gcv_ana_empty", 2);
}
#[test]
fn gcv_is_linear_complete_family() {
    super::replay_family(CHUNKS, "gcv_is_linear", 14);
}
#[test]
fn gcv_get_line_complete_family() {
    super::replay_family(CHUNKS, "gcv_get_line", 3);
}
#[test]
fn gcv_get_circle_complete_family() {
    super::replay_family(CHUNKS, "gcv_get_circle", 6);
}
#[test]
fn gcv_ana_surf_complete_family() {
    super::replay_family(CHUNKS, "gcv_ana_surf", 308);
}
#[test]
fn gcv_ana_surf_empty_complete_family() {
    super::replay_family(CHUNKS, "gcv_ana_surf_empty", 1);
}
#[test]
fn gcv_same_surf_complete_family() {
    super::replay_family(CHUNKS, "gcv_same_surf", 28);
}
#[test]
fn gcv_ls_dist_complete_family() {
    super::replay_family(CHUNKS, "gcv_ls_dist", 41);
}
#[test]
fn gcv_units_complete_family() {
    super::replay_family(CHUNKS, "gcv_units", 133);
}
#[test]
fn spatial_conversion_inputs_and_retained_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xfc494d96cdf1bd8a
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x24fb59fbff9f2d11
    );
}
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Handle,
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBezierSurface, GeomCurve, GeomSurface,
};
use martensite_tkgeombase::geom_convert::{
    self as convert, BSplineSurfaceToBezierSurface, CurveToAnaCurve, Units,
};
use martensite_tkmath::{
    convert::ParameterisationType,
    gp::{Direction2, Point2, Point3},
};
fn array<T: Copy + Default>(values: &[T]) -> Array1<'static, T> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    array.as_mut_slice().copy_from_slice(values);
    array
}
#[test]
fn straight_spatial_quadratic_conversion_matches_closed_form_values_and_derivatives() {
    let poles = array(&[
        Point3::new(0., 0., 0.),
        Point3::new(2., 1., 3.),
        Point3::new(4., 2., 6.),
    ]);
    let source: Handle<dyn GeomCurve> = Handle::new(GeomBezierCurve::new(&poles).unwrap()).cast();
    let result =
        convert::curve_to_bspline_curve(&source, ParameterisationType::TgtThetaOver2).unwrap();
    assert_eq!(
        (result.degree(), result.nb_poles(), result.is_rational()),
        (2, 3, false)
    );
    for u in [0., 0.25, 0.5, 0.75, 1.] {
        let (p, d) = result.d1(u).unwrap();
        assert_eq!(p, Point3::new(4. * u, 2. * u, 6. * u));
        assert_eq!((d.x(), d.y(), d.z()), (4., 2., 6.));
    }
    assert_eq!(source.value(0.5).unwrap(), Point3::new(2., 1., 3.));
}
#[test]
fn spatial_split_keeps_corner_geometry_and_poles() {
    let poles = array(&[
        Point3::new(0., 0., 0.),
        Point3::new(2., 0., 0.),
        Point3::new(2., 3., 4.),
    ]);
    let source = Handle::new(
        GeomBSplineCurve::new(&poles, &array(&[0., 1., 2.]), &array(&[2, 1, 2]), 1, false).unwrap(),
    );
    let result = convert::split_bspline_curve(&source, 0.5, 1.5, 1e-9, true).unwrap();
    for (u, p) in [
        (0.5, Point3::new(1., 0., 0.)),
        (1., Point3::new(2., 0., 0.)),
        (1.5, Point3::new(2., 1.5, 2.)),
    ] {
        assert_eq!(result.value(u).unwrap(), p);
    }
    assert_eq!(
        (
            source.first_parameter(),
            source.last_parameter(),
            source.nb_poles()
        ),
        (0., 2., 3)
    );
    assert_eq!(source.pole(3).unwrap(), Point3::new(2., 3., 4.));
}
#[test]
fn bilinear_surface_conversion_and_patch_follow_independent_affine_formula() {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for i in 1..=2 {
        for j in 1..=2 {
            let (u, v) = (f64::from(i - 1), f64::from(j - 1));
            poles
                .set_value(i, j, Point3::new(2. * u, 4. * v, u + 2. * v))
                .unwrap();
        }
    }
    let source: Handle<dyn GeomSurface> =
        Handle::new(GeomBezierSurface::new(&poles).unwrap()).cast();
    let result = convert::surface_to_bspline_surface(&source).unwrap();
    let patches = BSplineSurfaceToBezierSurface::new(&result).unwrap();
    assert_eq!(
        (
            patches.nb_u_patches().unwrap(),
            patches.nb_v_patches().unwrap()
        ),
        (1, 1)
    );
    let patch = patches.patch(1, 1).unwrap();
    for (u, v) in [(0., 0.), (0.5, 0.25), (1., 1.)] {
        let expected = Point3::new(2. * u, 4. * v, u + 2. * v);
        assert_eq!(result.value(u, v).unwrap(), expected);
        assert_eq!(patch.value(u, v).unwrap(), expected);
    }
    assert_eq!(
        patches.patch(0, 1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn mirrored_parameter_line_has_reflected_points_without_source_mutation() {
    let source: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::from_point(Point2::new(2., 3.), Direction2::Y)).cast();
    let mirrored = Units::mirror_pcurve(&source).unwrap();
    for t in [-2., 0., 4.] {
        assert_eq!(mirrored.value(t).unwrap(), Point2::new(2., -3. - t));
        assert_eq!(source.value(t).unwrap(), Point2::new(2., 3. + t));
    }
}
#[test]
fn recognized_line_uses_independent_three_four_five_length_and_endpoints() {
    let a = Point3::new(2., 3., 4.);
    let b = Point3::new(2., 6., 8.);
    let (line, first, last) = CurveToAnaCurve::get_line(a, b).unwrap();
    assert_eq!((first, last), (0., 5.));
    assert_eq!(line.location(), a);
    assert_eq!(
        (
            line.direction().x(),
            line.direction().y(),
            line.direction().z()
        ),
        (0., 0.6, 0.8)
    );
}
#[test]
fn null_geometry_serializers_preserve_type_and_curve_surface_sentinels() {
    use super::construction_corpus::{geom_convert, print::Output};
    let output = Output::default();
    geom_convert::curve(&output, &Handle::default());
    geom_convert::surface(&output, &Handle::default());
    geom_convert::bspline(&output, &Handle::default());
    geom_convert::bezier(&output, &Handle::default());
    geom_convert::bspline_surface(&output, &Handle::default());
    geom_convert::bezier_surface(&output, &Handle::default());
    assert_eq!(output.finish(), "null null -1 -1 -1 -1 ");
}
