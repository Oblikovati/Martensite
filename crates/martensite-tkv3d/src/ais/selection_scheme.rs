//! `AisSelectionScheme`: how a detection changes the current selection.

/// `AisSelectionScheme`: how a detection changes the current selection.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisSelectionScheme {
    /// `AisSelectionScheme_UNKNOWN`: undefined.
    Unknown = -1,
    /// `AisSelectionScheme_Replace`: the detected objects replace the selection.
    Replace = 0,
    /// `AisSelectionScheme_Add`: the detected objects are added.
    Add = 1,
    /// `AisSelectionScheme_Remove`: the detected objects are removed.
    Remove = 2,
    /// `AisSelectionScheme_XOR`: the detected objects are toggled.
    Xor = 3,
    /// `AisSelectionScheme_Clear`: the selection is cleared.
    Clear = 4,
    /// `AisSelectionScheme_ReplaceExtra`: replace, keeping a selected detected element when
    /// the replacement would be empty.
    ReplaceExtra = 5,
}
