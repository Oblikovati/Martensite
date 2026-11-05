use super::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkgeomalgo::int_patch::{IntPatchGLine, IntPatchLineTransitions, IntPatchPoint};
use martensite_tkmath::gp::{Circle3, Line3};
pub(super) fn line(periodic: bool) -> Line {
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
    for parameter in [0.0, 1.0, 2.0] {
        let mut point = IntPatchPoint::new();
        point.set_parameter(parameter);
        source.add_vertex(&point).unwrap();
    }
    let mut line = Line::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}
fn interference(transition: Transition, parameter: f64) -> Handle<Interference> {
    InterferenceTool::make_curve_interference(
        &transition,
        Kind::Curve,
        1,
        Kind::Point,
        1,
        parameter,
    )
}
fn with_pair(
    index: i32,
    previous: Option<Transition>,
    found: Handle<Interference>,
    same_parameter: bool,
    seam: bool,
    callback: impl FnOnce(&Pairing<'_>),
) {
    let mut point = VPoint::new();
    point.set_index(index);
    let mut list = Interferences::default();
    if let Some(previous) = previous {
        list.append(interference(previous, 1.0));
    }
    let pair = Pairing {
        found: &found,
        list: &list,
        point: &point,
        edge: &Shape::default(),
        same_parameter,
        seam,
        first: 1,
        last: 3,
    };
    callback(&pair);
}
#[test]
fn empty_geometric_line_accepts_only_entering_first_endpoint() {
    with_pair(1, None, Handle::default(), false, false, |pair| {
        let mut transition = Transition::default();
        let mut parameter = 0.0;
        assert!(
            geometric::keep(
                &TopOpeBRepKPartContext::default(),
                pair,
                &mut line(false),
                &DS::new(),
                1e-8,
                &mut parameter,
                &mut transition
            )
            .unwrap()
        );
        assert_eq!(transition, entering());
    });
}
#[test]
fn geometric_pair_requires_enter_then_exit() {
    with_pair(
        3,
        Some(entering()),
        Handle::default(),
        false,
        false,
        |pair| {
            let mut transition = exiting();
            let mut parameter = 2.0;
            assert!(
                geometric::keep(
                    &TopOpeBRepKPartContext::default(),
                    pair,
                    &mut line(false),
                    &DS::new(),
                    1e-8,
                    &mut parameter,
                    &mut transition
                )
                .unwrap()
            );
        },
    );
    with_pair(
        2,
        Some(exiting()),
        Handle::default(),
        false,
        false,
        |pair| {
            let mut transition = entering();
            let mut parameter = 1.0;
            assert!(
                !geometric::keep(
                    &TopOpeBRepKPartContext::default(),
                    pair,
                    &mut line(false),
                    &DS::new(),
                    1e-8,
                    &mut parameter,
                    &mut transition
                )
                .unwrap()
            );
        },
    );
}
#[test]
fn repeated_geometric_first_endpoint_is_rejected() {
    with_pair(
        1,
        Some(entering()),
        Handle::default(),
        false,
        false,
        |pair| {
            assert!(
                !geometric::keep(
                    &TopOpeBRepKPartContext::default(),
                    pair,
                    &mut line(false),
                    &DS::new(),
                    1e-8,
                    &mut 0.0,
                    &mut exiting()
                )
                .unwrap()
            );
        },
    );
}
#[test]
fn walking_first_exit_is_corrected_to_enter() {
    with_pair(1, None, Handle::default(), false, false, |pair| {
        let mut transition = exiting();
        assert!(walking::keep(pair, &line(false), &mut transition).unwrap());
        assert_eq!(transition, entering());
    });
}
#[test]
fn walking_last_enter_is_corrected_to_exit() {
    with_pair(
        3,
        Some(entering()),
        Handle::default(),
        false,
        false,
        |pair| {
            let mut transition = entering();
            assert!(walking::keep(pair, &line(false), &mut transition).unwrap());
            assert_eq!(transition, exiting());
        },
    );
}
#[test]
fn walking_duplicate_seam_endpoint_is_rejected() {
    with_pair(
        3,
        Some(entering()),
        interference(entering(), 1.0),
        true,
        true,
        |pair| assert!(!walking::keep(pair, &line(false), &mut exiting()).unwrap()),
    );
}
#[test]
fn parameter_comparison_is_strict_and_periodic_seams_match() {
    let found = interference(entering(), 0.0);
    assert!(
        !parameter::same_parameter(&line(false), &found, precision::p_confusion(), false).unwrap()
    );
    assert!(parameter::same_parameter(&line(true), &found, 100.0, true).unwrap());
    assert!(!parameter::same_parameter(&line(true), &Handle::default(), 0.0, true).unwrap());
}
#[test]
fn periodic_pair_places_forward_endpoints_at_full_period() {
    let found = interference(entering(), 2.0);
    let mut parameter = 0.0;
    let mut line = line(true);
    parameter::periodize(&mut line, &found, &mut parameter).unwrap();
    assert_eq!(InterferenceTool::parameter(&found).unwrap(), 0.0);
    assert_eq!(parameter, std::f64::consts::TAU);
}
#[test]
fn nonperiodic_pair_uses_lower_parameter() {
    let found = interference(entering(), 2.0);
    let mut parameter = 3.0;
    parameter::periodize(&mut line(false), &found, &mut parameter).unwrap();
    assert_eq!(parameter, 2.0);
    assert_eq!(InterferenceTool::parameter(&found).unwrap(), 2.0);
}
#[test]
fn public_endpoint_pairing_retains_context_per_operation() {
    use martensite_tkbrep::{topexp::Explorer, topods};
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1.0, 1.0, -1.0, 1.0))
        .unwrap()
        .face()
        .unwrap();
    let edge = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .current()
        .clone();
    let mut line = line(false);
    for index in 1..=3 {
        line.change_vpoint(index).unwrap().change_keep(true);
    }
    line.set_vp_bounds();
    let mut point = VPoint::new();
    point.set_index(1);
    let mut context = TopOpeBRepKPartContext::default();
    let mut transition = Transition::default();
    assert!(
        funbrep_topokpart(
            &mut context,
            &Handle::default(),
            &Interferences::default(),
            &mut line,
            &point,
            &DS::new(),
            &edge,
            face.as_ref(),
            1e-8,
            &mut 0.0,
            &mut transition
        )
        .unwrap()
    );
    assert_eq!(context.last_vpoint_index, 1);
    assert_eq!(transition, entering());
    assert_eq!(TopOpeBRepKPartContext::default().last_vpoint_index, 0);
    assert!(!topods::edge(&edge).unwrap().is_null());
}
