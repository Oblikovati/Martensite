//! Native interactive-object categories and drag actions.
/// Fast object categories; `AisKindOfInteractive::Datum as i32 == 1`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisKindOfInteractive {
    /// `AisKindOfInteractive_None`.
    None = 0,
    /// `AisKindOfInteractive_Datum`.
    Datum = 1,
    /// `AisKindOfInteractive_Shape`.
    Shape = 2,
    /// `AisKindOfInteractive_Object`.
    Object = 3,
    /// `AisKindOfInteractive_Relation`.
    Relation = 4,
    /// `AisKindOfInteractive_Dimension`.
    Dimension = 5,
    /// `AisKindOfInteractive_LightSource`.
    LightSource = 6,
}
impl AisKindOfInteractive {
    /// `AisKOI_None`.
    pub const KOI_NONE: Self = Self::None;
    /// `AisKOI_Datum`.
    pub const KOI_DATUM: Self = Self::Datum;
    /// `AisKOI_Shape`.
    pub const KOI_SHAPE: Self = Self::Shape;
    /// `AisKOI_Object`.
    pub const KOI_OBJECT: Self = Self::Object;
    /// `AisKOI_Relation`.
    pub const KOI_RELATION: Self = Self::Relation;
    /// `AisKOI_Dimension`.
    pub const KOI_DIMENSION: Self = Self::Dimension;
}
/// Lifecycle of a viewer drag; `AisDragAction::Abort as i32 == 4`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AisDragAction {
    /// Begin dragging.
    Start = 0,
    /// Confirm the drag interaction.
    Confirmed = 1,
    /// Update the dragged position.
    Update = 2,
    /// Commit the position.
    Stop = 3,
    /// Restore the position.
    Abort = 4,
}
