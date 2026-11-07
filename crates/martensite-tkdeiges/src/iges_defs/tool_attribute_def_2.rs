//! More of IgesDefsToolAttributeDef.
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
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesEntity, IgesDataIgesWriter,
};
use crate::iges_defs::{IgesDefsAttributeDef, IgesDefsToolAttributeDef};
use crate::iges_tool_support::iges_buffer_first;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceEntityIterator, InterfaceShareTool};

impl IgesDefsToolAttributeDef {
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsAttributeDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        if ent.has_table_name() {
            iw.send(ent.table_name())?;
        } else {
            iw.send_void()?;
        }
        iw.send(ent.list_type())?;
        let mut upper: i32 = ent.nb_attributes()?;
        iw.send(upper)?;
        for i in 1..=upper {
            let mut check: i32 = ent.attribute_value_data_type(i)?;
            let mut count: i32 = ent.attribute_value_count(i)?;
            iw.send(ent.attribute_type(i)?)?;
            iw.send(check)?;
            iw.send(count)?;
            if ent.form_number() > 0 {
                for j in 1..=count {
                    match check {
                        0 => {
                            iw.send_void()?;
                        }
                        1 => {
                            iw.send(ent.attribute_as_integer(i, j)?)?;
                        }
                        2 => {
                            iw.send(ent.attribute_as_real(i, j)?)?;
                        }
                        3 => {
                            iw.send(ent.attribute_as_string(i, j)?)?;
                        }
                        4 => {
                            iw.send_entity(&ent.attribute_as_entity(i, j)?, false)?;
                        }
                        5 => {
                            iw.send_void()?;
                        }
                        6 => {
                            iw.send_boolean(ent.attribute_as_logical(i, j)?)?;
                        }
                        _ => {}
                    }
                    if ent.form_number() == 2 {
                        iw.send_entity(&ent.attribute_text_display(i, j)?, false)?;
                    }
                }
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a AttributeDef `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDefsAttributeDef>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_attributes()?;
        for i in 1..=upper {
            let mut check: i32 = ent.attribute_value_data_type(i)?;
            let mut count: i32 = ent.attribute_value_count(i)?;
            if ent.form_number() > 0 {
                for j in 1..=count {
                    if check == 4 {
                        iter.get_one_item(&ent.attribute_as_entity(i, j)?.cast());
                    }
                    if ent.form_number() == 2 {
                        iter.get_one_item(&ent.attribute_text_display(i, j)?.cast());
                    }
                }
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsAttributeDef>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(322, 0, 2);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.line_weight(IgesDataDefType::DefValue);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.subordinate_status_required(0);
        dc.use_flag_required(2);
        dc.graphics_ignored(1);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDefsAttributeDef>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut nb: i32 = ent.nb_attributes()?;
        let mut fn_: i32 = ent.form_number();
        for i in 1..=nb {
            let mut mess = String::new();
            if (ent.attribute_type(i)? < 0) || (ent.attribute_type(i)? > 9999) {
                mess = format!(
                    "Attribute Type n0.{} not in <0 - 9999>",
                    ent.attribute_type(i)?
                );
                ach.add_fail_text(&mess, "");
            }
            let mut aty: i32 = ent.attribute_value_data_type(i)?;
            if (aty < 0) || (aty > 6) {
                mess = format!("Attribute Value Data Type n0.{} not in <0 - 6>", aty);
                ach.add_fail_text(&mess, "");
            }
            if ent.attribute_value_count(i)? <= 0 {
                continue;
            }
            let mut list: Handle<StandardTransient> = ent.attribute_list(i)?;
            if (fn_ > 0) && ent.is_null() {
                if (aty == 0) || (aty == 5) {
                    continue;
                }
                mess = format!(
                    "Form Number > 0 and Attribute Value List n0.{} undefined",
                    aty
                );
                ach.add_fail_text(&mess, "");
                continue;
            }

            if fn_ == 0 {
                continue;
            }

            mess.clear();
            match aty {
                1 => {
                    if !(list.is_kind(<HArray1<i32>>::static_type())) {
                        mess = format!("Attribute List n0.{} (Integers) badly defined", aty);
                    }
                }
                2 => {
                    if !(list.is_kind(<HArray1<f64>>::static_type())) {
                        mess = format!("Attribute List n0.{} (Reals) badly defined", aty);
                    }
                }
                3 => {
                    if !(list.is_kind(<HArray1<Handle<HAsciiString>>>::static_type())) {
                        mess = format!("Attribute List n0.{} (Strings) badly defined", aty);
                    }
                }
                4 => {
                    if !(list.is_kind(<HArray1<Handle<IgesDataIgesEntity>>>::static_type())) {
                        mess = format!("Attribute List n0.{} (IGES Pointers) badly defined", aty);
                    }
                }
                6 => {
                    if !(list.is_kind(<HArray1<i32>>::static_type())) {
                        mess = format!(
                            "Attribute List n0.{} (Logicals i.e. Integers) badly defined",
                            aty
                        );
                    }
                }
                _ => {}
            }
            if iges_buffer_first(&mess) != (b'\0') {
                ach.add_fail_text(&mess, "");
            }
        }
        Ok(())
    }
}
