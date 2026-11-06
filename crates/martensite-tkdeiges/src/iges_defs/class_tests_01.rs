//! The classes of IgesDefs: what their initialization keeps and their round through a file.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_test_samples::{
    assert_entity_round, assert_entity_round_after, sample_iges_defs_associativity_def,
    sample_iges_defs_attribute_def, sample_iges_defs_attribute_table,
    sample_iges_defs_generic_data, sample_iges_defs_macro_def, sample_iges_defs_tabular_data,
    sample_iges_defs_units_data,
};
use martensite_tkernel::standard::Failure;

#[test]
fn class_associativity_def() -> Result<(), Failure> {
    let object = sample_iges_defs_associativity_def()?;
    assert_entity_round(&object, "IgesDefsAssociativityDef")?;
    Ok(())
}

#[test]
fn class_attribute_def() -> Result<(), Failure> {
    let object = sample_iges_defs_attribute_def()?;
    assert_entity_round(&object, "IgesDefsAttributeDef")?;
    Ok(())
}

#[test]
fn class_attribute_table() -> Result<(), Failure> {
    let object = sample_iges_defs_attribute_table()?;
    assert_entity_round_after(&object.definition(), &object, "IgesDefsAttributeTable")?;
    Ok(())
}

#[test]
fn class_generic_data() -> Result<(), Failure> {
    let object = sample_iges_defs_generic_data()?;
    assert_entity_round(&object, "IgesDefsGenericData")?;
    Ok(())
}

#[test]
fn class_macro_def() -> Result<(), Failure> {
    let object = sample_iges_defs_macro_def()?;
    assert_eq!(object.entity_type_id(), 2);
    assert_entity_round(&object, "IgesDefsMacroDef")?;
    Ok(())
}

#[test]
fn class_tabular_data() -> Result<(), Failure> {
    let object = sample_iges_defs_tabular_data()?;
    assert_eq!(object.nb_property_values(), 13);
    assert_eq!(object.property_type(), 2);
    assert_entity_round(&object, "IgesDefsTabularData")?;
    Ok(())
}

#[test]
fn class_units_data() -> Result<(), Failure> {
    let object = sample_iges_defs_units_data()?;
    assert_entity_round(&object, "IgesDefsUnitsData")?;
    Ok(())
}
