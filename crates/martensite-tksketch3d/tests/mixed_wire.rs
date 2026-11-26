// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkbrep::{brep::BRepTool, topexp, topods::Edge};
use martensite_tkg3d::{geom::GeomCircle, top_abs::Orientation};
use martensite_tkmath::gp::Direction3;
use martensite_tksketch3d::{
    ArcDefinition, ArcFitTolerance, ArcId, CurveId, Edit, MinimumLength, PointRole, SketchArc,
    Snapshot,
    bend::{BendIds, BendOptions, BendRequest, BendResult},
    wire::{
        LineDirection, WireClosure, WireFailure, WirePhase,
        mixed::{DirectedCurveUse, MixedWireRecipe, NativeMixedTrim, WireArcScope},
    },
};
use std::{
    f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4},
    num::NonZeroU64,
};

fn arc_id(value: u64) -> ArcId {
    ArcId::new(NonZeroU64::new(value).unwrap())
}

fn bend() -> BendResult {
    let source = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                seed(1, [30., 0., 0.], PointRole::Defining),
                seed(2, [60., 0., 0.], PointRole::Defining),
                seed(3, [60., 30., 0.], PointRole::Defining),
                line(4, 1, 2),
                line(5, 2, 3),
            ],
            || false,
        )
        .unwrap();
    let request = BendRequest {
        lines: [lid(4), lid(5)],
        corner: pid(2),
        radius: 5.,
        ids: BendIds {
            lines: [lid(8), lid(9)],
            arc: arc_id(10),
            tangencies: [pid(6), pid(7)],
            center: pid(11),
        },
        options: BendOptions {
            max_position_error: 0.,
            max_radius_error: 0.,
            max_tangent_angle: f64::EPSILON,
            arc_fit: ArcFitTolerance::new(1e-12).unwrap(),
        },
    };
    source.bend_lines(request, || false).unwrap()
}

fn use_forward(curve: CurveId) -> DirectedCurveUse {
    DirectedCurveUse {
        curve,
        direction: LineDirection::Forward,
    }
}

fn bend_uses() -> [DirectedCurveUse; 3] {
    [
        CurveId::Line(lid(8)),
        CurveId::Arc(arc_id(10)),
        CurveId::Line(lid(9)),
    ]
    .map(use_forward)
}

fn wire(source: &Snapshot, uses: &[DirectedCurveUse], closure: WireClosure) -> MixedWireRecipe {
    source
        .mixed_wire_recipe(uses, closure, 64, || false)
        .unwrap()
}

#[test]
fn owned_bend_exports_actual_locus_parameters_and_shared_endpoint_ancestry() {
    let bend = bend();
    let recipe = wire(&bend.sketch, &bend_uses(), WireClosure::Open);
    assert_eq!(recipe.validation().pair_checks, 3);
    let output = recipe.materialize(|| false).unwrap();
    assert_eq!(output.edges.len(), 3);
    assert_eq!(
        output.points.iter().map(|b| b.point).collect::<Vec<_>>(),
        [pid(1), pid(3), pid(6), pid(7)]
    );
    let [before, arc, after] = [&output.edges[0], &output.edges[1], &output.edges[2]];
    assert_eq!(arc.source.curve, CurveId::Arc(arc_id(10)));
    assert_eq!(arc.endpoints, [pid(6), pid(7)]);
    assert_eq!(
        before.trim,
        NativeMixedTrim::Line {
            parameters: [0., 25.]
        }
    );
    assert_eq!(
        after.trim,
        NativeMixedTrim::Line {
            parameters: [0., 25.]
        }
    );
    assert_join(&before.edge, &arc.edge);
    assert_join(&arc.edge, &after.edge);
    assert_arc(
        &arc.edge,
        arc.trim,
        bend.evidence.nominal_terminal_error_upper,
    );
    println!(
        "mixed-wire-bend-receipt={:?}",
        output
            .edges
            .iter()
            .map(|entry| (entry.source, entry.endpoints, entry.trim))
            .collect::<Vec<_>>()
    );
}

fn assert_join(before: &Edge, after: &Edge) {
    assert!(topexp::last_vertex(before, true).is_same(&topexp::first_vertex(after, true)));
}

