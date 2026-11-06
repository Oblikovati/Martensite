//! The Prs3d enumerations, with their native discriminants. The legacy aliases of the native
//! enumerators (`Prs3dDA_XAxisLength`...) are associated constants.

/// `Prs3dDatumAttribute`: the numeric attributes of a datum.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Prs3dDatumAttribute {
    /// `Prs3dDatumAttribute_XAxisLength`.
    XAxisLength = 0,
    /// `Prs3dDatumAttribute_YAxisLength`.
    YAxisLength = 1,
    /// `Prs3dDatumAttribute_ZAxisLength`.
    ZAxisLength = 2,
    /// `Prs3dDatumAttribute_ShadingTubeRadiusPercent`.
    ShadingTubeRadiusPercent = 3,
    /// `Prs3dDatumAttribute_ShadingConeRadiusPercent`.
    ShadingConeRadiusPercent = 4,
    /// `Prs3dDatumAttribute_ShadingConeLengthPercent`.
    ShadingConeLengthPercent = 5,
    /// `Prs3dDatumAttribute_ShadingOriginRadiusPercent`.
    ShadingOriginRadiusPercent = 6,
    /// `Prs3dDatumAttribute_ShadingNumberOfFacettes`.
    ShadingNumberOfFacettes = 7,
}

impl Prs3dDatumAttribute {
    /// `Prs3dDatumAttribute_NB`: the number of attributes.
    pub const NB: usize = 8;
    /// `Prs3dDA_XAxisLength`.
    pub const DA_X_AXIS_LENGTH: Self = Self::XAxisLength;
    /// `Prs3dDA_YAxisLength`.
    pub const DA_Y_AXIS_LENGTH: Self = Self::YAxisLength;
    /// `Prs3dDA_ZAxisLength`.
    pub const DA_Z_AXIS_LENGTH: Self = Self::ZAxisLength;
    /// `Prs3dDP_ShadingTubeRadiusPercent`.
    pub const DP_SHADING_TUBE_RADIUS_PERCENT: Self = Self::ShadingTubeRadiusPercent;
    /// `Prs3dDP_ShadingConeRadiusPercent`.
    pub const DP_SHADING_CONE_RADIUS_PERCENT: Self = Self::ShadingConeRadiusPercent;
    /// `Prs3dDP_ShadingConeLengthPercent`.
    pub const DP_SHADING_CONE_LENGTH_PERCENT: Self = Self::ShadingConeLengthPercent;
    /// `Prs3dDP_ShadingOriginRadiusPercent`.
    pub const DP_SHADING_ORIGIN_RADIUS_PERCENT: Self = Self::ShadingOriginRadiusPercent;
    /// `Prs3dDP_ShadingNumberOfFacettes`.
    pub const DP_SHADING_NUMBER_OF_FACETTES: Self = Self::ShadingNumberOfFacettes;
}

/// `Prs3dDatumAxes`: the axes of a datum, a bit mask of X (1), Y (2) and Z (4).
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dDatumAxes {
    /// `Prs3dDatumAxes_XAxis`.
    XAxis = 0x01,
    /// `Prs3dDatumAxes_YAxis`.
    YAxis = 0x02,
    /// `Prs3dDatumAxes_XYAxes`.
    XYAxes = 0x03,
    /// `Prs3dDatumAxes_ZAxis`.
    ZAxis = 0x04,
    /// `Prs3dDatumAxes_XZAxes`.
    XZAxes = 0x05,
    /// `Prs3dDatumAxes_YZAxes`.
    YZAxes = 0x06,
    /// `Prs3dDatumAxes_XYZAxes`.
    XYZAxes = 0x07,
}

impl Prs3dDatumAxes {
    /// `Prs3dDA_XAxis`.
    pub const DA_X_AXIS: Self = Self::XAxis;
    /// `Prs3dDA_YAxis`.
    pub const DA_Y_AXIS: Self = Self::YAxis;
    /// `Prs3dDA_ZAxis`.
    pub const DA_Z_AXIS: Self = Self::ZAxis;
    /// `Prs3dDA_XYAxis`.
    pub const DA_XY_AXIS: Self = Self::XYAxes;
    /// `Prs3dDA_YZAxis`.
    pub const DA_YZ_AXIS: Self = Self::YZAxes;
    /// `Prs3dDA_XZAxis`.
    pub const DA_XZ_AXIS: Self = Self::XZAxes;
    /// `Prs3dDA_XYZAxis`.
    pub const DA_XYZ_AXIS: Self = Self::XYZAxes;
}

/// `Prs3dDatumMode`: how a datum is drawn.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dDatumMode {
    /// `Prs3dDM_WireFrame`.
    WireFrame = 0,
    /// `Prs3dDM_Shaded`.
    Shaded = 1,
}

/// `Prs3dDatumParts`: the parts of a datum.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Prs3dDatumParts {
    /// `Prs3dDatumParts_Origin`.
    Origin = 0,
    /// `Prs3dDatumParts_XAxis`.
    XAxis = 1,
    /// `Prs3dDatumParts_YAxis`.
    YAxis = 2,
    /// `Prs3dDatumParts_ZAxis`.
    ZAxis = 3,
    /// `Prs3dDatumParts_XArrow`.
    XArrow = 4,
    /// `Prs3dDatumParts_YArrow`.
    YArrow = 5,
    /// `Prs3dDatumParts_ZArrow`.
    ZArrow = 6,
    /// `Prs3dDatumParts_XOYAxis`.
    XOYAxis = 7,
    /// `Prs3dDatumParts_YOZAxis`.
    YOZAxis = 8,
    /// `Prs3dDatumParts_XOZAxis`.
    XOZAxis = 9,
    /// `Prs3dDatumParts_None`.
    None = 10,
}

