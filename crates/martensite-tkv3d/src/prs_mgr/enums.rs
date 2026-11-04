//! The PrsMgr enumerations with their native discriminants.

/// `PrsMgrDisplayStatus`: where an interactive object is displayed.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsMgrDisplayStatus {
    /// `PrsMgrDisplayStatus_Displayed`: displayed in the main viewer.
    Displayed = 0,
    /// `PrsMgrDisplayStatus_Erased`: hidden in the main viewer.
    Erased = 1,
    /// `PrsMgrDisplayStatus_None`: displayed nowhere.
    None = 2,
}

impl PrsMgrDisplayStatus {
    /// `AisDS_Displayed`, the legacy alias of [`PrsMgrDisplayStatus::Displayed`].
    pub const INTERACTIVE_DS_DISPLAYED: Self = Self::Displayed;
    /// `AisDS_Erased`, the legacy alias of [`PrsMgrDisplayStatus::Erased`].
    pub const INTERACTIVE_DS_ERASED: Self = Self::Erased;
    /// `AisDS_None`, the legacy alias of [`PrsMgrDisplayStatus::None`].
    pub const INTERACTIVE_DS_NONE: Self = Self::None;
}

/// `PrsMgrTypeOfPresentation3d`: whether the presentation depends on the view projector.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsMgrTypeOfPresentation3d {
    /// `PrsMgrTOP_AllView`: one presentation for every view.
    AllView = 0,
    /// `PrsMgrTOP_ProjectorDependent`: computed per view projector (hidden lines).
    ProjectorDependent = 1,
}

/// `PrsMgrTOP_ProjectorDependant`, the deprecated spelling of
/// [`PrsMgrTypeOfPresentation3d::ProjectorDependent`].
pub const PRS_MGR_TOP_PROJECTOR_DEPENDANT: PrsMgrTypeOfPresentation3d =
    PrsMgrTypeOfPresentation3d::ProjectorDependent;
