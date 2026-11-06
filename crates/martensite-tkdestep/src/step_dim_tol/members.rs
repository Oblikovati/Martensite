//! The select member of `StepDimTol`, and the members of
//! `StepDimTolGeometricToleranceWithDatumReference` that convert its two forms of datum system. A null array is the null dereference.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_data::StepDataSelectType;
use crate::step_entity::*;
use crate::step_members::step_enum_member;
use crate::step_repr::StepReprShapeAspect;

step_enum_member!(
    /// `StepDimTolSimpleDatumReferenceModifierMember`: a datum modifier, of kind enumeration.
    StepDimTolSimpleDatumReferenceModifierMember,
    "StepDimTolSimpleDatumReferenceModifierMember",
    StepDimTolSimpleDatumReferenceModifier,
    "SIMPLE_DATUM_REFERENCE_MODIFIER",
    kind: Some(4),
    [
        SDRMAnyCrossSection = ".ANY_CROSS_SECTION.",
        SDRMAnyLongitudinalSection = ".ANY_LONGITUDINAL_SECTION.",
        SDRMBasic = ".BASIC.",
        SDRMContactingFeature = ".CONTACTING_FEATURE.",
        SDRMDegreeOfFreedomConstraintU = ".DEGREE_OF_FREEDOM_CONSTRAINT_U.",
        SDRMDegreeOfFreedomConstraintV = ".DEGREE_OF_FREEDOM_CONSTRAINT_V.",
        SDRMDegreeOfFreedomConstraintW = ".DEGREE_OF_FREEDOM_CONSTRAINT_W.",
        SDRMDegreeOfFreedomConstraintX = ".DEGREE_OF_FREEDOM_CONSTRAINT_X.",
        SDRMDegreeOfFreedomConstraintY = ".DEGREE_OF_FREEDOM_CONSTRAINT_Y.",
        SDRMDegreeOfFreedomConstraintZ = ".DEGREE_OF_FREEDOM_CONSTRAINT_Z.",
        SDRMDistanceVariable = ".DISTANCE_VARIABLE.",
        SDRMFreeState = ".FREE_STATE.",
        SDRMLeastMaterialRequirement = ".LEAST_MATERIAL_REQUIREMENT.",
        SDRMLine = ".LINE.",
        SDRMMajorDiameter = ".MAJOR_DIAMETER.",
        SDRMMaximumMaterialRequirement = ".MAXIMUM_MATERIAL_REQUIREMENT.",
        SDRMMinorDiameter = ".MINOR_DIAMETER.",
        SDRMOrientation = ".ORIENTATION.",
        SDRMPitchDiameter = ".PITCH_DIAMETER.",
        SDRMPlane = ".PLANE.",
        SDRMPoint = ".POINT.",
        SDRMTranslation = ".TRANSLATION.",
    ]
);

/// The array of `AP242` datum systems holding each datum reference, over the same bounds.
fn datum_systems_of(
    references: &Handle<HArray1<Handle<StepDimTolDatumReference>>>,
    where_called: &str,
) -> Result<Handle<HArray1<StepDimTolDatumSystemOrReference>>, Failure> {
    let references = step_live(references, where_called)?.array1();
    let (lower, upper) = (references.lower(), references.upper());
    let systems =
        HArray1::with_bounds(lower, upper).map_err(|error| error.into_failure(where_called))?;
    for rank in lower..=upper {
        let mut system = StepDimTolDatumSystemOrReference::new();
        let reference = references
            .value(rank)
            .map_err(|error| error.into_failure(where_called))?;
        system.set_value(&reference.cast())?;
        systems
            .change_array1()
            .set_value(rank, system)
            .map_err(|error| error.into_failure(where_called))?;
    }
    Ok(Handle::new(systems))
}

