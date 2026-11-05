//! Current-line bounds include suppressed VPoints and retain periodic endpoint rules.
use martensite_tkbool::top_ope_brep::{TopOpeBRepFacesFiller, TopOpeBRepLineInter};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::{
    Handle,
    real::{real_first, real_last},
};
use martensite_tkgeomalgo::int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint};
use martensite_tkmath::gp::{Circle3, Line3};

fn line(parameters: &[f64], periodic: bool) -> TopOpeBRepLineInter {
    let source = if periodic {
        IntPatchGLine::from_circle(
            &Circle3::default(),
            false,
            IntPatchLineTransitions::Undecided,
        )
    } else {
        IntPatchGLine::from_line(&Line3::default(), false, IntPatchLineTransitions::Undecided)
            .unwrap()
    };
    for parameter in parameters {
        let mut point = IntPatchPoint::default();
        point.set_parameter(*parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}

#[test]
fn empty_line_preserves_native_reversed_finite_sentinels() {
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut TopOpeBRepLineInter::new()).unwrap(),
        (real_last(), real_first())
    );
}

#[test]
fn suppressed_points_still_enlarge_the_current_line_bounds() {
    let mut source = line(&[-4.0, 2.0, 9.0], false);
    source.change_vpoint(3).unwrap().change_keep(false);
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut source).unwrap(),
        (-4.0, 9.0)
    );
}

#[test]
fn coincident_periodic_endpoints_cover_one_full_period() {
    let mut source = line(&[3.0, 3.0], true);
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut source).unwrap(),
        (3.0, 3.0 + std::f64::consts::TAU)
    );
}

#[test]
fn single_periodic_point_and_noncoincident_endpoints_are_not_extended() {
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut line(&[3.0], true)).unwrap(),
        (3.0, 3.0)
    );
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut line(&[3.0, 5.0], true)).unwrap(),
        (3.0, 5.0)
    );
}

#[test]
fn restriction_membership_uses_the_face_tolerance_and_bounded_projection() {
    use martensite_tkernel::ncollection::List;
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
    let edge = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(0., 0.00001, 0.),
        Point3::new(0., 0.00001, 1.),
    )
    .unwrap()
    .edge()
    .unwrap();
    let mut edges = List::default();
    edges.append(edge.into());
    let mut source = line(&[0., 1.], false);
    assert!(!TopOpeBRepFacesFiller::l_same_domain_erl(&mut source, &edges, 1e-7).unwrap());
    assert!(TopOpeBRepFacesFiller::l_same_domain_erl(&mut source, &edges, 1e-4).unwrap());
    assert!(
        !TopOpeBRepFacesFiller::l_same_domain_erl(&mut line(&[0.], false), &edges, 1e-4).unwrap()
    );
    assert!(
        !TopOpeBRepFacesFiller::l_same_domain_erl(&mut source, &List::default(), 1e-4).unwrap()
    );
}

#[test]
fn restriction_tolerance_max_preserves_native_nan_operand_order() {
    use martensite_tkernel::ncollection::List;
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
    let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(0., 0., 1.))
        .unwrap()
        .edge()
        .unwrap();
    let mut edges = List::default();
    edges.append(edge.into());
    assert!(
        TopOpeBRepFacesFiller::l_same_domain_erl(&mut line(&[0., 1.], false), &edges, f64::NAN)
            .unwrap()
    );
}

#[test]
fn equal_zero_bounds_retain_the_first_native_operand_bits() {
    for first in [0.0_f64, -0.0_f64] {
        let (minimum, maximum) =
            TopOpeBRepFacesFiller::lminmax(&mut line(&[first, -first], false)).unwrap();
        assert_eq!(minimum.to_bits(), first.to_bits());
        assert_eq!(maximum.to_bits(), first.to_bits());
    }
    assert_eq!(
        TopOpeBRepFacesFiller::lminmax(&mut line(&[f64::NAN], false)).unwrap(),
        (real_last(), real_first())
    );
}
