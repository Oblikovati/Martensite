//! IgesDimenToolBasicDimension.
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
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xy,
};
use crate::iges_dimen::IgesDimenBasicDimension;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolBasicDimension,
    "The own-parameter work on `IgesDimenBasicDimension`."
);

impl IgesDimenToolBasicDimension {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenBasicDimension>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_prop_val: i32 = Default::default();
        let mut templl: Xy = Default::default();
        let mut templr: Xy = Default::default();
        let mut tempur: Xy = Default::default();
        let mut tempul: Xy = Default::default();
        pr.read_integer_text(&pr.current(), "Number of Property Values", &mut nb_prop_val)?;
        pr.read_xy(&pr.current_list(1, 2), "Lower Left Corner", &mut templl)?;
        pr.read_xy(&pr.current_list(1, 2), "Lower Right Corner", &mut templr)?;
        pr.read_xy(&pr.current_list(1, 2), "Upper Right Corner", &mut tempur)?;
        pr.read_xy(&pr.current_list(1, 2), "Upper Left Corner", &mut tempul)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(nb_prop_val, templl, templr, tempur, tempul);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenBasicDimension>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.lower_left().x())?;
        iw.send(ent.lower_left().y())?;
        iw.send(ent.lower_right().x())?;
        iw.send(ent.lower_right().y())?;
        iw.send(ent.upper_right().x())?;
        iw.send(ent.upper_right().y())?;
        iw.send(ent.upper_left().x())?;
        iw.send(ent.upper_left().y())?;
        Ok(())
    }
    /// Lists the Entities shared by a BasicDimension `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenBasicDimension>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a BasicDimension
    /// (NbPropertyValues forced to 8)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDimenBasicDimension>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 8;
        if res {
            ent.init(
                8,
                ent.lower_left().xy(),
                ent.lower_right().xy(),
                ent.upper_right().xy(),
                ent.upper_left().xy(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenBasicDimension>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 31);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.subordinate_status_required(1);
        dc.use_flag_required(2);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenBasicDimension>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 8 {
            ach.add_fail_text("Num of Property Values != 8", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenBasicDimension>,
        ent: &Handle<IgesDimenBasicDimension>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            8,
            another.lower_left().xy(),
            another.lower_right().xy(),
            another.upper_right().xy(),
            another.upper_left().xy(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenBasicDimension>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDimenBasicDimension\nNumber of Property Values : {}\n\n  Lower left corner  : ",
            ent.nb_property_values()
        )?;
        iges_dump_xy(s, &ent.lower_left().xy())?;
        write!(s, "\n  Lower right corner : ")?;
        iges_dump_xy(s, &ent.lower_right().xy())?;
        write!(s, "\n  Upper right corner : ")?;
        iges_dump_xy(s, &ent.upper_right().xy())?;
        write!(s, "\n  Upper left corner  : ")?;
        iges_dump_xy(s, &ent.upper_left().xy())?;
        writeln!(s)?;
        Ok(())
    }
}
