//! The enumerations of the datum and geometric tolerance objects, in native order and values.

/// `XCAFDimTolObjectsDatumModifWithValue`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDatumModifWithValue {
    /// `XCAFDimTolObjectsDatumModifWithValue::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsDatumModifWithValue::CircularOrCylindrical`.
    CircularOrCylindrical = 1,
    /// `XCAFDimTolObjectsDatumModifWithValue::Distance`.
    Distance = 2,
    /// `XCAFDimTolObjectsDatumModifWithValue::Projected`.
    Projected = 3,
    /// `XCAFDimTolObjectsDatumModifWithValue::Spherical`.
    Spherical = 4,
}

impl crate::NativeEnum for XCAFDimTolObjectsDatumModifWithValue {
    const ALL: &'static [Self] = &[
        Self::None,
        Self::CircularOrCylindrical,
        Self::Distance,
        Self::Projected,
        Self::Spherical,
    ];
}

/// `XCAFDimTolObjectsDatumSingleModif`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDatumSingleModif {
    /// `XCAFDimTolObjectsDatumSingleModif::AnyCrossSection`.
    #[default]
    AnyCrossSection = 0,
    /// `XCAFDimTolObjectsDatumSingleModif::AnyLongitudinalSection`.
    AnyLongitudinalSection = 1,
    /// `XCAFDimTolObjectsDatumSingleModif::Basic`.
    Basic = 2,
    /// `XCAFDimTolObjectsDatumSingleModif::ContactingFeature`.
    ContactingFeature = 3,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintU`.
    DegreeOfFreedomConstraintU = 4,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintV`.
    DegreeOfFreedomConstraintV = 5,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintW`.
    DegreeOfFreedomConstraintW = 6,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintX`.
    DegreeOfFreedomConstraintX = 7,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintY`.
    DegreeOfFreedomConstraintY = 8,
    /// `XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintZ`.
    DegreeOfFreedomConstraintZ = 9,
    /// `XCAFDimTolObjectsDatumSingleModif::DistanceVariable`.
    DistanceVariable = 10,
    /// `XCAFDimTolObjectsDatumSingleModif::FreeState`.
    FreeState = 11,
    /// `XCAFDimTolObjectsDatumSingleModif::LeastMaterialRequirement`.
    LeastMaterialRequirement = 12,
    /// `XCAFDimTolObjectsDatumSingleModif::Line`.
    Line = 13,
    /// `XCAFDimTolObjectsDatumSingleModif::MajorDiameter`.
    MajorDiameter = 14,
    /// `XCAFDimTolObjectsDatumSingleModif::MaximumMaterialRequirement`.
    MaximumMaterialRequirement = 15,
    /// `XCAFDimTolObjectsDatumSingleModif::MinorDiameter`.
    MinorDiameter = 16,
    /// `XCAFDimTolObjectsDatumSingleModif::Orientation`.
    Orientation = 17,
    /// `XCAFDimTolObjectsDatumSingleModif::PitchDiameter`.
    PitchDiameter = 18,
    /// `XCAFDimTolObjectsDatumSingleModif::Plane`.
    Plane = 19,
    /// `XCAFDimTolObjectsDatumSingleModif::Point`.
    Point = 20,
    /// `XCAFDimTolObjectsDatumSingleModif::Translation`.
    Translation = 21,
}

impl crate::NativeEnum for XCAFDimTolObjectsDatumSingleModif {
    const ALL: &'static [Self] = &[
        Self::AnyCrossSection,
        Self::AnyLongitudinalSection,
        Self::Basic,
        Self::ContactingFeature,
        Self::DegreeOfFreedomConstraintU,
        Self::DegreeOfFreedomConstraintV,
        Self::DegreeOfFreedomConstraintW,
        Self::DegreeOfFreedomConstraintX,
        Self::DegreeOfFreedomConstraintY,
        Self::DegreeOfFreedomConstraintZ,
        Self::DistanceVariable,
        Self::FreeState,
        Self::LeastMaterialRequirement,
        Self::Line,
        Self::MajorDiameter,
        Self::MaximumMaterialRequirement,
        Self::MinorDiameter,
        Self::Orientation,
        Self::PitchDiameter,
        Self::Plane,
        Self::Point,
        Self::Translation,
    ];
}

/// `XCAFDimTolObjectsDatumTargetType`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDatumTargetType {
    /// `XCAFDimTolObjectsDatumTargetType::Point`.
    #[default]
    Point = 0,
    /// `XCAFDimTolObjectsDatumTargetType::Line`.
    Line = 1,
    /// `XCAFDimTolObjectsDatumTargetType::Rectangle`.
    Rectangle = 2,
    /// `XCAFDimTolObjectsDatumTargetType::Circle`.
    Circle = 3,
    /// `XCAFDimTolObjectsDatumTargetType::Area`.
    Area = 4,
}

impl crate::NativeEnum for XCAFDimTolObjectsDatumTargetType {
    const ALL: &'static [Self] = &[
        Self::Point,
        Self::Line,
        Self::Rectangle,
        Self::Circle,
        Self::Area,
    ];
}

/// `XCAFDimTolObjectsGeomToleranceMatReqModif`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsGeomToleranceMatReqModif {
    /// `XCAFDimTolObjectsGeomToleranceMatReqModif::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsGeomToleranceMatReqModif::M`.
    M = 1,
    /// `XCAFDimTolObjectsGeomToleranceMatReqModif::L`.
    L = 2,
}

impl crate::NativeEnum for XCAFDimTolObjectsGeomToleranceMatReqModif {
    const ALL: &'static [Self] = &[Self::None, Self::M, Self::L];
}

/// `XCAFDimTolObjectsGeomToleranceModif`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsGeomToleranceModif {
    /// `XCAFDimTolObjectsGeomToleranceModif::AnyCrossSection`.
    #[default]
    AnyCrossSection = 0,
    /// `XCAFDimTolObjectsGeomToleranceModif::CommonZone`.
    CommonZone = 1,
    /// `XCAFDimTolObjectsGeomToleranceModif::EachRadialElement`.
    EachRadialElement = 2,
    /// `XCAFDimTolObjectsGeomToleranceModif::FreeState`.
    FreeState = 3,
    /// `XCAFDimTolObjectsGeomToleranceModif::LeastMaterialRequirement`.
    LeastMaterialRequirement = 4,
    /// `XCAFDimTolObjectsGeomToleranceModif::LineElement`.
    LineElement = 5,
    /// `XCAFDimTolObjectsGeomToleranceModif::MajorDiameter`.
    MajorDiameter = 6,
    /// `XCAFDimTolObjectsGeomToleranceModif::MaximumMaterialRequirement`.
    MaximumMaterialRequirement = 7,
    /// `XCAFDimTolObjectsGeomToleranceModif::MinorDiameter`.
    MinorDiameter = 8,
    /// `XCAFDimTolObjectsGeomToleranceModif::NotConvex`.
    NotConvex = 9,
    /// `XCAFDimTolObjectsGeomToleranceModif::PitchDiameter`.
    PitchDiameter = 10,
    /// `XCAFDimTolObjectsGeomToleranceModif::ReciprocityRequirement`.
    ReciprocityRequirement = 11,
    /// `XCAFDimTolObjectsGeomToleranceModif::SeparateRequirement`.
    SeparateRequirement = 12,
    /// `XCAFDimTolObjectsGeomToleranceModif::StatisticalTolerance`.
    StatisticalTolerance = 13,
    /// `XCAFDimTolObjectsGeomToleranceModif::TangentPlane`.
    TangentPlane = 14,
    /// `XCAFDimTolObjectsGeomToleranceModif::AllAround`.
    AllAround = 15,
    /// `XCAFDimTolObjectsGeomToleranceModif::AllOver`.
    AllOver = 16,
}

impl crate::NativeEnum for XCAFDimTolObjectsGeomToleranceModif {
    const ALL: &'static [Self] = &[
        Self::AnyCrossSection,
        Self::CommonZone,
        Self::EachRadialElement,
        Self::FreeState,
        Self::LeastMaterialRequirement,
        Self::LineElement,
        Self::MajorDiameter,
        Self::MaximumMaterialRequirement,
        Self::MinorDiameter,
        Self::NotConvex,
        Self::PitchDiameter,
        Self::ReciprocityRequirement,
        Self::SeparateRequirement,
        Self::StatisticalTolerance,
        Self::TangentPlane,
        Self::AllAround,
        Self::AllOver,
    ];
}

/// `XCAFDimTolObjectsGeomToleranceType`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsGeomToleranceType {
    /// `XCAFDimTolObjectsGeomToleranceType::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsGeomToleranceType::Angularity`.
    Angularity = 1,
    /// `XCAFDimTolObjectsGeomToleranceType::CircularRunout`.
    CircularRunout = 2,
    /// `XCAFDimTolObjectsGeomToleranceType::CircularityOrRoundness`.
    CircularityOrRoundness = 3,
    /// `XCAFDimTolObjectsGeomToleranceType::Coaxiality`.
    Coaxiality = 4,
    /// `XCAFDimTolObjectsGeomToleranceType::Concentricity`.
    Concentricity = 5,
    /// `XCAFDimTolObjectsGeomToleranceType::Cylindricity`.
    Cylindricity = 6,
    /// `XCAFDimTolObjectsGeomToleranceType::Flatness`.
    Flatness = 7,
    /// `XCAFDimTolObjectsGeomToleranceType::Parallelism`.
    Parallelism = 8,
    /// `XCAFDimTolObjectsGeomToleranceType::Perpendicularity`.
    Perpendicularity = 9,
    /// `XCAFDimTolObjectsGeomToleranceType::Position`.
    Position = 10,
    /// `XCAFDimTolObjectsGeomToleranceType::ProfileOfLine`.
    ProfileOfLine = 11,
    /// `XCAFDimTolObjectsGeomToleranceType::ProfileOfSurface`.
    ProfileOfSurface = 12,
    /// `XCAFDimTolObjectsGeomToleranceType::Straightness`.
    Straightness = 13,
    /// `XCAFDimTolObjectsGeomToleranceType::Symmetry`.
    Symmetry = 14,
    /// `XCAFDimTolObjectsGeomToleranceType::TotalRunout`.
    TotalRunout = 15,
}

impl crate::NativeEnum for XCAFDimTolObjectsGeomToleranceType {
    const ALL: &'static [Self] = &[
        Self::None,
        Self::Angularity,
        Self::CircularRunout,
        Self::CircularityOrRoundness,
        Self::Coaxiality,
        Self::Concentricity,
        Self::Cylindricity,
        Self::Flatness,
        Self::Parallelism,
        Self::Perpendicularity,
        Self::Position,
        Self::ProfileOfLine,
        Self::ProfileOfSurface,
        Self::Straightness,
        Self::Symmetry,
        Self::TotalRunout,
    ];
}

/// `XCAFDimTolObjectsGeomToleranceTypeValue`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsGeomToleranceTypeValue {
    /// `XCAFDimTolObjectsGeomToleranceTypeValue::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsGeomToleranceTypeValue::Diameter`.
    Diameter = 1,
    /// `XCAFDimTolObjectsGeomToleranceTypeValue::SphericalDiameter`.
    SphericalDiameter = 2,
}

impl crate::NativeEnum for XCAFDimTolObjectsGeomToleranceTypeValue {
    const ALL: &'static [Self] = &[Self::None, Self::Diameter, Self::SphericalDiameter];
}

/// `XCAFDimTolObjectsGeomToleranceZoneModif`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsGeomToleranceZoneModif {
    /// `XCAFDimTolObjectsGeomToleranceZoneModif::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsGeomToleranceZoneModif::Projected`.
    Projected = 1,
    /// `XCAFDimTolObjectsGeomToleranceZoneModif::Runout`.
    Runout = 2,
    /// `XCAFDimTolObjectsGeomToleranceZoneModif::NonUniform`.
    NonUniform = 3,
}

impl crate::NativeEnum for XCAFDimTolObjectsGeomToleranceZoneModif {
    const ALL: &'static [Self] = &[Self::None, Self::Projected, Self::Runout, Self::NonUniform];
}

/// `XCAFDimTolObjectsToleranceZoneAffectedPlane`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsToleranceZoneAffectedPlane {
    /// `XCAFDimTolObjectsToleranceZoneAffectedPlane::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsToleranceZoneAffectedPlane::Intersection`.
    Intersection = 1,
    /// `XCAFDimTolObjectsToleranceZoneAffectedPlane::Orientation`.
    Orientation = 2,
}

impl crate::NativeEnum for XCAFDimTolObjectsToleranceZoneAffectedPlane {
    const ALL: &'static [Self] = &[Self::None, Self::Intersection, Self::Orientation];
}
