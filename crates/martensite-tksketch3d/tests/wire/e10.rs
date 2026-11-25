// SPDX-License-Identifier: GPL-2.0-only
//! E10 production-entry replay through actual Sweep and Loft builders.
use super::{archive, forward, point, sample_path, sketch};
use crate::fixture::pid;
use martensite_tkbo::{
    body_check::{CheckMode, StageVerdict, ValidationStage, validate_body},
    profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap},
};
use martensite_tkbrep::{brep::BRepBuilder, topexp::Explorer, topods::Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkoffset::brep_offset_api::{BRepOffsetAPIMakePipeShell, BRepOffsetAPIThruSections};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tksketch3d::{
    Edit, Snapshot,
    wire::{DetachedWire, WireClosure, WireRecipe},
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransitionMode},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

fn square(points: [[f64; 3]; 4]) -> Snapshot {
    sketch(&points, &[(10, 1, 2), (11, 2, 3), (12, 3, 4), (13, 4, 1)])
}

fn section(snapshot: &Snapshot) -> WireRecipe {
    let uses = [10, 11, 12, 13].map(forward);
    snapshot
        .wire_recipe(&uses, WireClosure::Closed, 6, || false)
        .unwrap()
}

fn section_xy(height: f64) -> Snapshot {
    square([
        [-1., -1., height],
        [1., -1., height],
        [1., 1., height],
        [-1., 1., height],
    ])
}

fn mutate_input_vertex(export: &DetachedWire) {
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&export.points[0].vertex, point([90., 90., 90.]), 1e-7)
        .unwrap();
}

