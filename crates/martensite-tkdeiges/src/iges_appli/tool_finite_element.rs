//! IgesAppliToolFiniteElement.
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
use crate::iges_appli::{IgesAppliFiniteElement, IgesAppliNode};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_entities, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolFiniteElement,
    "The own-parameter work on `IgesAppliFiniteElement`."
);

impl IgesAppliToolFiniteElement {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliFiniteElement>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_topology: i32 = Default::default();
        let mut temp_name: Handle<HAsciiString> = Default::default();
        let mut nbval: i32 = 0;
        let mut temp_data: Handle<HArray1<Handle<IgesAppliNode>>> = Default::default();
        pr.read_integer_text(&pr.current(), "Topology type", &mut temp_topology)?;
        pr.read_integer_text(&pr.current(), "No. of nodes defining element", &mut nbval)?;
        temp_data = iges_new_array1::<Handle<IgesAppliNode>>(1, nbval)?;
        for i in 1..=nbval {
            let mut temp_node: Handle<IgesAppliNode> = Default::default();
            if pr.read_entity_typed(
                ir,
                &pr.current(),
                "Node defining element",
                <IgesAppliNode>::static_type(),
                &mut temp_node,
                false,
            )? {
                iges_array1_set(
                    &temp_data,
                    i,
                    temp_node.clone(),
                    "IgesAppliToolFiniteElement::read_own_params",
                )?;
            }
        }
        pr.read_text(&pr.current(), "Element type name", &mut temp_name)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_topology, &temp_data, &temp_name)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliFiniteElement>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_nodes()?;
        iw.send(ent.topology())?;
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent.node(i)?, false)?;
        }
        iw.send(ent.name())?;
        Ok(())
    }
    /// Lists the Entities shared by a FiniteElement `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliFiniteElement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut upper: i32 = ent.nb_nodes()?;
        for i in 1..=upper {
            iter.get_one_item(&ent.node(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliFiniteElement>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(136, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.subordinate_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesAppliFiniteElement>,
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
        another: &Handle<IgesAppliFiniteElement>,
        ent: &Handle<IgesAppliFiniteElement>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_topology: i32 = another.topology();
        let mut a_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.name(),
            "IgesAppliToolFiniteElement::own_copy",
        )?));
        let mut nbval: i32 = another.nb_nodes()?;
        let mut a_list: Handle<HArray1<Handle<IgesAppliNode>>> =
            iges_new_array1::<Handle<IgesAppliNode>>(1, nbval)?;
        for i in 1..=nbval {
            let mut a_entity: Handle<IgesAppliNode> =
                tc.transferred(&another.node(i)?.cast())?.cast();
            iges_array1_set(
                &a_list,
                i,
                a_entity.clone(),
                "IgesAppliToolFiniteElement::own_copy",
            )?;
        }
        ent.init(a_topology, &a_list, &a_name)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliFiniteElement>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliFiniteElement")?;
        writeln!(s, "Topology type : {}", ent.topology())?;
        write!(s, "Nodes : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_nodes()?, |first| {
            ent.node(first)
        })?;
        write!(s, "\nElement Name : ")?;
        s.write_all(
            iges_live(&ent.name(), "IgesAppliToolFiniteElement::own_dump")?
                .string()
                .clone()
                .as_bytes(),
        )?;
        writeln!(s)?;
        Ok(())
    }
}
