//! The AP214 protocol: the case number of every entity type and the schema name of a model.
use super::protocol_index::PROTOCOL_TYPES_PARTS;
use crate::de_step::DeStepWriteModeStepSchema;
use crate::header_section::HeaderSection;
use crate::step_data::{StepDataProtocolOps, StepDataProtocolTables, StepDataStepModel};
use crate::step_entity::*;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::RuntimeType;
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceProtocol};
use std::collections::HashMap;
use std::sync::{LazyLock, OnceLock};

/// An entity type and its case number.
pub(super) type ProtocolRow = (fn() -> &'static RuntimeType, i32);

/// A row of the `types.Bind(STANDARD_TYPE(T), number)`.
macro_rules! protocol_row {
    ($class:ty, $number:literal) => {
        (<$class as RuntimeTyped>::static_type, $number)
    };
}
pub(super) use protocol_row;

/// The address of a runtime type, its identity. Each type has one descriptor for the whole process.
fn type_key(atype: &RuntimeType) -> usize {
    std::ptr::from_ref(atype) as usize
}

static TYPE_NUMBERS: LazyLock<HashMap<usize, i32>> = LazyLock::new(|| {
    let rows = PROTOCOL_TYPES_PARTS.iter().flat_map(|part| part.iter());
    rows.map(|(kind, number)| (type_key(kind()), *number))
        .collect()
});

/// Protocol.
#[derive(Default)]
pub struct StepAP214Protocol {
    transient: StandardTransient,
    tables: StepDataProtocolTables,
}
impl_transient!(
    StepAP214Protocol,
    "StepAP214Protocol",
    dyn StepDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn StepDataProtocolOps]
);
crate::impl_step_protocol!(StepAP214Protocol);

impl StepAP214Protocol {
    /// `StepAP214Protocol()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl StepDataProtocolOps for StepAP214Protocol {
    fn protocol_tables(&self) -> &StepDataProtocolTables {
        &self.tables
    }
    /// `nb_resources`: the header section.
    fn step_nb_resources(&self) -> i32 {
        1
    }
    /// `resource`: the header section, whatever the number.
    fn step_resource(&self, _num: i32) -> Handle<dyn InterfaceProtocol> {
        HeaderSection::protocol().cast()
    }
    /// `type_number`: 0 for a type of no case.
    fn step_type_number(&self, atype: &RuntimeType) -> i32 {
        TYPE_NUMBERS.get(&type_key(atype)).copied().unwrap_or(0)
    }
    /// `schema_name`: by the write schema of a step model, else the AP214 IS one.
    fn schema_name(&self, model: &Handle<dyn InterfaceInterfaceModel>) -> String {
        let step_model = model.cast::<StepDataStepModel>();
        let schema = step_model
            .get()
            .map_or(DeStepWriteModeStepSchema::AP214_IS, |live| {
                live.internal_parameters().write_schema
            });
        match schema {
            DeStepWriteModeStepSchema::AP203 => "CONFIG_CONTROL_DESIGN",
            DeStepWriteModeStepSchema::AP214_IS => "AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }",
            DeStepWriteModeStepSchema::AP242_DIS => {
                "AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF {1 0 10303 442 1 1 4 }"
            }
            DeStepWriteModeStepSchema::AP214_CD => {
                "AUTOMOTIVE_DESIGN_CC2 { 1 2 10303 214 -1 1 5 4 }"
            }
            _ => "AUTOMOTIVE_DESIGN { 1 2 10303 214 0 1 1 1 }",
        }
        .to_owned()
    }
}

/// `StepAP214`: gives the shared protocol.
#[derive(Debug, Clone, Copy, Default)]
pub struct StepAP214;

impl StepAP214 {
    /// `Protocol`: the one protocol of the process.
    ///
    /// ```
    /// use martensite_tkdestep::step_ap214::StepAP214;
    /// assert!(StepAP214::protocol().is_some());
    /// ```
    pub fn protocol() -> Handle<StepAP214Protocol> {
        static PROTOCOL: OnceLock<Handle<StepAP214Protocol>> = OnceLock::new();
        PROTOCOL
            .get_or_init(|| Handle::new(StepAP214Protocol::new()))
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_basic::StepBasicAddress;
    use crate::step_geom::StepGeomCartesianPoint;

    #[test]
    fn type_numbers_are_the_ones() {
        let protocol = StepAP214Protocol::new();
        assert_eq!(
            protocol.step_type_number(StepBasicAddress::static_type()),
            1
        );
        assert_eq!(
            protocol.step_type_number(StepGeomCartesianPoint::static_type()),
            59
        );
        assert_eq!(
            protocol.step_type_number(StandardTransient::static_type()),
            0
        );
    }

    #[test]
    fn schema_defaults_to_ap214_is_without_a_step_model() {
        let protocol = StepAP214Protocol::new();
        assert_eq!(
            protocol.schema_name(&Handle::default()),
            "AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }"
        );
    }

    #[test]
    fn schema_follows_the_write_mode_of_a_step_model() {
        let protocol = StepAP214Protocol::new();
        let model = StepDataStepModel::new();
        model.internal_parameters_mut().write_schema = DeStepWriteModeStepSchema::AP203;
        assert_eq!(protocol.schema_name(&model.cast()), "CONFIG_CONTROL_DESIGN");
    }

    #[test]
    fn the_package_gives_one_protocol_with_one_resource() {
        let first = StepAP214::protocol();
        assert_eq!(
            first.object_address(),
            StepAP214::protocol().object_address()
        );
        assert_eq!(first.get().unwrap().step_nb_resources(), 1);
        assert!(first.get().unwrap().step_resource(1).is_some());
    }
}
