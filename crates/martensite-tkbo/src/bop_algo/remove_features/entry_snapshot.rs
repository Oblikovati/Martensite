//! Public source-handle history and raw facade state are separate from publication validity.
use super::*;
use crate::bop_algo::progress_test_support::parent_census::ParentCensusJournal;
use crate::bop_algo::{
    AlertEmptyShape, AlertNoFacesToRemove, AlertRemoveFeaturesFailed, AlertTooFewArguments,
    AlertUnableToRemoveTheFeature, AlertUnsupportedType, AlertUserBreak,
};
use crate::brep_algo_api::BRepAlgoAPIDefeaturing;
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::standard::RuntimeTyped;
use std::sync::atomic::Ordering;
pub(super) fn whole_entry_snapshot(
    api: &mut BRepAlgoAPIDefeaturing,
    source: &Shape,
    journal: &ParentCensusJournal,
) -> String {
    let state = whole_entry_state(api, source, journal);
    let reports = whole_entry_reports(api);
    let sources = supported_entry_inputs(source);
    let flags: Vec<_> = sources
        .iter()
        .map(|input| whole_entry_relation(api, input))
        .collect();
    format!("{state:?};{reports:?};{flags:?}")
}
fn whole_entry_state(
    api: &mut BRepAlgoAPIDefeaturing,
    source: &Shape,
    journal: &ParentCensusJournal,
) -> Vec<i64> {
    let shape = api.shape();
    let present = !shape.is_null();
    let solids = if present {
        explore(shape, ShapeEnum::Solid).unwrap().len()
    } else {
        0
    };
    let faces = if present {
        explore(shape, ShapeEnum::Face).unwrap().len()
    } else {
        0
    };
    let kind = if present {
        shape.shape_type().unwrap() as i64
    } else {
        -1
    };
    let same = present && shape.is_same(source);
    vec![
        journal.queries.load(Ordering::SeqCst) as i64,
        i64::from(journal.observed.load(Ordering::SeqCst)),
        i64::from(api.is_done()),
        i64::from(api.has_errors()),
        i64::from(api.has_warnings()),
        i64::from(!present),
        i64::from(same),
        solids as i64,
        faces as i64,
        i64::from(api.has_history()),
        i64::from(api.history().is_some()),
        i64::from(api.has_modified()),
        i64::from(api.has_generated()),
        i64::from(api.has_deleted()),
        kind,
    ]
}
fn whole_entry_reports(api: &BRepAlgoAPIDefeaturing) -> Vec<usize> {
    vec![
        api.has_error(AlertNoFacesToRemove::static_type()),
        api.has_error(AlertTooFewArguments::static_type()),
        api.has_error(AlertEmptyShape::static_type()),
        api.has_error(AlertRemoveFeaturesFailed::static_type()),
        api.has_error(AlertUserBreak::static_type()),
        api.has_warning(AlertUnableToRemoveTheFeature::static_type()),
        api.has_warning(AlertUnsupportedType::static_type()),
    ]
    .into_iter()
    .map(usize::from)
    .collect()
}
fn supported_entry_inputs(source: &Shape) -> Vec<Shape> {
    if source.is_null() {
        return Vec::new();
    }
    let mut all = IndexedMapOfShape::default();
    topexp::map_shapes(source, &mut all, true, true);
    all.iter()
        .filter(|shape| History::is_supported_type(shape).unwrap())
        .cloned()
        .collect()
}
fn whole_entry_relation(api: &BRepAlgoAPIDefeaturing, input: &Shape) -> Vec<usize> {
    let mut result = IndexedMapOfShape::default();
    if !api.shape().is_null() {
        topexp::map_shapes(api.shape(), &mut result, true, true);
    }
    let modified = api.modified(input).unwrap();
    vec![
        input.shape_type().unwrap() as usize,
        modified.len(),
        api.generated(input).unwrap().len(),
        usize::from(api.is_deleted(input).unwrap()),
        usize::from(result.contains(input)),
        modified
            .iter()
            .filter(|image| result.contains(image))
            .count(),
    ]
}
