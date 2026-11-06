//! Partial serial observations: final DS and scope bits; Native per-query DS is not projected.
#[path = "census_shapes.rs"]
mod shapes;
#[path = "census_sticky.rs"]
mod sticky;
#[path = "census_trace.rs"]
mod trace;
use super::BOPAlgoPaveFiller;
use crate::bop_algo::{AlertUserBreak, ProgressCancellation};
use crate::bop_ds::tools::type_to_integer;
use martensite_tkernel::standard::RuntimeTyped;

#[test]
fn passive_serial_edge_census_reports_actual_final_state_and_full_scope_bits() {
    for profile in [
        "none", "cross", "shared", "endpoint", "triple", "far", "flagged",
    ] {
        report_serial_case(profile);
    }
}

fn report_serial_case(profile: &str) {
    let mut filler = shapes::configured_census(profile);
    let (indicator, trace) = trace::passive_indicator();
    let observed = ProgressCancellation::default();
    let range = indicator.start();
    let completions = filler.perform_ee_with_progress(&range, &observed).unwrap();
    drop(range);
    assert!(
        !observed.is_cancelled(),
        "EE {profile}; expected an uncancelled passive receiver"
    );
    let queries = trace.query_count();
    let events = trace.event_json();
    let final_state = final_state_json(&filler);
    let outputs = completions.len();
    println!(
        "{{\"profile\":\"{profile}\",\"parallel\":false,{final_state},\"poll_count\":{queries},\"events\":{events},\"worker_outcome_count\":{outputs}}}"
    );
}

fn final_state_json(filler: &BOPAlgoPaveFiller) -> String {
    let structure = filler.data_structure().unwrap();
    let state = [
        structure.nb_shapes() as usize,
        structure.interf_ee_records().len(),
        structure.interf_ve_records().len(),
        structure.same_domain_binding_count(),
    ];
    let more = filler.iterator().unwrap().more();
    let errors = filler.options().has_errors();
    let warnings = filler.options().has_warnings();
    let stopped = filler.options().has_error(AlertUserBreak::static_type());
    let records = records_json(filler);
    format!(
        "\"state\":{state:?},\"iterator_more\":{more},\"has_errors\":{errors},\"has_warnings\":{warnings},\"user_break\":{stopped},\"records\":{records}"
    )
}

fn records_json(filler: &BOPAlgoPaveFiller) -> String {
    let rows: Vec<String> = filler
        .data_structure()
        .unwrap()
        .interf_ee_records()
        .iter()
        .map(|record| {
            let (first, second) = record.indices();
            let new_index = record.index_new();
            let kind = type_to_integer(record.common_part().shape_type());
            format!("[{first},{second},{new_index},{kind}]")
        })
        .collect();
    format!("[{}]", rows.join(","))
}

#[test]
fn configured_parallel_edge_results_repeat_the_serial_census_state() {
    for profile in [
        "none", "cross", "shared", "endpoint", "triple", "far", "flagged",
    ] {
        let expected = completed_edge_census_state(profile, false);
        for _ in 0..16 {
            assert_eq!(
                completed_edge_census_state(profile, true),
                expected,
                "{profile}"
            );
        }
    }
}

fn completed_edge_census_state(profile: &str, parallel: bool) -> String {
    let mut filler = shapes::configured_census(profile);
    filler.options_mut().set_run_parallel(parallel);
    filler.perform_ee().unwrap();
    final_state_json(&filler)
}
