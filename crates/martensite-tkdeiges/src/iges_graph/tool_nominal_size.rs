//! IgesGraphToolNominalSize.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_string, iges_live,
    iges_real_text,
};
use crate::iges_graph::IgesGraphNominalSize;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceParamType,
    InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolNominalSize,
    "The own-parameter work on `IgesGraphNominalSize`."
);

impl IgesGraphToolNominalSize {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphNominalSize>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0046!(
            nb_property_values,
            nominal_size_name,
            nominal_size_value,
            standard_name
        );
        pr.read_integer_text(
            &pr.current(),
            "No. of property values",
            &mut nb_property_values,
        )?;
        if (nb_property_values != 2) && (nb_property_values != 3) {
            pr.add_fail("No. of Property values : Value is not 2/3", "");
        }
        pr.read_real_text(&pr.current(), "Nominal size value", &mut nominal_size_value)?;
        pr.read_text(&pr.current(), "Nominal size name", &mut nominal_size_name)?;
        if pr.nb_params() >= pr.current_number() {
            let mut num: i32 = pr.current_number();
            if pr.param_type(num)? == (InterfaceParamType::ParamText) {
                pr.read_text(
                    &pr.current(),
                    "Name of relevant engg. standard",
                    &mut standard_name,
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            nb_property_values,
            nominal_size_value,
            &nominal_size_name,
            &standard_name,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphNominalSize>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.nominal_size_value())?;
        iw.send(ent.nominal_size_name())?;
        if ent.has_standard_name() {
            iw.send(ent.standard_name())?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a NominalSize `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphNominalSize>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a NominalSize
    /// (NbPropertyValues forced to 2 or 3 according HasStandardName)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesGraphNominalSize>) -> Result<bool, Failure> {
        let mut nbp: i32 = 2;
        if ent.has_standard_name() {
            nbp = 3;
        }
        let mut res: bool = ent.nb_property_values() != nbp;
        if res {
            ent.init(
                nbp,
                ent.nominal_size_value(),
                &ent.nominal_size_name(),
                &ent.standard_name(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphNominalSize>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 13);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphNominalSize>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut nbp: i32 = 2;
        if ent.has_standard_name() {
            nbp = 3;
        }
        if ent.nb_property_values() != nbp {
            ach.add_fail_text(
                "No. of Property values : Value != 2/3 according Standard Name Status",
                "",
            );
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphNominalSize>,
        ent: &Handle<IgesGraphNominalSize>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0046!(
            nb_property_values,
            nominal_size_name,
            nominal_size_value,
            standard_name
        );
        nb_property_values = another.nb_property_values();
        nominal_size_value = another.nominal_size_value();
        nominal_size_name = Handle::new(HAsciiString::copied(iges_live(
            &another.nominal_size_name(),
            "IgesGraphToolNominalSize::own_copy",
        )?));
        if another.has_standard_name() {
            standard_name = Handle::new(HAsciiString::copied(iges_live(
                &another.standard_name(),
                "IgesGraphToolNominalSize::own_copy",
            )?));
        }
        ent.init(
            nb_property_values,
            nominal_size_value,
            &nominal_size_name,
            &standard_name,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphNominalSize>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesGraphNominalSize\nNo. of property values : {}\nNominal size value : {}\nNominal size name  : ",
            ent.nb_property_values(),
            iges_real_text(ent.nominal_size_value())
        )?;
        iges_dump_string(s, &ent.nominal_size_name())?;
        write!(s, "\nName of relevant engineering standard : ")?;
        iges_dump_string(s, &ent.standard_name())?;
        writeln!(s)?;
        Ok(())
    }
}
