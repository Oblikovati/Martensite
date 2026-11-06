#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;

/// Area unit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolAreaUnitType {
    /// Circular.
    #[default]
    Circular,
    /// Rectangular.
    Rectangular,
    /// Square.
    Square,
}

/// Datum reference modifier type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolDatumReferenceModifierType {
    /// Circular or cylindrical.
    #[default]
    CircularOrCylindrical,
    /// Distance.
    Distance,
    /// Projected.
    Projected,
    /// Spherical.
    Spherical,
}

/// Geometric tolerance modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolGeometricToleranceModifier {
    /// GTM any cross section.
    #[default]
    GTMAnyCrossSection,
    /// GTM common zone.
    GTMCommonZone,
    /// GTM each radial element.
    GTMEachRadialElement,
    /// GTM free state.
    GTMFreeState,
    /// GTM least material requirement.
    GTMLeastMaterialRequirement,
    /// GTM line element.
    GTMLineElement,
    /// GTM major diameter.
    GTMMajorDiameter,
    /// GTM maximum material requirement.
    GTMMaximumMaterialRequirement,
    /// GTM minor diameter.
    GTMMinorDiameter,
    /// GTM not convex.
    GTMNotConvex,
    /// GTM pitch diameter.
    GTMPitchDiameter,
    /// GTM reciprocity requirement.
    GTMReciprocityRequirement,
    /// GTM separate requirement.
    GTMSeparateRequirement,
    /// GTM statistical tolerance.
    GTMStatisticalTolerance,
    /// GTM tangent plane.
    GTMTangentPlane,
}

/// Geometric tolerance type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolGeometricToleranceType {
    /// GTT angularity tolerance.
    #[default]
    GTTAngularityTolerance,
    /// GTT circular runout tolerance.
    GTTCircularRunoutTolerance,
    /// GTT coaxiality tolerance.
    GTTCoaxialityTolerance,
    /// GTT concentricity tolerance.
    GTTConcentricityTolerance,
    /// GTT cylindricity tolerance.
    GTTCylindricityTolerance,
    /// GTT flatness tolerance.
    GTTFlatnessTolerance,
    /// GTT line profile tolerance.
    GTTLineProfileTolerance,
    /// GTT parallelism tolerance.
    GTTParallelismTolerance,
    /// GTT perpendicularity tolerance.
    GTTPerpendicularityTolerance,
    /// GTT position tolerance.
    GTTPositionTolerance,
    /// GTT roundness tolerance.
    GTTRoundnessTolerance,
    /// GTT straightness tolerance.
    GTTStraightnessTolerance,
    /// GTT surface profile tolerance.
    GTTSurfaceProfileTolerance,
    /// GTT symmetry tolerance.
    GTTSymmetryTolerance,
    /// GTT total runout tolerance.
    GTTTotalRunoutTolerance,
}

/// Limit condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolLimitCondition {
    /// Maximum material condition.
    #[default]
    MaximumMaterialCondition,
    /// Least material condition.
    LeastMaterialCondition,
    /// Regardless of feature size.
    RegardlessOfFeatureSize,
}

/// Simple datum reference modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepDimTolSimpleDatumReferenceModifier {
    /// SDRM any cross section.
    #[default]
    SDRMAnyCrossSection,
    /// SDRM any longitudinal section.
    SDRMAnyLongitudinalSection,
    /// SDRM basic.
    SDRMBasic,
    /// SDRM contacting feature.
    SDRMContactingFeature,
    /// SDRM degree of freedom constraint U.
    SDRMDegreeOfFreedomConstraintU,
    /// SDRM degree of freedom constraint V.
    SDRMDegreeOfFreedomConstraintV,
    /// SDRM degree of freedom constraint W.
    SDRMDegreeOfFreedomConstraintW,
    /// SDRM degree of freedom constraint X.
    SDRMDegreeOfFreedomConstraintX,
    /// SDRM degree of freedom constraint Y.
    SDRMDegreeOfFreedomConstraintY,
    /// SDRM degree of freedom constraint Z.
    SDRMDegreeOfFreedomConstraintZ,
    /// SDRM distance variable.
    SDRMDistanceVariable,
    /// SDRM free state.
    SDRMFreeState,
    /// SDRM least material requirement.
    SDRMLeastMaterialRequirement,
    /// SDRM line.
    SDRMLine,
    /// SDRM major diameter.
    SDRMMajorDiameter,
    /// SDRM maximum material requirement.
    SDRMMaximumMaterialRequirement,
    /// SDRM minor diameter.
    SDRMMinorDiameter,
    /// SDRM orientation.
    SDRMOrientation,
    /// SDRM pitch diameter.
    SDRMPitchDiameter,
    /// SDRM plane.
    SDRMPlane,
    /// SDRM point.
    SDRMPoint,
    /// SDRM translation.
    SDRMTranslation,
}

