//! IgesAppliToolNodalResults.
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
use crate::iges_appli::{IgesAppliNodalResults, IgesAppliNode};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_array2_set, iges_new_array1, iges_new_array2, iges_real_text,
};
use crate::iges_dimen::IgesDimenGeneralNote;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolNodalResults,
    "The own-parameter work on `IgesAppliNodalResults`."
);

impl IgesAppliToolNodalResults {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliNodalResults>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_sub_case_num: i32 = 0;
        let mut temp_time: f64 = Default::default();
        let mut nbval: i32 = 0;
        let mut nbnodes: i32 = 0;
        let mut temp_note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut temp_data: Handle<HArray2<f64>> = Default::default();
        let mut temp_nodes: Handle<HArray1<Handle<IgesAppliNode>>> = Default::default();
        let mut temp_node_identifiers: Handle<HArray1<i32>> = Default::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "General Note describing the analysis case",
            <IgesDimenGeneralNote>::static_type(),
            &mut temp_note,
            false,
        )?;
        pr.read_integer_text(&pr.current(), "Subcase number", &mut temp_sub_case_num)?;
        pr.read_real_text(&pr.current(), "Analysis time used", &mut temp_time)?;
        let mut temp_flag: bool =
            pr.read_integer_text(&pr.current(), "No. of values", &mut nbval)?;
        if pr.read_integer_text(&pr.current(), "No. of nodes", &mut nbnodes)? {
            temp_data = iges_new_array2::<f64>(1, nbnodes, 1, nbval)?;
            temp_nodes = iges_new_array1::<Handle<IgesAppliNode>>(1, nbnodes)?;
            temp_node_identifiers = iges_new_array1::<i32>(1, nbnodes)?;
            for i in 1..=nbnodes {
                let mut aitem: i32 = Default::default();
                let mut a_node: Handle<IgesAppliNode> = Default::default();
                if pr.read_integer_text(&pr.current(), "Node no. identifier", &mut aitem)? {
                    iges_array1_set(
                        &temp_node_identifiers,
                        i,
                        aitem,
                        "IgesAppliToolNodalResults::read_own_params",
                    )?;
                }
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "FEM Node",
                    <IgesAppliNode>::static_type(),
                    &mut a_node,
                    false,
                )? {
                    iges_array1_set(
                        &temp_nodes,
                        i,
                        a_node.clone(),
                        "IgesAppliToolNodalResults::read_own_params",
                    )?;
                }
                if temp_flag {
                    for j in 1..=nbval {
                        let mut aval: f64 = Default::default();
                        if pr.read_real_text(&pr.current(), "Value", &mut aval)? {
                            iges_array2_set(
                                &temp_data,
                                i,
                                j,
                                aval,
                                "IgesAppliToolNodalResults::read_own_params",
                            )?;
                        }
                    }
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &temp_note,
            temp_sub_case_num,
            temp_time,
            &temp_node_identifiers,
            &temp_nodes,
            &temp_data,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliNodalResults>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut nbnodes: i32 = ent.nb_nodes()?;
        let mut nbdata: i32 = ent.nb_data()?;
        iw.send_entity(&ent.note(), false)?;
        iw.send(ent.sub_case_number())?;
        iw.send(ent.time())?;
        iw.send(nbdata)?;
        iw.send(nbnodes)?;
        for i in 1..=nbnodes {
            iw.send(ent.node_identifier(i)?)?;
            iw.send_entity(&ent.node(i)?, false)?;
            for j in 1..=nbdata {
                iw.send(ent.data(i, j)?)?;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a NodalResults `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliNodalResults>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut nbnodes: i32 = ent.nb_nodes()?;
        iter.get_one_item(&ent.note().cast());
        for i in 1..=nbnodes {
            iter.get_one_item(&ent.node(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliNodalResults>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(146, 0, 34);
        dc.structure(IgesDataDefType::DefVoid);
        crate::iges_shared_blocks::iges_block_0027!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliNodalResults>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut form_num: i32 = ent.form_number();
        let mut nv: i32 = ent.nb_data()?;
        let mut ok: bool = true;
        match form_num {
            0 => {
                if nv < 0 {
                    ok = false;
                }
            }
            1 => {
                if nv != 1 {
                    ok = false;
                }
            }
            2 => {
                if nv != 1 {
                    ok = false;
                }
            }
            3 => {
                if nv != 3 {
                    ok = false;
                }
            }
            4 => {
                if nv != 6 {
                    ok = false;
                }
            }
            5 => {
                if nv != 3 {
                    ok = false;
                }
            }
            6 => {
                if nv != 3 {
                    ok = false;
                }
            }
            7 => {
                if nv != 3 {
                    ok = false;
                }
            }
            8 => {
                if nv != 3 {
                    ok = false;
                }
            }
            9 => {
                if nv != 3 {
                    ok = false;
                }
            }
            10 => {
                if nv != 1 {
                    ok = false;
                }
            }
            11 => {
                if nv != 1 {
                    ok = false;
                }
            }
            12 => {
                if nv != 3 {
                    ok = false;
                }
            }
            13 => {
                if nv != 1 {
                    ok = false;
                }
            }
            14 => {
                if nv != 1 {
                    ok = false;
                }
            }
            15 => {
                if nv != 3 {
                    ok = false;
                }
            }
            16 => {
                if nv != 1 {
                    ok = false;
                }
            }
            17 => {
                if nv != 3 {
                    ok = false;
                }
            }
            18 => {
                if nv != 3 {
                    ok = false;
                }
            }
            19 => {
                if nv != 3 {
                    ok = false;
                }
            }
            20 => {
                if nv != 3 {
                    ok = false;
                }
            }
            21 => {
                if nv != 3 {
                    ok = false;
                }
            }
            22 => {
                if nv != 3 {
                    ok = false;
                }
            }
            23 => {
                if nv != 6 {
                    ok = false;
                }
            }
            24 => {
                if nv != 6 {
                    ok = false;
                }
            }
            25 => {
                if nv != 6 {
                    ok = false;
                }
            }
            26 => {
                if nv != 6 {
                    ok = false;
                }
            }
            27 => {
                if nv != 6 {
                    ok = false;
                }
            }
            28 => {
                if nv != 6 {
                    ok = false;
                }
            }
            29 => {
                if nv != 9 {
                    ok = false;
                }
            }
            30 => {
                if nv != 9 {
                    ok = false;
                }
            }
            31 => {
                if nv != 9 {
                    ok = false;
                }
            }
            32 => {
                if nv != 9 {
                    ok = false;
                }
            }
            33 => {
                if nv != 9 {
                    ok = false;
                }
            }
            34 => {
                if nv != 9 {
                    ok = false;
                }
            }
            _ => {
                ach.add_fail_text("Incorrect Form Number", "");
            }
        }
        if !ok {
            ach.add_fail_text("Incorrect count of real values in array V for FEM node", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliNodalResults>,
        ent: &Handle<IgesAppliNodalResults>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut a_sub_case_num: i32 = another.sub_case_number();
        let mut a_time: f64 = another.time();
        let mut nbnodes: i32 = another.nb_nodes()?;
        let mut nbval: i32 = another.nb_data()?;
        crate::iges_shared_blocks::iges_block_0071!(a_node_identifiers, a_nodes, nbnodes);
        let mut a_data: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, nbnodes, 1, nbval)?;
        for i in 1..=nbnodes {
            let mut a_item: i32 = another.node_identifier(i)?;
            iges_array1_set(
                &a_node_identifiers,
                i,
                a_item,
                "IgesAppliToolNodalResults::own_copy",
            )?;
            let mut anentity: Handle<IgesAppliNode> =
                tc.transferred(&another.node(i)?.cast())?.cast();
            iges_array1_set(
                &a_nodes,
                i,
                anentity.clone(),
                "IgesAppliToolNodalResults::own_copy",
            )?;
            for j in 1..=nbval {
                iges_array2_set(
                    &a_data,
                    i,
                    j,
                    another.data(i, j)?,
                    "IgesAppliToolNodalResults::own_copy",
                )?;
            }
        }
        ent.init(
            &a_note,
            a_sub_case_num,
            a_time,
            &a_node_identifiers,
            &a_nodes,
            &a_data,
        )?;
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliNodalResults>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliNodalResults")?;
        write!(s, "General Note : ")?;
        dumper.dump(&ent.note(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        writeln!(s)?;
        write!(s, "Analysis subcase number : {}  ", ent.sub_case_number())?;
        writeln!(s, "Time used : {}", iges_real_text(ent.time()))?;
        write!(s, "No. of nodes : {}  ", ent.nb_nodes()?)?;
        writeln!(s, "No. of values for a node : {}", ent.nb_data()?)?;
        writeln!(s, "Node Identifiers :")?;
        writeln!(s, "Nodes :")?;
        write!(s, "Data : ")?;
        if level < 6 {
            write!(s, " [ask level > 5]")?;
        }
        writeln!(s)?;
        if level > 4 {
            for i in 1..=ent.nb_nodes()? {
                write!(s, "[{}]: ", i)?;
                write!(s, "NodeIdentifier : {}  ", ent.node_identifier(i)?)?;
                write!(s, "Node : ")?;
                dumper.dump(&ent.node(i)?, s, 1, -1)?;
                writeln!(s)?;
                if level < 6 {
                    continue;
                }
                write!(s, "Data : [ ")?;
                for j in 1..=ent.nb_data()? {
                    write!(s, "  {}", iges_real_text(ent.data(i, j)?))?;
                }
                writeln!(s, " ]")?;
            }
        }
        Ok(())
    }
}
