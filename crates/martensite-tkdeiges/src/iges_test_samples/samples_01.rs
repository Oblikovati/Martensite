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
use crate::iges_appli::{
    IgesAppliDrilledHole, IgesAppliElementResults, IgesAppliFiniteElement, IgesAppliFlow,
    IgesAppliFlowLineSpec, IgesAppliLevelFunction, IgesAppliLevelToPWBLayerMap,
    IgesAppliLineWidening, IgesAppliNodalConstraint, IgesAppliNodalDisplAndRot,
    IgesAppliNodalResults, IgesAppliNode, IgesAppliPWBArtworkStackup, IgesAppliPWBDrilledHole,
    IgesAppliPartNumber, IgesAppliPinNumber, IgesAppliPipingFlow, IgesAppliReferenceDesignator,
    IgesAppliRegionRestriction,
};
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal,
    IgesBasicHArray1OfHArray1OfXyz, IgesBasicSubfigureDef,
};
use crate::iges_data::{IgesDataIgesEntity, iges_array2_set, iges_new_array2};
use crate::iges_defs::IgesDefsTabularData;
use crate::iges_dimen::IgesDimenGeneralNote;
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_geom::{IgesGeomPlane, IgesGeomTransformationMatrix};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::iges_test_samples::{
    h_array1_of_h_array1_of_integer_sample, h_array1_of_h_array1_of_real_sample,
    h_array1_of_h_array1_of_xyz_sample, sample_iges_defs_tabular_data,
    sample_iges_dimen_general_note, sample_iges_draw_connect_point, sample_iges_graph_color,
    sample_iges_graph_text_display_template, sample_list,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;

pub(crate) fn sample_iges_basic_subfigure_def() -> Result<Handle<IgesBasicSubfigureDef>, Failure> {
    let object = Handle::new(IgesBasicSubfigureDef::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_geom_plane() -> Result<Handle<IgesGeomPlane>, Failure> {
    let object = Handle::new(IgesGeomPlane::new());
    let arg_0: f64 = 1.5;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    let arg_3: f64 = 4.5;
    let arg_4: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_5: Xyz = Xyz::new(5.25, 1.5, 2.5);
    let arg_6: f64 = 7.5;
    object.init(arg_0, arg_1, arg_2, arg_3, &arg_4, arg_5, arg_6);
    Ok(object)
}

pub(crate) fn sample_iges_geom_transformation_matrix()
-> Result<Handle<IgesGeomTransformationMatrix>, Failure> {
    let object = Handle::new(IgesGeomTransformationMatrix::new());
    let values = iges_new_array2::<f64>(1, 3, 1, 4)?;
    for rank in 1..=3 {
        iges_array2_set(&values, rank, rank, 1.0, "sample")?;
    }
    object.init(&values)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_drilled_hole() -> Result<Handle<IgesAppliDrilledHole>, Failure> {
    let object = Handle::new(IgesAppliDrilledHole::new());
    let arg_0: i32 = 5;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    let arg_3: i32 = 2;
    let arg_4: i32 = 2;
    let arg_5: i32 = 2;
    object.init(arg_0, arg_1, arg_2, arg_3, arg_4, arg_5);
    Ok(object)
}

pub(crate) fn sample_iges_appli_element_results() -> Result<Handle<IgesAppliElementResults>, Failure>
{
    let object = Handle::new(IgesAppliElementResults::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: i32 = 2;
    let arg_2: f64 = 3.5;
    let arg_3: i32 = 1;
    let arg_4: i32 = 2;
    let arg_5: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_6: Handle<HArray1<Handle<IgesAppliFiniteElement>>> =
        sample_list::<Handle<IgesAppliFiniteElement>>([
            sample_iges_appli_finite_element()?.cast(),
            sample_iges_appli_finite_element()?.cast(),
        ])?;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_8: Handle<HArray1<i32>> = sample_list::<i32>([1, 1])?;
    let arg_9: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_10: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_11: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    let arg_12: Handle<IgesBasicHArray1OfHArray1OfReal> = h_array1_of_h_array1_of_real_sample()?;
    object.init(
        &arg_0, arg_1, arg_2, arg_3, arg_4, &arg_5, &arg_6, &arg_7, &arg_8, &arg_9, &arg_10,
        &arg_11, &arg_12,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_finite_element() -> Result<Handle<IgesAppliFiniteElement>, Failure>
{
    let object = Handle::new(IgesAppliFiniteElement::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HArray1<Handle<IgesAppliNode>>> = sample_list::<Handle<IgesAppliNode>>([
        sample_iges_appli_node()?.cast(),
        sample_iges_appli_node()?.cast(),
    ])?;
    let arg_2: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample2"));
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_flow() -> Result<Handle<IgesAppliFlow>, Failure> {
    let object = Handle::new(IgesAppliFlow::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_4: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
        sample_list::<Handle<IgesDrawConnectPoint>>([
            sample_iges_draw_connect_point()?.cast(),
            sample_iges_draw_connect_point()?.cast(),
        ])?;
    let arg_5: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_6: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample6")),
        Handle::new(HAsciiString::new("sample7")),
    ])?;
    let arg_7: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>> =
        sample_list::<Handle<IgesGraphTextDisplayTemplate>>([
            sample_iges_graph_text_display_template()?.cast(),
            sample_iges_graph_text_display_template()?.cast(),
        ])?;
    let arg_8: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(
        arg_0, arg_1, arg_2, &arg_3, &arg_4, &arg_5, &arg_6, &arg_7, &arg_8,
    );
    Ok(object)
}

pub(crate) fn sample_iges_appli_flow_line_spec() -> Result<Handle<IgesAppliFlowLineSpec>, Failure> {
    let object = Handle::new(IgesAppliFlowLineSpec::new());
    let arg_0: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample0")),
        Handle::new(HAsciiString::new("sample1")),
    ])?;
    object.init(&arg_0)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_level_function() -> Result<Handle<IgesAppliLevelFunction>, Failure>
{
    let object = Handle::new(IgesAppliLevelFunction::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample2"));
    object.init(arg_0, arg_1, &arg_2);
    Ok(object)
}

pub(crate) fn sample_iges_appli_level_to_pwb_layer_map()
-> Result<Handle<IgesAppliLevelToPWBLayerMap>, Failure> {
    let object = Handle::new(IgesAppliLevelToPWBLayerMap::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_2: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample2")),
        Handle::new(HAsciiString::new("sample3")),
    ])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample4")),
        Handle::new(HAsciiString::new("sample5")),
    ])?;
    object.init(arg_0, &arg_1, &arg_2, &arg_3, &arg_4)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_line_widening() -> Result<Handle<IgesAppliLineWidening>, Failure> {
    let object = Handle::new(IgesAppliLineWidening::new());
    let arg_0: i32 = 5;
    let arg_1: f64 = 2.5;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: i32 = 2;
    let arg_5: f64 = 6.5;
    object.init(arg_0, arg_1, arg_2, arg_3, arg_4, arg_5);
    Ok(object)
}

pub(crate) fn sample_iges_appli_nodal_constraint()
-> Result<Handle<IgesAppliNodalConstraint>, Failure> {
    let object = Handle::new(IgesAppliNodalConstraint::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<IgesAppliNode> = sample_iges_appli_node()?.cast();
    let arg_2: Handle<HArray1<Handle<IgesDefsTabularData>>> =
        sample_list::<Handle<IgesDefsTabularData>>([
            sample_iges_defs_tabular_data()?.cast(),
            sample_iges_defs_tabular_data()?.cast(),
        ])?;
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_nodal_displ_and_rot()
-> Result<Handle<IgesAppliNodalDisplAndRot>, Failure> {
    let object = Handle::new(IgesAppliNodalDisplAndRot::new());
    let arg_0: Handle<HArray1<Handle<IgesDimenGeneralNote>>> =
        sample_list::<Handle<IgesDimenGeneralNote>>([
            sample_iges_dimen_general_note()?.cast(),
            sample_iges_dimen_general_note()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_2: Handle<HArray1<Handle<IgesAppliNode>>> = sample_list::<Handle<IgesAppliNode>>([
        sample_iges_appli_node()?.cast(),
        sample_iges_appli_node()?.cast(),
    ])?;
    let arg_3: Handle<IgesBasicHArray1OfHArray1OfXyz> = h_array1_of_h_array1_of_xyz_sample()?;
    let arg_4: Handle<IgesBasicHArray1OfHArray1OfXyz> = h_array1_of_h_array1_of_xyz_sample()?;
    object.init(&arg_0, &arg_1, &arg_2, &arg_3, &arg_4)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_nodal_results() -> Result<Handle<IgesAppliNodalResults>, Failure> {
    let object = Handle::new(IgesAppliNodalResults::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: i32 = 2;
    let arg_2: f64 = 3.5;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<IgesAppliNode>>> = sample_list::<Handle<IgesAppliNode>>([
        sample_iges_appli_node()?.cast(),
        sample_iges_appli_node()?.cast(),
    ])?;
    let arg_5: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 2, 1, 2)?;
    object.init(&arg_0, arg_1, arg_2, &arg_3, &arg_4, &arg_5)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_node() -> Result<Handle<IgesAppliNode>, Failure> {
    let object = Handle::new(IgesAppliNode::new());
    let arg_0: Xyz = Xyz::new(0.25, 1.5, 2.5);
    let arg_1: Handle<IgesGeomTransformationMatrix> =
        sample_iges_geom_transformation_matrix()?.cast();
    object.init(arg_0, &arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_appli_pwb_artwork_stackup()
-> Result<Handle<IgesAppliPWBArtworkStackup>, Failure> {
    let object = Handle::new(IgesAppliPWBArtworkStackup::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    let arg_2: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_pwb_drilled_hole()
-> Result<Handle<IgesAppliPWBDrilledHole>, Failure> {
    let object = Handle::new(IgesAppliPWBDrilledHole::new());
    let arg_0: i32 = 3;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    let arg_3: i32 = 2;
    object.init(arg_0, arg_1, arg_2, arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_appli_part_number() -> Result<Handle<IgesAppliPartNumber>, Failure> {
    let object = Handle::new(IgesAppliPartNumber::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    let arg_2: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample2"));
    let arg_3: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample3"));
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    object.init(arg_0, &arg_1, &arg_2, &arg_3, &arg_4);
    Ok(object)
}

pub(crate) fn sample_iges_appli_pin_number() -> Result<Handle<IgesAppliPinNumber>, Failure> {
    let object = Handle::new(IgesAppliPinNumber::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    object.init(arg_0, &arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_appli_piping_flow() -> Result<Handle<IgesAppliPipingFlow>, Failure> {
    let object = Handle::new(IgesAppliPipingFlow::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_3: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
        sample_list::<Handle<IgesDrawConnectPoint>>([
            sample_iges_draw_connect_point()?.cast(),
            sample_iges_draw_connect_point()?.cast(),
        ])?;
    let arg_4: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_5: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample5")),
        Handle::new(HAsciiString::new("sample6")),
    ])?;
    let arg_6: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>> =
        sample_list::<Handle<IgesGraphTextDisplayTemplate>>([
            sample_iges_graph_text_display_template()?.cast(),
            sample_iges_graph_text_display_template()?.cast(),
        ])?;
    let arg_7: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(arg_0, arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6, &arg_7)?;
    Ok(object)
}

pub(crate) fn sample_iges_appli_reference_designator()
-> Result<Handle<IgesAppliReferenceDesignator>, Failure> {
    let object = Handle::new(IgesAppliReferenceDesignator::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    object.init(arg_0, &arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_appli_region_restriction()
-> Result<Handle<IgesAppliRegionRestriction>, Failure> {
    let object = Handle::new(IgesAppliRegionRestriction::new());
    let arg_0: i32 = 3;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    object.init(arg_0, arg_1, arg_2, arg_3);
    Ok(object)
}
