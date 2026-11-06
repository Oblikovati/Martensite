//! `STEPCAFControlGDTProperty`: the tools to access the GD&T properties: names of the XCAF values in
//! STEP, and back, and the tessellation of a presentation.
use super::gdt_tessellation::tessellation_of;
use crate::geom_to_step::shared_array1;
use crate::step_basic::{
    StepBasicLengthMeasureWithUnit, StepBasicMeasureValueMember, StepBasicUnit,
};
use crate::step_data::{StepDataSelectMember, StepDataSelectType};
use crate::step_dim_tol::{
    StepDimTolCylindricityTolerance, StepDimTolDatumReferenceModifier,
    StepDimTolDatumReferenceModifierType, StepDimTolDatumReferenceModifierWithValue,
    StepDimTolFlatnessTolerance, StepDimTolGeometricTolerance,
    StepDimTolGeometricToleranceModifier, StepDimTolGeometricToleranceType,
    StepDimTolLineProfileTolerance, StepDimTolPositionTolerance, StepDimTolRoundnessTolerance,
    StepDimTolSimpleDatumReferenceModifierMember, StepDimTolStraightnessTolerance,
    StepDimTolSurfaceProfileTolerance,
};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_repr::{StepReprCompoundRepresentationItem, StepReprDescriptiveRepresentationItem};
use crate::step_shape::StepShapeLimitsAndFits;
use crate::step_visual::StepVisualTessellatedGeometricSet;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxcaf::NativeEnum;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumModifWithValue, XCAFDimTolObjectsDatumSingleModif,
    XCAFDimTolObjectsDatumTargetType, XCAFDimTolObjectsDimensionFormVariance,
    XCAFDimTolObjectsDimensionGrade, XCAFDimTolObjectsDimensionModif,
    XCAFDimTolObjectsDimensionQualifier, XCAFDimTolObjectsDimensionType,
    XCAFDimTolObjectsGeomToleranceModif, XCAFDimTolObjectsGeomToleranceType,
    XCAFDimTolObjectsGeomToleranceTypeValue,
};

mod conversions;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use conversions::*;

const WHERE: &str = "STEPCAFControlGDTProperty";

/// The names of the dimension modifiers.
const DIM_MODIFIERS: [(&str, XCAFDimTolObjectsDimensionModif); 23] = [
    (
        "controlled radius",
        XCAFDimTolObjectsDimensionModif::ControlledRadius,
    ),
    ("square", XCAFDimTolObjectsDimensionModif::Square),
    (
        "statistical",
        XCAFDimTolObjectsDimensionModif::StatisticalTolerance,
    ),
    (
        "continuous feature",
        XCAFDimTolObjectsDimensionModif::ContinuousFeature,
    ),
    (
        "two point size",
        XCAFDimTolObjectsDimensionModif::TwoPointSize,
    ),
    (
        "local size defined by a sphere",
        XCAFDimTolObjectsDimensionModif::LocalSizeDefinedBySphere,
    ),
    (
        "least squares association criteria",
        XCAFDimTolObjectsDimensionModif::LeastSquaresAssociationCriterion,
    ),
    (
        "maximum inscribed association criteria",
        XCAFDimTolObjectsDimensionModif::MaximumInscribedAssociation,
    ),
    (
        "minimum circumscribed association criteria",
        XCAFDimTolObjectsDimensionModif::MinimumCircumscribedAssociation,
    ),
    (
        "circumference diameter calculated size",
        XCAFDimTolObjectsDimensionModif::CircumferenceDiameter,
    ),
    (
        "area diameter calculated size",
        XCAFDimTolObjectsDimensionModif::AreaDiameter,
    ),
    (
        "volume diameter calculated size",
        XCAFDimTolObjectsDimensionModif::VolumeDiameter,
    ),
    (
        "maximum rank order size",
        XCAFDimTolObjectsDimensionModif::MaximumSize,
    ),
    (
        "minimum rank order size",
        XCAFDimTolObjectsDimensionModif::MinimumSize,
    ),
    (
        "average rank order size",
        XCAFDimTolObjectsDimensionModif::AverageSize,
    ),
    (
        "median rank order size",
        XCAFDimTolObjectsDimensionModif::MedianSize,
    ),
    (
        "mid range rank order size",
        XCAFDimTolObjectsDimensionModif::MidRangeSize,
    ),
    (
        "range rank order size",
        XCAFDimTolObjectsDimensionModif::RangeOfSizes,
    ),
    (
        "any part of the feature",
        XCAFDimTolObjectsDimensionModif::AnyRestrictedPortionOfFeature,
    ),
    (
        "any cross section",
        XCAFDimTolObjectsDimensionModif::AnyCrossSection,
    ),
    (
        "specific fixed cross section",
        XCAFDimTolObjectsDimensionModif::SpecificFixedCrossSection,
    ),
    (
        "common tolerance",
        XCAFDimTolObjectsDimensionModif::CommonTolerance,
    ),
    (
        "free state condition",
        XCAFDimTolObjectsDimensionModif::FreeStateCondition,
    ),
];

