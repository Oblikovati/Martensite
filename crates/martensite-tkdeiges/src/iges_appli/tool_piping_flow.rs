//! IgesAppliToolPipingFlow.
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
use crate::iges_appli::IgesAppliPipingFlow;
use crate::iges_basic::entity_shape::iges_array1_length;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::iges_read_text_item;
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataParamReader, iges_array1_set,
    iges_new_array1,
};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;

iges_tool!(
    IgesAppliToolPipingFlow,
    "The own-parameter work on `IgesAppliPipingFlow`."
);

impl IgesAppliToolPipingFlow {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliPipingFlow>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_context_flags: i32 = Default::default();
        let mut temp_type_of_flow: i32 = Default::default();
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
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
            temp_nb_context_flags = 1;
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Flow Associativities", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_flow_assocs = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        } else {
            pr.add_fail("Number of Flow Associativities: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Connect Points", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_connect_points = iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, num)?;
        } else {
            pr.add_fail("Number of Connect Points: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Joins", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_joins = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        } else {
            pr.add_fail("Number of Joins: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Flow Names", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_flow_names = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        } else {
            pr.add_fail("Number of Flow Names: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Text Displays", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_text_display_templates =
                iges_new_array1::<Handle<IgesGraphTextDisplayTemplate>>(1, num)?;
        } else {
            pr.add_fail("Number of Text Displays: Not Positive", "");
        }
        if !(pr.read_integer_text(&pr.current(), "Number of Continuation Flows", &mut num)?) {
            num = 0;
        }
        if num > 0 {
            temp_cont_flow_assocs = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        } else {
            pr.add_fail("Number of Continuation Flows: Not Positive", "");
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Type of Flow", &mut temp_type_of_flow)?;
        } else {
            temp_type_of_flow = 0;
        }
        if !temp_flow_assocs.is_null() {
            {
                num = iges_array1_length(
                    &temp_flow_assocs,
                    "IgesAppliToolPipingFlow::read_own_params",
                )?;
                i = 1;
                while i <= num {
                    let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
                    if pr.read_entity_typed(
                        ir,
                        &pr.current(),
                        "Flow Associativity",
                        <IgesDataIgesEntity>::static_type(),
                        &mut temp_entity,
                        false,
                    )? {
                        iges_array1_set(
                            &temp_flow_assocs,
                            i,
                            temp_entity.clone(),
                            "IgesAppliToolPipingFlow::read_own_params",
                        )?;
                    }
                    i += 1;
                }
            }
        }
        if !temp_connect_points.is_null() {
            {
                num = iges_array1_length(
                    &temp_connect_points,
                    "IgesAppliToolPipingFlow::read_own_params",
                )?;
                i = 1;
                while i <= num {
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
                            "IgesAppliToolPipingFlow::read_own_params",
                        )?;
                    }
                    i += 1;
                }
            }
        }
        if !temp_joins.is_null() {
            {
                num = iges_array1_length(&temp_joins, "IgesAppliToolPipingFlow::read_own_params")?;
                i = 1;
                while i <= num {
                    let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
                    if pr.read_entity(ir, &pr.current(), "Join", &mut temp_entity, false)? {
                        iges_array1_set(
                            &temp_joins,
                            i,
                            temp_entity.clone(),
                            "IgesAppliToolPipingFlow::read_own_params",
                        )?;
                    }
                    i += 1;
                }
            }
        }
        if !temp_flow_names.is_null() {
            {
                num = iges_array1_length(
                    &temp_flow_names,
                    "IgesAppliToolPipingFlow::read_own_params",
                )?;
                i = 1;
                while i <= num {
                    iges_read_text_item(
                        pr,
                        "Flow Name",
                        &temp_flow_names,
                        i,
                        "IgesAppliToolPipingFlow::read_own_params",
                    )?;
                    i += 1;
                }
            }
        }
        if !temp_text_display_templates.is_null() {
            {
                num = iges_array1_length(
                    &temp_text_display_templates,
                    "IgesAppliToolPipingFlow::read_own_params",
                )?;
                i = 1;
                while i <= num {
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
                            "IgesAppliToolPipingFlow::read_own_params",
                        )?;
                    }
                    i += 1;
                }
            }
        }
        if !temp_cont_flow_assocs.is_null() {
            {
                num = iges_array1_length(
                    &temp_cont_flow_assocs,
                    "IgesAppliToolPipingFlow::read_own_params",
                )?;
                i = 1;
                while i <= num {
                    let mut temp_entity: Handle<IgesDataIgesEntity> = Default::default();
                    if pr.read_entity(
                        ir,
                        &pr.current(),
                        "Continuation Flow Assocs",
                        &mut temp_entity,
                        false,
                    )? {
                        iges_array1_set(
                            &temp_cont_flow_assocs,
                            i,
                            temp_entity.clone(),
                            "IgesAppliToolPipingFlow::read_own_params",
                        )?;
                    }
                    i += 1;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_context_flags,
            temp_type_of_flow,
            &temp_flow_assocs,
            &temp_connect_points,
            &temp_joins,
            &temp_flow_names,
            &temp_text_display_templates,
            &temp_cont_flow_assocs,
        )?;
        Ok(())
    }
}
