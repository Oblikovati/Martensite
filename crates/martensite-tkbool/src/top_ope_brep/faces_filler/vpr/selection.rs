//! Interference selections preserve source order and shared handle identity.
use crate::top_ope_brep_ds::{
    ListOfInterference, TopOpeBRepDSHDataStructure, TopOpeBRepDSPoint, TopOpeBRepDSPointIterator,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

pub(super) fn on_geometry(list: &ListOfInterference, geometry: i32) -> ListOfInterference {
    list.iter()
        .filter(|entry| {
            entry
                .get()
                .is_some_and(|entry| entry.geometry() == geometry)
        })
        .cloned()
        .collect()
}

pub(super) fn on_transition_shape(
    list: &ListOfInterference,
    shape: i32,
) -> Result<ListOfInterference, Failure> {
    let mut selected = ListOfInterference::default();
    for entry in list.iter() {
        let transition = entry
            .get()
            .ok_or_else(super::super::missing_interference)?
            .transition();
        // Reference validates the shared shape index even for EXTERNAL transitions.
        let transition_index = transition.index()?;
        if transition.orientation(State::In, ShapeEnum::Face) != Orientation::External
            && transition_index == shape
        {
            selected.append(entry.clone());
        }
    }
    Ok(selected)
}

pub(super) fn same_geometry_support(
    list: &ListOfInterference,
    geometry: i32,
    support: i32,
) -> Result<ListOfInterference, Failure> {
    let mut selected = ListOfInterference::default();
    let mut points = TopOpeBRepDSPointIterator::new(list)?;
    while points.more() {
        let entry = points.value()?;
        let interference = entry.get().ok_or_else(super::super::missing_interference)?;
        if interference.geometry() == geometry && interference.support() == support {
            selected.append(entry.clone());
        }
        points.next()?;
    }
    Ok(selected)
}

pub(super) fn at_point(
    storage: &TopOpeBRepDSHDataStructure,
    list: &ListOfInterference,
    point: &TopOpeBRepDSPoint,
) -> Result<ListOfInterference, Failure> {
    let mut selected = ListOfInterference::default();
    let mut position = 0;
    while storage.scan_interf_list(list, &mut position, point)? {
        if let Some(entry) = list.iter().nth(position) {
            selected.append(entry.clone());
        }
        position += 1;
    }
    Ok(selected)
}
