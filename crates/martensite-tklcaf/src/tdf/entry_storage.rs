//! Mutually exclusive entry indexing and sparse transaction evidence reuse one slot (#489).
use super::{TdfLabel, sparse_commit::HintState};
use martensite_tkernel::{ncollection::DataMap, tcollection::AsciiString};
type EntryMap = DataMap<AsciiString, TdfLabel>;

pub(super) enum EntryStorage {
    Indexed(EntryMap),
    Tracked(HintState),
}
impl Default for EntryStorage {
    fn default() -> Self {
        // Index creation permanently retires tracking, so old revisions cannot rearm.
        Self::Tracked(HintState::default())
    }
}
impl EntryStorage {
    pub(super) fn hint(&self) -> Option<&HintState> {
        match self {
            Self::Tracked(hint) => Some(hint),
            Self::Indexed(_) => None,
        }
    }
    pub(super) fn hint_mut(&mut self) -> Option<&mut HintState> {
        match self {
            Self::Tracked(hint) => Some(hint),
            Self::Indexed(_) => None,
        }
    }
    pub(super) fn register(&mut self, entry: AsciiString, label: TdfLabel) {
        // Manual registration works even while automatic entry lookup is disabled.
        self.index_mut().bind(entry, label);
    }
    pub(super) fn find(&self, entry: &AsciiString) -> Option<&TdfLabel> {
        match self {
            Self::Indexed(index) => index.find(entry).ok(),
            Self::Tracked(_) => None,
        }
    }
    pub(super) fn clear(&mut self, enable: bool) {
        if enable {
            self.index_mut().clear(true);
            return;
        }
        if let Self::Indexed(index) = self {
            index.clear(true);
        }
    }
    fn index_mut(&mut self) -> &mut EntryMap {
        match self {
            Self::Indexed(index) => index,
            Self::Tracked(_) => {
                *self = Self::Indexed(EntryMap::default());
                self.index_mut()
            }
        }
    }
}
