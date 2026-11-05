//! Geometry and transition processing at an intersection with a face restriction.
use super::TopOpeBRepFacesFiller;
use crate::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::{TopOpeBRepDSInterference, TopOpeBRepDSKind};
use martensite_tkernel::standard::{Failure, Handle};

pub(super) mod geometry;
pub(super) mod selection;
pub(super) use geometry::vp_index;

/// Independent edge/curve discoveries while processing one face-intersection vertex.
/// `let search = martensite_tkbool::top_ope_brep::TopOpeBRepFacesFillerGeometrySearch::default();`.
pub struct GeometrySearch {
    /// The selected geometry category, a point until a vertex is found or created.
    pub kind: TopOpeBRepDSKind,
    /// The selected one-based storage index; zero means no geometry selected.
    pub index: i32,
    /// Whether the edge interference search found existing geometry.
    pub edge_found: bool,
    /// The edge search cursor handle, which can remain null on an empty search.
    pub edge_interference: Handle<TopOpeBRepDSInterference>,
    /// Whether the current curve interference search found existing geometry.
    pub curve_found: bool,
    /// The current curve search cursor handle, including a null empty cursor.
    pub curve_interference: Handle<TopOpeBRepDSInterference>,
}

impl Default for GeometrySearch {
    fn default() -> Self {
        Self {
            kind: TopOpeBRepDSKind::Point,
            index: 0,
            edge_found: false,
            edge_interference: Handle::default(),
            curve_found: false,
            curve_interference: Handle::default(),
        }
    }
}

/// The retained line is explicit so shared intersector slots are never detached.
impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn find_vp_geometry(
        &mut self,
        line: &TopOpeBRepLineInter,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        mode: i32,
        search: &mut GeometrySearch,
    ) -> Result<(), Failure> {
        vp_index(self, line, point, side, mode, search)
    }
}

#[cfg(test)]
mod tests;

mod context;
mod line_transition;
pub(super) mod new_transition;
pub(super) use new_transition::{EdgeTransitionQuery, new_transition_edge};

mod edge_interferences;

mod curve_point;
mod process;

#[cfg(test)]
mod process_tests;

#[cfg(test)]
mod transition_tests;

mod iterator_wrappers;
