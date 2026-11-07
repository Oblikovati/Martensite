//! The forms of an attribute definition: without values, with values, and with values and templates.
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_new_array1};
use crate::iges_defs::{
    IgesDefsAttributeDef, IgesDefsGenericData, IgesDefsHArray1OfHArray1OfTextDisplayTemplate,
};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::iges_test_samples::{
    assert_entity_round, sample_iges_graph_color, sample_iges_graph_text_display_template,
    sample_list,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;

fn values() -> Result<Handle<HArray1<Handle<StandardTransient>>>, Failure> {
    sample_list::<Handle<StandardTransient>>([
        sample_list::<i32>([3, 4])?.cast(),
        sample_list::<f64>([1.5, 2.5])?.cast(),
    ])
}

fn definition(
    values: &Handle<HArray1<Handle<StandardTransient>>>,
    templates: &Handle<IgesDefsHArray1OfHArray1OfTextDisplayTemplate>,
) -> Result<Handle<IgesDefsAttributeDef>, Failure> {
    let object = Handle::new(IgesDefsAttributeDef::new());
    object.init(
        &Handle::new(HAsciiString::new("attributes")),
        2,
        &sample_list::<i32>([1, 2])?,
        &sample_list::<i32>([1, 2])?,
        &sample_list::<i32>([2, 2])?,
        values,
        templates,
    )?;
    Ok(object)
}

/// A value of every data type: none, integer, real, text, entity, none, logical.
fn values_of_every_type() -> Result<Handle<HArray1<Handle<StandardTransient>>>, Failure> {
    let text = Handle::new(HAsciiString::new("value"));
    let entity: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let list = iges_new_array1::<Handle<StandardTransient>>(1, 7)?;
    let set =
        |rank: i32, value: Handle<StandardTransient>| iges_array1_set(&list, rank, value, "sample");
    set(2, sample_list::<i32>([3, 4])?.cast())?;
    set(3, sample_list::<f64>([1.5, 2.5])?.cast())?;
    set(
        4,
        sample_list::<Handle<HAsciiString>>([text.clone(), text])?.cast(),
    )?;
    set(
        5,
        sample_list::<Handle<IgesDataIgesEntity>>([entity.clone(), entity])?.cast(),
    )?;
    set(7, sample_list::<i32>([0, 1])?.cast())?;
    Ok(list)
}

fn every_type() -> Result<Handle<HArray1<i32>>, Failure> {
    let list = iges_new_array1::<i32>(1, 7)?;
    for rank in 1..=7 {
        iges_array1_set(&list, rank, rank - 1, "sample")?;
    }
    Ok(list)
}

#[test]
fn a_definition_holds_every_data_type() -> Result<(), Failure> {
    let object = Handle::new(IgesDefsAttributeDef::new());
    let counts = iges_new_array1::<i32>(1, 7)?;
    for rank in 1..=7 {
        iges_array1_set(&counts, rank, 2, "sample")?;
    }
    object.init(
        &Handle::new(HAsciiString::new("attributes")),
        2,
        &every_type()?,
        &every_type()?,
        &counts,
        &values_of_every_type()?,
        &Handle::default(),
    )?;
    assert_eq!(
        object
            .attribute_as_string(4, 1)?
            .get()
            .map(|text| text.length()),
        Some(5)
    );
    assert!(object.attribute_as_entity(5, 2)?.is_some());
    assert!(object.attribute_as_logical(7, 2)?);
    assert_entity_round(&object, "every type")
}

#[test]
fn generic_data_holds_every_data_type() -> Result<(), Failure> {
    let object = Handle::new(IgesDefsGenericData::new());
    // Unlike an attribute's, a text or an entity of generic data is the value itself, not a list.
    let values = values_of_every_type()?;
    iges_array1_set(
        &values,
        4,
        Handle::new(HAsciiString::new("value")).cast(),
        "sample",
    )?;
    let entity: Handle<StandardTransient> = sample_iges_graph_color()?.cast();
    iges_array1_set(&values, 5, entity, "sample")?;
    object.init(
        16,
        &Handle::new(HAsciiString::new("generic")),
        &every_type()?,
        &values,
    )?;
    assert_eq!(object.type_(2)?, 1);
    assert_eq!(object.value_as_integer(2)?, 3);
    assert_eq!(object.value_as_real(3)?, 1.5);
    assert!(object.value_as_string(4)?.is_some());
    assert!(object.value_as_entity(5)?.is_some());
    assert!(!object.value_as_logical(7)?);
    assert_entity_round(&object, "every type")
}

#[test]
fn form_0_keeps_only_the_types() -> Result<(), Failure> {
    let object = definition(&Handle::default(), &Handle::default())?;
    assert_eq!(object.form_number(), 0);
    assert!(!object.has_values());
    assert_eq!(object.attribute_value_data_type(2)?, 2);
    assert_entity_round(&object, "form 0")
}

#[test]
fn form_1_keeps_the_values() -> Result<(), Failure> {
    let object = definition(&values()?, &Handle::default())?;
    assert_eq!(object.form_number(), 1);
    assert_eq!(object.attribute_as_integer(1, 2)?, 4);
    assert_eq!(object.attribute_as_real(2, 1)?, 1.5);
    assert_entity_round(&object, "form 1")
}

#[test]
fn form_2_keeps_the_values_and_their_templates() -> Result<(), Failure> {
    let templates = Handle::new(IgesDefsHArray1OfHArray1OfTextDisplayTemplate::with_bounds(
        1, 2,
    )?);
    for rank in 1..=2 {
        let list = sample_list::<Handle<IgesGraphTextDisplayTemplate>>([
            sample_iges_graph_text_display_template()?,
            sample_iges_graph_text_display_template()?,
        ])?;
        templates.set_value(rank, &list)?;
    }
    let object = definition(&values()?, &templates)?;
    assert_eq!(object.form_number(), 2);
    assert!(object.has_text_display());
    assert!(object.attribute_text_display(1, 1)?.is_some());
    assert_entity_round(&object, "form 2")
}
