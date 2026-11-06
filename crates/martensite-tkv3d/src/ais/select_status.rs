//! Native outcome of a selection-list update.
/// Result of adding/removing an owner; `AisSelectStatus::Added as i32 == 0`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisSelectStatus {
    /// `AisSS_Added`.
    Added = 0,
    /// `AisSS_Removed`.
    Removed = 1,
    /// `AisSS_NotDone`.
    NotDone = 2,
}
