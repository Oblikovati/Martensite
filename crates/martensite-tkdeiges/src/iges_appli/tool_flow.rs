//! IgesAppliToolFlow.
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
use crate::iges_appli::IgesAppliFlow;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_new_array1,
};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceEntityIterator;

iges_tool!(
    IgesAppliToolFlow,
    "The own-parameter work on `IgesAppliFlow`."
);

impl IgesAppliToolFlow {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliFlow>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_context_flags: i32 = Default::default();
        let mut temp_type_of_flow: i32 = Default::default();
        let mut temp_function_flag: i32 = Default::default();
        let mut i: i32 = Default::default();
        let mut nf: i32 = Default::default();
        let mut nc: i32 = Default::default();
        let mut nj: i32 = Default::default();
        let mut nn: i32 = Default::default();
        let mut nt: i32 = Default::default();
        let mut np: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0019!(
            temp_connect_points,
            temp_cont_flow_assocs,
            temp_flow_assocs,
            temp_flow_names,
            temp_joins,
            temp_text_display_templates
        );
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Number of Context Flags",
                &mut temp_nb_context_flags,
            )?;
        } else {
            temp_nb_context_flags = 2;
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Flow Associativities", &mut nf)?) {
            nf = 0;
        }
        if nf > 0 {
            temp_flow_assocs = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nf)?;
        } else {
            pr.add_fail("Number of Flow Associativities: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Connect Points", &mut nc)?) {
            nc = 0;
        }
        if nc > 0 {
            temp_connect_points = iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, nc)?;
        } else {
            pr.add_fail("Number of Connect Points: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Joins", &mut nj)?) {
            nj = 0;
        }
        if nj > 0 {
            temp_joins = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nj)?;
        } else {
            pr.add_fail("Number of Joins: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Flow Names", &mut nn)?) {
            nn = 0;
        }
        if nn > 0 {
            temp_flow_names = iges_new_array1::<Handle<HAsciiString>>(1, nn)?;
        } else {
            pr.add_fail("Number of Flow Names: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Text Displays", &mut nt)?) {
            nt = 0;
        }
        if nt > 0 {
            temp_text_display_templates =
                iges_new_array1::<Handle<IgesGraphTextDisplayTemplate>>(1, nt)?;
        } else {
            pr.add_fail("Number of Text Displays: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Continuation Flows", &mut np)?) {
            np = 0;
        }
        if np > 0 {
            temp_cont_flow_assocs = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, np)?;
        } else {
            pr.add_fail("Number of Continuation Flows Not Positive", "");
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Type of Flow", &mut temp_type_of_flow)?;
        } else {
            temp_type_of_flow = 0;
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Function Flag", &mut temp_function_flag)?;
        } else {
            temp_function_flag = 0;
        }
        for i in 1..=nf {
            let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
            if pr.read_entity(
                ir,
                &pr.current(),
                "Flow Associativity",
                &mut temp_entity,
                false,
            )? {
                iges_array1_set(
                    &temp_flow_assocs,
                    i,
                    temp_entity.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        for i in 1..=nc {
            let mut temp_entity: Handle<IgesDrawConnectPoint> = Default::default();
            if pr.read_entity_typed(
                ir,
                &pr.current(),
                "Connect Point",
                <IgesDrawConnectPoint>::static_type(),
                &mut temp_entity,
                false,
            )? {
                iges_array1_set(
                    &temp_connect_points,
                    i,
                    temp_entity.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        for i in 1..=nj {
            let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
            if pr.read_entity(ir, &pr.current(), "Join", &mut temp_entity, false)? {
                iges_array1_set(
                    &temp_joins,
                    i,
                    temp_entity.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        for i in 1..=nn {
            let mut temp_string: Handle<HAsciiString> = Default::default();
            if pr.read_text(&pr.current(), "Flow Name", &mut temp_string)? {
                iges_array1_set(
                    &temp_flow_names,
                    i,
                    temp_string.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        for i in 1..=nt {
            let mut temp_entity: Handle<IgesGraphTextDisplayTemplate> = Default::default();
            if pr.read_entity_typed(
                ir,
                &pr.current(),
                "Text Display Template",
                <IgesGraphTextDisplayTemplate>::static_type(),
                &mut temp_entity,
                false,
            )? {
                iges_array1_set(
                    &temp_text_display_templates,
                    i,
                    temp_entity.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        for i in 1..=np {
            let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
            if pr.read_entity(
                ir,
                &pr.current(),
                "Continuation Flow Associativities",
                &mut temp_entity,
                false,
            )? {
                iges_array1_set(
                    &temp_cont_flow_assocs,
                    i,
                    temp_entity.clone(),
                    "IgesAppliToolFlow::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_context_flags,
            temp_type_of_flow,
            temp_function_flag,
            &temp_flow_assocs,
            &temp_connect_points,
            &temp_joins,
            &temp_flow_names,
            &temp_text_display_templates,
            &temp_cont_flow_assocs,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliFlow>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0003!(ent, i, iw, num);
        iw.send(ent.function_flag())?;
        crate::iges_shared_blocks::iges_block_0018!(ent, i, iw, num);
        Ok(())
    }
    /// Lists the Entities shared by a Flow `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliFlow>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0015!(ent, i, iter, num);
        Ok(())
    }
}
