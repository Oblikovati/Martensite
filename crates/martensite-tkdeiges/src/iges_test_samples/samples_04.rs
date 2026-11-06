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
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicSubfigureDef};
use crate::iges_graph::{
    IgesGraphColor, IgesGraphDefinitionLevel, IgesGraphDrawingSize, IgesGraphDrawingUnits,
    IgesGraphHighLight, IgesGraphIntercharacterSpacing, IgesGraphLineFontDefPattern,
    IgesGraphLineFontDefTemplate, IgesGraphLineFontPredefined, IgesGraphNominalSize, IgesGraphPick,
    IgesGraphTextDisplayTemplate, IgesGraphTextFontDef, IgesGraphUniformRectGrid,
};
use crate::iges_test_samples::{
    h_array1_of_h_array1_of_integer_sample, sample_iges_basic_subfigure_def, sample_list,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Xy, Xyz};

pub(crate) fn sample_iges_graph_color() -> Result<Handle<IgesGraphColor>, Failure> {
    let object = Handle::new(IgesGraphColor::new());
    let arg_0: f64 = 1.5;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    let arg_3: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample3"));
    object.init(arg_0, arg_1, arg_2, &arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_graph_definition_level()
-> Result<Handle<IgesGraphDefinitionLevel>, Failure> {
    let object = Handle::new(IgesGraphDefinitionLevel::new());
    let arg_0: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    object.init(&arg_0)?;
    Ok(object)
}

pub(crate) fn sample_iges_graph_drawing_size() -> Result<Handle<IgesGraphDrawingSize>, Failure> {
    let object = Handle::new(IgesGraphDrawingSize::new());
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: f64 = 3.5;
    object.init(arg_0, arg_1, arg_2);
    Ok(object)
}

pub(crate) fn sample_iges_graph_drawing_units() -> Result<Handle<IgesGraphDrawingUnits>, Failure> {
    let object = Handle::new(IgesGraphDrawingUnits::new());
    let arg_0: i32 = 2;
    let arg_1: i32 = 2;
    let arg_2: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample2"));
    object.init(arg_0, arg_1, &arg_2);
    Ok(object)
}

pub(crate) fn sample_iges_graph_high_light() -> Result<Handle<IgesGraphHighLight>, Failure> {
    let object = Handle::new(IgesGraphHighLight::new());
    let arg_0: i32 = 1;
    let arg_1: i32 = 2;
    object.init(arg_0, arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_graph_intercharacter_spacing()
-> Result<Handle<IgesGraphIntercharacterSpacing>, Failure> {
    let object = Handle::new(IgesGraphIntercharacterSpacing::new());
    let arg_0: i32 = 1;
    let arg_1: f64 = 2.5;
    object.init(arg_0, arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_graph_line_font_def_pattern()
-> Result<Handle<IgesGraphLineFontDefPattern>, Failure> {
    let object = Handle::new(IgesGraphLineFontDefPattern::new());
    let arg_0: Handle<HArray1<f64>> = sample_list::<f64>([1.5, 2.5])?;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    object.init(&arg_0, &arg_1)?;
    Ok(object)
}

pub(crate) fn sample_iges_graph_line_font_def_template()
-> Result<Handle<IgesGraphLineFontDefTemplate>, Failure> {
    let object = Handle::new(IgesGraphLineFontDefTemplate::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<IgesBasicSubfigureDef> = sample_iges_basic_subfigure_def()?.cast();
    let arg_2: f64 = 3.5;
    let arg_3: f64 = 4.5;
    object.init(arg_0, &arg_1, arg_2, arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_graph_line_font_predefined()
-> Result<Handle<IgesGraphLineFontPredefined>, Failure> {
    let object = Handle::new(IgesGraphLineFontPredefined::new());
    let arg_0: i32 = 1;
    let arg_1: i32 = 2;
    object.init(arg_0, arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_graph_nominal_size() -> Result<Handle<IgesGraphNominalSize>, Failure> {
    let object = Handle::new(IgesGraphNominalSize::new());
    let arg_0: i32 = 2;
    let arg_1: f64 = 2.5;
    let arg_2: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample2"));
    let arg_3: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample3"));
    object.init(arg_0, arg_1, &arg_2, &arg_3);
    Ok(object)
}

pub(crate) fn sample_iges_graph_pick() -> Result<Handle<IgesGraphPick>, Failure> {
    let object = Handle::new(IgesGraphPick::new());
    let arg_0: i32 = 1;
    let arg_1: i32 = 2;
    object.init(arg_0, arg_1);
    Ok(object)
}

pub(crate) fn sample_iges_graph_text_display_template()
-> Result<Handle<IgesGraphTextDisplayTemplate>, Failure> {
    let object = Handle::new(IgesGraphTextDisplayTemplate::new());
    let arg_0: f64 = 1.5;
    let arg_1: f64 = 2.5;
    let arg_2: i32 = 2;
    let arg_3: Handle<IgesGraphTextFontDef> = sample_iges_graph_text_font_def()?.cast();
    let arg_4: f64 = 5.5;
    let arg_5: f64 = 6.5;
    let arg_6: i32 = 2;
    let arg_7: i32 = 2;
    let arg_8: Xyz = Xyz::new(8.25, 1.5, 2.5);
    object.init(
        arg_0, arg_1, arg_2, &arg_3, arg_4, arg_5, arg_6, arg_7, arg_8,
    );
    Ok(object)
}

pub(crate) fn sample_iges_graph_text_font_def() -> Result<Handle<IgesGraphTextFontDef>, Failure> {
    let object = Handle::new(IgesGraphTextFontDef::new());
    let arg_0: i32 = 2;
    let arg_1: Handle<HAsciiString> = Handle::new(HAsciiString::new("sample1"));
    let arg_2: i32 = 2;
    let arg_3: Handle<IgesGraphTextFontDef> = Handle::default();
    let arg_4: i32 = 2;
    let arg_5: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_6: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_7: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_8: Handle<HArray1<i32>> = sample_list::<i32>([2, 2])?;
    let arg_9: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    let arg_10: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    let arg_11: Handle<IgesBasicHArray1OfHArray1OfInteger> =
        h_array1_of_h_array1_of_integer_sample()?;
    object.init(
        arg_0, &arg_1, arg_2, &arg_3, arg_4, &arg_5, &arg_6, &arg_7, &arg_8, &arg_9, &arg_10,
        &arg_11,
    )?;
    Ok(object)
}

pub(crate) fn sample_iges_graph_uniform_rect_grid()
-> Result<Handle<IgesGraphUniformRectGrid>, Failure> {
    let object = Handle::new(IgesGraphUniformRectGrid::new());
    let arg_0: i32 = 9;
    let arg_1: i32 = 2;
    let arg_2: i32 = 2;
    let arg_3: i32 = 2;
    let arg_4: Xy = Xy::new(4.25, 1.5);
    let arg_5: Xy = Xy::new(5.25, 1.5);
    let arg_6: i32 = 2;
    let arg_7: i32 = 2;
    object.init(arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7);
    Ok(object)
}
