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
use crate::iges_data::{IgesDataIgesEntity, IgesDataLineFontEntity, IgesDataViewKindEntity};
use crate::iges_dimen::{
    IgesDimenGeneralNote, IgesDimenLeaderArrow, IgesDimenNewDimensionedGeometry,
    IgesDimenNewGeneralNote, IgesDimenOrdinateDimension, IgesDimenPointDimension,
    IgesDimenRadiusDimension, IgesDimenSection, IgesDimenSectionedArea, IgesDimenWitnessLine,
};
use crate::iges_draw::{
    IgesDrawCircArraySubfigure, IgesDrawConnectPoint, IgesDrawDrawing, IgesDrawDrawingWithRotation,
    IgesDrawLabelDisplay, IgesDrawNetworkSubfigure, IgesDrawNetworkSubfigureDef,
    IgesDrawPerspectiveView, IgesDrawPlanar, IgesDrawRectArraySubfigure,
    IgesDrawSegmentedViewsVisible, IgesDrawView, IgesDrawViewsVisible,
    IgesDrawViewsVisibleWithAttr,
};
use crate::iges_geom::{IgesGeomPlane, IgesGeomTransformationMatrix};
use crate::iges_graph::{IgesGraphColor, IgesGraphTextDisplayTemplate};
use crate::iges_test_samples::{
    sample_iges_dimen_general_note, sample_iges_dimen_leader_arrow, sample_iges_geom_plane,
    sample_iges_geom_transformation_matrix, sample_iges_graph_color,
    sample_iges_graph_line_font_predefined, sample_iges_graph_text_display_template, sample_list,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Xy, Xyz};

