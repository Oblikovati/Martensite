#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

/// Ahead or behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepBasicAheadOrBehind {
    /// Ahead.
    #[default]
    AobAhead,
    /// Exact.
    AobExact,
    /// Behind.
    AobBehind,
}

/// Si prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepBasicSiPrefix {
    /// Exa.
    #[default]
    SpExa,
    /// Peta.
    SpPeta,
    /// Tera.
    SpTera,
    /// Giga.
    SpGiga,
    /// Mega.
    SpMega,
    /// Kilo.
    SpKilo,
    /// Hecto.
    SpHecto,
    /// Deca.
    SpDeca,
    /// Deci.
    SpDeci,
    /// Centi.
    SpCenti,
    /// Milli.
    SpMilli,
    /// Micro.
    SpMicro,
    /// Nano.
    SpNano,
    /// Pico.
    SpPico,
    /// Femto.
    SpFemto,
    /// Atto.
    SpAtto,
}

/// Si unit name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepBasicSiUnitName {
    /// Metre.
    #[default]
    SunMetre,
    /// Gram.
    SunGram,
    /// Second.
    SunSecond,
    /// Ampere.
    SunAmpere,
    /// Kelvin.
    SunKelvin,
    /// Mole.
    SunMole,
    /// Candela.
    SunCandela,
    /// Radian.
    SunRadian,
    /// Steradian.
    SunSteradian,
    /// Hertz.
    SunHertz,
    /// Newton.
    SunNewton,
    /// Pascal.
    SunPascal,
    /// Joule.
    SunJoule,
    /// Watt.
    SunWatt,
    /// Coulomb.
    SunCoulomb,
    /// Volt.
    SunVolt,
    /// Farad.
    SunFarad,
    /// Ohm.
    SunOhm,
    /// Siemens.
    SunSiemens,
    /// Weber.
    SunWeber,
    /// Tesla.
    SunTesla,
    /// Henry.
    SunHenry,
    /// Degree celsius.
    SunDegreeCelsius,
    /// Lumen.
    SunLumen,
    /// Lux.
    SunLux,
    /// Becquerel.
    SunBecquerel,
    /// Gray.
    SunGray,
    /// Sievert.
    SunSievert,
}

/// Source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepBasicSource {
    /// Made.
    #[default]
    SMade,
    /// Bought.
    SBought,
    /// Not known.
    SNotKnown,
}

step_select! {
    /// Date time select.
    StepBasicDateTimeSelect, [StepBasicDate, StepBasicLocalTime, StepBasicDateAndTime], {
        date: StepBasicDate,
        local_time: StepBasicLocalTime,
        date_and_time: StepBasicDateAndTime,
    }
}

step_select! {
    /// Person organization select.
    StepBasicPersonOrganizationSelect, [StepBasicPerson, StepBasicOrganization, StepBasicPersonAndOrganization], {
        person: StepBasicPerson,
        organization: StepBasicOrganization,
        person_and_organization: StepBasicPersonAndOrganization,
    }
}

step_select! {
    /// Product definition or reference.
    StepBasicProductDefinitionOrReference, [StepBasicProductDefinition, StepBasicProductDefinitionReference, StepBasicProductDefinitionReferenceWithLocalRepresentation], {
        product_definition: StepBasicProductDefinition,
        product_definition_reference: StepBasicProductDefinitionReference,
        product_definition_reference_with_local_representation: StepBasicProductDefinitionReferenceWithLocalRepresentation,
    }
}

step_select! {
    /// Representation of STEP SELECT type ProductOrFormationOrDefinition
    StepBasicProductOrFormationOrDefinition, [StepBasicProduct, StepBasicProductDefinitionFormation, StepBasicProductDefinition], {
        product: StepBasicProduct,
        product_definition_formation: StepBasicProductDefinitionFormation,
        product_definition: StepBasicProductDefinition,
    }
}

step_select! {
    /// Representation of STEP SELECT type RoleSelect
    StepBasicRoleSelect, [StepBasicActionAssignment, StepBasicActionRequestAssignment, StepBasicApprovalAssignment, StepBasicApprovalDateTime, StepBasicCertificationAssignment, StepBasicContractAssignment, StepBasicDocumentReference, StepBasicEffectivityAssignment, StepBasicGroupAssignment, StepBasicNameAssignment, StepBasicSecurityClassificationAssignment], {
        action_assignment: StepBasicActionAssignment,
        action_request_assignment: StepBasicActionRequestAssignment,
        approval_assignment: StepBasicApprovalAssignment,
        approval_date_time: StepBasicApprovalDateTime,
        certification_assignment: StepBasicCertificationAssignment,
        contract_assignment: StepBasicContractAssignment,
        document_reference: StepBasicDocumentReference,
        effectivity_assignment: StepBasicEffectivityAssignment,
        group_assignment: StepBasicGroupAssignment,
        name_assignment: StepBasicNameAssignment,
        security_classification_assignment: StepBasicSecurityClassificationAssignment,
    }
}

step_select! {
    /// Implements a select type unit (NamedUnit or DerivedUnit)
    StepBasicUnit, [StepBasicNamedUnit, StepBasicDerivedUnit], {
        named_unit: StepBasicNamedUnit,
        derived_unit: StepBasicDerivedUnit,
    }
}

step_entity! {
    /// Representation of STEP entity Action
    StepBasicAction, StepBasicActionState, "StepBasicAction", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_chosen_method: Handle<StepBasicActionMethod>,
        def_description: bool,
    }
}
impl StepBasicAction {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get chosen_method = the_chosen_method: Handle<StepBasicActionMethod>;
        set set_chosen_method = the_chosen_method: Handle<StepBasicActionMethod>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_chosen_method: &Handle<StepBasicActionMethod>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_chosen_method = a_chosen_method.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ActionAssignment
    StepBasicActionAssignment, StepBasicActionAssignmentState, "StepBasicActionAssignment", StandardTransient, {
        the_assigned_action: Handle<StepBasicAction>,
    }
}
impl StepBasicActionAssignment {
    step_methods! {
        set init = the_assigned_action: Handle<StepBasicAction>;
        get assigned_action = the_assigned_action: Handle<StepBasicAction>;
        set set_assigned_action = the_assigned_action: Handle<StepBasicAction>;
    }
}
