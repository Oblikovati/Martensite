//! IgesDrawToolSegmentedViewsVisible.
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
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLineFontEntity, IgesDataParamReader,
    IgesDataViewKindEntity, iges_array1_set, iges_new_array1, iges_real_text,
};
use crate::iges_data::{iges_read_integer_item, iges_read_real_item};
use crate::iges_draw::IgesDrawSegmentedViewsVisible;
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolSegmentedViewsVisible,
    "The own-parameter work on `IgesDrawSegmentedViewsVisible`."
);

impl IgesDrawToolSegmentedViewsVisible {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawSegmentedViewsVisible>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0004!(
            breakpoint_parameters,
            color_definitions,
            color_values,
            display_flags,
            line_font_definitions,
            line_font_values,
            line_weights,
            nbval,
            views
        );
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of View/segment blocks", &mut nbval)?;
        if st && (nbval > 0) {
            views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
            breakpoint_parameters = iges_new_array1::<f64>(1, nbval)?;
            display_flags = iges_new_array1::<i32>(1, nbval)?;
            color_values = iges_new_array1::<i32>(1, nbval)?;
            color_definitions = iges_new_array1::<Handle<IgesGraphColor>>(1, nbval)?;
            line_font_values = iges_new_array1::<i32>(1, nbval)?;
            line_font_definitions =
                iges_new_array1::<Handle<dyn IgesDataLineFontEntity>>(1, nbval)?;
            line_weights = iges_new_array1::<i32>(1, nbval)?;
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> = Default::default();
            let mut temp_color_value: i32 = Default::default();
            let mut temp_color_def: Handle<IgesGraphColor> = Default::default();
            let mut temp_line_font_value: i32 = Default::default();
            let mut temp_line_font_def: Handle<dyn IgesDataLineFontEntity> = Default::default();
            for i in 1..=nbval {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Instance of views",
                    <dyn IgesDataViewKindEntity>::static_type(),
                    &mut temp_view,
                    false,
                )? {
                    iges_array1_set(
                        &views,
                        i,
                        temp_view.clone(),
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                }
                iges_read_real_item(
                    pr,
                    "array breakpointParameters",
                    &breakpoint_parameters,
                    i,
                    "IgesDrawToolSegmentedViewsVisible::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "array displayFlags",
                    &display_flags,
                    i,
                    "IgesDrawToolSegmentedViewsVisible::read_own_params",
                )?;
                let mut curnum: i32 = pr.current_number();
                pr.read_integer_text(&pr.current(), "array colorValues", &mut temp_color_value)?;
                if temp_color_value < 0 {
                    iges_array1_set(
                        &color_values,
                        i,
                        -1,
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                    temp_color_def = pr.param_entity(ir, curnum)?.cast::<IgesGraphColor>();
                    if temp_color_def.is_null() {
                        pr.add_fail("A Color Definition Entity is incorrect", "");
                    } else {
                        iges_array1_set(
                            &color_definitions,
                            i,
                            temp_color_def.clone(),
                            "IgesDrawToolSegmentedViewsVisible::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &color_values,
                        i,
                        temp_color_value,
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                }
                curnum = pr.current_number();
                pr.read_integer_text(
                    &pr.current(),
                    "array lineFontValues",
                    &mut temp_line_font_value,
                )?;
                if temp_line_font_value < 0 {
                    iges_array1_set(
                        &line_font_values,
                        i,
                        -1,
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                    temp_line_font_def = pr
                        .param_entity(ir, curnum)?
                        .cast::<dyn IgesDataLineFontEntity>();
                    if temp_line_font_def.is_null() {
                        pr.add_fail("A Line Font Definition Entity is incorrect", "");
                    }
                    iges_array1_set(
                        &line_font_definitions,
                        i,
                        temp_line_font_def.clone(),
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                } else {
                    iges_array1_set(
                        &line_font_values,
                        i,
                        temp_line_font_value,
                        "IgesDrawToolSegmentedViewsVisible::read_own_params",
                    )?;
                }
                iges_read_integer_item(
                    pr,
                    "array lineWeights",
                    &line_weights,
                    i,
                    "IgesDrawToolSegmentedViewsVisible::read_own_params",
                )?;
            }
        } else {
            pr.add_fail("No. of View/segment blocks : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &views,
            &breakpoint_parameters,
            &display_flags,
            &color_values,
            &color_definitions,
            &line_font_values,
            &line_font_definitions,
            &line_weights,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawSegmentedViewsVisible>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_segment_blocks()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send_entity(&ent.view_item(i)?, false)?;
            iw.send(ent.breakpoint_parameter(i)?)?;
            iw.send(ent.display_flag(i)?)?;
            if ent.is_color_definition(i)? {
                iw.send_entity(&ent.color_definition(i)?, true)?;
            } else {
                iw.send(ent.color_value(i)?)?;
            }
            if ent.is_font_definition(i)? {
                iw.send_entity(&ent.line_font_definition(i)?, true)?;
            } else {
                iw.send(ent.line_font_value(i)?)?;
            }
            iw.send(ent.line_weight_item(i)?)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a SegmentedViewsVisible `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawSegmentedViewsVisible>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_segment_blocks()?;
        for i in 1..=up {
            iter.get_one_item(&ent.view_item(i)?.cast());
            if ent.is_color_definition(i)? {
                iter.get_one_item(&ent.color_definition(i)?.cast());
            }
            if !ent.is_font_definition(i)? {
                iter.get_one_item(&ent.line_font_definition(i)?.cast());
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDrawSegmentedViewsVisible>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 19);
        crate::iges_shared_blocks::iges_block_0013!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDrawSegmentedViewsVisible>,
        _unused_1: &InterfaceShareTool,
        _unused_2: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawSegmentedViewsVisible>,
        ent: &Handle<IgesDrawSegmentedViewsVisible>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0004!(
            breakpoint_parameters,
            color_definitions,
            color_values,
            display_flags,
            line_font_definitions,
            line_font_values,
            line_weights,
            nbval,
            views
        );
        let mut ret_view: Handle<dyn IgesDataViewKindEntity> = Default::default();
        let mut ret_color_def: Handle<IgesGraphColor> = Default::default();
        let mut ret_line_font_def: Handle<dyn IgesDataLineFontEntity> = Default::default();
        nbval = another.nb_segment_blocks()?;
        views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
        breakpoint_parameters = iges_new_array1::<f64>(1, nbval)?;
        display_flags = iges_new_array1::<i32>(1, nbval)?;
        color_values = iges_new_array1::<i32>(1, nbval)?;
        line_font_values = iges_new_array1::<i32>(1, nbval)?;
        color_definitions = iges_new_array1::<Handle<IgesGraphColor>>(1, nbval)?;
        line_font_definitions = iges_new_array1::<Handle<dyn IgesDataLineFontEntity>>(1, nbval)?;
        line_weights = iges_new_array1::<i32>(1, nbval)?;
        for i in 1..=nbval {
            ret_view = another.view_item(i)?;
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> =
                tc.transferred(&ret_view.cast())?.cast();
            iges_array1_set(
                &views,
                i,
                temp_view.clone(),
                "IgesDrawToolSegmentedViewsVisible::own_copy",
            )?;
            iges_array1_set(
                &breakpoint_parameters,
                i,
                another.breakpoint_parameter(i)?,
                "IgesDrawToolSegmentedViewsVisible::own_copy",
            )?;
            iges_array1_set(
                &display_flags,
                i,
                another.display_flag(i)?,
                "IgesDrawToolSegmentedViewsVisible::own_copy",
            )?;
            if another.is_color_definition(i)? {
                ret_color_def = another.color_definition(i)?;
                let mut temp_color_def: Handle<IgesGraphColor> =
                    tc.transferred(&ret_color_def.cast())?.cast();
                iges_array1_set(
                    &color_definitions,
                    i,
                    temp_color_def.clone(),
                    "IgesDrawToolSegmentedViewsVisible::own_copy",
                )?;
            } else {
                iges_array1_set(
                    &color_values,
                    i,
                    another.color_value(i)?,
                    "IgesDrawToolSegmentedViewsVisible::own_copy",
                )?;
            }
            if another.is_font_definition(i)? {
                ret_line_font_def = another.line_font_definition(i)?;
                let mut temp_line_font_def: Handle<dyn IgesDataLineFontEntity> =
                    tc.transferred(&ret_line_font_def.cast())?.cast();
                iges_array1_set(
                    &line_font_definitions,
                    i,
                    temp_line_font_def.clone(),
                    "IgesDrawToolSegmentedViewsVisible::own_copy",
                )?;
            } else {
                iges_array1_set(
                    &line_font_values,
                    i,
                    another.line_font_value(i)?,
                    "IgesDrawToolSegmentedViewsVisible::own_copy",
                )?;
            }
            iges_array1_set(
                &line_weights,
                i,
                another.line_weight_item(i)?,
                "IgesDrawToolSegmentedViewsVisible::own_copy",
            )?;
        }
        ent.init(
            &views,
            &breakpoint_parameters,
            &display_flags,
            &color_values,
            &color_definitions,
            &line_font_values,
            &line_font_definitions,
            &line_weights,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawSegmentedViewsVisible>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        writeln!(
            s,
            "IgesDrawSegmentedViewsVisible\nView Entities            :\nBreakpoint parameters    :\nDisplay flags            :\nColor Values             :\nColor Definitions        :\nLineFont Values          :\nLineFont Definitions     :\nLine Weights :\nCount Of Blocks = {}",
            ent.nb_segment_blocks()?
        )?;
        match level {
            4 => {
                writeln!(s, " [ for content, ask level > 4 ]")?;
            }
            5 | 6 => {
                let mut i: i32 = Default::default();
                let mut up: i32 = ent.nb_segment_blocks()?;
                for i in 1..=up {
                    write!(s, "[{}]:\nView Entity : ", i)?;
                    dumper.dump(&ent.view_item(i)?, s, sublevel, -1)?;
                    writeln!(
                        s,
                        "\nBreakpoint parameter : {}\nDisplay Flag : {}",
                        iges_real_text(ent.breakpoint_parameter(i)?),
                        ent.display_flag(i)?
                    )?;
                    if ent.color_definition(i)?.is_null() {
                        write!(s, "Color Value : {}", ent.color_value(i)?)?;
                    } else {
                        write!(s, "Color Definition : ")?;
                        dumper.dump(&ent.color_definition(i)?, s, sublevel, -1)?;
                    }
                    writeln!(s)?;
                    if ent.line_font_definition(i)?.is_null() {
                        write!(s, "LineFont Value : {}", ent.line_font_value(i)?)?;
                    } else {
                        write!(s, "LineFont Definition : ")?;
                        dumper.dump(&ent.line_font_definition(i)?, s, sublevel, -1)?;
                    }
                    writeln!(s, "\nLine Weight : {}", ent.line_weight_item(i)?)?;
                }
            }
            _ => {}
        }
        writeln!(s)?;
        Ok(())
    }
}
