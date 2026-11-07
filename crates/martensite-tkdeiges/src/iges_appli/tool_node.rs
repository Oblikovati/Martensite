//! IgesAppliToolNode.
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
use crate::iges_appli::IgesAppliNode;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_live, iges_real_text,
};
use crate::iges_geom::IgesGeomTransformationMatrix;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolNode,
    "The own-parameter work on `IgesAppliNode`."
);

impl IgesAppliToolNode {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliNode>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_coordinates: Xyz = Default::default();
        let mut temp_system: Handle<IgesGeomTransformationMatrix> = Default::default();
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Coordinates of Node (XYZ)",
            &mut temp_coordinates,
        )?;
        if pr.defined_else_skip()? {
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Transformation Matrix",
                <IgesGeomTransformationMatrix>::static_type(),
                &mut temp_system,
                true,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_coordinates, &temp_system);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliNode>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.coord().x())?;
        iw.send(ent.coord().y())?;
        iw.send(ent.coord().z())?;
        iw.send_entity(&ent.system(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a Node `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliNode>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.system().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliNode>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(134, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefVoid);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(4);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliNode>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if !ent.has_sub_script_number() {
            ach.add_fail_text(
                "SubScript Number expected (for Node Number) not present",
                "",
            );
        }
        if !ent.has_transf() {
            ach.add_fail_text("Transformation Matrix expected, not present", "");
        }
        if !(ent.system().is_null()) {
            if (iges_live(&ent.system(), "IgesAppliToolNode::own_check")?
                .iges_entity()
                .form_number())
                < 10
            {
                ach.add_fail_text("System : Incorrect FormNumber (not 10-11-12)", "");
            }
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliNode>,
        ent: &Handle<IgesAppliNode>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_coord: Xyz = another.coord().xyz();
        let mut a_system: Handle<IgesGeomTransformationMatrix> =
            tc.transferred(&another.system().cast())?.cast();
        ent.init(a_coord, &a_system);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliNode>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliNode")?;
        writeln!(
            s,
            " Nodal Coords : 1st {}  2nd : {}  3rd : {}",
            iges_real_text(ent.coord().x()),
            iges_real_text(ent.coord().y()),
            iges_real_text(ent.coord().z())
        )?;
        write!(s, "Nodal Displacement Coordinate System : ")?;
        if !(ent.system().is_null()) {
            dumper.dump(&ent.system(), s, level, -1)?;
        } else {
            write!(s, "Global Cartesian Coordinate System (default)")?;
        }
        writeln!(s)?;
        Ok(())
    }
}
