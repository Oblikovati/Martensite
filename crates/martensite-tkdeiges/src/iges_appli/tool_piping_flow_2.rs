//! More of IgesAppliToolPipingFlow.
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
use crate::iges_appli::{IgesAppliPipingFlow, IgesAppliToolPipingFlow};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesEntity, IgesDataIgesWriter, iges_array1_set,
    iges_live, iges_new_array1,
};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};

impl IgesAppliToolPipingFlow {
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliPipingFlow>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0003!(ent, i, iw, num);
        crate::iges_shared_blocks::iges_block_0018!(ent, i, iw, num);
        Ok(())
    }
    /// Lists the Entities shared by a PipingFlow `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliPipingFlow>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0015!(ent, i, iter, num);
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a PipingFlow
    /// (NbContextFlags forced to 1)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliPipingFlow>) -> Result<bool, Failure> {
        Ok(ent.own_correct())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliPipingFlow>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 20);
        crate::iges_shared_blocks::iges_block_0010!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliPipingFlow>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_context_flags() != 1 {
            ach.add_fail_text("Number of Context Flags != 1", "");
        }
        if (ent.type_of_flow() < 0) || (ent.type_of_flow() > 2) {
            ach.add_fail_text("Type of Flow != 0,1,2", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliPipingFlow>,
        ent: &Handle<IgesAppliPipingFlow>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_nb_context_flags: i32 = another.nb_context_flags();
        let mut temp_type_of_flow: i32 = another.type_of_flow();
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        num = another.nb_flow_associativities()?;
        let mut temp_flow_assocs: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesDataIgesEntity> = tc
                .transferred(&another.flow_associativity(i)?.cast())?
                .cast();
            iges_array1_set(
                &temp_flow_assocs,
                i,
                new_item.clone(),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
        num = another.nb_connect_points()?;
        let mut temp_connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
            iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesDrawConnectPoint> =
                tc.transferred(&another.connect_point(i)?.cast())?.cast();
            iges_array1_set(
                &temp_connect_points,
                i,
                new_item.clone(),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
        num = another.nb_joins()?;
        let mut temp_joins: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.join(i)?.cast())?.cast();
            iges_array1_set(
                &temp_joins,
                i,
                new_item.clone(),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
        num = another.nb_flow_names()?;
        let mut temp_flow_names: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_flow_names,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another.flow_name(i)?,
                    "IgesAppliToolPipingFlow::own_copy",
                )?)),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
        num = another.nb_text_display_templates()?;
        let mut temp_text_display_templates: Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>> =
            iges_new_array1::<Handle<IgesGraphTextDisplayTemplate>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesGraphTextDisplayTemplate> = tc
                .transferred(&another.text_display_template(i)?.cast())?
                .cast();
            iges_array1_set(
                &temp_text_display_templates,
                i,
                new_item.clone(),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
        num = another.nb_cont_flow_associativities()?;
        let mut temp_cont_flow_assocs: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        for i in 1..=num {
            let mut new_item: Handle<IgesDataIgesEntity> = tc
                .transferred(&another.cont_flow_associativity(i)?.cast())?
                .cast();
            iges_array1_set(
                &temp_cont_flow_assocs,
                i,
                new_item.clone(),
                "IgesAppliToolPipingFlow::own_copy",
            )?;
        }
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
