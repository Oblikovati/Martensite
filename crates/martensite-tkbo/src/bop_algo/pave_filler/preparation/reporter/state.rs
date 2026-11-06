//! Shared report/edge observations avoid aliasing the mutably executing receiver.
use crate::bop_algo::BOPAlgoPaveFiller;
use crate::bop_algo::{AlertUserBreak, Options};
use martensite_tkbrep::{brep::BRepTool, topods::Edge};
use martensite_tkernel::standard::RuntimeTyped;

pub(super) type FrontState = [i32; 6];

pub(super) struct ReceiverStateSource {
    structure: [i32; 2],
    options: Options,
    non_destructive: bool,
    edge: Option<Edge>,
}

impl ReceiverStateSource {
    /// Capture only fields invariant during Prepare; `ReceiverStateSource::new(filler, edge)`.
    pub(super) fn new(filler: &BOPAlgoPaveFiller, edge: Option<Edge>) -> Self {
        let structure = filler.data_structure();
        Self {
            structure: [
                i32::from(structure.is_some()),
                structure.map_or(0, |ds| ds.nb_source_shapes()),
            ],
            options: filler.options().clone(),
            non_destructive: filler.non_destructive(),
            edge,
        }
    }

    /// Read the shared live report and stored representation only; `source.read()`.
    pub(super) fn read(&self) -> FrontState {
        [
            self.structure[0],
            self.structure[1],
            i32::from(self.options.has_errors()),
            i32::from(self.options.has_error(AlertUserBreak::static_type())),
            i32::from(self.non_destructive),
            i32::from(self.edge.as_ref().is_some_and(stored_representation)),
        ]
    }
}

fn stored_representation(edge: &Edge) -> bool {
    // The bound fixture has exactly one face representation; this never projects a curve.
    !BRepTool::first_curve_on_surface(edge)
        .unwrap()
        .pcurve
        .is_null()
}
