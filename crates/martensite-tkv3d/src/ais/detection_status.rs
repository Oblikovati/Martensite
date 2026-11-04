//! Native detection outcomes, including values retained for compatibility.
/// Result of a mouse or axis detection; `AisStatusOfDetection::Nothing as i32 == 1`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisStatusOfDetection {
    /// `AisSOD_Error`.
    Error = 0,
    /// `AisSOD_Nothing`.
    Nothing = 1,
    /// `AisSOD_AllBad`.
    AllBad = 2,
    /// `AisSOD_Selected`.
    Selected = 3,
    /// `AisSOD_OnlyOneDetected`.
    OnlyOneDetected = 4,
    /// `AisSOD_OnlyOneGood`.
    OnlyOneGood = 5,
    /// `AisSOD_SeveralGood`.
    SeveralGood = 6,
}
