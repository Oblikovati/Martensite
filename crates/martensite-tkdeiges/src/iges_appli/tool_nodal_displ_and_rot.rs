//! IgesAppliToolNodalDisplAndRot.
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
use crate::iges_appli::{IgesAppliNodalDisplAndRot, IgesAppliNode};
use crate::iges_basic::IgesBasicHArray1OfHArray1OfXyz;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_entities, iges_dump_vals, iges_live, iges_new_array1, iges_real_text,
};
use crate::iges_dimen::IgesDimenGeneralNote;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolNodalDisplAndRot,
    "The own-parameter work on `IgesAppliNodalDisplAndRot`."
);

impl IgesAppliToolNodalDisplAndRot {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliNodalDisplAndRot>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbcases: i32 = 0;
        let mut nbnodes: i32 = 0;
        let mut temp_notes: Handle<HArray1<Handle<IgesDimenGeneralNote>>> = Default::default();
        let mut temp_node_identifiers: Handle<HArray1<i32>> = Default::default();
        let mut temp_nodes: Handle<HArray1<Handle<IgesAppliNode>>> = Default::default();
        let mut temp_trans_param: Handle<IgesBasicHArray1OfHArray1OfXyz> = Default::default();
        let mut temp_rot_param: Handle<IgesBasicHArray1OfHArray1OfXyz> = Default::default();
        let mut temp_flag: bool =
            pr.read_integer_text(&pr.current(), "No. of analysis cases", &mut nbcases)?;
        if temp_flag {
            temp_notes = iges_new_array1::<Handle<IgesDimenGeneralNote>>(1, nbcases)?;
            for i in 1..=nbcases {
                let mut anentity: Handle<IgesDimenGeneralNote> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "General Note",
                    <IgesDimenGeneralNote>::static_type(),
                    &mut anentity,
                    false,
                )? {
                    iges_array1_set(
                        &temp_notes,
                        i,
                        anentity.clone(),
                        "IgesAppliToolNodalDisplAndRot::read_own_params",
                    )?;
                }
            }
        }
        if pr.read_integer_text(&pr.current(), "No. of nodes", &mut nbnodes)? {
            temp_node_identifiers = iges_new_array1::<i32>(1, nbnodes)?;
            temp_nodes = iges_new_array1::<Handle<IgesAppliNode>>(1, nbnodes)?;
            temp_trans_param =
                Handle::new(IgesBasicHArray1OfHArray1OfXyz::with_bounds(1, nbnodes)?);
            temp_rot_param = Handle::new(IgesBasicHArray1OfHArray1OfXyz::with_bounds(1, nbnodes)?);
            for j in 1..=nbnodes {
                let mut a_node: Handle<IgesAppliNode> = Handle::new(IgesAppliNode::new());
                let mut nodeno: i32 = Default::default();
                if pr.read_integer_text(&pr.current(), "Node number", &mut nodeno)? {
                    iges_array1_set(
                        &temp_node_identifiers,
                        j,
                        nodeno,
                        "IgesAppliToolNodalDisplAndRot::read_own_params",
                    )?;
                }
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Node Directory Entry",
                    <IgesAppliNode>::static_type(),
                    &mut a_node,
                    false,
                )? {
                    iges_array1_set(
                        &temp_nodes,
                        j,
                        a_node.clone(),
                        "IgesAppliToolNodalDisplAndRot::read_own_params",
                    )?;
                }
                let mut temp_array1: Handle<HArray1<Xyz>> = Default::default();
                let mut temp_array: Handle<HArray1<Xyz>> = Default::default();
                if temp_flag {
                    temp_array1 = iges_new_array1::<Xyz>(1, nbcases)?;
                    temp_array = iges_new_array1::<Xyz>(1, nbcases)?;
                    for k in 1..=nbcases {
                        let mut atrans: Xyz = Default::default();
                        let mut arot: Xyz = Default::default();
                        if pr.read_xyz(&pr.current_list(1, 3), "Translation XYZ", &mut atrans)? {
                            iges_array1_set(
                                &temp_array,
                                k,
                                atrans,
                                "IgesAppliToolNodalDisplAndRot::read_own_params",
                            )?;
                        }
                        if pr.read_xyz(&pr.current_list(1, 3), "Rotational XYZ", &mut arot)? {
                            iges_array1_set(
                                &temp_array1,
                                k,
                                arot,
                                "IgesAppliToolNodalDisplAndRot::read_own_params",
                            )?;
                        }
                    }
                }
                iges_live(
                    &temp_trans_param,
                    "IgesAppliToolNodalDisplAndRot::read_own_params",
                )?
                .set_value(j, &temp_array)?;
                iges_live(
                    &temp_rot_param,
                    "IgesAppliToolNodalDisplAndRot::read_own_params",
                )?
                .set_value(j, &temp_array1)?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &temp_notes,
            &temp_node_identifiers,
            &temp_nodes,
            &temp_rot_param,
            &temp_trans_param,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliNodalDisplAndRot>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut nbcases: i32 = ent.nb_cases()?;
        let mut nbnodes: i32 = ent.nb_nodes()?;
        iw.send(nbcases)?;
        for i in 1..=nbcases {
            iw.send_entity(&ent.note(i)?, false)?;
        }
        iw.send(nbnodes)?;
        for j in 1..=nbnodes {
            iw.send(ent.node_identifier(j)?)?;
            iw.send_entity(&ent.node(j)?, false)?;
            for k in 1..=nbcases {
                iw.send(ent.translation_parameter(j, k)?.x())?;
                iw.send(ent.translation_parameter(j, k)?.y())?;
                iw.send(ent.translation_parameter(j, k)?.z())?;
                iw.send(ent.rotational_parameter(j, k)?.x())?;
                iw.send(ent.rotational_parameter(j, k)?.y())?;
                iw.send(ent.rotational_parameter(j, k)?.z())?;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a NodalDisplAndRot `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliNodalDisplAndRot>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut nbcases: i32 = ent.nb_cases()?;
        let mut nbnodes: i32 = ent.nb_nodes()?;
        for i in 1..=nbcases {
            iter.get_one_item(&ent.note(i)?.cast());
        }
        for j in 1..=nbnodes {
            iter.get_one_item(&ent.node(j)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliNodalDisplAndRot>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(138, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesAppliNodalDisplAndRot>,
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
        another: &Handle<IgesAppliNodalDisplAndRot>,
        ent: &Handle<IgesAppliNodalDisplAndRot>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbcases: i32 = another.nb_cases()?;
        let mut nbnodes: i32 = another.nb_nodes()?;
        let mut a_notes: Handle<HArray1<Handle<IgesDimenGeneralNote>>> =
            iges_new_array1::<Handle<IgesDimenGeneralNote>>(1, nbcases)?;
        crate::iges_shared_blocks::iges_block_0071!(a_node_identifiers, a_nodes, nbnodes);
        let mut a_trans_param: Handle<IgesBasicHArray1OfHArray1OfXyz> =
            Handle::new(IgesBasicHArray1OfHArray1OfXyz::with_bounds(1, nbnodes)?);
        let mut a_rot_param: Handle<IgesBasicHArray1OfHArray1OfXyz> =
            Handle::new(IgesBasicHArray1OfHArray1OfXyz::with_bounds(1, nbnodes)?);
        for i in 1..=nbnodes {
            iges_array1_set(
                &a_node_identifiers,
                i,
                another.node_identifier(i)?,
                "IgesAppliToolNodalDisplAndRot::own_copy",
            )?;
            let mut anitem: Handle<IgesAppliNode> =
                tc.transferred(&another.node(i)?.cast())?.cast();
            iges_array1_set(
                &a_nodes,
                i,
                anitem.clone(),
                "IgesAppliToolNodalDisplAndRot::own_copy",
            )?;
        }
        for j in 1..=nbcases {
            let mut anitem: Handle<IgesDimenGeneralNote> =
                tc.transferred(&another.note(j)?.cast())?.cast();
            iges_array1_set(
                &a_notes,
                j,
                anitem.clone(),
                "IgesAppliToolNodalDisplAndRot::own_copy",
            )?;
        }
        for n in 1..=nbnodes {
            let mut temp_array1: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(1, nbcases)?;
            let mut temp_array2: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(1, nbcases)?;
            for k in 1..=nbcases {
                iges_array1_set(
                    &temp_array1,
                    k,
                    another.translation_parameter(n, k)?,
                    "IgesAppliToolNodalDisplAndRot::own_copy",
                )?;
                iges_array1_set(
                    &temp_array2,
                    k,
                    another.rotational_parameter(n, k)?,
                    "IgesAppliToolNodalDisplAndRot::own_copy",
                )?;
            }
            iges_live(&a_trans_param, "IgesAppliToolNodalDisplAndRot::own_copy")?
                .set_value(n, &temp_array1)?;
            iges_live(&a_rot_param, "IgesAppliToolNodalDisplAndRot::own_copy")?
                .set_value(n, &temp_array2)?;
        }
        ent.init(
            &a_notes,
            &a_node_identifiers,
            &a_nodes,
            &a_rot_param,
            &a_trans_param,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliNodalDisplAndRot>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliNodalDisplAndRot")?;
        let mut nbcases: i32 = ent.nb_cases()?;
        let mut nbnodes: i32 = ent.nb_nodes()?;
        writeln!(s, "No. of analysis cases : {}", nbcases)?;
        write!(s, "General Notes : ")?;
        iges_dump_entities(s, dumper, level, 1, nbcases, |first| ent.note(first))?;
        writeln!(s)?;
        match level {
            4 => {
                write!(s, "Nodes : ")?;
                iges_dump_entities(s, dumper, -level, 1, nbnodes, |first| ent.node(first))?;
                write!(s, "NodeIdentifiers : ")?;
                iges_dump_vals(s, -level, 1, nbnodes, |first| ent.node_identifier(first))?;
                write!(s, "Translation Parameters : ")?;
                write!(s, "Rotational Parameters : ")?;
                writeln!(s, "  TO BE DONE")?;
            }
            5 | 6 => {
                for i in 1..=nbcases {
                    write!(s, "[{}]: ", i)?;
                    dumper.dump(&ent.note(i)?, s, 1, -1)?;
                    writeln!(s)?;
                }
                writeln!(s, "Nodes :")?;
                writeln!(s, "Node Identifiers :")?;
                writeln!(s, "Translation Parameters :")?;
                write!(s, "Rotational Parameters : Count = {}", nbcases)?;
                writeln!(s)?;
                for j in 1..=nbnodes {
                    writeln!(
                        s,
                        "[{}]:  -  NodeIdentifier : {}  -  Node :",
                        j,
                        ent.node_identifier(j)?
                    )?;
                    write!(s, "Node : ")?;
                    dumper.dump(&ent.node(j)?, s, 1, -1)?;
                    writeln!(s, "  -  Parameters :")?;
                    for k in 1..=nbcases {
                        writeln!(s, " [{}]:", k)?;
                        writeln!(
                            s,
                            "Translational Parameter : X={}, Y={}, Z={}",
                            iges_real_text(ent.translation_parameter(j, k)?.x()),
                            iges_real_text(ent.translation_parameter(j, k)?.y()),
                            iges_real_text(ent.translation_parameter(j, k)?.z())
                        )?;
                        write!(
                            s,
                            "Rotational Parameter : X={}, Y={}, Z={}",
                            iges_real_text(ent.rotational_parameter(j, k)?.x()),
                            iges_real_text(ent.rotational_parameter(j, k)?.y()),
                            iges_real_text(ent.rotational_parameter(j, k)?.z())
                        )?;
                        if k == nbcases {
                            write!(s, "] ")?;
                        }
                        writeln!(s)?;
                    }
                }
            }
            _ => {}
        }
        writeln!(s)?;
        Ok(())
    }
}