pub(crate) fn sample_iges_dimen_new_dimensioned_geometry()
-> Result<Handle<IgesDimenNewDimensionedGeometry>, Failure> {
    let object = Handle::new(IgesDimenNewDimensionedGeometry::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_2: i32 = 2;
    let arg_3: f64 = 4.5;
    let arg_4: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_5: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_6: Handle<HArray1<Xyz>> =
        sample_list::<Xyz>([Xyz::new(6.25, 1.5, 2.5), Xyz::new(7.25, 1.5, 2.5)])?;
    object.init(arg_0, &arg_1, arg_2, arg_3, &arg_4, &arg_5, &arg_6)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_new_general_note()
-> Result<Handle<IgesDimenNewGeneralNote>, Failure> {
    let object = Handle::new(IgesDimenNewGeneralNote::new());
    let arg_0: f64 = 1.5;
    let arg_1: f64 = 2.5;
    let arg_2: i32 = 2;
    let arg_3: Xyz = Xyz::new(3.25, 1.5, 2.5);
    let arg_4: f64 = 5.5;
    let arg_5: Xyz = Xyz::new(5.25, 1.5, 2.5);
    let arg_6: f64 = 7.5;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_8: Handle<HArray1<f64>> = sample_list::<f64>([9.5, 10.5])?;
    let arg_9: Handle<HArray1<f64>> = sample_list::<f64>([10.5, 11.5])?;
    let arg_10: Handle<HArray1<f64>> = sample_list::<f64>([11.5, 12.5])?;
    let arg_11: Handle<HArray1<f64>> = sample_list::<f64>([12.5, 13.5])?;
    let arg_12: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_13: Handle<HArray1<f64>> = sample_list::<f64>([14.5, 15.5])?;
    let arg_14: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample14")),
        Handle::new(HAsciiString::new("sample15")),
    ])?;
    let arg_15: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_16: Handle<HArray1<f64>> = sample_list::<f64>([17.5, 18.5])?;
    let arg_17: Handle<HArray1<f64>> = sample_list::<f64>([18.5, 19.5])?;
    let arg_18: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_19: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_20: Handle<HArray1<f64>> = sample_list::<f64>([21.5, 22.5])?;
    let arg_21: Handle<HArray1<f64>> = sample_list::<f64>([22.5, 23.5])?;
    let arg_22: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_23: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_24: Handle<HArray1<Xyz>> =
        sample_list::<Xyz>([Xyz::new(24.25, 1.5, 2.5), Xyz::new(25.25, 1.5, 2.5)])?;
    let arg_25: Handle<HArray1<Handle<HAsciiString>>> = sample_list::<Handle<HAsciiString>>([
        Handle::new(HAsciiString::new("sample25")),
        Handle::new(HAsciiString::new("sample26")),
    ])?;
    object.init(
        arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, &arg_7, &arg_8, &arg_9, &arg_10, &arg_11,
        &arg_12, &arg_13, &arg_14, &arg_15, &arg_16, &arg_17, &arg_18, &arg_19, &arg_20, &arg_21,
        &arg_22, &arg_23, &arg_24, &arg_25,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_ordinate_dimension()
-> Result<Handle<IgesDimenOrdinateDimension>, Failure> {
    let object = Handle::new(IgesDimenOrdinateDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: bool = true;
    let arg_2: Handle<IgesDimenWitnessLine> = sample_iges_dimen_witness_line()?.cast();
    let arg_3: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    object.init(&arg_0, arg_1, &arg_2, &arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_point_dimension() -> Result<Handle<IgesDimenPointDimension>, Failure>
{
    let object = Handle::new(IgesDimenPointDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_2: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    object.init(&arg_0, &arg_1, &arg_2);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_radius_dimension()
-> Result<Handle<IgesDimenRadiusDimension>, Failure> {
    let object = Handle::new(IgesDimenRadiusDimension::new());
    let arg_0: Handle<IgesDimenGeneralNote> = sample_iges_dimen_general_note()?.cast();
    let arg_1: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    let arg_2: Xy = Xy::new(2.25, 1.5);
    let arg_3: Handle<IgesDimenLeaderArrow> = sample_iges_dimen_leader_arrow()?.cast();
    object.init(&arg_0, &arg_1, arg_2, &arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_dimen_section() -> Result<Handle<IgesDimenSection>, Failure> {
    let object = Handle::new(IgesDimenSection::new());
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(2.25, 1.5), Xy::new(3.25, 1.5)])?;
    object.init(arg_0, arg_1, &arg_2)?;
    object.set_form_number(31)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_sectioned_area() -> Result<Handle<IgesDimenSectionedArea>, Failure>
{
    let object = Handle::new(IgesDimenSectionedArea::new());
    let arg_0: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_1: i32 = 2;
    let arg_2: Xyz = Xyz::new(2.25, 1.5, 2.5);
    let arg_3: f64 = 4.5;
    let arg_4: f64 = 5.5;
    let arg_5: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, arg_1, arg_2, arg_3, arg_4, &arg_5)?;
    Ok(object)
}

pub(crate) fn sample_iges_dimen_witness_line() -> Result<Handle<IgesDimenWitnessLine>, Failure> {
    let object = Handle::new(IgesDimenWitnessLine::new());
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(2.25, 1.5), Xy::new(3.25, 1.5)])?;
    object.init(arg_0, arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_circ_array_subfigure()
-> Result<Handle<IgesDrawCircArraySubfigure>, Failure> {
    let object = Handle::new(IgesDrawCircArraySubfigure::new());
    let arg_0: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_1: i32 = 2;
    let arg_2: Xyz = Xyz::new(2.25, 1.5, 2.5);
    let arg_3: f64 = 4.5;
    let arg_4: f64 = 5.5;
    let arg_5: f64 = 6.5;
    let arg_6: i32 = 2;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(&arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, &arg_7)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_connect_point() -> Result<Handle<IgesDrawConnectPoint>, Failure> {
    let object = Handle::new(IgesDrawConnectPoint::new());
    let arg_0: Xyz = Xyz::new(0.25, 1.5, 2.5);
    let arg_1: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    let arg_5: Handle<IgesGraphTextDisplayTemplate> =
        sample_iges_graph_text_display_template()?.cast();
    let arg_6: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample6"));
    let arg_7: Handle<IgesGraphTextDisplayTemplate> =
        sample_iges_graph_text_display_template()?.cast();
    let arg_8: i32 = 2;
    let arg_9: i32 = 2;
    let arg_10: i32 = 2;
    let arg_11: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    object.init(
        arg_0, &arg_1, arg_2, arg_3, &arg_4, &arg_5, &arg_6, &arg_7, arg_8, arg_9, arg_10, &arg_11,
    );
    Ok(object)
}

pub(crate) fn sample_iges_draw_drawing() -> Result<Handle<IgesDrawDrawing>, Failure> {
    let object = Handle::new(IgesDrawDrawing::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(1.25, 1.5), Xy::new(2.25, 1.5)])?;
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_drawing_with_rotation()
-> Result<Handle<IgesDrawDrawingWithRotation>, Failure> {
    let object = Handle::new(IgesDrawDrawingWithRotation::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<Xy>> = sample_list::<Xy>([Xy::new(1.25, 1.5), Xy::new(2.25, 1.5)])?;
    let arg_2: Handle<HArray1<f64>> = sample_list::<f64>([3.5, 4.5])?;
    let arg_3: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1, &arg_2, &arg_3)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_label_display() -> Result<Handle<IgesDrawLabelDisplay>, Failure> {
    let object = Handle::new(IgesDrawLabelDisplay::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<Xyz>> =
        sample_list::<Xyz>([Xyz::new(1.25, 1.5, 2.5), Xyz::new(2.25, 1.5, 2.5)])?;
    let arg_2: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> =
        sample_list::<Handle<IgesDimenLeaderArrow>>([
            sample_iges_dimen_leader_arrow()?.cast(),
            sample_iges_dimen_leader_arrow()?.cast(),
        ])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1, &arg_2, &arg_3, &arg_4)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_network_subfigure()
-> Result<Handle<IgesDrawNetworkSubfigure>, Failure> {
    let object = Handle::new(IgesDrawNetworkSubfigure::new());
    let arg_0: Handle<IgesDrawNetworkSubfigureDef> =
        sample_iges_draw_network_subfigure_def()?.cast();
    let arg_1: Xyz = Xyz::new(1.25, 1.5, 2.5);
    let arg_2: Xyz = Xyz::new(2.25, 1.5, 2.5);
    let arg_3: i32 = 2;
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    let arg_5: Handle<IgesGraphTextDisplayTemplate> =
        sample_iges_graph_text_display_template()?.cast();
    let arg_6: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
        sample_list::<Handle<IgesDrawConnectPoint>>([
            sample_iges_draw_connect_point()?.cast(),
            sample_iges_draw_connect_point()?.cast(),
        ])?;
    object.init(&arg_0, arg_1, arg_2, arg_3, &arg_4, &arg_5, &arg_6)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_network_subfigure_def()
-> Result<Handle<IgesDrawNetworkSubfigureDef>, Failure> {
    let object = Handle::new(IgesDrawNetworkSubfigureDef::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    let arg_3: i32 = 2;
    let arg_4: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample4"));
    let arg_5: Handle<IgesGraphTextDisplayTemplate> =
        sample_iges_graph_text_display_template()?.cast();
    let arg_6: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
        sample_list::<Handle<IgesDrawConnectPoint>>([
            sample_iges_draw_connect_point()?.cast(),
            sample_iges_draw_connect_point()?.cast(),
        ])?;
    object.init(arg_0, &arg_1, &arg_2, arg_3, &arg_4, &arg_5, &arg_6)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_perspective_view() -> Result<Handle<IgesDrawPerspectiveView>, Failure>
{
    let object = IgesDrawPerspectiveView::new_bound();
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Xyz = Xyz::new(2.25, 1.5, 2.5);
    let arg_3: Xyz = Xyz::new(3.25, 1.5, 2.5);
    let arg_4: Xyz = Xyz::new(4.25, 1.5, 2.5);
    let arg_5: Xyz = Xyz::new(5.25, 1.5, 2.5);
    let arg_6: f64 = 7.5;
    let arg_7: Xy = Xy::new(7.25, 1.5);
    let arg_8: Xy = Xy::new(8.25, 1.5);
    let arg_9: i32 = 2;
    let arg_10: f64 = 11.5;
    let arg_11: f64 = 12.5;
    object.init(
        arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10, arg_11,
    );
    Ok(object)
}

pub(crate) fn sample_iges_draw_planar() -> Result<Handle<IgesDrawPlanar>, Failure> {
    let object = Handle::new(IgesDrawPlanar::new());
    let arg_0: i32 = 1;
    let arg_1: Handle<IgesGeomTransformationMatrix> =
        sample_iges_geom_transformation_matrix()?.cast();
    let arg_2: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(arg_0, &arg_1, &arg_2)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_rect_array_subfigure()
-> Result<Handle<IgesDrawRectArraySubfigure>, Failure> {
    let object = Handle::new(IgesDrawRectArraySubfigure::new());
    let arg_0: Handle<IgesDataIgesEntity> = sample_iges_graph_color()?.cast();
    let arg_1: f64 = 2.5;
    let arg_2: Xyz = Xyz::new(2.25, 1.5, 2.5);
    let arg_3: i32 = 2;
    let arg_4: i32 = 2;
    let arg_5: f64 = 6.5;
    let arg_6: f64 = 7.5;
    let arg_7: f64 = 8.5;
    let arg_8: i32 = 2;
    let arg_9: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(
        &arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, &arg_9,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_segmented_views_visible()
-> Result<Handle<IgesDrawSegmentedViewsVisible>, Failure> {
    let object = Handle::new(IgesDrawSegmentedViewsVisible::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<f64>> = sample_list::<f64>([2.5, 3.5])?;
    let arg_2: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<IgesGraphColor>>> = sample_list::<Handle<IgesGraphColor>>([
        sample_iges_graph_color()?.cast(),
        sample_iges_graph_color()?.cast(),
    ])?;
    let arg_5: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_6: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>> =
        sample_list::<Handle<dyn IgesDataLineFontEntity>>([
            sample_iges_graph_line_font_predefined()?.cast(),
            sample_iges_graph_line_font_predefined()?.cast(),
        ])?;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(
        &arg_0, &arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6, &arg_7,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_view() -> Result<Handle<IgesDrawView>, Failure> {
    let object = IgesDrawView::new_bound();
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    let arg_3: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    let arg_4: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    let arg_5: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    let arg_6: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    let arg_7: Handle<IgesGeomPlane> = sample_iges_geom_plane()?.cast();
    object.init(arg_0, arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6, &arg_7);
    Ok(object)
}

pub(crate) fn sample_iges_draw_views_visible() -> Result<Handle<IgesDrawViewsVisible>, Failure> {
    let object = Handle::new(IgesDrawViewsVisible::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1)?;
    Ok(object)
}

pub(crate) fn sample_iges_draw_views_visible_with_attr()
-> Result<Handle<IgesDrawViewsVisibleWithAttr>, Failure> {
    let object = Handle::new(IgesDrawViewsVisibleWithAttr::new());
    let arg_0: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
        sample_list::<Handle<dyn IgesDataViewKindEntity>>([
            sample_iges_draw_view()?.cast(),
            sample_iges_draw_view()?.cast(),
        ])?;
    let arg_1: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_2: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>> =
        sample_list::<Handle<dyn IgesDataLineFontEntity>>([
            sample_iges_graph_line_font_predefined()?.cast(),
            sample_iges_graph_line_font_predefined()?.cast(),
        ])?;
    let arg_3: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_4: Handle<HArray1<Handle<IgesGraphColor>>> = sample_list::<Handle<IgesGraphColor>>([
        sample_iges_graph_color()?.cast(),
        sample_iges_graph_color()?.cast(),
    ])?;
    let arg_5: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_6: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
        sample_list::<Handle<IgesDataIgesEntity>>([
            sample_iges_graph_color()?.cast(),
            sample_iges_graph_color()?.cast(),
        ])?;
    object.init(&arg_0, &arg_1, &arg_2, &arg_3, &arg_4, &arg_5, &arg_6)?;
    Ok(object)
}