#[test]
fn nonplanar_path_builds_the_e10_round_corner_sweep() {
    let (source, uses) = sample_path();
    let route = source
        .wire_recipe(&uses, WireClosure::Open, 3, || false)
        .unwrap();
    let profile = section(&square([
        [0., -0.5, -0.5],
        [0., 0.5, -0.5],
        [0., 0.5, 0.5],
        [0., -0.5, 0.5],
    ]));
    let path = route.materialize(|| false).unwrap();
    let section = profile.materialize(|| false).unwrap();
    let inputs = [archive(&path.wire), archive(&section.wire)];
    let mut pipe = BRepOffsetAPIMakePipeShell::new(&path.wire).unwrap();
    pipe.set_transition_mode(BRepBuilderAPITransitionMode::RoundCorner);
    pipe.add(&section.wire, false, false).unwrap();
    pipe.build(&ProgressRange::default()).unwrap();
    assert!(pipe.is_done());
    let shell = pipe.shape().unwrap().clone();
    assert_eq!(shell.shape_type().unwrap(), ShapeEnum::Shell);
    assert!(
        BRepCheckAnalyzer::new(&shell, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert_section_center(&pipe.first_shape().unwrap(), point([0., 0., 0.]));
    assert_section_center(&pipe.last_shape().unwrap(), point([4., 4., 4.]));
    assert_sweep_history(&mut pipe, &section, &shell);
    let before = archive(&shell);
    mutate_input_vertex(&path);
    mutate_input_vertex(&section);
    assert_eq!(archive(&shell), before);
    assert_eq!(
        archive(&route.materialize(|| false).unwrap().wire),
        inputs[0]
    );
    assert_eq!(
        archive(&profile.materialize(|| false).unwrap().wire),
        inputs[1]
    );
}

fn assert_section_center(section: &Shape, expected: Point3) {
    let mut props = GProps::default();
    BRepGProp::linear_properties(section, &mut props, false, false).unwrap();
    assert!(props.centre_of_mass().distance(&expected) < 1e-8);
    assert!((props.mass() - 4.).abs() < 1e-8);
}

fn assert_sweep_history(
    pipe: &mut BRepOffsetAPIMakePipeShell,
    profile: &DetachedWire,
    shell: &Shape,
) {
    let faces: Vec<_> = Explorer::new(shell, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    for source in &profile.edges {
        let generated = pipe.generated(&source.edge).unwrap();
        assert!(!generated.is_empty(), "{:?}", source.source);
        assert!(
            generated
                .iter()
                .all(|image| faces.iter().any(|face| face.is_same(image)))
        );
    }
}

#[test]
fn ordered_closed_sections_build_and_rebuild_the_e10_loft() {
    let lower = section(&section_xy(0.));
    let upper_snapshot = section_xy(3.);
    let upper = section(&upper_snapshot);
    let old = loft(&lower, &upper, 3.);
    let before = archive(&old);
    let updates = [[-1., -1., 5.], [1., -1., 5.], [1., 1., 5.], [-1., 1., 5.]];
    let edits = updates
        .into_iter()
        .enumerate()
        .map(|(i, xyz)| Edit::MovePoint {
            point: pid(i as u64 + 1),
            position: point(xyz),
        })
        .collect::<Vec<_>>();
    let edited = upper_snapshot.apply(&edits, || false).unwrap();
    let rebuilt_upper = section(&edited);
    let rebuilt = loft(&lower, &rebuilt_upper, 5.);
    assert_eq!(archive(&old), before);
    assert_ne!(archive(&rebuilt), before);
    assert_eq!(
        upper.segments()[0].source,
        rebuilt_upper.segments()[0].source
    );
    assert_eq!(upper.segments()[0].authored.points[0].z(), 3.);
}

fn loft(lower: &WireRecipe, upper: &WireRecipe, height: f64) -> Shape {
    let lower = lower.materialize(|| false).unwrap();
    let upper = upper.materialize(|| false).unwrap();
    assert!(lower.wire.closed().unwrap() && upper.wire.closed().unwrap());
    let mut loft = BRepOffsetAPIThruSections::new(true, true, 1e-6);
    loft.set_mutable_input(false);
    loft.add_wire(&lower.wire);
    loft.add_wire(&upper.wire);
    loft.build(&ProgressRange::default()).unwrap();
    assert!(loft.is_done(), "{:?}", loft.get_status());
    let solid = loft.shape().unwrap().clone();
    assert_eq!(solid.shape_type().unwrap(), ShapeEnum::Solid);
    assert_volume_and_body(&solid, height);
    assert_cap(loft.first_shape(), 0.);
    assert_cap(loft.last_shape(), height);
    assert_loft_history(&mut loft, &lower, &upper, &solid);
    let before = archive(&solid);
    mutate_input_vertex(&lower);
    mutate_input_vertex(&upper);
    assert_eq!(archive(&solid), before);
    solid
}

fn assert_volume_and_body(solid: &Shape, height: f64) {
    let mut props = GProps::default();
    let relative = BRepGProp::volume_properties_eps(solid, &mut props, 1e-9, true, false).unwrap();
    let expected = 4. * height;
    assert!(relative.is_finite() && (0.0..=1e-9).contains(&relative));
    assert!((props.mass() - expected).abs() <= expected * (relative + 64. * f64::EPSILON));
    let tolerances = Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    );
    let validation = validate_body(solid, &tolerances).unwrap();
    assert!(validation.is_valid(), "{validation:?}");
    assert_eq!(
        validation.stage(ValidationStage::Topology).mode,
        CheckMode::Exact
    );
    assert_eq!(
        validation.stage(ValidationStage::SelfIntersection).verdict,
        StageVerdict::Passed
    );
}

fn assert_cap(shape: &Shape, height: f64) {
    let mut props = GProps::default();
    BRepGProp::surface_properties(shape, &mut props, false, false).unwrap();
    assert!((props.mass() - 4.).abs() < 1e-10);
    assert!(
        props
            .centre_of_mass()
            .distance(&Point3::new(0., 0., height))
            < 1e-10
    );
}

fn assert_loft_history(
    loft: &mut BRepOffsetAPIThruSections,
    lower: &DetachedWire,
    upper: &DetachedWire,
    solid: &Shape,
) {
    let faces: Vec<_> = Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let mut side_images = Vec::new();
    for (first, last) in lower.edges.iter().zip(&upper.edges) {
        let controlled = loft
            .generated(&first.edge)
            .unwrap()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let output = loft.generated(&last.edge).unwrap();
        assert_eq!(controlled.len(), 1);
        assert_eq!(output.len(), 1);
        let side = &controlled[0];
        assert!(output.iter().all(|face| face.is_same(side)));
        assert!(faces.iter().any(|face| face.is_same(side)));
        assert!(!side_images.iter().any(|prior| side.is_same(prior)));
        side_images.push(side.clone());
    }
    assert_eq!(side_images.len(), 4);
}
