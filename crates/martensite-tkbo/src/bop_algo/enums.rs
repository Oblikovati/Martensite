//! Public option values shared by the intersection, builder, and Boolean algorithms.

/// `BOPAlgoGlueEnum`: gluing assumption passed to algorithms that support coinciding faces.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum Glue {
    /// `BOPAlgoGlueOff` (the native default).
    #[default]
    Off = 0,
    /// `BOPAlgoGlueShift`: partially coinciding faces without real intersections.
    Shift = 1,
    /// `BOPAlgoGlueFull`: fully coinciding shapes without partial overlap.
    Full = 2,
}

/// `BOPAlgoOperation`: operation requested from the Boolean builder.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum Operation {
    /// `BOPAlgoCOMMON`.
    Common = 0,
    /// `BOPAlgoFUSE`.
    Fuse = 1,
    /// `BOPAlgoCUT`.
    Cut = 2,
    /// `BOPAlgoCUT21` (subtract the first operand from the second).
    Cut21 = 3,
    /// `BOPAlgoSECTION`.
    Section = 4,
    /// `BOPAlgoUNKNOWN` (the native default).
    #[default]
    Unknown = 5,
}
