//! The enumerations of the dimension objects, in native order and values.

/// `XCAFDimTolObjectsAngularQualifier`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsAngularQualifier {
    /// `XCAFDimTolObjectsAngularQualifier::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsAngularQualifier::Small`.
    Small = 1,
    /// `XCAFDimTolObjectsAngularQualifier::Large`.
    Large = 2,
    /// `XCAFDimTolObjectsAngularQualifier::Equal`.
    Equal = 3,
}

impl crate::NativeEnum for XCAFDimTolObjectsAngularQualifier {
    const ALL: &'static [Self] = &[Self::None, Self::Small, Self::Large, Self::Equal];
}

/// `XCAFDimTolObjectsDimensionFormVariance`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDimensionFormVariance {
    /// `XCAFDimTolObjectsDimensionFormVariance::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsDimensionFormVariance::A`.
    A = 1,
    /// `XCAFDimTolObjectsDimensionFormVariance::B`.
    B = 2,
    /// `XCAFDimTolObjectsDimensionFormVariance::C`.
    C = 3,
    /// `XCAFDimTolObjectsDimensionFormVariance::CD`.
    CD = 4,
    /// `XCAFDimTolObjectsDimensionFormVariance::D`.
    D = 5,
    /// `XCAFDimTolObjectsDimensionFormVariance::E`.
    E = 6,
    /// `XCAFDimTolObjectsDimensionFormVariance::EF`.
    EF = 7,
    /// `XCAFDimTolObjectsDimensionFormVariance::F`.
    F = 8,
    /// `XCAFDimTolObjectsDimensionFormVariance::FG`.
    FG = 9,
    /// `XCAFDimTolObjectsDimensionFormVariance::G`.
    G = 10,
    /// `XCAFDimTolObjectsDimensionFormVariance::H`.
    H = 11,
    /// `XCAFDimTolObjectsDimensionFormVariance::JS`.
    JS = 12,
    /// `XCAFDimTolObjectsDimensionFormVariance::J`.
    J = 13,
    /// `XCAFDimTolObjectsDimensionFormVariance::K`.
    K = 14,
    /// `XCAFDimTolObjectsDimensionFormVariance::M`.
    M = 15,
    /// `XCAFDimTolObjectsDimensionFormVariance::N`.
    N = 16,
    /// `XCAFDimTolObjectsDimensionFormVariance::P`.
    P = 17,
    /// `XCAFDimTolObjectsDimensionFormVariance::R`.
    R = 18,
    /// `XCAFDimTolObjectsDimensionFormVariance::S`.
    S = 19,
    /// `XCAFDimTolObjectsDimensionFormVariance::T`.
    T = 20,
    /// `XCAFDimTolObjectsDimensionFormVariance::U`.
    U = 21,
    /// `XCAFDimTolObjectsDimensionFormVariance::V`.
    V = 22,
    /// `XCAFDimTolObjectsDimensionFormVariance::X`.
    X = 23,
    /// `XCAFDimTolObjectsDimensionFormVariance::Y`.
    Y = 24,
    /// `XCAFDimTolObjectsDimensionFormVariance::Z`.
    Z = 25,
    /// `XCAFDimTolObjectsDimensionFormVariance::ZA`.
    ZA = 26,
    /// `XCAFDimTolObjectsDimensionFormVariance::ZB`.
    ZB = 27,
    /// `XCAFDimTolObjectsDimensionFormVariance::ZC`.
    ZC = 28,
}

impl crate::NativeEnum for XCAFDimTolObjectsDimensionFormVariance {
    const ALL: &'static [Self] = &[
        Self::None,
        Self::A,
        Self::B,
        Self::C,
        Self::CD,
        Self::D,
        Self::E,
        Self::EF,
        Self::F,
        Self::FG,
        Self::G,
        Self::H,
        Self::JS,
        Self::J,
        Self::K,
        Self::M,
        Self::N,
        Self::P,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::X,
        Self::Y,
        Self::Z,
        Self::ZA,
        Self::ZB,
        Self::ZC,
    ];
}

