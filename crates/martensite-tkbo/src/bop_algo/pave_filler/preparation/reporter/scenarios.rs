//! Each scenario builds its declared input anew and invokes the actual receiver entry.
use super::super::{BOPAlgoPaveFiller, PlanarPcurveInput, fixture::plane_input};
use super::journal::front_journal;
use super::output::emit_front_record;
use super::state::ReceiverStateSource;
use crate::bop_algo::{BOPAlgoCheckerSI, ProgressCancellation};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Edge, Shape, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;

/// Keep empty/null scope placement observable; `record_initialization(1)`.
pub(super) fn record_initialization(kind: usize) {
    let mut filler = initialization_receiver(kind);
    let source = ReceiverStateSource::new(&filler, None);
    let (indicator, journal) = front_journal(source, 1);
    let range = indicator.start();
    filler.initialize_with_progress(&range).unwrap();
    let names = ["init-empty", "init-null", "init-plane", "init-locked"];
    let state = ReceiverStateSource::new(&filler, None).read();
    emit_front_record(names[kind], false, 1, &journal, state);
}

fn initialization_receiver(kind: usize) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    if kind == 1 {
        filler.add_argument(&Shape::default());
    }
    if kind >= 2 {
        let input = plane_input();
        input.face.set_locked(kind == 3).unwrap();
        filler.add_argument(&input.face);
    }
    filler
}

/// Invoke the real specialized initializer, never the base implementation; `record_checker_initialization()`.
pub(super) fn record_checker_initialization() {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.add_argument(&plane_input().face);
    let level = checker.level_of_check();
    let source = ReceiverStateSource::new(&checker, None);
    let (indicator, journal) = front_journal(source, 1);
    let _range = indicator.start();
    crate::bop_algo::checker_si::initialize_checker_for_progress_test(&mut checker, level).unwrap();
    let state = ReceiverStateSource::new(&checker, None).read();
    emit_front_record("checker-init", false, 1, &journal, state);
}

/// Preserve the real early bypass before DS access; `record_non_destructive_bypass()`.
pub(super) fn record_non_destructive_bypass() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_non_destructive(true);
    record_preparation(&mut filler, None, "prepare-nondestructive", false, 1);
}

/// No synthetic empty DS substitutes for an initialized real vertex; `record_no_face_preparation()`.
pub(super) fn record_no_face_preparation() {
    let mut filler = BOPAlgoPaveFiller::new();
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::default(), 1.0e-7)
        .unwrap();
    filler.add_argument(&vertex);
    filler.initialize().unwrap();
    record_preparation(&mut filler, None, "prepare-no-face", false, 1);
}

/// Execute the same serialized planar inputs and cancellation query; `record_planar_preparation(false, true, 9)`.
pub(super) fn record_planar_preparation(parallel: bool, missing: bool, target: usize) {
    let input = preparation_input(missing);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    filler.add_argument(&input.face);
    filler.add_argument(&plane_input().face);
    filler.initialize().unwrap();
    let name = if missing {
        "prepare-missing"
    } else {
        "prepare-stored"
    };
    record_preparation(&mut filler, Some(input.edge), name, parallel, target);
}

fn preparation_input(missing: bool) -> PlanarPcurveInput {
    let input = plane_input();
    assert_single_surface_binding(&input);
    if missing {
        let tolerance = BRepTool::edge_tolerance(&input.edge).unwrap();
        BRepBuilder::default()
            .update_edge_pcurve_on_face(&input.edge, &Handle::default(), &input.face, tolerance)
            .unwrap();
    }
    input
}

fn assert_single_surface_binding(input: &PlanarPcurveInput) {
    let stored = BRepTool::first_curve_on_surface(&input.edge).unwrap();
    let (surface, location) = BRepTool::surface_with_location(&input.face).unwrap();
    assert_eq!(stored.surface.object_address(), surface.object_address());
    assert_eq!(stored.location, location);
    assert!(!stored.pcurve.is_null());
    assert!(
        BRepTool::curve_on_surface_by_index(&input.edge, 2)
            .unwrap()
            .pcurve
            .is_null()
    );
}

fn record_preparation(
    filler: &mut BOPAlgoPaveFiller,
    edge: Option<Edge>,
    name: &str,
    parallel: bool,
    target: usize,
) {
    let source = ReceiverStateSource::new(filler, edge.clone());
    let before = source.read();
    let (indicator, journal) = front_journal(source, target);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    filler.prepare_with_progress(&range, &observed).unwrap();
    let after = ReceiverStateSource::new(filler, edge).read();
    assert_eq!(
        [before[0], before[1], before[4]],
        [after[0], after[1], after[4]]
    );
    emit_front_record(name, parallel, target, &journal, after);
}
