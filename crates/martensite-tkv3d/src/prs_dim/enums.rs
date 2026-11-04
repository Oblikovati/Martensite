//! The PrsDim enumerations.

/// `PrsDimKindOfDimension`: the kind of a measured value.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimKindOfDimension {
    /// `PrsDimKOD_NONE`.
    #[default]
    None = 0,
    /// `PrsDimKOD_LENGTH`.
    Length,
    /// `PrsDimKOD_PLANEANGLE`.
    PlaneAngle,
    /// `PrsDimKOD_SOLIDANGLE`.
    SolidAngle,
    /// `PrsDimKOD_AREA`.
    Area,
    /// `PrsDimKOD_VOLUME`.
    Volume,
    /// `PrsDimKOD_MASS`.
    Mass,
    /// `PrsDimKOD_TIME`.
    Time,
    /// `PrsDimKOD_RADIUS`.
    Radius,
    /// `PrsDimKOD_DIAMETER`.
    Diameter,
    /// `PrsDimKOD_CHAMF2D`.
    Chamf2d,
    /// `PrsDimKOD_CHAMF3D`.
    Chamf3d,
    /// `PrsDimKOD_OFFSET`.
    Offset,
    /// `PrsDimKOD_ELLIPSERADIUS`.
    EllipseRadius,
}

/// `PrsDimKindOfRelation`: the kind of a relation.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimKindOfRelation {
    /// `PrsDimKOR_NONE`.
    #[default]
    None = 0,
    /// `PrsDimKOR_CONCENTRIC`.
    Concentric,
    /// `PrsDimKOR_EQUALDISTANCE`.
    EqualDistance,
    /// `PrsDimKOR_EQUALRADIUS`.
    EqualRadius,
    /// `PrsDimKOR_FIX`.
    Fix,
    /// `PrsDimKOR_IDENTIC`.
    Identic,
    /// `PrsDimKOR_OFFSET`.
    Offset,
    /// `PrsDimKOR_PARALLEL`.
    Parallel,
    /// `PrsDimKOR_PERPENDICULAR`.
    Perpendicular,
    /// `PrsDimKOR_TANGENT`.
    Tangent,
    /// `PrsDimKOR_SYMMETRIC`.
    Symmetric,
}

/// `PrsDimKindOfSurface`: the kind of a measured surface.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimKindOfSurface {
    /// `PrsDimKOS_Plane`.
    Plane = 0,
    /// `PrsDimKOS_Cylinder`.
    Cylinder,
    /// `PrsDimKOS_Cone`.
    Cone,
    /// `PrsDimKOS_Sphere`.
    Sphere,
    /// `PrsDimKOS_Torus`.
    Torus,
    /// `PrsDimKOS_Revolution`.
    Revolution,
    /// `PrsDimKOS_Extrusion`.
    Extrusion,
    /// `PrsDimKOS_OtherSurface`.
    #[default]
    OtherSurface,
}

/// `PrsDimDimensionSelectionMode`: the selection modes of a dimension.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimDimensionSelectionMode {
    /// `PrsDimDimensionSelectionMode_All`: the whole dimension.
    #[default]
    All = 0,
    /// `PrsDimDimensionSelectionMode_Line`: the dimension line and arrows.
    Line,
    /// `PrsDimDimensionSelectionMode_Text`: the label.
    Text,
}
impl PrsDimDimensionSelectionMode {
    /// The mode of a selection mode number (`All` for unknown numbers, as the native switch).
    /// Example: `PrsDimDimensionSelectionMode::from_i32(2)`.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::All),
            1 => Some(Self::Line),
            2 => Some(Self::Text),
            _ => None,
        }
    }
}

/// `PrsDimDisplaySpecialSymbol`: where the special symbol of a value is shown.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimDisplaySpecialSymbol {
    /// `PrsDimDisplaySpecialSymbol_No`.
    #[default]
    No = 0,
    /// `PrsDimDisplaySpecialSymbol_Before`.
    Before,
    /// `PrsDimDisplaySpecialSymbol_After`.
    After,
}

/// `PrsDimTypeOfAngleArrowVisibility`: the visible arrows of an angle.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimTypeOfAngleArrowVisibility {
    /// `PrsDimTypeOfAngleArrowVisibility_Both`.
    #[default]
    Both = 0,
    /// `PrsDimTypeOfAngleArrowVisibility_First`.
    First,
    /// `PrsDimTypeOfAngleArrowVisibility_Second`.
    Second,
    /// `PrsDimTypeOfAngleArrowVisibility_None`.
    None,
}

/// `PrsDimTypeOfAngle`: the interior angle or its complement to a full turn.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimTypeOfAngle {
    /// `PrsDimTypeOfAngle_Interior`.
    #[default]
    Interior = 0,
    /// `PrsDimTypeOfAngle_Exterior`.
    Exterior,
}

/// `PrsDimTypeOfDist`: the direction of an offset distance.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrsDimTypeOfDist {
    /// `PrsDimTypeOfDist_Unknown`.
    #[default]
    Unknown = 0,
    /// `PrsDimTypeOfDist_Horizontal`.
    Horizontal,
    /// `PrsDimTypeOfDist_Vertical`.
    Vertical,
}
