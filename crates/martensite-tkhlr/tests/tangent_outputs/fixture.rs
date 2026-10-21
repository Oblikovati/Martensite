//! One owned analytic fixture matrix, shared with the isolated capture test.
#[path = "../common/geometry_records/mod.rs"]
pub(crate) mod geometry_records;
use geometry_records::common::Reader;
use martensite_tkhlr::{hlr_algo::init_min_max, hlr_brep::Curve};
use std::sync::Arc;
#[path = "curve_fixture.rs"]
pub(crate) mod tangent_curve_fixture;

pub(crate) fn projected_fixture(label: &str) -> Curve {
    let root = (0.5_f64).sqrt();
    let special = [0., 0., 0., 0., 0., 1., 1., root, -0.5, 0.5];
    let view: &[f64] = if label.starts_with("type0") {
        &special
    } else {
        &[2.]
    };
    let projector = tangent_curve_fixture::projector(&mut Reader::new(view)).unwrap();
    let edge = tangent_curve_fixture::edge(&mut Reader::new(geometry(label))).unwrap();
    let mut curve = Curve::new();
    curve.set_projector(Some(Arc::new(projector)));
    curve.set_curve(&edge).unwrap();
    let (mut low, mut high) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut low, &mut high);
    curve.update(&mut low, &mut high).unwrap();
    curve
}

fn geometry(label: &str) -> &'static [f64] {
    match label {
        "top_x" | "type0_x" => &[0., 1., 2., 3., 1., 0., 0., 0., 2.],
        "top_z" => &[0., 1., 2., 3., 0., 0., 1., 0., 2.],
        "type0_normal" => &[0., 1., 2., 3., 0., 1., 1., 0., 2.],
        "stationary_start" => &[3., 3., 0., 0., 3., 0., 0., 3., 1., 0., 3., 0., 1.],
        _ => panic!("unexpected tangent fixture {label}; expected one of five analytic cases"),
    }
}
