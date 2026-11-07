//! IgesDrawToolViewsVisibleWithAttr.
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
use crate::iges_data::iges_read_integer_item;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesEntity, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataLineFontEntity, IgesDataParamReader, IgesDataViewKindEntity,
    iges_array1_set, iges_new_array1,
};
use crate::iges_draw::IgesDrawViewsVisibleWithAttr;
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::InterfaceEntityIterator;

iges_tool!(
    IgesDrawToolViewsVisibleWithAttr,
    "The own-parameter work on `IgesDrawViewsVisibleWithAttr`."
);

impl IgesDrawToolViewsVisibleWithAttr {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_blocks: i32 = Default::default();
        let mut temp_nb_entity: i32 = Default::default();
        let mut temp_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
            Default::default();
        let mut temp_color_definitions: Handle<HArray1<Handle<IgesGraphColor>>> =
            Default::default();
        let mut temp_line_fonts: Handle<HArray1<i32>> = Default::default();
        let mut temp_color_values: Handle<HArray1<i32>> = Default::default();
        let mut temp_line_weights: Handle<HArray1<i32>> = Default::default();
        let mut temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        let mut temp_line_definitions: Handle<HArray1<Handle<dyn IgesDataLineFontEntity>>> =
            Default::default();
        if pr.read_integer_text(&pr.current(), "Number Of Blocks", &mut temp_nb_blocks)? {
            if temp_nb_blocks <= 0 {
                pr.add_fail("Number Of Blocks : Not Positive", "");
            } else {
                temp_view_entities =
                    iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, temp_nb_blocks)?;
                temp_line_fonts = iges_new_array1::<i32>(1, temp_nb_blocks)?;
                temp_line_definitions =
                    iges_new_array1::<Handle<dyn IgesDataLineFontEntity>>(1, temp_nb_blocks)?;
                temp_color_values = iges_new_array1::<i32>(1, temp_nb_blocks)?;
                temp_color_definitions =
                    iges_new_array1::<Handle<IgesGraphColor>>(1, temp_nb_blocks)?;
                temp_line_weights = iges_new_array1::<i32>(1, temp_nb_blocks)?;
            }
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Number of Entities Displayed",
                &mut temp_nb_entity,
            )?;
        } else {
            temp_nb_entity = 0;
            pr.add_warning("Number of Entities Displayed : undefined, set to Zero", "");
        }
        if temp_nb_entity < 0 {
            pr.add_fail("Number Of Entities Displayed : Less than Zero", "");
        }
        if !temp_view_entities.is_null() {
            let mut i: i32 = Default::default();
            for i in 1..=temp_nb_blocks {
                let mut temp_view: Handle<dyn IgesDataViewKindEntity> = Default::default();
                let mut temp_line_font: i32 = Default::default();
                let mut temp_entity1: Handle<dyn IgesDataLineFontEntity> = Default::default();
                let mut temp_color_value: i32 = Default::default();
                let mut temp_color_def: Handle<IgesGraphColor> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "View Entity",
                    <dyn IgesDataViewKindEntity>::static_type(),
                    &mut temp_view,
                    false,
                )? {
                    iges_array1_set(
                        &temp_view_entities,
                        i,
                        temp_view.clone(),
                        "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                    )?;
                }
                if pr.read_integer_text(&pr.current(), "Line Font Value", &mut temp_line_font)? {
                    iges_array1_set(
                        &temp_line_fonts,
                        i,
                        temp_line_font,
                        "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                    )?;
                }
                if temp_line_font != 0 {
                    pr.set_current_number(pr.current_number() + 1);
                }
                if (temp_line_font == 0)
                    && (pr.read_entity_typed(
                        ir,
                        &pr.current(),
                        "Line Font Definition",
                        <dyn IgesDataLineFontEntity>::static_type(),
                        &mut temp_entity1,
                        true,
                    )?)
                {
                    iges_array1_set(
                        &temp_line_definitions,
                        i,
                        temp_entity1.clone(),
                        "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                    )?;
                }
                let mut curnum: i32 = pr.current_number();
                if pr.defined_else_skip()? {
                    pr.read_integer_text(&pr.current(), "Color Value", &mut temp_color_value)?;
                } else {
                    temp_color_value = 0;
                    pr.add_warning("Color Value : undefined, set to Zero", "");
                }
                if temp_color_value < 0 {
                    iges_array1_set(
                        &temp_color_values,
                        i,
                        -1,
                        "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                    )?;
                    temp_color_def = pr.param_entity(ir, curnum)?.cast::<IgesGraphColor>();
                    if temp_color_def.is_null() {
                        pr.add_fail("A Color Definition Entity is incorrect", "");
                    } else {
                        iges_array1_set(
                            &temp_color_definitions,
                            i,
                            temp_color_def.clone(),
                            "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &temp_color_values,
                        i,
                        temp_color_value,
                        "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                    )?;
                }
                iges_read_integer_item(
                    pr,
                    "Line Weight Value",
                    &temp_line_weights,
                    i,
                    "IgesDrawToolViewsVisibleWithAttr::read_own_params",
                )?;
            }
        }
        if temp_nb_entity > 0 {
            pr.read_ents(
                ir,
                &pr.current_list(temp_nb_entity, 1),
                "Displayed Entities",
                &mut temp_display_entities,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
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
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_views()?;
        iw.send(up)?;
        iw.send(ent.nb_displayed_entities()?)?;
        let mut i: i32 = Default::default();
        for i in 1..=up {
            iw.send_entity(&ent.view_item(i)?, false)?;
            iw.send(ent.line_font_value(i)?)?;
            iw.send_entity(&ent.font_definition(i)?, false)?;
            if ent.is_color_definition(i)? {
                iw.send_entity(&ent.color_definition(i)?, true)?;
            } else {
                iw.send(ent.color_value(i)?)?;
            }
            iw.send(ent.line_weight_item(i)?)?;
        }
        up = ent.nb_displayed_entities()?;
        for i in 1..=up {
            iw.send_entity(&ent.displayed_entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a ViewsVisibleWithAttr `ent`, from
    /// its specific (own) parameters shared not implied, i.e. all but
    /// the Displayed Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_views()?;
        let mut i: i32 = Default::default();
        for i in 1..=up {
            iter.get_one_item(&ent.view_item(i)?.cast());
            iter.get_one_item(&ent.font_definition(i)?.cast());
            if ent.is_color_definition(i)? {
                iter.get_one_item(&ent.color_definition(i)?.cast());
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a ViewsVisible `ent`, from
    /// its specific (own) implied parameters : the Displayed Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_implied(
        &self,
        ent: &Handle<IgesDrawViewsVisibleWithAttr>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0045!(ent, i, iter, up);
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDrawViewsVisibleWithAttr>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 4);
        crate::iges_shared_blocks::iges_block_0013!(dc);
        dc
    }
}
