#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::{
    IgesBasicAssocGroupType, IgesBasicExternalRefFile, IgesBasicExternalRefFileIndex,
    IgesBasicExternalRefFileName, IgesBasicExternalRefLibName, IgesBasicExternalRefName,
    IgesBasicExternalReferenceFile, IgesBasicGroup, IgesBasicGroupWithoutBackP, IgesBasicHierarchy,
    IgesBasicName, IgesBasicOrderedGroup, IgesBasicOrderedGroupWithoutBackP, IgesBasicSingleParent,
    IgesBasicSingularSubfigure, IgesBasicSubfigureDef,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_geom::samples_01::sample_iges_geom_line;
use crate::iges_test_samples::sample_list;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

pub(crate) fn sample_iges_basic_assoc_group_type()
-> Result<Handle<IgesBasicAssocGroupType>, Failure> {
    let object = Handle::new(IgesBasicAssocGroupType::new());
    let nb_data_fields: i32 = 2;
    let a_type: i32 = 3;
    let a_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 3"));
    object.init(nb_data_fields, a_type, &a_name);
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_ref_file()
-> Result<Handle<IgesBasicExternalRefFile>, Failure> {
    let object = Handle::new(IgesBasicExternalRefFile::new());
    let a_file_ident: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 1"));
    object.init(&a_file_ident);
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_ref_file_index()
-> Result<Handle<IgesBasicExternalRefFileIndex>, Failure> {
    let object = Handle::new(IgesBasicExternalRefFileIndex::new());
    let a_name_array: Handle<HArray1<Handle<HAsciiString>>> = sample_list([
        Handle::new(HAsciiString::new("first")),
        Handle::new(HAsciiString::new("second")),
    ])?;
    let all_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    object.init(&a_name_array, &all_entities)?;
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_ref_file_name()
-> Result<Handle<IgesBasicExternalRefFileName>, Failure> {
    let object = Handle::new(IgesBasicExternalRefFileName::new());
    let a_file_ident: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 1"));
    let an_ext_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 2"));
    object.init(&a_file_ident, &an_ext_name);
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_ref_lib_name()
-> Result<Handle<IgesBasicExternalRefLibName>, Failure> {
    let object = Handle::new(IgesBasicExternalRefLibName::new());
    let a_lib_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 1"));
    let an_ext_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 2"));
    object.init(&a_lib_name, &an_ext_name);
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_ref_name()
-> Result<Handle<IgesBasicExternalRefName>, Failure> {
    let object = Handle::new(IgesBasicExternalRefName::new());
    let an_ext_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 1"));
    object.init(&an_ext_name);
    Ok(object)
}
pub(crate) fn sample_iges_basic_external_reference_file()
-> Result<Handle<IgesBasicExternalReferenceFile>, Failure> {
    let object = Handle::new(IgesBasicExternalReferenceFile::new());
    let a_name_array: Handle<HArray1<Handle<HAsciiString>>> = sample_list([
        Handle::new(HAsciiString::new("first")),
        Handle::new(HAsciiString::new("second")),
    ])?;
    object.init(&a_name_array)?;
    Ok(object)
}
pub(crate) fn sample_iges_basic_group() -> Result<Handle<IgesBasicGroup>, Failure> {
    let object = Handle::new(IgesBasicGroup::new());
    let all_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    object.init(&all_entities)?;
    Ok(object)
}
pub(crate) fn sample_iges_basic_group_without_back_p()
-> Result<Handle<IgesBasicGroupWithoutBackP>, Failure> {
    let object = Handle::new(IgesBasicGroupWithoutBackP::new());
    Ok(object)
}
pub(crate) fn sample_iges_basic_hierarchy() -> Result<Handle<IgesBasicHierarchy>, Failure> {
    let object = Handle::new(IgesBasicHierarchy::new());
    let nb_prop_val: i32 = 6;
    let a_line_font: i32 = 3;
    let a_view: i32 = 4;
    let an_entity_level: i32 = 5;
    let a_blank_status: i32 = 6;
    let a_line_wt: i32 = 7;
    let a_color_num: i32 = 8;
    object.init(
        nb_prop_val,
        a_line_font,
        a_view,
        an_entity_level,
        a_blank_status,
        a_line_wt,
        a_color_num,
    );
    Ok(object)
}
pub(crate) fn sample_iges_basic_name() -> Result<Handle<IgesBasicName>, Failure> {
    let object = Handle::new(IgesBasicName::new());
    let nb_prop_val: i32 = 2;
    let a_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 2"));
    object.init(nb_prop_val, &a_name);
    Ok(object)
}
pub(crate) fn sample_iges_basic_ordered_group() -> Result<Handle<IgesBasicOrderedGroup>, Failure> {
    let object = Handle::new(IgesBasicOrderedGroup::new());
    Ok(object)
}
pub(crate) fn sample_iges_basic_ordered_group_without_back_p()
-> Result<Handle<IgesBasicOrderedGroupWithoutBackP>, Failure> {
    let object = Handle::new(IgesBasicOrderedGroupWithoutBackP::new());
    Ok(object)
}
pub(crate) fn sample_iges_basic_single_parent() -> Result<Handle<IgesBasicSingleParent>, Failure> {
    let object = Handle::new(IgesBasicSingleParent::new());
    let nb_parent_entities: i32 = 2;
    let a_parent_entity: Handle<IgesDataIgesEntity> =
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>();
    let all_children: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    object.init(nb_parent_entities, &a_parent_entity, &all_children)?;
    Ok(object)
}
pub(crate) fn sample_iges_basic_singular_subfigure()
-> Result<Handle<IgesBasicSingularSubfigure>, Failure> {
    let object = Handle::new(IgesBasicSingularSubfigure::new());
    let a_subfigure_def: Handle<IgesBasicSubfigureDef> = sample_iges_basic_subfigure_def()?;
    let a_translation: Xyz = Xyz::new(2.5, 3.5, 4.5);
    let has_scale: bool = true;
    let a_scale: f64 = 4.5;
    object.init(&a_subfigure_def, a_translation, has_scale, a_scale);
    Ok(object)
}
pub(crate) fn sample_iges_basic_subfigure_def() -> Result<Handle<IgesBasicSubfigureDef>, Failure> {
    let object = Handle::new(IgesBasicSubfigureDef::new());
    let a_depth: i32 = 2;
    let a_name: Handle<HAsciiString> = Handle::new(HAsciiString::new("text 2"));
    let all_assoc_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = sample_list([
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
        sample_iges_geom_line()?.cast::<IgesDataIgesEntity>(),
    ])?;
    object.init(a_depth, &a_name, &all_assoc_entities)?;
    Ok(object)
}
