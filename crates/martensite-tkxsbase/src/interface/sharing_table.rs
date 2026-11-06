//! One authoritative live table and derived immutable convenience snapshots.
use martensite_tkernel::{
    ncollection::{Array1, BaseAllocator, HArray1, INC_DEFAULT_BLOCK_SIZE, List},
    standard::{Failure, Handle, SharedState, exceptions},
};
use std::sync::Arc;

/// Immutable rows in physical array order: index 0 is entity 1 for normal graph bounds.
///
/// This convenience snapshot has no live mutation capability; use
/// [`super::InterfaceGraph::sharing_table_handle`] for the mutable bounded array.
pub type InterfaceSharingTable = Arc<Vec<Vec<i32>>>;

/// Shared bounded array of sharing lists, with changes visible through every owner.
///
/// `graph.sharing_table_handle()` returns this native-equivalent ownership surface.
pub type InterfaceSharingTableHandle = Handle<HArray1<List<i32>>>;

#[derive(Clone)]
pub(super) struct GraphSharingTable {
    live: InterfaceSharingTableHandle,
    // The cache preserves ordinary snapshot identity without becoming authoritative.
    snapshot: Arc<SharedState<Option<InterfaceSharingTable>>>,
}

impl GraphSharingTable {
    pub(super) fn new(rows: Vec<Vec<i32>>) -> Self {
        let allocator = BaseAllocator::incremental(INC_DEFAULT_BLOCK_SIZE);
        let mut values = Array1::<List<i32>>::with_size(rows.len());
        values.update_lower_bound(1);
        for (target, row) in values.iter_mut().zip(&rows) {
            *target = List::from_iter_in(row.iter().copied(), allocator.clone());
        }
        Self {
            live: Handle::new(HArray1::from_owned(values)),
            snapshot: Arc::new(SharedState::new(None)),
        }
    }

    pub(super) fn handle(&self) -> &InterfaceSharingTableHandle {
        &self.live
    }

    pub(super) fn snapshot(&self) -> Option<InterfaceSharingTable> {
        let values = self.live.get()?.array1();
        let mut cached = self.snapshot.write();
        if cached
            .as_ref()
            .is_some_and(|snapshot| matches_snapshot(&values, snapshot))
        {
            return cached.clone();
        }
        *cached = Some(Arc::new(
            values
                .iter()
                .map(|row| row.iter().copied().collect())
                .collect(),
        ));
        cached.clone()
    }

    pub(super) fn numbers(&self, num: i32) -> Result<Vec<i32>, Failure> {
        let values = self.live_array()?.array1();
        let row = values.value(num).map_err(|_| {
            exceptions::out_of_range(&format!(
                "InterfaceGraph: entity {num}; expected [{}, {}]",
                values.lower(),
                values.upper()
            ))
        })?;
        Ok(row.iter().copied().collect())
    }

    pub(super) fn length(&self) -> Result<i32, Failure> {
        Ok(self.live_array()?.array1().length())
    }

    pub(super) fn row_is_empty(&self, num: i32) -> Result<bool, Failure> {
        let values = self.live_array()?.array1();
        values
            .value(num)
            .map(List::is_empty)
            .map_err(|error| error.into_failure("InterfaceGraph::SharingTable::Value"))
    }

    fn live_array(&self) -> Result<&HArray1<List<i32>>, Failure> {
        self.live.get().ok_or_else(|| {
            exceptions::null_object(
                "InterfaceGraph::SharingTable: null handle; expected a live sharing array",
            )
        })
    }
}

fn matches_snapshot(values: &Array1<'_, List<i32>>, snapshot: &[Vec<i32>]) -> bool {
    values.len() == snapshot.len()
        && values
            .iter()
            .zip(snapshot)
            .all(|(row, saved)| row.iter().copied().eq(saved.iter().copied()))
}