fn assert_arc(edge: &Edge, trim: NativeMixedTrim, bound: f64) {
    let NativeMixedTrim::Arc {
        parameters,
        center,
        center_position,
        radius,
        tangents,
        position_error_upper,
        ..
    } = trim
    else {
        panic!("expected native arc trim");
    };
    assert_eq!(parameters, [0., FRAC_PI_2]);
    assert_eq!(center, pid(11));
    assert_eq!(center_position, point([55., 5., 0.]));
    assert_eq!(radius, 5.);
    assert_eq!(position_error_upper, bound);
    assert_eq!(
        tangents.map(|end| end.direction),
        [Direction3::X, Direction3::Y]
    );
    assert_eq!(
        tangents.map(|end| end.angle_error_upper),
        [0., f64::EPSILON]
    );
    let stored = BRepTool::curve_with_location(edge).unwrap();
    assert_eq!([stored.first, stored.last], parameters);
    let native = stored.curve.downcast_ref::<GeomCircle>().unwrap();
    assert_eq!(native.radius(), 5.);
    let expected = point([55. + 5. * FRAC_1_SQRT_2, 5. - 5. * FRAC_1_SQRT_2, 0.]);
    let middle = stored.curve.get().unwrap().eval_d0(FRAC_PI_4).unwrap();
    assert!(middle.distance(&expected) < 1e-12);
}

#[test]
fn reversed_bend_and_closing_wrap_keep_native_and_authored_senses() {
    let bend = bend();
    let mut uses = bend_uses();
    uses.reverse();
    uses.iter_mut()
        .for_each(|entry| entry.direction = LineDirection::Reversed);
    let output = wire(&bend.sketch, &uses, WireClosure::Open)
        .materialize(|| false)
        .unwrap();
    assert_eq!(output.edges[1].edge.orientation(), Orientation::Reversed);
    assert_eq!(output.edges[1].endpoints, [pid(7), pid(6)]);
    let NativeMixedTrim::Arc {
        parameters,
        tangents,
        ..
    } = output.edges[1].trim
    else {
        panic!("expected native arc");
    };
    assert_eq!(parameters, [FRAC_PI_2, 0.]);
    assert_eq!(
        tangents.map(|end| end.direction),
        [Direction3::Y.reversed(), Direction3::X.reversed()]
    );
    assert_eq!(
        tangents.map(|end| end.angle_error_upper),
        [f64::EPSILON, 0.]
    );
    for pair in output.edges.windows(2) {
        assert_join(&pair[0].edge, &pair[1].edge);
    }
    assert_wrapped_join(&bend.sketch);
}

fn assert_wrapped_join(source: &Snapshot) {
    let closed = source
        .apply(
            &[
                seed(12, [20., 30., 0.], PointRole::Defining),
                seed(13, [20., 0., 0.], PointRole::Defining),
                line(14, 3, 12),
                line(15, 12, 13),
                line(16, 13, 1),
            ],
            || false,
        )
        .unwrap();
    let uses = [
        CurveId::Arc(arc_id(10)),
        CurveId::Line(lid(9)),
        CurveId::Line(lid(14)),
        CurveId::Line(lid(15)),
        CurveId::Line(lid(16)),
        CurveId::Line(lid(8)),
    ]
    .map(use_forward);
    let recipe = wire(&closed, &uses, WireClosure::Closed);
    assert_eq!(recipe.validation().pair_checks, 15);
    let output = recipe.materialize(|| false).unwrap();
    assert!(BRepTool::is_closed(&output.wire).unwrap());
    assert_join(&output.edges[5].edge, &output.edges[0].edge);
}