/// The names of the dimension types.
const DIM_TYPES: [(&str, XCAFDimTolObjectsDimensionType); 24] = [
    (
        "curve length",
        XCAFDimTolObjectsDimensionType::SizeCurveLength,
    ),
    ("diameter", XCAFDimTolObjectsDimensionType::SizeDiameter),
    (
        "spherical diameter",
        XCAFDimTolObjectsDimensionType::SizeSphericalDiameter,
    ),
    ("radius", XCAFDimTolObjectsDimensionType::SizeRadius),
    (
        "spherical radius",
        XCAFDimTolObjectsDimensionType::SizeSphericalRadius,
    ),
    (
        "toroidal minor diameter",
        XCAFDimTolObjectsDimensionType::SizeToroidalMinorDiameter,
    ),
    (
        "toroidal major diameter",
        XCAFDimTolObjectsDimensionType::SizeToroidalMajorDiameter,
    ),
    (
        "toroidal minor radius",
        XCAFDimTolObjectsDimensionType::SizeToroidalMinorRadius,
    ),
    (
        "toroidal major radius",
        XCAFDimTolObjectsDimensionType::SizeToroidalMajorRadius,
    ),
    (
        "toroidal high major diameter",
        XCAFDimTolObjectsDimensionType::SizeToroidalHighMajorDiameter,
    ),
    (
        "toroidal low major diameter",
        XCAFDimTolObjectsDimensionType::SizeToroidalLowMajorDiameter,
    ),
    (
        "toroidal high major radius",
        XCAFDimTolObjectsDimensionType::SizeToroidalHighMajorRadius,
    ),
    (
        "toroidal low major radius",
        XCAFDimTolObjectsDimensionType::SizeToroidalLowMajorRadius,
    ),
    ("thickness", XCAFDimTolObjectsDimensionType::SizeThickness),
    (
        "curved distance",
        XCAFDimTolObjectsDimensionType::LocationCurvedDistance,
    ),
    (
        "linear distance",
        XCAFDimTolObjectsDimensionType::LocationLinearDistance,
    ),
    (
        "linear distance centre outer",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromCenterToOuter,
    ),
    (
        "linear distance centre inner",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromCenterToInner,
    ),
    (
        "linear distance outer centre",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToCenter,
    ),
    (
        "linear distance outer outer",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToOuter,
    ),
    (
        "linear distance outer inner",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromOuterToInner,
    ),
    (
        "linear distance inner centre",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToCenter,
    ),
    (
        "linear distance inner outer",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToOuter,
    ),
    (
        "linear distance inner inner",
        XCAFDimTolObjectsDimensionType::LocationLinearDistanceFromInnerToInner,
    ),
];

/// The names of the datum target types.
const DATUM_TARGETS: [(&str, XCAFDimTolObjectsDatumTargetType); 5] = [
    ("area", XCAFDimTolObjectsDatumTargetType::Area),
    ("line", XCAFDimTolObjectsDatumTargetType::Line),
    ("circle", XCAFDimTolObjectsDatumTargetType::Circle),
    ("rectangle", XCAFDimTolObjectsDatumTargetType::Rectangle),
    ("point", XCAFDimTolObjectsDatumTargetType::Point),
];

