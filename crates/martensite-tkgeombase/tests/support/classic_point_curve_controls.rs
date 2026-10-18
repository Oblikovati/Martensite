//! Exact line geometry supplements the retained baseline and preserves discrete sampling semantics.
use super::super::construction_corpus::extrema_pc::{Space, Space2, Space3};
use super::super::construction_corpus::print::Reader;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::extrema::{
    Curve2dTool, CurveTool, ExtremaCurveTool, GCurveLocator, GFuncExtPC, GGExtPC, GGenExtPC,
    GLocateExtPC, GenLocateExtPC,
};
use martensite_tkmath::gp::{Point2, Point3};

fn planar_segment() -> Handle<dyn Adaptor2dCurve2d> {
    let fields = [0., 0., 0., 1., 0., 1., 0., 6.];
    Space2::adaptor(&mut Reader::new(&fields)).unwrap()
}
fn spatial_segment() -> Handle<dyn Adaptor3dCurve> {
    let fields = [0., 0., 0., 0., 1., 0., 0., 1., 0., 6.];
    Space3::adaptor(&mut Reader::new(&fields)).unwrap()
}
#[test]
fn planar_closed_projection_and_both_endpoint_distances_are_exact() {
    let curve = planar_segment();
    let result = GGExtPC::<Curve2dTool>::with_curve(Point2::new(3., 4.), &*curve, 1e-9).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.point(1).unwrap().parameter(), 3.);
    assert_eq!(result.square_distance(1).unwrap(), 16.);
    assert!(result.is_min(1).unwrap());
    assert_eq!(
        result.trimmed_square_distances(),
        (25., 25., Point2::new(0., 0.), Point2::new(6., 0.))
    );
}
#[test]
fn spatial_closed_projection_and_both_endpoint_distances_are_exact() {
    let curve = spatial_segment();
    let result = GGExtPC::<CurveTool>::with_curve(Point3::new(3., 4., 5.), &*curve, 1e-9).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.point(1).unwrap().parameter(), 3.);
    assert_eq!(result.square_distance(1).unwrap(), 41.);
    assert_eq!(
        result.trimmed_square_distances(),
        (50., 50., Point3::new(0., 0., 0.), Point3::new(6., 0., 0.))
    );
}
#[test]
fn planar_generic_function_has_normalized_linear_value_and_unit_derivative() {
    let curve = planar_segment();
    let mut function =
        GFuncExtPC::<Curve2dTool>::with_point_curve(Point2::new(3., 4.), &*curve).unwrap();
    for u in [0., 2., 3., 6.] {
        let (mut f, mut df) = (7.5, 7.5);
        assert!(function.values(u, &mut f, &mut df).unwrap());
        assert_eq!((f, df), (u - 3., 1.));
    }
}
#[test]
fn spatial_generic_function_has_normalized_linear_value_and_unit_derivative() {
    let curve = spatial_segment();
    let mut function =
        GFuncExtPC::<CurveTool>::with_point_curve(Point3::new(3., 4., 5.), &*curve).unwrap();
    for u in [0., 2., 3., 6.] {
        let (mut f, mut df) = (7.5, 7.5);
        assert!(function.values(u, &mut f, &mut df).unwrap());
        assert_eq!((f, df), (u - 3., 1.));
    }
}
#[test]
fn explicit_state_registration_saves_the_line_minimum() {
    let curve = planar_segment();
    let mut function =
        GFuncExtPC::<Curve2dTool>::with_point_curve(Point2::new(3., 4.), &*curve).unwrap();
    let mut f = 7.5;
    assert!(function.value(3., &mut f).unwrap());
    assert_eq!(f, 0.);
    assert_eq!(function.get_state_number().unwrap(), 0);
    assert_eq!(function.nb_ext(), 1);
    assert_eq!(function.square_distance(1).unwrap(), 16.);
    assert!(function.is_min(1).unwrap());
    assert_eq!(*function.point(1).unwrap().value(), Point2::new(3., 0.));
}
#[test]
fn uninitialized_generic_function_rejects_before_mutating_the_output() {
    let mut function = GFuncExtPC::<Curve2dTool>::new();
    let mut f = 7.5;
    assert_eq!(
        function.value(3., &mut f).unwrap_err().exception_type(),
        "Standard_TypeMismatch"
    );
    function.set_point(Point2::new(3., 4.));
    assert_eq!(
        function.value(3., &mut f).unwrap_err().exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!(f, 7.5);
    assert_eq!(function.nb_ext(), 0);
}
#[test]
fn generic_planar_root_search_recovers_the_unique_projection() {
    let curve = planar_segment();
    let result =
        GGenExtPC::<Curve2dTool>::with_range(Point2::new(3., 4.), &*curve, 8, (0., 6.), 1e-9, 1e-9)
            .unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.point(1).unwrap().parameter(), 3.);
    assert_eq!(result.square_distance(1).unwrap(), 16.);
}
#[test]
fn generic_locate_recovers_the_unique_spatial_line_root() {
    let curve = spatial_segment();
    let result = GenLocateExtPC::<CurveTool>::with_range(
        Point3::new(3., 4., 5.),
        &*curve,
        0.5,
        (0., 6.),
        1e-9,
    )
    .unwrap();
    assert!(result.is_done());
    assert_eq!(result.point().unwrap().parameter(), 3.);
    assert_eq!(result.square_distance().unwrap(), 41.);
}
#[test]
fn borrowed_and_owned_planar_locators_share_defined_projection_values() {
    let curve = planar_segment();
    let borrowed =
        GLocateExtPC::<Curve2dTool>::with_curve(Point2::new(3., 4.), &*curve, 0.5, 1e-9).unwrap();
    let mut owned = GLocateExtPC::<Curve2dTool>::new();
    owned
        .initialize_owned(curve.clone(), (0., 6.), 1e-9)
        .unwrap();
    owned.perform(Point2::new(3., 4.), 0.5).unwrap();
    assert_eq!(owned.point().unwrap(), borrowed.point().unwrap());
    assert_eq!(owned.square_distance().unwrap(), 16.);
}
#[test]
fn borrowed_and_owned_spatial_locators_share_defined_projection_values() {
    let curve = spatial_segment();
    let borrowed =
        GLocateExtPC::<CurveTool>::with_curve(Point3::new(3., 4., 5.), &*curve, 0.5, 1e-9).unwrap();
    let mut owned = GLocateExtPC::<CurveTool>::new();
    owned
        .initialize_owned(curve.clone(), (0., 6.), 1e-9)
        .unwrap();
    owned.perform(Point3::new(3., 4., 5.), 0.5).unwrap();
    assert_eq!(owned.point().unwrap(), borrowed.point().unwrap());
    assert_eq!(owned.square_distance().unwrap(), 41.);
}
#[test]
fn discrete_planar_locator_chooses_among_zero_two_four_and_omits_six() {
    let curve = planar_segment();
    let point = GCurveLocator::<Curve2dTool>::locate(Point2::new(6., 4.), &*curve, 4).unwrap();
    assert_eq!(
        (point.parameter(), *point.value()),
        (4., Point2::new(4., 0.))
    );
}
#[test]
fn discrete_spatial_locator_chooses_among_zero_two_four_and_omits_six() {
    let curve = spatial_segment();
    let point =
        GCurveLocator::<CurveTool>::locate_range(Point3::new(6., 4., 5.), &*curve, 4, 0., 6.)
            .unwrap();
    assert_eq!(
        (point.parameter(), *point.value()),
        (4., Point3::new(4., 0., 0.))
    );
}
#[test]
fn discrete_locator_rejects_fewer_than_two_samples() {
    let curve = planar_segment();
    for count in [-1, 0, 1] {
        let error =
            GCurveLocator::<Curve2dTool>::locate(Point2::new(6., 4.), &*curve, count).unwrap_err();
        assert_eq!(error.exception_type(), "Standard_OutOfRange");
    }
}
#[test]
fn line_tools_preserve_nonperiodicity_and_unit_parameter_resolution() {
    let planar = planar_segment();
    let spatial = spatial_segment();
    assert!(!Curve2dTool::is_periodic(&*planar).unwrap());
    assert!(!Curve2dTool::is_closed(&*planar).unwrap());
    assert!(!CurveTool::is_periodic(&*spatial).unwrap());
    assert_eq!(Curve2dTool::resolution(&*planar, 1e-7).unwrap(), 1e-7);
    assert_eq!(CurveTool::resolution(&*spatial, 1e-7).unwrap(), 1e-7);
}