/// `XCAFDimTolObjectsDimensionGrade`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDimensionGrade {
    /// `XCAFDimTolObjectsDimensionGrade::IT01`.
    #[default]
    IT01 = 0,
    /// `XCAFDimTolObjectsDimensionGrade::IT0`.
    IT0 = 1,
    /// `XCAFDimTolObjectsDimensionGrade::IT1`.
    IT1 = 2,
    /// `XCAFDimTolObjectsDimensionGrade::IT2`.
    IT2 = 3,
    /// `XCAFDimTolObjectsDimensionGrade::IT3`.
    IT3 = 4,
    /// `XCAFDimTolObjectsDimensionGrade::IT4`.
    IT4 = 5,
    /// `XCAFDimTolObjectsDimensionGrade::IT5`.
    IT5 = 6,
    /// `XCAFDimTolObjectsDimensionGrade::IT6`.
    IT6 = 7,
    /// `XCAFDimTolObjectsDimensionGrade::IT7`.
    IT7 = 8,
    /// `XCAFDimTolObjectsDimensionGrade::IT8`.
    IT8 = 9,
    /// `XCAFDimTolObjectsDimensionGrade::IT9`.
    IT9 = 10,
    /// `XCAFDimTolObjectsDimensionGrade::IT10`.
    IT10 = 11,
    /// `XCAFDimTolObjectsDimensionGrade::IT11`.
    IT11 = 12,
    /// `XCAFDimTolObjectsDimensionGrade::IT12`.
    IT12 = 13,
    /// `XCAFDimTolObjectsDimensionGrade::IT13`.
    IT13 = 14,
    /// `XCAFDimTolObjectsDimensionGrade::IT14`.
    IT14 = 15,
    /// `XCAFDimTolObjectsDimensionGrade::IT15`.
    IT15 = 16,
    /// `XCAFDimTolObjectsDimensionGrade::IT16`.
    IT16 = 17,
    /// `XCAFDimTolObjectsDimensionGrade::IT17`.
    IT17 = 18,
    /// `XCAFDimTolObjectsDimensionGrade::IT18`.
    IT18 = 19,
}

impl crate::NativeEnum for XCAFDimTolObjectsDimensionGrade {
    const ALL: &'static [Self] = &[
        Self::IT01,
        Self::IT0,
        Self::IT1,
        Self::IT2,
        Self::IT3,
        Self::IT4,
        Self::IT5,
        Self::IT6,
        Self::IT7,
        Self::IT8,
        Self::IT9,
        Self::IT10,
        Self::IT11,
        Self::IT12,
        Self::IT13,
        Self::IT14,
        Self::IT15,
        Self::IT16,
        Self::IT17,
        Self::IT18,
    ];
}

