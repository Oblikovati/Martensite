//! Sample entities, each set up by its initialization.
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
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal};
use crate::iges_data::{IgesDataIgesEntity, iges_array2_set, iges_new_array2};
use crate::iges_defs::{
    IgesDefsAssociativityDef, IgesDefsAttributeDef, IgesDefsAttributeTable, IgesDefsGenericData,
    IgesDefsMacroDef, IgesDefsTabularData, IgesDefsUnitsData,
};
use crate::iges_dimen::{
    IgesDimenAngularDimension, IgesDimenBasicDimension, IgesDimenCenterLine,
    IgesDimenCurveDimension, IgesDimenDiameterDimension, IgesDimenDimensionDisplayData,
    IgesDimenDimensionTolerance, IgesDimenDimensionUnits, IgesDimenDimensionedGeometry,
    IgesDimenFlagNote, IgesDimenGeneralLabel, IgesDimenGeneralNote, IgesDimenGeneralSymbol,
    IgesDimenLeaderArrow, IgesDimenLinearDimension, IgesDimenWitnessLine,
};
use crate::iges_graph::IgesGraphTextFontDef;
use crate::iges_test_samples::{
    h_array1_of_h_array1_of_integer_sample, h_array1_of_h_array1_of_real_sample,
    sample_iges_dimen_witness_line, sample_iges_graph_color, sample_iges_graph_text_font_def,
    sample_list,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Xy, Xyz};

pub(crate) fn sample_iges_defs_associativity_def()
-> Result<Handle<IgesDefsAssociativityDef>, Failure> {
    let object = Handle::new(IgesDefsAssociativityDef::new());
    let arg_0: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_1: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_2: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_3: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    object.init(&arg_0, &arg_1, &arg_2, &arg_3)?;
    Ok(object)
}