#[test]
fn exhaustive_arc_pair_proof_refuses_box_overlap_and_real_crossing() {
    let bend = bend();
    for line_end in [56., 61.] {
        let source = bend
            .sketch
            .apply(
                &[
                    seed(12, [54., 30., 0.], PointRole::Defining),
                    seed(13, [54., 2., 0.], PointRole::Defining),
                    seed(14, [line_end, 2., 0.], PointRole::Defining),
                    line(15, 3, 12),
                    line(16, 12, 13),
                    line(17, 13, 14),
                ],
                || false,
            )
            .unwrap();
        let mut uses = bend_uses().to_vec();
        uses.extend([15, 16, 17].map(|key| use_forward(CurveId::Line(lid(key)))));
        assert!(matches!(
            source.mixed_wire_recipe(&uses, WireClosure::Open, 64, || false),
            Err(WireFailure::UnprovedCurvePair { curves, adjacent: false })
                if curves == [CurveId::Arc(arc_id(10)), CurveId::Line(lid(17))]
        ));
    }
}

#[test]
fn refuses_non_tangent_use_unproved_arcs_and_small_pair_grant() {
    let bend = bend();
    let uses = bend_uses();
    assert!(matches!(
        bend.sketch
            .mixed_wire_recipe(&uses, WireClosure::Open, 2, || false),
        Err(WireFailure::MixedPairLimit { limit: 2, .. })
    ));
    let source = bend
        .sketch
        .apply(
            &[
                seed(12, [55., -5., 0.], PointRole::Defining),
                line(13, 12, 6),
            ],
            || false,
        )
        .unwrap();
    let bad = [use_forward(CurveId::Line(lid(13))), uses[1], uses[2]];
    assert!(matches!(
        source.mixed_wire_recipe(&bad, WireClosure::Open, 3, || false),
        Err(WireFailure::UnprovedCurvePair { adjacent: true, .. })
    ));
    let source = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                seed(1, [5., 0., 0.], PointRole::Defining),
                seed(2, [-5., 0., 0.], PointRole::Defining),
                seed(3, [0., 0., 0.], PointRole::CenterMarker),
                Edit::InsertArc(SketchArc {
                    id: arc_id(4),
                    endpoints: [pid(1), pid(2)],
                    definition: ArcDefinition::Center {
                        center: pid(3),
                        normal: Direction3::Z,
                    },
                    fit_tolerance: ArcFitTolerance::new(1e-12).unwrap(),
                }),
            ],
            || false,
        )
        .unwrap();
    assert!(matches!(
        source.mixed_wire_recipe(
            &[use_forward(CurveId::Arc(arc_id(4)))],
            WireClosure::Open,
            0,
            || false
        ),
        Err(WireFailure::ArcDomain {
            cause: WireArcScope::Range,
            ..
        })
    ));
}

#[test]
fn repeated_concurrent_exports_are_independently_mutable_and_cancellable() {
    let bend = bend();
    let uses = bend_uses();
    let recipe = wire(&bend.sketch, &uses, WireClosure::Open);
    let (a, b) = std::thread::scope(|workers| {
        let a = workers.spawn(|| recipe.materialize(|| false).unwrap());
        let b = workers.spawn(|| recipe.materialize(|| false).unwrap());
        (a.join().unwrap(), b.join().unwrap())
    });
    assert!(!a.wire.is_partner(&b.wire));
    let first = BRepTool::curve_with_location(&a.edges[1].edge).unwrap();
    let second = BRepTool::curve_with_location(&b.edges[1].edge).unwrap();
    assert_ne!(first.curve, second.curve);
    first
        .curve
        .downcast_ref::<GeomCircle>()
        .unwrap()
        .set_radius(6.)
        .unwrap();
    assert_eq!(
        second.curve.downcast_ref::<GeomCircle>().unwrap().radius(),
        5.
    );
    assert_eq!(a.edges[1].trim, b.edges[1].trim);

    let mut polls = 0;
    recipe
        .materialize(|| {
            polls += 1;
            false
        })
        .unwrap();
    for stop in 1..=polls {
        let mut current = 0;
        let result = recipe.materialize(|| {
            current += 1;
            current == stop
        });
        assert!(matches!(
            result,
            Err(WireFailure::Cancelled(
                WirePhase::Materialize | WirePhase::VerifyNative
            ))
        ));
    }
    assert!(matches!(
        bend.sketch
            .mixed_wire_recipe(&uses, WireClosure::Open, 3, || true),
        Err(WireFailure::Cancelled(WirePhase::Resolve))
    ));
}
