//! Native final-state census and full serial callback bits; no mutable receiver alias is observed.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkbrep::{brep::BRepTool, topods::Vertex};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks},
    standard::RuntimeTyped,
};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
struct RepeatJournal {
    queries: AtomicUsize,
    events: Mutex<Vec<(u64, Vec<NativeScopeFrame>)>>,
}
struct RepeatHooks {
    journal: Arc<RepeatJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for RepeatHooks {
    fn user_break(&self) -> bool {
        self.journal.queries.fetch_add(1, Ordering::SeqCst) + 1 == self.target
    }
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        self.journal
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), native_scope_frames(scope)));
    }
}
#[test]
fn repeat_census_matches_51_native_final_states_and_33_serial_traces() {
    let traces: Vec<_> = include_str!("events.expected").lines().collect();
    let mut seen = std::collections::HashSet::new();
    for (index, line) in include_str!("states.expected").lines().enumerate() {
        let fields: Vec<_> = line.splitn(4, '|').collect();
        let kind: usize = fields[0].parse().unwrap();
        let parallel = fields[1] == "1";
        let target: usize = fields[2].parse().unwrap();
        assert!(seen.insert((kind, parallel, target)));
        let (state, events) = observe_repeat(kind, parallel, target);
        let key = format!("{kind}|{}|{target}|", u8::from(parallel));
        println!("RP_CASE|{key}{state}|{events}");
        assert_eq!(state, fields[3], "repeat state {key}");
        if !parallel {
            assert_eq!(
                events,
                traces[index].strip_prefix(&key).unwrap(),
                "repeat trace {key}"
            );
        }
    }
    assert_eq!(seen.len(), 51);
}
fn observe_repeat(kind: usize, parallel: bool, target: usize) -> (String, String) {
    let mut filler = repeat_case(kind, parallel);
    let journal = Arc::new(RepeatJournal::default());
    let indicator = ProgressIndicator::new(Box::new(RepeatHooks {
        journal: journal.clone(),
        target,
    }));
    let observed = ProgressCancellation::default();
    let completion = filler
        .repeat_intersection_with_progress(&indicator.start(), &observed)
        .unwrap();
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        observed.is_cancelled()
    );
    let state = repeat_state(
        &filler,
        kind,
        observed.is_cancelled(),
        journal.queries.load(Ordering::SeqCst),
    );
    let events = format!("{:?}", journal.events.lock().unwrap());
    (state, events)
}
fn repeat_case(kind: usize, parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = if kind == 4 {
        mixed_case()
    } else {
        super::tests::nearby_vertices()
    };
    filler.options_mut().set_run_parallel(parallel);
    match kind {
        1 => {
            filler.increased_ss.add(1);
        }
        2 => {
            filler.perform_vv().unwrap();
            filler.increased_ss.add(4);
        }
        3 => {
            filler.increased_ss.add(0);
        }
        4 => {
            let structure = filler.data_structure().unwrap();
            let vertices: Vec<_> = (0..structure.nb_source_shapes())
                .filter(|i| structure.shape_info(*i).unwrap().shape_type() == ShapeEnum::Vertex)
                .collect();
            for vertex in vertices {
                filler.increased_ss.add(vertex);
            }
        }
        _ => {}
    }
    filler
}
fn mixed_case() -> BOPAlgoPaveFiller {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::default())
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.))
            .unwrap()
            .edge()
            .unwrap()
            .into_shape();
    let plane = Plane3::new(Point3::default(), Direction3::Z).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut filler = BOPAlgoPaveFiller::new();
    for shape in [vertex, edge, face] {
        filler.add_argument(&shape);
    }
    filler.options_mut().set_fuzzy_value(1e-6);
    filler.initialize().unwrap();
    filler
}
fn repeat_state(filler: &BOPAlgoPaveFiller, kind: usize, observed: bool, queries: usize) -> String {
    let structure = filler.data_structure().unwrap();
    let core = format!(
        "[{},{},{},{},{},{},{},{},{}]",
        structure.nb_shapes(),
        structure.interf_vv_records().len(),
        structure.interf_ve_records().len(),
        structure.interf_vf_records().len(),
        structure.shape_sd_items().count(),
        filler.options.has_errors(),
        filler
            .options
            .has_error(crate::bop_algo::AlertUserBreak::static_type()),
        observed,
        queries
    );
    let vertex = if kind < 4 && structure.nb_shapes() == 5 {
        let shape = Vertex::unchecked(structure.shape(4).unwrap().clone());
        format!(
            "[{},{}]",
            BRepTool::pnt(&shape).unwrap().x().to_bits(),
            BRepTool::vertex_tolerance(&shape).unwrap().to_bits()
        )
    } else {
        "[]".to_owned()
    };
    format!("{core};{vertex}")
}
#[test]
fn actual_repeat_ve_and_vf_keep_closed_form_parameters() {
    let mut filler = repeat_case(4, false);
    assert_eq!(
        filler
            .repeat_intersection_with_progress(
                &ProgressRange::default(),
                &ProgressCancellation::default()
            )
            .unwrap(),
        ProgressCompletion::Completed
    );
    let structure = filler.data_structure().unwrap();
    assert_eq!(structure.interf_ve_records().len(), 1);
    for record in structure.interf_ve_records().iter() {
        assert_eq!(record.parameter(), 1.);
    }
    assert_eq!(structure.interf_vf_records().len(), 3);
    for record in structure.interf_vf_records().iter() {
        let vertex = Vertex::unchecked(structure.shape(record.index1()).unwrap().clone());
        let point = BRepTool::pnt(&vertex).unwrap();
        assert_eq!(record.uv(), (point.x(), point.y()));
    }
}