pub(crate) fn sample_iges_defs_attribute_def() -> Result<Handle<IgesDefsAttributeDef>, Failure> {
    let object = Handle::new(IgesDefsAttributeDef::new());
    let values = sample_list::<Handle<StandardTransient>>([
        sample_list::<i32>([3, 4])?.cast(),
        sample_list::<f64>([1.5, 2.5])?.cast(),
    ])?;
    object.init(
        &Handle::new(HAsciiString::new("attributes")),
        2,
        &sample_list::<i32>([1, 2])?,
        &sample_list::<i32>([1, 2])?,
        &sample_list::<i32>([2, 2])?,
        &values,
        &Handle::default(),
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_defs_attribute_table() -> Result<Handle<IgesDefsAttributeTable>, Failure>
{
    let object = Handle::new(IgesDefsAttributeTable::new());
    let definition = sample_iges_defs_attribute_def()?;
    let attributes = iges_new_array2::<Handle<StandardTransient>>(1, 2, 1, 1)?;
    iges_array2_set(
        &attributes,
        1,
        1,
        sample_list::<i32>([3, 4])?.cast(),
        "sample",
    )?;
    iges_array2_set(
        &attributes,
        2,
        1,
        sample_list::<f64>([1.5, 2.5])?.cast(),
        "sample",
    )?;
    object.init(&attributes)?;
    object.init_misc(&definition.cast(), &Handle::default(), 0);
    Ok(object)
}

pub(crate) fn sample_iges_defs_generic_data() -> Result<Handle<IgesDefsGenericData>, Failure> {
    let object = Handle::new(IgesDefsGenericData::new());
    let values = sample_list::<Handle<StandardTransient>>([
        sample_list::<i32>([3, 4])?.cast(),
        sample_list::<f64>([1.5, 2.5])?.cast(),
    ])?;
    object.init(
        6,
        &Handle::new(HAsciiString::new("generic")),
        &sample_list::<i32>([1, 2])?,
        &values,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_defs_macro_def() -> Result<Handle<IgesDefsMacroDef>, Failure> {
    let object = Handle::new(IgesDefsMacroDef::new());
    let arg_0: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample0"));
    let arg_1: i32 = 2;
    let arg_2: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample2")),
        Handle::new(HAsciiString::new("sample3")),
    ])?;
    let arg_3: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample3"));
    object.init(&arg_0, arg_1, &arg_2, &arg_3)?;
    Ok(object)
}

pub(crate) fn sample_iges_defs_tabular_data() -> Result<Handle<IgesDefsTabularData>, Failure> {
    let object = Handle::new(IgesDefsTabularData::new());
    let arg_0: i32 = 13;
    let arg_1: i32 = 2;
    let arg_2: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<IgesBasicHArray1OfHArray1OfReal> = h_array1_of_h_array1_of_real_sample()?;
    let arg_5: Handle<IgesBasicHArray1OfHArray1OfReal> = h_array1_of_h_array1_of_real_sample()?;
    object.init(arg_0, arg_1, &arg_2, &arg_3, &arg_4, &arg_5)?;
    Ok(object)
}

pub(crate) fn sample_iges_defs_units_data() -> Result<Handle<IgesDefsUnitsData>, Failure> {
    let object = Handle::new(IgesDefsUnitsData::new());
    let arg_0: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample0")),
        Handle::new(HAsciiString::new("sample1")),
    ])?;
    let arg_1: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample1")),
        Handle::new(HAsciiString::new("sample2")),
    ])?;
    let arg_2: Handle<HArray1<f64>> = sample_list::<f64>([3.5, 4.5])?;
    object.init(&arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_angular_dimension()
-> Result<Handle<IgesDimenAngularDimension>, Failure> {
    let object = Handle::new(IgesDimenAngularDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    let arg_2: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    let arg_3: Xy = Xy::new(3.25, 1.5);
    let arg_4: f64 = 5.5;
    let arg_5: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_6: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    object.init(&arg_0, &arg_1, &arg_2, arg_3, arg_4, &arg_5, &arg_6);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_basic_dimension() -> Result<Handle<IgesDimenBasicDimension>, Failure>
{
    let object = Handle::new(IgesDimenBasicDimension::new());
    let arg_0: i32 = 8;
    let arg_1: Xy = Xy::new(1.25, 1.5);
    let arg_2: Xy = Xy::new(2.25, 1.5);
    let arg_3: Xy = Xy::new(3.25, 1.5);
    let arg_4: Xy = Xy::new(4.25, 1.5);
    object.init(arg_0, arg_1, arg_2, arg_3, arg_4);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_center_line() -> Result<Handle<IgesDimenCenterLine>, Failure> {
    let object = Handle::new(IgesDimenCenterLine::new());
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(2.25, 1.5), Xy::new(3.25, 1.5)])?;
    object.init(arg_0, arg_1, &arg_2)?;
    object.set_cross_hair(false);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_curve_dimension() -> Result<Handle<IgesDimenCurveDimension>, Failure>
{
    let object = Handle::new(IgesDimenCurveDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_2: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_3: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_4: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_5: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    let arg_6: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    object.init(&arg_0, &arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_diameter_dimension()
-> Result<Handle<IgesDimenDiameterDimension>, Failure> {
    let object = Handle::new(IgesDimenDiameterDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_2: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_3: Xy = Xy::new(3.25, 1.5);
    object.init(&arg_0, &arg_1, &arg_2, arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_dimension_display_data()
-> Result<Handle<IgesDimenDimensionDisplayData>, Failure> {
    let object = Handle::new(IgesDimenDimensionDisplayData::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    let arg_5: i32 = 2;
    let arg_6: f64 = 7.5;
    let arg_7: i32 = 2;
    let arg_8: i32 = 2;
    let arg_9: i32 = 2;
    let arg_10: i32 = 2;
    let arg_11: f64 = 12.5;
    let arg_12: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_13: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_14: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(
        arg_0, arg_1, arg_2, arg_3, &arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10, arg_11,
        &arg_12, &arg_13, &arg_14,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_dimension_tolerance()
-> Result<Handle<IgesDimenDimensionTolerance>, Failure> {
    let object = Handle::new(IgesDimenDimensionTolerance::new());
    let arg_0: i32 = 8;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: f64 = 5.5;
    let arg_5: f64 = 6.5;
    let arg_6: bool = true;
    let arg_7: i32 = 2;
    let arg_8: i32 = 2;
    object.init(
        arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8,
    );
    Ok(object)
}

pub(crate) fn sample_iges_dimen_dimension_units() -> Result<Handle<IgesDimenDimensionUnits>, Failure>
{
    let object = Handle::new(IgesDimenDimensionUnits::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    let arg_5: i32 = 2;
    let arg_6: i32 = 2;
    object.init(arg_0, arg_1, arg_2, arg_3, &arg_4, arg_5, arg_6);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_dimensioned_geometry()
-> Result<Handle<IgesDimenDimensionedGeometry>, Failure> {
    let object = Handle::new(IgesDimenDimensionedGeometry::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_flag_note() -> Result<Handle<IgesDimenFlagNote>, Failure> {
    let object = Handle::new(IgesDimenFlagNote::new());
    let arg_0: Xyz = Xyz::new(0.25, 1.5, 2.5);
    let arg_1: f64 = 2.5;
    let arg_2: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_3: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
        sample_list::<Handle<IgesDimenLeaderArrow>>([
            sample_iges_dimen_leader_arrow()?.cast(),
            sample_iges_dimen_leader_arrow()?.cast(),
        ])?;
    object.init(arg_0, arg_1, &arg_2, &arg_3)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_general_label() -> Result<Handle<IgesDimenGeneralLabel>, Failure> {
    let object = Handle::new(IgesDimenGeneralLabel::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
        sample_list::<Handle<IgesDimenLeaderArrow>>([
            sample_iges_dimen_leader_arrow()?.cast(),
            sample_iges_dimen_leader_arrow()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_general_note() -> Result<Handle<IgesDimenGeneralNote>, Failure> {
    let object = Handle::new(IgesDimenGeneralNote::new());
    let arg_0: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_1: Handle<HArray1<f64>> = sample_list::<f64>([2.5, 3.5])?;
    let arg_2: Handle<HArray1<f64>> = sample_list::<f64>([3.5, 4.5])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<IgesGraphTextFontDef>>> =
        sample_list::<Handle<IgesGraphTextFontDef>>([
            sample_iges_graph_text_font_def()?.cast(),
            sample_iges_graph_text_font_def()?.cast(),
        ])?;
    let arg_5: Handle<HArray1<f64>> = sample_list::<f64>([6.5, 7.5])?;
    let arg_6: Handle<HArray1<f64>> = sample_list::<f64>([7.5, 8.5])?;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_8: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_9: Handle<HArray1<Xyz>> =
        sample_list::<Xyz>([Xyz::new(9.25, 1.5, 2.5), Xyz::new(10.25, 1.5, 2.5)])?;
    let arg_10: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample10")),
        Handle::new(HAsciiString::new("sample11")),
    ])?;
    object.init(
        &arg_0, &arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6, &arg_7, &arg_8, &arg_9, &arg_10,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_general_symbol() -> Result<Handle<IgesDimenGeneralSymbol>, Failure>
{
    let object = Handle::new(IgesDimenGeneralSymbol::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_2: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
        sample_list::<Handle<IgesDimenLeaderArrow>>([
            sample_iges_dimen_leader_arrow()?.cast(),
            sample_iges_dimen_leader_arrow()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_leader_arrow() -> Result<Handle<IgesDimenLeaderArrow>, Failure> {
    let object = Handle::new(IgesDimenLeaderArrow::new());
    let arg_0: f64 = 1.5;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    let arg_3: Xy = Xy::new(3.25, 1.5);
    let arg_4: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(4.25, 1.5), Xy::new(5.25, 1.5)])?;
    object.init(arg_0, arg_1, arg_2, arg_3, &arg_4)?;
    object.set_form_number(1)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_linear_dimension()
-> Result<Handle<IgesDimenLinearDimension>, Failure> {
    let object = Handle::new(IgesDimenLinearDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_2: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_3: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    let arg_4: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    object.init(&arg_0, &arg_1, &arg_2, &arg_3, &arg_4);
    Ok(object)
}
