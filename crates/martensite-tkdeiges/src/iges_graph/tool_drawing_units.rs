//! IgesGraphToolDrawingUnits.
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
use crate::iges_graph::IgesGraphDrawingUnits;
use crate::iges_tool_support::iges_text_compare;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolDrawingUnits,
    "The own-parameter work on `IgesGraphDrawingUnits`."
);

impl IgesGraphToolDrawingUnits {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphDrawingUnits>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_property_values: i32 = Default::default();
        let mut flag: i32 = Default::default();
        let mut unit: Handle<HAsciiString> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of property values",
            &mut nb_property_values,
        )?;
        if nb_property_values != 2 {
            pr.add_fail("No. of Property values : Value is not 2", "");
        }
        pr.read_integer_text(&pr.current(), "Units Flag", &mut flag)?;
        pr.read_text(&pr.current(), "Units Name", &mut unit)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(nb_property_values, flag, &unit);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphDrawingUnits>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.flag())?;
        iw.send(ent.unit())?;
        Ok(())
    }
    /// Lists the Entities shared by a DrawingUnits `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphDrawingUnits>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DrawingUnits
    /// (NbPropertyValues forced to 2)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesGraphDrawingUnits>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 2;
        let mut unf: i32 = ent.flag();
        let mut name: Handle<HAsciiString> = Default::default();
        let mut unm: AsciiString = AsciiString::new("");
        if !(ent.unit().is_null()) {
            unm = iges_live(&ent.unit(), "IgesGraphToolDrawingUnits::own_correct")?
                .string()
                .clone();
        }
        match unf {
            1 => {
                if (iges_text_compare(&unm, "IN") == 0) || (iges_text_compare(&unm, "INCH") == 0) {
                    name = Handle::new(HAsciiString::new("IN"));
                }
            }
            2 => {
                if iges_text_compare(&unm, "MM") == 0 {
                    name = Handle::new(HAsciiString::new("MM"));
                }
            }
            3 => {}
            4 => {
                if iges_text_compare(&unm, "FT") == 0 {
                    name = Handle::new(HAsciiString::new("FT"));
                }
            }
            5 => {
                if iges_text_compare(&unm, "MI") == 0 {
                    name = Handle::new(HAsciiString::new("MI"));
                }
            }
            6 => {
                if iges_text_compare(&unm, "M") == 0 {
                    name = Handle::new(HAsciiString::new("M"));
                }
            }
            7 => {
                if iges_text_compare(&unm, "KM") == 0 {
                    name = Handle::new(HAsciiString::new("KM"));
                }
            }
            8 => {
                if iges_text_compare(&unm, "MIL") == 0 {
                    name = Handle::new(HAsciiString::new("MIL"));
                }
            }
            9 => {
                if iges_text_compare(&unm, "UM") == 0 {
                    name = Handle::new(HAsciiString::new("UM"));
                }
            }
            10 => {
                if iges_text_compare(&unm, "CM") == 0 {
                    name = Handle::new(HAsciiString::new("CM"));
                }
            }
            11 => {
                if iges_text_compare(&unm, "UIN") == 0 {
                    name = Handle::new(HAsciiString::new("UIN"));
                }
            }
            _ => {}
        }
        res |= !name.is_null();
        if name.is_null() {
            name = ent.unit();
        }
        if res {
            ent.init(2, unf, &name);
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphDrawingUnits>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 17);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphDrawingUnits>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 2 {
            ach.add_fail_text("No. of Property values : Value != 2", "");
        }
        let mut unf: i32 = ent.flag();
        if ent.unit().is_null() {
            if unf == 3 {
                ach.add_fail_text("Unit Flag = 3 (user def.) and Unit Name undefined", "");
            }
        } else {
            let mut unm: AsciiString =
                iges_live(&ent.unit(), "IgesGraphToolDrawingUnits::own_check")?
                    .string()
                    .clone();
            let mut unok: bool = true;
            match unf {
                1 => {
                    unok = (!(iges_text_compare(&unm, "IN") != 0))
                        || (!(iges_text_compare(&unm, "INCH") != 0));
                }
                2 => {
                    unok = !(iges_text_compare(&unm, "MM") != 0);
                }
                3 => {
                    unok = true;
                }
                4 => {
                    unok = !(iges_text_compare(&unm, "FT") != 0);
                }
                5 => {
                    unok = !(iges_text_compare(&unm, "MI") != 0);
                }
                6 => {
                    unok = !(iges_text_compare(&unm, "M") != 0);
                }
                7 => {
                    unok = !(iges_text_compare(&unm, "KM") != 0);
                }
                8 => {
                    unok = !(iges_text_compare(&unm, "MIL") != 0);
                }
                9 => {
                    unok = !(iges_text_compare(&unm, "UM") != 0);
                }
                10 => {
                    unok = !(iges_text_compare(&unm, "CM") != 0);
                }
                11 => {
                    unok = !(iges_text_compare(&unm, "UIN") != 0);
                }
                _ => {
                    ach.add_fail_text("Unit Flag not in range 1 - 11", "");
                }
            }
            if !unok {
                ach.add_fail_text("Unit Flag & Name not accorded", "");
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
        another: &Handle<IgesGraphDrawingUnits>,
        ent: &Handle<IgesGraphDrawingUnits>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nb_property_values: i32 = Default::default();
        let mut flag: i32 = Default::default();
        let mut unit: Handle<HAsciiString> = Default::default();
        nb_property_values = another.nb_property_values();
        flag = another.flag();
        unit = Handle::new(HAsciiString::copied(iges_live(
            &another.unit(),
            "IgesGraphToolDrawingUnits::own_copy",
        )?));
        ent.init(nb_property_values, flag, &unit);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphDrawingUnits>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesGraphDrawingUnits\nNo. of property values : {}\n  Units Flag : {}  Units Name : ",
            ent.nb_property_values(),
            ent.flag()
        )?;
        iges_dump_string(s, &ent.unit())?;
        writeln!(
            s,
            "  computed Value (in meters) : {}",
            iges_real_text(ent.unit_value())
        )?;
        Ok(())
    }
}
