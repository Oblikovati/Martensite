//! Bidirectional reference binding precedes every numerical certificate call.
use super::{
    binding_geometry, binding_representations, binding_topology, locations, topology, types::*,
};
use martensite_tkbrep::brep::face_input_capture::{
    CapturedFaceCell as Cell, CapturedLocation, CapturedOccurrence, FaceInputSnapshot,
};
use std::collections::{BTreeMap, BTreeSet};
pub(super) struct InputBinding<'a> {
    pub graph: &'a GraphInput,
    snapshot: &'a FaceInputSnapshot,
    owners: BTreeMap<usize, usize>,
    inverse: BTreeMap<usize, usize>,
    datums: BTreeMap<usize, usize>,
    inverse_datums: BTreeMap<usize, usize>,
}
pub(super) fn assert_graph(graph: &GraphInput, snapshot: &FaceInputSnapshot) {
    let mut binding = InputBinding {
        graph,
        snapshot,
        owners: BTreeMap::new(),
        inverse: BTreeMap::new(),
        datums: BTreeMap::new(),
        inverse_datums: BTreeMap::new(),
    };
    binding.occurrence(graph.root, snapshot.root());
    assert_eq!(binding.owners.len(), graph.owners.len());
    assert_eq!(binding.datums.len(), graph.datums.len());
    let actual: BTreeSet<_> = snapshot.cells().iter().map(|(id, _)| *id).collect();
    assert_eq!(actual, binding.inverse.keys().copied().collect());
}
impl InputBinding<'_> {
    pub fn owner(&mut self, expected: usize, actual: usize) {
        if let Some(previous) = self.owners.get(&expected) {
            assert_eq!(*previous, actual);
            return;
        }
        assert!(
            self.inverse.insert(actual, expected).is_none(),
            "actual owner {actual}: expected a unique source alias"
        );
        self.owners.insert(expected, actual);
        self.fields(self.graph.owners[expected], actual);
    }
    fn fields(&mut self, input: OwnerInput, actual: usize) {
        if topology::fields(input).is_some() {
            binding_topology::verify(self, input, actual);
            return;
        }
        if matches!(
            input,
            OwnerInput::Spatial { .. } | OwnerInput::Parametric { .. } | OwnerInput::Seam { .. }
        ) {
            binding_representations::verify(self, input, actual);
            return;
        }
        binding_geometry::verify(self, input, actual);
    }
    pub fn occurrence(&mut self, expected: OccurrenceInput, actual: &CapturedOccurrence) {
        assert_eq!(actual.orientation as i32, expected.orientation);
        self.location(expected.location, &actual.location);
        self.owner(expected.owner, actual.owner);
    }
    pub fn references(&mut self, expected: &[usize], actual: &[usize]) {
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(actual) {
            self.owner(*expected, *actual);
        }
    }
    pub fn location(&mut self, expected: LocationInput, actual: &CapturedLocation) {
        assert_eq!(expected.steps.len(), actual.steps().len());
        for ((owner, power), (actual_owner, actual_power, transform)) in
            expected.steps.iter().zip(actual.steps())
        {
            assert_eq!(power, actual_power);
            self.datum(*owner, *actual_owner);
            locations::assert_transform(*transform, self.graph.datums[*owner]);
        }
        locations::assert_transform(actual.effective_transform(), expected.effective);
    }
    fn datum(&mut self, expected: usize, actual: usize) {
        if let Some(previous) = self.datums.get(&expected) {
            assert_eq!(*previous, actual);
            return;
        }
        assert!(
            self.inverse_datums.insert(actual, expected).is_none(),
            "datum {actual}: expected distinct declared aliases"
        );
        self.datums.insert(expected, actual);
    }
    pub fn count(&self, owner: usize, expected: usize) {
        assert_eq!(
            self.snapshot
                .cells()
                .iter()
                .filter(|(id, _)| *id == owner)
                .count(),
            expected,
            "owner {owner}: unexpected captured fields"
        );
    }
    pub fn cell(&self, owner: usize, predicate: impl Fn(&Cell) -> bool) -> Cell {
        let cells: Vec<_> = self
            .snapshot
            .cells()
            .iter()
            .filter(|(id, cell)| *id == owner && predicate(cell))
            .collect();
        assert_eq!(
            cells.len(),
            1,
            "owner {owner}: expected one requested typed field"
        );
        cells[0].1.clone()
    }
}