impl StepDimTolGeometricToleranceWithDatumReference {
    /// `init` (AP214): all fields, the datum references converted to datum systems.
    ///
    /// # Errors
    /// a null object failure for a null array; a type mismatch failure from the select.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_datum_system: &Handle<HArray1<Handle<StepDimTolDatumReference>>>,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        )?;
        self.state.write().my_datum_system = datum_systems_of(the_datum_system, "Init")?;
        Ok(())
    }
    /// `init` (AP242): all fields.
    ///
    /// # Errors
    /// None; the form follows the AP214 overload, which can fail.
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_datum_system: &Handle<HArray1<StepDimTolDatumSystemOrReference>>,
    ) -> Result<(), Failure> {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        );
        self.state.write().my_datum_system = the_datum_system.clone();
        Ok(())
    }
    /// `datum_system` (AP214): a new array of the datum references, null where another kind is held.
    ///
    /// # Errors
    /// a null object failure for a null array.
    pub fn datum_system(
        &self,
    ) -> Result<Handle<HArray1<Handle<StepDimTolDatumReference>>>, Failure> {
        let systems = self.state.read().my_datum_system.clone();
        let systems = step_live(&systems, "DatumSystem")?.array1();
        let (lower, upper) = (systems.lower(), systems.upper());
        let references = HArray1::with_bounds(lower, upper)
            .map_err(|error| error.into_failure("DatumSystem"))?;
        for rank in lower..=upper {
            let system = systems
                .value(rank)
                .map_err(|error| error.into_failure("DatumSystem"))?;
            references
                .change_array1()
                .set_value(rank, system.datum_reference())
                .map_err(|error| error.into_failure("DatumSystem"))?;
        }
        Ok(Handle::new(references))
    }
    /// `set_datum_system` (AP214): the datum references converted to datum systems.
    ///
    /// # Errors
    /// a null object failure for a null array; a type mismatch failure from the select.
    pub fn set_datum_system(
        &self,
        the_datum_system: &Handle<HArray1<Handle<StepDimTolDatumReference>>>,
    ) -> Result<(), Failure> {
        self.state.write().my_datum_system = datum_systems_of(the_datum_system, "SetDatumSystem")?;
        Ok(())
    }
    /// `set_datum_system` (AP242).
    ///
    /// # Errors
    /// None; the form follows the AP214 overload, which can fail.
    pub fn set_datum_system_fields(
        &self,
        the_datum_system: &Handle<HArray1<StepDimTolDatumSystemOrReference>>,
    ) -> Result<(), Failure> {
        self.state.write().my_datum_system = the_datum_system.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_data::StepDataSelectMember;

    fn one_reference() -> (
        Handle<StepDimTolDatumReference>,
        Handle<HArray1<Handle<StepDimTolDatumReference>>>,
    ) {
        let reference = Handle::new(StepDimTolDatumReference::new());
        let array = HArray1::with_value(1, 1, reference.clone()).unwrap();
        (reference, Handle::new(array))
    }

    #[test]
    fn datum_system_round_trips_through_its_ap242_form() {
        let tolerance = StepDimTolGeometricToleranceWithDatumReference::new();
        let (reference, array) = one_reference();
        tolerance.set_datum_system(&array).unwrap();
        let systems = tolerance.datum_system_ap242();
        assert_eq!(systems.get().unwrap().array1().length(), 1);
        let back = tolerance.datum_system().unwrap();
        let first = back.get().unwrap().array1().value(1).unwrap().clone();
        assert_eq!(first.object_address(), reference.object_address());
    }

    #[test]
    fn null_datum_arrays_are_the_null_dereference() {
        let tolerance = StepDimTolGeometricToleranceWithDatumReference::new();
        assert!(tolerance.datum_system().is_err());
        assert!(tolerance.set_datum_system(&Handle::default()).is_err());
        let empty = Handle::default();
        assert!(
            tolerance
                .init(
                    &empty,
                    &empty,
                    &Handle::default(),
                    &Handle::default(),
                    &Handle::default()
                )
                .is_err()
        );
    }

    #[test]
    fn modifier_member_round_trips_the_last_enumerator_and_its_text() {
        let member = StepDimTolSimpleDatumReferenceModifierMember::new();
        member.set_value(StepDimTolSimpleDatumReferenceModifier::SDRMTranslation);
        assert_eq!(member.enum_text().as_bytes(), b".TRANSLATION.");
        member.set_enum_text(0, b".BASIC.");
        assert_eq!(
            member.value(),
            StepDimTolSimpleDatumReferenceModifier::SDRMBasic
        );
        assert_eq!(member.kind(), 4);
    }
}