step_select! {
    /// Datum or common datum.
    StepDimTolDatumOrCommonDatum, [StepDimTolDatum, HArray1<Handle<StepDimTolDatumReferenceElement>>], {
        datum: StepDimTolDatum,
        common_datum_list: HArray1<Handle<StepDimTolDatumReferenceElement>>,
    }
}

step_select! {
    /// Datum reference modifier.
    StepDimTolDatumReferenceModifier, [StepDimTolDatumReferenceModifierWithValue, StepDimTolSimpleDatumReferenceModifierMember], {
        datum_reference_modifier_with_value: StepDimTolDatumReferenceModifierWithValue,
        simple_datum_reference_modifier_member: StepDimTolSimpleDatumReferenceModifierMember,
    }
}

step_select! {
    /// Datum system or reference.
    StepDimTolDatumSystemOrReference, [StepDimTolDatumSystem, StepDimTolDatumReference], {
        datum_system: StepDimTolDatumSystem,
        datum_reference: StepDimTolDatumReference,
    }
}

step_select! {
    /// Geometric tolerance target.
    StepDimTolGeometricToleranceTarget, [StepShapeDimensionalLocation, StepShapeDimensionalSize, StepReprProductDefinitionShape, StepReprShapeAspect], {
        dimensional_location: StepShapeDimensionalLocation,
        dimensional_size: StepShapeDimensionalSize,
        product_definition_shape: StepReprProductDefinitionShape,
        shape_aspect: StepReprShapeAspect,
    }
}

step_select! {
    /// Tolerance zone target.
    StepDimTolToleranceZoneTarget, [StepShapeDimensionalLocation, StepShapeDimensionalSize, StepDimTolGeometricTolerance, StepDimTolGeneralDatumReference], {
        dimensional_location: StepShapeDimensionalLocation,
        dimensional_size: StepShapeDimensionalSize,
        geometric_tolerance: StepDimTolGeometricTolerance,
        general_datum_reference: StepDimTolGeneralDatumReference,
    }
}

step_entity! {
    /// Representation of STEP entity AngularityTolerance
    StepDimTolAngularityTolerance, StepDimTolAngularityToleranceState, "StepDimTolAngularityTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity CircularRunoutTolerance
    StepDimTolCircularRunoutTolerance, StepDimTolCircularRunoutToleranceState, "StepDimTolCircularRunoutTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity CoaxialityTolerance
    StepDimTolCoaxialityTolerance, StepDimTolCoaxialityToleranceState, "StepDimTolCoaxialityTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity CommonDatum
    StepDimTolCommonDatum, StepDimTolCommonDatumState, "StepDimTolCommonDatum", StepReprCompositeShapeAspect, {
        my_datum: Handle<StepDimTolDatum>,
    }
}
impl StepDimTolCommonDatum {
    step_methods! {
        get datum = my_datum: Handle<StepDimTolDatum>;
        set set_datum = my_datum: Handle<StepDimTolDatum>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_shape_aspect_name: &Handle<HAsciiString>,
        the_shape_aspect_description: &Handle<HAsciiString>,
        the_shape_aspect_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_shape_aspect_product_definitional: StepDataLogical,
        the_datum_name: &Handle<HAsciiString>,
        the_datum_description: &Handle<HAsciiString>,
        the_datum_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_datum_product_definitional: StepDataLogical,
        the_datum_identification: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            the_shape_aspect_name,
            the_shape_aspect_description,
            the_shape_aspect_of_shape,
            the_shape_aspect_product_definitional,
        );
        let mut state = self.state.write();
        if state.my_datum.is_null() {
            state.my_datum = Handle::new(StepDimTolDatum::new());
        }
        drop(state);
        let field_my_datum = self.state.read().my_datum.clone();
        field_my_datum.init(
            the_datum_name,
            the_datum_description,
            the_datum_of_shape,
            the_datum_product_definitional,
            the_datum_identification,
        );
    }
}

step_entity! {
    /// Representation of STEP entity ConcentricityTolerance
    StepDimTolConcentricityTolerance, StepDimTolConcentricityToleranceState, "StepDimTolConcentricityTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity CylindricityTolerance
    StepDimTolCylindricityTolerance, StepDimTolCylindricityToleranceState, "StepDimTolCylindricityTolerance", StepDimTolGeometricTolerance, {
    }
}