/// `XCAFDimTolObjectsDimensionModif`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDimensionModif {
    /// `XCAFDimTolObjectsDimensionModif::ControlledRadius`.
    #[default]
    ControlledRadius = 0,
    /// `XCAFDimTolObjectsDimensionModif::Square`.
    Square = 1,
    /// `XCAFDimTolObjectsDimensionModif::StatisticalTolerance`.
    StatisticalTolerance = 2,
    /// `XCAFDimTolObjectsDimensionModif::ContinuousFeature`.
    ContinuousFeature = 3,
    /// `XCAFDimTolObjectsDimensionModif::TwoPointSize`.
    TwoPointSize = 4,
    /// `XCAFDimTolObjectsDimensionModif::LocalSizeDefinedBySphere`.
    LocalSizeDefinedBySphere = 5,
    /// `XCAFDimTolObjectsDimensionModif::LeastSquaresAssociationCriterion`.
    LeastSquaresAssociationCriterion = 6,
    /// `XCAFDimTolObjectsDimensionModif::MaximumInscribedAssociation`.
    MaximumInscribedAssociation = 7,
    /// `XCAFDimTolObjectsDimensionModif::MinimumCircumscribedAssociation`.
    MinimumCircumscribedAssociation = 8,
    /// `XCAFDimTolObjectsDimensionModif::CircumferenceDiameter`.
    CircumferenceDiameter = 9,
    /// `XCAFDimTolObjectsDimensionModif::AreaDiameter`.
    AreaDiameter = 10,
    /// `XCAFDimTolObjectsDimensionModif::VolumeDiameter`.
    VolumeDiameter = 11,
    /// `XCAFDimTolObjectsDimensionModif::MaximumSize`.
    MaximumSize = 12,
    /// `XCAFDimTolObjectsDimensionModif::MinimumSize`.
    MinimumSize = 13,
    /// `XCAFDimTolObjectsDimensionModif::AverageSize`.
    AverageSize = 14,
    /// `XCAFDimTolObjectsDimensionModif::MedianSize`.
    MedianSize = 15,
    /// `XCAFDimTolObjectsDimensionModif::MidRangeSize`.
    MidRangeSize = 16,
    /// `XCAFDimTolObjectsDimensionModif::RangeOfSizes`.
    RangeOfSizes = 17,
    /// `XCAFDimTolObjectsDimensionModif::AnyRestrictedPortionOfFeature`.
    AnyRestrictedPortionOfFeature = 18,
    /// `XCAFDimTolObjectsDimensionModif::AnyCrossSection`.
    AnyCrossSection = 19,
    /// `XCAFDimTolObjectsDimensionModif::SpecificFixedCrossSection`.
    SpecificFixedCrossSection = 20,
    /// `XCAFDimTolObjectsDimensionModif::CommonTolerance`.
    CommonTolerance = 21,
    /// `XCAFDimTolObjectsDimensionModif::FreeStateCondition`.
    FreeStateCondition = 22,
    /// `XCAFDimTolObjectsDimensionModif::Between`.
    Between = 23,
}

impl crate::NativeEnum for XCAFDimTolObjectsDimensionModif {
    const ALL: &'static [Self] = &[
        Self::ControlledRadius,
        Self::Square,
        Self::StatisticalTolerance,
        Self::ContinuousFeature,
        Self::TwoPointSize,
        Self::LocalSizeDefinedBySphere,
        Self::LeastSquaresAssociationCriterion,
        Self::MaximumInscribedAssociation,
        Self::MinimumCircumscribedAssociation,
        Self::CircumferenceDiameter,
        Self::AreaDiameter,
        Self::VolumeDiameter,
        Self::MaximumSize,
        Self::MinimumSize,
        Self::AverageSize,
        Self::MedianSize,
        Self::MidRangeSize,
        Self::RangeOfSizes,
        Self::AnyRestrictedPortionOfFeature,
        Self::AnyCrossSection,
        Self::SpecificFixedCrossSection,
        Self::CommonTolerance,
        Self::FreeStateCondition,
        Self::Between,
    ];
}

/// `XCAFDimTolObjectsDimensionQualifier`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDimensionQualifier {
    /// `XCAFDimTolObjectsDimensionQualifier::None`.
    #[default]
    None = 0,
    /// `XCAFDimTolObjectsDimensionQualifier::Min`.
    Min = 1,
    /// `XCAFDimTolObjectsDimensionQualifier::Max`.
    Max = 2,
    /// `XCAFDimTolObjectsDimensionQualifier::Avg`.
    Avg = 3,
}

impl crate::NativeEnum for XCAFDimTolObjectsDimensionQualifier {
    const ALL: &'static [Self] = &[Self::None, Self::Min, Self::Max, Self::Avg];
}