impl Prs3dDatumParts {
    /// `Prs3dDatumParts_NB`: the number of parts.
    pub const NB: usize = 11;
    /// Every part from `Origin` to `XOZAxis`, in native order.
    pub const DRAWN: [Self; 10] = [
        Self::Origin,
        Self::XAxis,
        Self::YAxis,
        Self::ZAxis,
        Self::XArrow,
        Self::YArrow,
        Self::ZArrow,
        Self::XOYAxis,
        Self::YOZAxis,
        Self::XOZAxis,
    ];
    /// `Prs3dDP_Origin`.
    pub const DP_ORIGIN: Self = Self::Origin;
    /// `Prs3dDP_XAxis`.
    pub const DP_X_AXIS: Self = Self::XAxis;
    /// `Prs3dDP_YAxis`.
    pub const DP_Y_AXIS: Self = Self::YAxis;
    /// `Prs3dDP_ZAxis`.
    pub const DP_Z_AXIS: Self = Self::ZAxis;
    /// `Prs3dDP_XArrow`.
    pub const DP_X_ARROW: Self = Self::XArrow;
    /// `Prs3dDP_YArrow`.
    pub const DP_Y_ARROW: Self = Self::YArrow;
    /// `Prs3dDP_ZArrow`.
    pub const DP_Z_ARROW: Self = Self::ZArrow;
    /// `Prs3dDP_XOYAxis`.
    pub const DP_XOY_AXIS: Self = Self::XOYAxis;
    /// `Prs3dDP_YOZAxis`.
    pub const DP_YOZ_AXIS: Self = Self::YOZAxis;
    /// `Prs3dDP_XOZAxis`.
    pub const DP_XOZ_AXIS: Self = Self::XOZAxis;
    /// `Prs3dDP_None`.
    pub const DP_NONE: Self = Self::None;
}

/// `Prs3dDimensionArrowOrientation`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dDimensionArrowOrientation {
    /// `Prs3dDAO_Internal`.
    Internal = 0,
    /// `Prs3dDAO_External`.
    External = 1,
    /// `Prs3dDAO_Fit`.
    Fit = 2,
}

/// `Prs3dDimensionTextHorizontalPosition`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dDimensionTextHorizontalPosition {
    /// `Prs3dDTHP_Left`.
    Left = 0,
    /// `Prs3dDTHP_Right`.
    Right = 1,
    /// `Prs3dDTHP_Center`.
    Center = 2,
    /// `Prs3dDTHP_Fit`.
    Fit = 3,
}

/// `Prs3dDimensionTextVerticalPosition`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dDimensionTextVerticalPosition {
    /// `Prs3dDTVP_Above`.
    Above = 0,
    /// `Prs3dDTVP_Below`.
    Below = 1,
    /// `Prs3dDTVP_Center`.
    Center = 2,
}

/// `Prs3dTypeOfHighlight`: the highlight styles of an interactive context.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Prs3dTypeOfHighlight {
    /// `Prs3dTypeOfHighlight_None`.
    None = 0,
    /// `Prs3dTypeOfHighlight_Selected`.
    Selected = 1,
    /// `Prs3dTypeOfHighlight_Dynamic`.
    Dynamic = 2,
    /// `Prs3dTypeOfHighlight_LocalSelected`.
    LocalSelected = 3,
    /// `Prs3dTypeOfHighlight_LocalDynamic`.
    LocalDynamic = 4,
    /// `Prs3dTypeOfHighlight_SubIntensity`.
    SubIntensity = 5,
    /// `Prs3dTypeOfHighlight_NB`: the number of styles.
    Nb = 6,
}

/// `Prs3dTypeOfHLR`: the hidden line removal algorithm.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dTypeOfHlr {
    /// `Prs3dTOH_NotSet`.
    NotSet = 0,
    /// `Prs3dTOH_PolyAlgo`.
    PolyAlgo = 1,
    /// `Prs3dTOH_Algo`.
    Algo = 2,
}

/// `Prs3dTypeOfLinePicking`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dTypeOfLinePicking {
    /// `Prs3dTOLP_Point`.
    Point = 0,
    /// `Prs3dTOLP_Segment`.
    Segment = 1,
}

/// `Prs3dVertexDrawMode`: which vertices of a shape are drawn.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3dVertexDrawMode {
    /// `Prs3dVDM_Isolated`: only the isolated vertices.
    Isolated = 0,
    /// `Prs3dVDM_All`: every vertex.
    All = 1,
    /// `Prs3dVDM_Inherited`: the mode of the link drawer.
    Inherited = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_and_aliases_match_the_native_values() {
        assert_eq!(
            Prs3dDatumAttribute::ShadingNumberOfFacettes as usize + 1,
            Prs3dDatumAttribute::NB
        );
        assert_eq!(Prs3dDatumParts::None as usize + 1, Prs3dDatumParts::NB);
        assert_eq!(Prs3dDatumAxes::XYZAxes as i32, 7);
        assert_eq!(Prs3dDatumAxes::DA_XZ_AXIS as i32, 5);
        assert_eq!(Prs3dDatumParts::DP_XOZ_AXIS, Prs3dDatumParts::XOZAxis);
        assert_eq!(Prs3dTypeOfHighlight::Nb as i32, 6);
        assert_eq!(Prs3dVertexDrawMode::Inherited as i32, 2);
        assert_eq!(Prs3dDimensionTextHorizontalPosition::Fit as i32, 3);
    }
}
