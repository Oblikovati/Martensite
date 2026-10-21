//! Complete edge500/face300 native families and independent record lifecycles.
use super::foundation_numeric::replay::geometry_records;
use super::foundation_numeric::{check_request_slice, replay};
use geometry_records::common::Reader;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkhlr::hlr_brep::{EdgeData, FaceData, FaceIterator};
use martensite_tkmath::gp::Point3;
use std::ops::Range;
const REQUESTS: &str = include_str!("fixtures/brep_records_numeric/requests.txt");
const EXPECTATIONS: &str = include_str!("fixtures/brep_records_numeric/reference-expected.txt");
fn run_case(request: Range<usize>, expected: Range<usize>) {
    check_request_slice(&REQUESTS[request], &EXPECTATIONS[expected]);
}
#[test]
fn original_record_families_have_exact_separate_input_accounting() {
    assert_eq!(REQUESTS.lines().count(), 800);
    assert_eq!(EXPECTATIONS.lines().count(), 800);
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|s| s.starts_with("hb_edgedata "))
            .count(),
        500
    );
    assert_eq!(
        REQUESTS
            .lines()
            .filter(|s| s.starts_with("hb_facedata "))
            .count(),
        300
    );
}
fn line_edge() -> Edge {
    replay::brep_record_fixture::vertex_edge(&mut Reader::new(&[
        0., 1., 2., 3., 1., 0., 0., 0., 2.,
    ]))
    .unwrap()
}
fn plane_face() -> Face {
    replay::brep_face_fixture::face(&mut Reader::new(&[
        0., 0., 0., 3., 0., 0., 1., 1., 0., 0., -2., 2., -3., 3.,
    ]))
    .unwrap()
}
fn set_line_record(record: &mut EdgeData, first: f64, last: f64) {
    record
        .set(
            true,
            false,
            &line_edge(),
            11,
            22,
            true,
            false,
            false,
            true,
            first,
            0.125,
            last,
            0.25,
        )
        .unwrap();
}
#[test]
fn edge_set_replaces_bounds_and_visibility_but_preserves_unrelated_flags() {
    let mut record = EdgeData::new();
    record.set_vertical(true);
    record.set_selected(false);
    set_line_record(&mut record, 0., 2.);
    assert_eq!((record.v_sta(), record.v_end()), (11, 22));
    assert_eq!(record.status().bounds(), (0., 0.125, 2., 0.25));
    record.set_used(true);
    record.status_mut().set_all_hidden(true);
    set_line_record(&mut record, 0.5, 1.5);
    assert!(!record.used() && record.vertical() && !record.selected());
    assert!(record.rg1_line() && record.out_lv_sta() && record.cut_at_end());
    assert_eq!(record.status().bounds(), (0.5, 0.125, 1.5, 0.25));
    assert_eq!(record.status().nb_visible_part(), 1);
}
#[test]
fn edge_record_copy_keeps_geometry_but_has_independent_flags_and_status() {
    let mut source = EdgeData::new();
    set_line_record(&mut source, 0., 2.);
    let mut copy = source.try_clone().unwrap();
    copy.set_selected(false);
    copy.status_mut().set_all_hidden(true);
    assert!(source.selected());
    assert_eq!(source.status().nb_visible_part(), 1);
    assert_eq!(copy.status().nb_visible_part(), 0);
    drop(source);
    assert_eq!(
        copy.geometry().value_3d(0.5).unwrap(),
        Point3::new(1.5, 2., 3.)
    );
}
fn initialized_face() -> FaceData {
    let mut record = FaceData::new();
    record
        .set(&plane_face(), Orientation::Reversed, true, 2)
        .unwrap();
    record.set_wire(1, 2).unwrap();
    record.set_wire(2, 1).unwrap();
    for (wire, slot, edge) in [(1, 1, 11), (1, 2, 12), (2, 1, 21)] {
        record
            .set_w_edge(
                wire,
                slot,
                edge,
                Orientation::Forward,
                true,
                false,
                true,
                false,
            )
            .unwrap();
    }
    record
}
#[test]
fn iterator_retains_wire_blocks_after_the_face_record_drops() {
    let mut iterator = FaceIterator::new();
    iterator.init_edge(&initialized_face()).unwrap();
    for (edge, begin, end) in [(11, true, false), (12, false, true), (21, true, true)] {
        assert!(iterator.more_edge());
        assert_eq!(iterator.edge().unwrap(), edge);
        assert_eq!(
            (iterator.beginning_of_wire(), iterator.end_of_wire()),
            (begin, end)
        );
        assert!(iterator.out_line().unwrap() && iterator.double().unwrap());
        assert!(!iterator.internal().unwrap() && !iterator.iso_line().unwrap());
        iterator.next_edge().unwrap();
    }
    assert!(!iterator.more_edge());
}
#[test]
fn copied_face_records_share_wire_edits_but_keep_scalar_flags_independent() {
    let source = initialized_face();
    let mut copy = source.try_clone().unwrap();
    copy.set_selected(false);
    copy.set_w_edge(1, 1, 99, Orientation::Internal, false, true, false, true)
        .unwrap();
    assert!(source.selected() && !copy.selected());
    let mut iterator = FaceIterator::new();
    iterator.init_edge(&source).unwrap();
    assert_eq!(iterator.edge().unwrap(), 99);
    assert_eq!(iterator.orientation().unwrap(), Orientation::Internal);
    assert!(iterator.internal().unwrap() && iterator.iso_line().unwrap());
}
#[test]
fn replacing_face_wires_preserves_the_previous_iterators_retained_blocks() {
    let mut record = initialized_face();
    let mut old_iterator = FaceIterator::new();
    old_iterator.init_edge(&record).unwrap();
    record
        .set(&plane_face(), Orientation::External, false, 0)
        .unwrap();
    let mut new_iterator = FaceIterator::new();
    new_iterator.init_edge(&record).unwrap();
    assert!(!new_iterator.more_edge());
    assert_eq!(old_iterator.edge().unwrap(), 11);
    assert_eq!(record.orientation(), Orientation::External);
    assert!(!record.closed());
}
#[path = "brep_records_numeric/report_guards.rs"]
mod report_guards;

#[path = "brep_records_numeric/cases_00.rs"]
mod cases_00;

#[path = "brep_records_numeric/cases_01.rs"]
mod cases_01;

#[path = "brep_records_numeric/cases_02.rs"]
mod cases_02;

#[path = "brep_records_numeric/cases_03.rs"]
mod cases_03;

#[path = "brep_records_numeric/cases_04.rs"]
mod cases_04;

#[path = "brep_records_numeric/cases_05.rs"]
mod cases_05;

#[path = "brep_records_numeric/cases_06.rs"]
mod cases_06;

#[path = "brep_records_numeric/cases_07.rs"]
mod cases_07;

#[path = "brep_records_numeric/cases_08.rs"]
mod cases_08;

#[path = "brep_records_numeric/cases_09.rs"]
mod cases_09;
