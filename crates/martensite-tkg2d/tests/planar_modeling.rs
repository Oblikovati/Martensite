//! Geometry regression fixtures separated by their actual algorithm dependencies.
//! Planar geometry construction, evaluation and boundary contracts.
#![allow(non_snake_case)]
mod common;
use common::{
    EXPECT_DOUBLE_EQ, EXPECT_NEAR as assert_modeling_near, modeling_circle as analytic_circle_fixture,
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{
    Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dGeometry, Geom2dLine, Geom2dOffsetCurve,
};
use martensite_tkmath::gp::{Axis2d, Axis22d, Direction2, Point2, Vector2};
use std::f64::consts::PI;

fn collapsed_offset_fixture() -> Geom2dOffsetCurve {
    Geom2dOffsetCurve::new(
        &analytic_circle_fixture(Point2::default(), 3.0),
        -3.0,
        false,
    )
    .unwrap()
}

fn regular_offset_fixture() -> Geom2dOffsetCurve {
    let circle: Handle<dyn Geom2dCurve> = Handle::new(
        Geom2dCircle::from_axis(Axis2d::new(Point2::default(), Direction2::X), 5.0, true).unwrap(),
    )
    .cast();
    Geom2dOffsetCurve::new(&circle, 2.0, false).unwrap()
}

#[test]
fn curve_eval_circle_eval_d0_d1_valid_results() {
    let curve = analytic_circle_fixture(Point2::default(), 4.0);
    let point = curve.eval_d0(PI / 4.0).unwrap();
    assert_modeling_near(
        point.distance(Point2::default()),
        4.0,
        precision::confusion(),
    );
    let result = curve.eval_d1(PI / 4.0).unwrap();
    assert_modeling_near(result.point.distance(point), 0.0, precision::confusion());
    assert_modeling_near(
        Vector2::between(Point2::default(), result.point).dot(result.d1),
        0.0,
        1e-10,
    );
}

#[test]
fn curve_eval_offset_curve_eval_d0_valid_at_degenerate_center() {
    let point = collapsed_offset_fixture().eval_d0(0.0).unwrap();
    assert_modeling_near(
        point.distance(Point2::default()),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn curve_eval_ellipse_eval_d2_d3_valid_results() {
    let curve = Geom2dEllipse::new(
        Axis22d::new(Point2::default(), Direction2::X, true).unwrap(),
        5.0,
        3.0,
    )
    .unwrap();
    assert!(curve.eval_d2(PI / 4.0).unwrap().d2.magnitude() > 0.0);
    assert!(curve.eval_d3(PI / 4.0).unwrap().d3.magnitude() > 0.0);
}

#[test]
fn curve_eval_line_eval_dn_valid_results() {
    let curve = Geom2dLine::from_point(Point2::new(1.0, 2.0), Direction2::X);
    let first = curve.eval_dn(5.0, 1).unwrap();
    assert_modeling_near(first.x(), 1.0, precision::confusion());
    assert_modeling_near(first.y(), 0.0, precision::confusion());
    assert_modeling_near(
        curve.eval_dn(5.0, 2).unwrap().magnitude(),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn curve_eval_offset_curve_eval_d1_degenerate_at_collapsed() {
    // The basis tangent is valid, so this succeeds even though the offset collapses.
    let result = collapsed_offset_fixture().eval_d1(0.0).unwrap();
    assert_modeling_near(
        result.point.distance(Point2::default()),
        0.0,
        precision::confusion(),
    );
}

#[test]
fn offset_curve_copy_constructor_basic_properties() {
    let original = regular_offset_fixture();
    let copy = Geom2dOffsetCurve::copy_of(&original).unwrap();
    EXPECT_DOUBLE_EQ(original.offset(), copy.offset());
    assert_eq!(original.is_periodic(), copy.is_periodic());
    assert_eq!(original.is_closed(), copy.is_closed());
}

#[test]
fn offset_curve_copy_constructor_basis_curve() {
    let original = regular_offset_fixture();
    let copy = Geom2dOffsetCurve::copy_of(&original).unwrap();
    let first = original.basis_curve();
    let second = copy.basis_curve();
    assert!(first != second);
    EXPECT_DOUBLE_EQ(first.first_parameter(), second.first_parameter());
    EXPECT_DOUBLE_EQ(first.last_parameter(), second.last_parameter());
}

#[test]
fn offset_curve_copy_method_uses_optimized_constructor() {
    let original = regular_offset_fixture();
    let copy = original.copy().unwrap().cast::<Geom2dOffsetCurve>();
    assert!(!copy.is_null());
    EXPECT_DOUBLE_EQ(original.offset(), copy.offset());
    let mut u = original.first_parameter();
    let last = original.last_parameter();
    let step = (last - u) / 4.0;
    while u <= last {
        assert!(
            original
                .value(u)
                .unwrap()
                .is_equal(copy.value(u).unwrap(), 1e-10)
        );
        u += step;
    }
}

#[test]
fn offset_curve_copy_independence() {
    let original = regular_offset_fixture();
    let copy = Geom2dOffsetCurve::copy_of(&original).unwrap();
    let offset = copy.offset();
    original.set_offset_value(10.0);
    EXPECT_DOUBLE_EQ(copy.offset(), offset);
    assert_ne!(copy.offset(), original.offset());
}
