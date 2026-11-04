//! Native outcomes of interactive-context selection actions.
/// Selection result; `AisStatusOfPick::NothingSelected as i32 == 1`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisStatusOfPick {
    /// `AisSOP_Error`.
    Error = 0,
    /// `AisSOP_NothingSelected`.
    NothingSelected = 1,
    /// `AisSOP_Removed`.
    Removed = 2,
    /// `AisSOP_OneSelected`.
    OneSelected = 3,
    /// `AisSOP_SeveralSelected`.
    SeveralSelected = 4,
}
