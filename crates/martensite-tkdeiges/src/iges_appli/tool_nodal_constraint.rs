//! IgesAppliToolNodalConstraint.
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
use crate::iges_appli::{IgesAppliNodalConstraint, IgesAppliNode};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_entities, iges_new_array1,
};
use crate::iges_defs::IgesDefsTabularData;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolNodalConstraint,
    "The own-parameter work on `IgesAppliNodalConstraint`."
);

impl IgesAppliToolNodalConstraint {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliNodalConstraint>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut num: i32 = Default::default();
        let mut i: i32 = Default::default();
        let mut temp_type: i32 = Default::default();
        let mut temp_node: Handle<IgesAppliNode> = Default::default();
        let mut temp_tabular_data_props: Handle<HArray1<Handle<IgesDefsTabularData>>> =
            Default::default();
        if !(pr.read_integer_text(&pr.current(), "Number of cases", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_tabular_data_props = iges_new_array1::<Handle<IgesDefsTabularData>>(1, num)?;
        } else {
            pr.add_fail("Number of cases: Not Positive", "");
        }
        pr.read_integer_text(&pr.current(), "Type of Constraint", &mut temp_type)?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Node",
            <IgesAppliNode>::static_type(),
            &mut temp_node,
            false,
        )?;
        if !temp_tabular_data_props.is_null() {
            for i in 1..=num {
                let mut temp_entity: Handle<IgesDefsTabularData> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Tabular Data Property",
                    <IgesDefsTabularData>::static_type(),
                    &mut temp_entity,
                    false,
                )? {
                    iges_array1_set(
                        &temp_tabular_data_props,
                        i,
                        temp_entity.clone(),
                        "IgesAppliToolNodalConstraint::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_type, &temp_node, &temp_tabular_data_props)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliNodalConstraint>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_cases()?)?;
        iw.send(ent.type_())?;
        iw.send_entity(&ent.node_entity(), false)?;
        {
            num = ent.nb_cases()?;
            i = 1;
            while i <= num {
                iw.send_entity(&ent.tabular_data(i)?, false)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a NodalConstraint `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliNodalConstraint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iter.get_one_item(&ent.node_entity().cast());
        {
            num = ent.nb_cases()?;
            i = 1;
            while i <= num {
                iter.get_one_item(&ent.tabular_data(i)?.cast());
                i += 1;
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliNodalConstraint>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(418, 0);
        crate::iges_shared_blocks::iges_block_0037!(dc);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliNodalConstraint>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (ent.type_() != 1) && (ent.type_() != 2) {
            ach.add_fail_text("Type of Constraint != 1,2", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliNodalConstraint>,
        ent: &Handle<IgesAppliNodalConstraint>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut num: i32 = another.nb_cases()?;
        let mut temp_type: i32 = another.type_();
        let mut temp_node: Handle<IgesAppliNode> =
            tc.transferred(&another.node_entity().cast())?.cast();
        let mut temp_tabular_data_props: Handle<HArray1<Handle<IgesDefsTabularData>>> =
            iges_new_array1::<Handle<IgesDefsTabularData>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesDefsTabularData> =
                tc.transferred(&another.tabular_data(i)?.cast())?.cast();
            iges_array1_set(
                &temp_tabular_data_props,
                i,
                new_item.clone(),
                "IgesAppliToolNodalConstraint::own_copy",
            )?;
        }
        ent.init(temp_type, &temp_node, &temp_tabular_data_props)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliNodalConstraint>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        writeln!(s, "IgesAppliNodalConstraint")?;
        writeln!(s, "Type of Constraint : {}", ent.type_())?;
        write!(s, "Node : ")?;
        dumper.dump(&ent.node_entity(), s, sublevel, -1)?;
        writeln!(s)?;
        write!(s, "Tabular Data Properties : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_cases()?, |first| {
            ent.tabular_data(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
