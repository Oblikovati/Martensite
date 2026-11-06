#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::samples_01::{
    sample_iges_basic_assoc_group_type, sample_iges_basic_external_ref_file,
    sample_iges_basic_external_ref_file_index, sample_iges_basic_external_ref_file_name,
    sample_iges_basic_external_ref_lib_name, sample_iges_basic_external_ref_name,
    sample_iges_basic_external_reference_file, sample_iges_basic_group,
    sample_iges_basic_group_without_back_p, sample_iges_basic_hierarchy, sample_iges_basic_name,
    sample_iges_basic_ordered_group, sample_iges_basic_ordered_group_without_back_p,
    sample_iges_basic_single_parent, sample_iges_basic_singular_subfigure,
    sample_iges_basic_subfigure_def,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_test_samples::assert_entity_round;
use martensite_tkernel::standard::{Failure, RuntimeTyped};

#[test]
fn class_iges_basic_assoc_group_type() -> Result<(), Failure> {
    let object = sample_iges_basic_assoc_group_type()?;
    assert_eq!(object.nb_data(), 2);
    assert_eq!(object.assoc_type(), 3);
    assert_eq!((object.type_number(), object.form_number()), (406, 23));
    assert_entity_round(&object, "IgesBasicAssocGroupType")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_ref_file() -> Result<(), Failure> {
    let object = sample_iges_basic_external_ref_file()?;
    assert_eq!((object.type_number(), object.form_number()), (416, 1));
    assert_entity_round(&object, "IgesBasicExternalRefFile")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_ref_file_index() -> Result<(), Failure> {
    let object = sample_iges_basic_external_ref_file_index()?;
    assert_eq!((object.type_number(), object.form_number()), (402, 12));
    assert_entity_round(&object, "IgesBasicExternalRefFileIndex")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_ref_file_name() -> Result<(), Failure> {
    let object = sample_iges_basic_external_ref_file_name()?;
    assert_entity_round(&object, "IgesBasicExternalRefFileName")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_ref_lib_name() -> Result<(), Failure> {
    let object = sample_iges_basic_external_ref_lib_name()?;
    assert_eq!((object.type_number(), object.form_number()), (416, 4));
    assert_entity_round(&object, "IgesBasicExternalRefLibName")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_ref_name() -> Result<(), Failure> {
    let object = sample_iges_basic_external_ref_name()?;
    assert_eq!((object.type_number(), object.form_number()), (416, 3));
    assert_entity_round(&object, "IgesBasicExternalRefName")?;
    Ok(())
}
#[test]
fn class_iges_basic_external_reference_file() -> Result<(), Failure> {
    let object = sample_iges_basic_external_reference_file()?;
    assert_eq!((object.type_number(), object.form_number()), (406, 12));
    assert_entity_round(&object, "IgesBasicExternalReferenceFile")?;
    Ok(())
}
#[test]
fn class_iges_basic_group() -> Result<(), Failure> {
    let object = sample_iges_basic_group()?;
    assert_eq!((object.type_number(), object.form_number()), (402, 1));
    assert_entity_round(&object, "IgesBasicGroup")?;
    Ok(())
}
#[test]
fn class_iges_basic_group_without_back_p() -> Result<(), Failure> {
    let object = sample_iges_basic_group_without_back_p()?;
    assert!(object.is_kind(IgesDataIgesEntity::static_type()));
    Ok(())
}
#[test]
fn class_iges_basic_hierarchy() -> Result<(), Failure> {
    let object = sample_iges_basic_hierarchy()?;
    assert_eq!(object.new_line_font(), 3);
    assert_eq!(object.new_view(), 4);
    assert_eq!(object.new_entity_level(), 5);
    assert_eq!(object.new_blank_status(), 6);
    assert_eq!(object.new_line_weight(), 7);
    assert_eq!(object.new_color_num(), 8);
    assert_eq!((object.type_number(), object.form_number()), (406, 10));
    assert_entity_round(&object, "IgesBasicHierarchy")?;
    Ok(())
}
#[test]
fn class_iges_basic_name() -> Result<(), Failure> {
    let object = sample_iges_basic_name()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!((object.type_number(), object.form_number()), (406, 15));
    assert_entity_round(&object, "IgesBasicName")?;
    Ok(())
}
#[test]
fn class_iges_basic_ordered_group() -> Result<(), Failure> {
    let object = sample_iges_basic_ordered_group()?;
    assert!(object.is_kind(IgesDataIgesEntity::static_type()));
    Ok(())
}
#[test]
fn class_iges_basic_ordered_group_without_back_p() -> Result<(), Failure> {
    let object = sample_iges_basic_ordered_group_without_back_p()?;
    assert!(object.is_kind(IgesDataIgesEntity::static_type()));
    Ok(())
}
#[test]
fn class_iges_basic_single_parent() -> Result<(), Failure> {
    let object = sample_iges_basic_single_parent()?;
    assert_eq!(object.nb_parent_entities(), 2);
    assert_eq!((object.type_number(), object.form_number()), (402, 9));
    assert_entity_round(&object, "IgesBasicSingleParent")?;
    Ok(())
}
#[test]
fn class_iges_basic_singular_subfigure() -> Result<(), Failure> {
    let object = sample_iges_basic_singular_subfigure()?;
    assert_eq!(object.scale_factor(), 4.5);
    assert_eq!((object.type_number(), object.form_number()), (408, 0));
    assert_entity_round(&object, "IgesBasicSingularSubfigure")?;
    Ok(())
}
#[test]
fn class_iges_basic_subfigure_def() -> Result<(), Failure> {
    let object = sample_iges_basic_subfigure_def()?;
    assert_eq!(object.depth(), 2);
    assert_eq!((object.type_number(), object.form_number()), (308, 0));
    assert_entity_round(&object, "IgesBasicSubfigureDef")?;
    Ok(())
}