/// The texts of the simple datum modifiers.
const SINGLE_MODIFIERS: [(XCAFDimTolObjectsDatumSingleModif, &str); 22] = [
    (
        XCAFDimTolObjectsDatumSingleModif::AnyCrossSection,
        ".ANY_CROSS_SECTION.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::AnyLongitudinalSection,
        ".ANY_LONGITUDINAL_SECTION.",
    ),
    (XCAFDimTolObjectsDatumSingleModif::Basic, ".BASIC."),
    (
        XCAFDimTolObjectsDatumSingleModif::ContactingFeature,
        ".CONTACTING_FEATURE.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintU,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_U.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintV,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_V.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintW,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_W.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintX,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_X.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintY,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_Y.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DegreeOfFreedomConstraintZ,
        ".DEGREE_OF_FREEDOM_CONSTRAINT_Z.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::DistanceVariable,
        ".DISTANCE_VARIABLE.",
    ),
    (XCAFDimTolObjectsDatumSingleModif::FreeState, ".FREE_STATE."),
    (
        XCAFDimTolObjectsDatumSingleModif::LeastMaterialRequirement,
        ".LEAST_MATERIAL_REQUIREMENT.",
    ),
    (XCAFDimTolObjectsDatumSingleModif::Line, ".LINE."),
    (
        XCAFDimTolObjectsDatumSingleModif::MajorDiameter,
        ".MAJOR_DIAMETER.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::MaximumMaterialRequirement,
        ".MAXIMUM_MATERIAL_REQUIREMENT.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::MinorDiameter,
        ".MINOR_DIAMETER.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::Orientation,
        ".ORIENTATION.",
    ),
    (
        XCAFDimTolObjectsDatumSingleModif::PitchDiameter,
        ".PITCH_DIAMETER.",
    ),
    (XCAFDimTolObjectsDatumSingleModif::Plane, ".PLANE."),
    (XCAFDimTolObjectsDatumSingleModif::Point, ".POINT."),
    (
        XCAFDimTolObjectsDatumSingleModif::Translation,
        ".TRANSLATION.",
    ),
];

/// The kinds of geometric tolerance of both packages.
const TOLERANCE_TYPES: [(
    XCAFDimTolObjectsGeomToleranceType,
    StepDimTolGeometricToleranceType,
); 15] = [
    (
        XCAFDimTolObjectsGeomToleranceType::Angularity,
        StepDimTolGeometricToleranceType::GTTAngularityTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::CircularRunout,
        StepDimTolGeometricToleranceType::GTTCircularRunoutTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::CircularityOrRoundness,
        StepDimTolGeometricToleranceType::GTTRoundnessTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Coaxiality,
        StepDimTolGeometricToleranceType::GTTCoaxialityTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Concentricity,
        StepDimTolGeometricToleranceType::GTTConcentricityTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Cylindricity,
        StepDimTolGeometricToleranceType::GTTCylindricityTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Flatness,
        StepDimTolGeometricToleranceType::GTTFlatnessTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Parallelism,
        StepDimTolGeometricToleranceType::GTTParallelismTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Perpendicularity,
        StepDimTolGeometricToleranceType::GTTPerpendicularityTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Position,
        StepDimTolGeometricToleranceType::GTTPositionTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::ProfileOfLine,
        StepDimTolGeometricToleranceType::GTTLineProfileTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::ProfileOfSurface,
        StepDimTolGeometricToleranceType::GTTSurfaceProfileTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Straightness,
        StepDimTolGeometricToleranceType::GTTStraightnessTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::Symmetry,
        StepDimTolGeometricToleranceType::GTTSymmetryTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceType::TotalRunout,
        StepDimTolGeometricToleranceType::GTTTotalRunoutTolerance,
    ),
];

/// The tolerance modifiers of both packages.
const TOLERANCE_MODIFIERS: [(
    XCAFDimTolObjectsGeomToleranceModif,
    StepDimTolGeometricToleranceModifier,
); 15] = [
    (
        XCAFDimTolObjectsGeomToleranceModif::AnyCrossSection,
        StepDimTolGeometricToleranceModifier::GTMAnyCrossSection,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::CommonZone,
        StepDimTolGeometricToleranceModifier::GTMCommonZone,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::EachRadialElement,
        StepDimTolGeometricToleranceModifier::GTMEachRadialElement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::FreeState,
        StepDimTolGeometricToleranceModifier::GTMFreeState,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::LeastMaterialRequirement,
        StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::LineElement,
        StepDimTolGeometricToleranceModifier::GTMLineElement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::MajorDiameter,
        StepDimTolGeometricToleranceModifier::GTMMajorDiameter,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::MaximumMaterialRequirement,
        StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::MinorDiameter,
        StepDimTolGeometricToleranceModifier::GTMMinorDiameter,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::NotConvex,
        StepDimTolGeometricToleranceModifier::GTMNotConvex,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::PitchDiameter,
        StepDimTolGeometricToleranceModifier::GTMPitchDiameter,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::ReciprocityRequirement,
        StepDimTolGeometricToleranceModifier::GTMReciprocityRequirement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::SeparateRequirement,
        StepDimTolGeometricToleranceModifier::GTMSeparateRequirement,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::StatisticalTolerance,
        StepDimTolGeometricToleranceModifier::GTMStatisticalTolerance,
    ),
    (
        XCAFDimTolObjectsGeomToleranceModif::TangentPlane,
        StepDimTolGeometricToleranceModifier::GTMTangentPlane,
    ),
];
