//! Owned identity follows a record through Rust moves without depending on its storage slot.
//! Each Default/new record mints a token; SetLine updates fields without replacing it.
//! Replacing a value carries the incoming record token; SetLine retains the existing token.
//! The record is not Clone. Any future record-copy API must mint a distinct token.
//! A retained token also prevents allocation reuse from matching a later record.
use super::TopOpeBRepLineInter;
use std::sync::Arc;

/// Cloning this private token retains one record's identity, not a copy of that record.
#[derive(Clone)]
pub(crate) struct LineInterRecordIdentity(Arc<()>);

impl Default for LineInterRecordIdentity {
    fn default() -> Self {
        Self(Arc::new(()))
    }
}

impl LineInterRecordIdentity {
    /// Recognizes a retained record token; `identity.is_same_record(line.record_identity())`.
    pub(crate) fn is_same_record(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl TopOpeBRepLineInter {
    /// Retains identity across storage moves; `let identity = line.record_identity().clone();`.
    pub(crate) fn record_identity(&self) -> &LineInterRecordIdentity {
        &self.record_identity
    }
}
