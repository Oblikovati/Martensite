//! More of IgesDrawToolViewsVisibleWithAttr.
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
use crate::iges_data::{
    IgesDataDumpResult, IgesDataIgesDumper, IgesDataIgesEntity, IgesDataLineFontEntity,
    IgesDataViewKindEntity, iges_array1_set, iges_dump_entities, iges_live, iges_new_array1,
};
use crate::iges_draw::{IgesDrawToolViewsVisibleWithAttr, IgesDrawViewsVisibleWithAttr};
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesDrawToolViewsVisibleWithAttr {
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut nb: i32 = ent.nb_views()?;
        let mut i: i32 = Default::default();
        for i in 1..=nb {
            if (ent.line_font_value(i)? != 0) && ent.is_font_definition(i)? {
                ach.add_fail_text(
                    "At least one Line Font Definition Mismatch (both Value and Entity",
                    "",
                );
            }
        }
        let mut entcomp: Handle<dyn IgesDataViewKindEntity> = ent.cast();
        let mut res: i32 = 0;
        nb = ent.nb_displayed_entities()?;
        for i in 1..=nb {
            let mut displayed: Handle<IgesDataIgesEntity> = ent.displayed_entity(i)?;
            if entcomp.object_address()
                != iges_live(&displayed, "IgesDrawToolViewsVisibleWithAttr::own_check")?
                    .view()
                    .object_address()
            {
                res += 1;
            }
        }
        crate::iges_shared_blocks::iges_block_0050!(ach, mess, res);
        Ok(())
    }
    /// Copies Specific Parameters shared not implied, i.e. all but
    /// the Displayed Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawViewsVisibleWithAttr>,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut up: i32 = another.nb_views()?;
        let mut temp_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
            iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, up)?;
        let mut temp_line_fonts: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, up)?;
        let mut temp_line_definitions: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>> =
            iges_new_array1::<Handle<dyn IgesDataLineFontEntity>>(1, up)?;
        let mut temp_color_values: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, up)?;
        let mut temp_color_definitions: Handle<HArray1<Handle<IgesGraphColor>>> =
            iges_new_array1::<Handle<IgesGraphColor>>(1, up)?;
        let mut temp_line_weights: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, up)?;
        for i in 1..=up {
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> =
                tc.transferred(&another.view_item(i)?.cast())?.cast();
            iges_array1_set(
                &temp_view_entities,
                i,
                temp_view.clone(),
                "IgesDrawToolViewsVisibleWithAttr::own_copy",
            )?;
            let mut temp_line_font: i32 = another.line_font_value(i)?;
            iges_array1_set(
                &temp_line_fonts,
                i,
                temp_line_font,
                "IgesDrawToolViewsVisibleWithAttr::own_copy",
            )?;
            if another.is_font_definition(i)? {
                let mut temp_entity1: Handle<dyn IgesDataLineFontEntity> =
                    tc.transferred(&another.font_definition(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_line_definitions,
                    i,
                    temp_entity1.clone(),
                    "IgesDrawToolViewsVisibleWithAttr::own_copy",
                )?;
            }
            if another.is_color_definition(i)? {
                let mut temp_entity2: Handle<IgesGraphColor> =
                    tc.transferred(&another.color_definition(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_color_definitions,
                    i,
                    temp_entity2.clone(),
                    "IgesDrawToolViewsVisibleWithAttr::own_copy",
                )?;
            } else {
                let mut temp_color_value: i32 = another.color_value(i)?;
                iges_array1_set(
                    &temp_color_values,
                    i,
                    temp_color_value,
                    "IgesDrawToolViewsVisibleWithAttr::own_copy",
                )?;
            }
            let mut temp_line_weight: i32 = another.line_weight_item(i)?;
            iges_array1_set(
                &temp_line_weights,
                i,
                temp_line_weight,
                "IgesDrawToolViewsVisibleWithAttr::own_copy",
            )?;
        }
        let mut temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        ent.init(
            &temp_view_entities,
            &temp_line_fonts,
            &temp_line_definitions,
            &temp_color_values,
            &temp_color_definitions,
            &temp_line_weights,
            &temp_display_entities,
        )?;
        Ok(())
    }
    /// Copies Specific implied Parameters : the Displayed Entities
    /// which have already been copied
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_renew(
        &self,
        another: &Handle<IgesDrawViewsVisibleWithAttr>,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        tc: &InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0002!(
            anew,
            another,
            i,
            newdisp,
            tc,
            temp_display_entities,
            up
        );
        {
            newdisp.start();
            while newdisp.more() {
                i += 1;
                let mut temp_entity: Handle<IgesDataIgesEntity> = newdisp.value()?.cast();
                iges_array1_set(
                    &temp_display_entities,
                    i,
                    temp_entity.clone(),
                    "IgesDrawToolViewsVisibleWithAttr::own_renew",
                )?;
                newdisp.next();
            }
        }
        ent.init_implied(&temp_display_entities)?;
        Ok(())
    }
    /// Clears specific implied parameters, which cause looping
    /// structures; required for deletion
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_when_delete(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
    ) -> Result<(), Failure> {
        let mut temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        ent.init_implied(&temp_display_entities)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        writeln!(
            s,
            "IgesDrawViewsVisibleWithAttr\nView Entities            :\nLine Font Values         :\nLine Font Definitions    :\nColor Number/Definitions :\nLine Weights             :\nCount of View Blocks : {}",
            ent.nb_views()?
        )?;
        if level > 4 {
            let mut i: i32 = Default::default();
            let mut upper: i32 = ent.nb_views()?;
            for i in 1..=upper {
                write!(s, "[{}]:\nView Entity : ", i)?;
                dumper.dump(&ent.view_item(i)?, s, temp_sub_level, -1)?;
                writeln!(s)?;
                if ent.is_font_definition(i)? {
                    write!(s, "Line Font Definition  : ")?;
                    dumper.dump(&ent.font_definition(i)?, s, temp_sub_level, -1)?;
                    writeln!(s)?;
                } else {
                    writeln!(s, "Line Font Value       : {}", ent.line_font_value(i)?)?;
                }
                if ent.is_color_definition(i)? {
                    write!(s, "Color Definition : ")?;
                    dumper.dump(&ent.color_definition(i)?, s, temp_sub_level, -1)?;
                    writeln!(s)?;
                } else {
                    writeln!(s, "Color Value      : {}", ent.color_value(i)?)?;
                }
                writeln!(s, "Line Weight      : {}", ent.line_weight_item(i)?)?;
            }
        }
        write!(s, "Displayed Entities : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_displayed_entities()?, |first| {
            ent.displayed_entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
