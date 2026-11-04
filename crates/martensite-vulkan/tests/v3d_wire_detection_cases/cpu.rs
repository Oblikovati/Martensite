use super::*;
use martensite_tkbrep::topexp;
use martensite_tkernel::quantity::Color;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkgeombase::gcpnts::UniformAbscissa;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

#[test]
fn authored_pixel_predicate_requires_every_pair_to_differ() {
    let colors = ["0 0 0".to_owned(), "1 0 0".to_owned(), "0 1 0".to_owned()];
    assert!(detection::distinct_observations(&colors));
    for pair in [(0, 1), (0, 2), (1, 2)] {
        let mut repeated = colors.clone();
        repeated[pair.1] = repeated[pair.0].clone();
        assert!(!detection::distinct_observations(&repeated));
    }
    assert_eq!(
        detection::rgb_result(
            Color::new(0., 1., 0., martensite_tkernel::quantity::ColorSpace::Rgb).unwrap()
        ),
        "0 1 0"
    );
}

#[test]
fn actual_wire_command_computes_all_case_samples_without_view() {
    let wire = read_dataset_shape("OCC105.brep").unwrap();
    assert!(crate::shape_checks::check_shape_valid(&wire).unwrap());
    wire_abscissa::sample_wire(&wire).unwrap();
}

fn closed_line_samples(reverse: bool) -> Vec<martensite_tkmath::gp::Point3> {
    let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(2., 0., 0.))
        .unwrap()
        .edge()
        .unwrap();
    let (first_vertex, last_vertex) = topexp::vertices(&edge, false);
    let mut previous = if reverse { last_vertex } else { first_vertex };
    let (curve, first, last) = wire_abscissa::oriented_curve(&edge, &mut previous).unwrap();
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    let samples = UniformAbscissa::with_abscissa_range(&adaptor, 0.5, first, last, -1.).unwrap();
    assert!(samples.is_done());
    (1..=samples.nb_points().unwrap())
        .map(|i| adaptor.value(samples.parameter(i).unwrap()).unwrap())
        .collect()
}

#[test]
fn both_actual_orientation_branches_sample_closed_form_half_unit_line_points() {
    for reverse in [false, true] {
        let mut points = closed_line_samples(reverse);
        points.sort_by(|a, b| a.x().total_cmp(&b.x()));
        assert_eq!(points.len(), 5);
        for (index, point) in points.iter().enumerate() {
            assert_eq!(
                (point.x(), point.y(), point.z()),
                (index as f64 * 0.5, 0., 0.)
            );
        }
    }
}