/// `XCAFDimTolObjectsDimensionType`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum XCAFDimTolObjectsDimensionType {
    /// `XCAFDimTolObjectsDimensionType::LocationNone`.
    #[default]
    LocationNone = 0,
    /// `XCAFDimTolObjectsDimensionType::LocationCurvedDistance`.
    LocationCurvedDistance = 1,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistance`.
    LocationLinearDistance = 2,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromCenterToOuter`.
    LocationLinearDistanceFromCenterToOuter = 3,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromCenterToInner`.
    LocationLinearDistanceFromCenterToInner = 4,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToCenter`.
    LocationLinearDistanceFromOuterToCenter = 5,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToOuter`.
    LocationLinearDistanceFromOuterToOuter = 6,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToInner`.
    LocationLinearDistanceFromOuterToInner = 7,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToCenter`.
    LocationLinearDistanceFromInnerToCenter = 8,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToOuter`.
    LocationLinearDistanceFromInnerToOuter = 9,
    /// `XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToInner`.
    LocationLinearDistanceFromInnerToInner = 10,
    /// `XCAFDimTolObjectsDimensionType::LocationAngular`.
    LocationAngular = 11,
    /// `XCAFDimTolObjectsDimensionType::LocationOriented`.
    LocationOriented = 12,
    /// `XCAFDimTolObjectsDimensionType::LocationWithPath`.
    LocationWithPath = 13,
    /// `XCAFDimTolObjectsDimensionType::SizeCurveLength`.
    SizeCurveLength = 14,
    /// `XCAFDimTolObjectsDimensionType::SizeDiameter`.
    SizeDiameter = 15,
    /// `XCAFDimTolObjectsDimensionType::SizeSphericalDiameter`.
    SizeSphericalDiameter = 16,
    /// `XCAFDimTolObjectsDimensionType::SizeRadius`.
    SizeRadius = 17,
    /// `XCAFDimTolObjectsDimensionType::SizeSphericalRadius`.
    SizeSphericalRadius = 18,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalMinorDiameter`.
    SizeToroidalMinorDiameter = 19,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalMajorDiameter`.
    SizeToroidalMajorDiameter = 20,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalMinorRadius`.
    SizeToroidalMinorRadius = 21,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalMajorRadius`.
    SizeToroidalMajorRadius = 22,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalHighMajorDiameter`.
    SizeToroidalHighMajorDiameter = 23,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalLowMajorDiameter`.
    SizeToroidalLowMajorDiameter = 24,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalHighMajorRadius`.
    SizeToroidalHighMajorRadius = 25,
    /// `XCAFDimTolObjectsDimensionType::SizeToroidalLowMajorRadius`.
    SizeToroidalLowMajorRadius = 26,
    /// `XCAFDimTolObjectsDimensionType::SizeThickness`.
    SizeThickness = 27,
    /// `XCAFDimTolObjectsDimensionType::SizeAngular`.
    SizeAngular = 28,
    /// `XCAFDimTolObjectsDimensionType::SizeWithPath`.
    SizeWithPath = 29,
    /// `XCAFDimTolObjectsDimensionType::CommonLabel`.
    CommonLabel = 30,
    /// `XCAFDimTolObjectsDimensionType::DimensionPresentation`.
    DimensionPresentation = 31,
}

impl crate::NativeEnum for XCAFDimTolObjectsDimensionType {
    const ALL: &'static [Self] = &[
        Self::LocationNone,
        Self::LocationCurvedDistance,
        Self::LocationLinearDistance,
        Self::LocationLinearDistanceFromCenterToOuter,
        Self::LocationLinearDistanceFromCenterToInner,
        Self::LocationLinearDistanceFromOuterToCenter,
        Self::LocationLinearDistanceFromOuterToOuter,
        Self::LocationLinearDistanceFromOuterToInner,
        Self::LocationLinearDistanceFromInnerToCenter,
        Self::LocationLinearDistanceFromInnerToOuter,
        Self::LocationLinearDistanceFromInnerToInner,
        Self::LocationAngular,
        Self::LocationOriented,
        Self::LocationWithPath,
        Self::SizeCurveLength,
        Self::SizeDiameter,
        Self::SizeSphericalDiameter,
        Self::SizeRadius,
        Self::SizeSphericalRadius,
        Self::SizeToroidalMinorDiameter,
        Self::SizeToroidalMajorDiameter,
        Self::SizeToroidalMinorRadius,
        Self::SizeToroidalMajorRadius,
        Self::SizeToroidalHighMajorDiameter,
        Self::SizeToroidalLowMajorDiameter,
        Self::SizeToroidalHighMajorRadius,
        Self::SizeToroidalLowMajorRadius,
        Self::SizeThickness,
        Self::SizeAngular,
        Self::SizeWithPath,
        Self::CommonLabel,
        Self::DimensionPresentation,
    ];
}
