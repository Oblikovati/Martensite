//! IgesDimenToolDimensionUnits.
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
};
use crate::iges_dimen::IgesDimenDimensionUnits;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolDimensionUnits,
    "The own-parameter work on `IgesDimenDimensionUnits`."
);

impl IgesDimenToolDimensionUnits {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionUnits>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_props: i32 = Default::default();
        let mut temp_second_dimen_pos: i32 = Default::default();
        let mut temp_units_indic: i32 = Default::default();
        let mut temp_char_set: i32 = Default::default();
        let mut temp_frac_flag: i32 = Default::default();
        let mut temp_precision: i32 = Default::default();
        let mut temp_format_string: Handle<HAsciiString> = Default::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Number of Properties", &mut temp_nb_props)?;
        } else {
            temp_nb_props = 6;
        }
        pr.read_integer_text(
            &pr.current(),
            "Secondary Dimension Position",
            &mut temp_second_dimen_pos,
        )?;
        pr.read_integer_text(&pr.current(), "Units Indicator", &mut temp_units_indic)?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Character Set", &mut temp_char_set)?;
        } else {
            temp_char_set = 1;
        }
        pr.read_text(&pr.current(), "Format String", &mut temp_format_string)?;
        crate::iges_shared_blocks::iges_block_0058!(ent, pr, self, temp_frac_flag, temp_precision);
        ent.init(
            temp_nb_props,
            temp_second_dimen_pos,
            temp_units_indic,
            temp_char_set,
            &temp_format_string,
            temp_frac_flag,
            temp_precision,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionUnits>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.secondary_dimen_position())?;
        iw.send(ent.units_indicator())?;
        iw.send(ent.character_set())?;
        iw.send(ent.format_string())?;
        iw.send(ent.fraction_flag())?;
        iw.send(ent.precision_or_denominator())?;
        Ok(())
    }
    /// Lists the Entities shared by a DimensionUnits `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenDimensionUnits>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DimensionUnits
    /// (NbPropertyValues forced to 6)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDimenDimensionUnits>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 6;
        if res {
            ent.init(
                6,
                ent.secondary_dimen_position(),
                ent.units_indicator(),
                ent.character_set(),
                &ent.format_string(),
                ent.fraction_flag(),
                ent.precision_or_denominator(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenDimensionUnits>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 28);
        crate::iges_shared_blocks::iges_block_0007!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenDimensionUnits>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 6 {
            ach.add_fail_text("Number of properties != 6", "");
        }
        if (ent.secondary_dimen_position() < 0) || (ent.secondary_dimen_position() > 4) {
            ach.add_fail_text("Secondary Dimension Position != 0-4", "");
        }
        if (ent.character_set() != 1)
            && ((ent.character_set() < 1001) || (ent.character_set() > 1003))
        {
            ach.add_fail_text("Character Set != 1,1001-1003", "");
        }
        if (ent.fraction_flag() != 0) && (ent.fraction_flag() != 1) {
            ach.add_fail_text("Fraction Flag != 0,1", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenDimensionUnits>,
        ent: &Handle<IgesDimenDimensionUnits>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_nb_props: i32 = another.nb_property_values();
        let mut temp_second_dimen_pos: i32 = another.secondary_dimen_position();
        let mut temp_units_indic: i32 = another.units_indicator();
        let mut temp_char_set: i32 = another.character_set();
        let mut temp_format_string: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.format_string(),
                "IgesDimenToolDimensionUnits::own_copy",
            )?));
        let mut temp_frac_flag: i32 = another.fraction_flag();
        let mut temp_precision: i32 = another.precision_or_denominator();
        ent.init(
            temp_nb_props,
            temp_second_dimen_pos,
            temp_units_indic,
            temp_char_set,
            &temp_format_string,
            temp_frac_flag,
            temp_precision,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenDimensionUnits>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDimenDimensionUnits\nNumber of property values : {}\nSecondary Dimension Position : {}\nUnits Indicator : {}\nCharacter Set   : {}\nFormat String   : ",
            ent.nb_property_values(),
            ent.secondary_dimen_position(),
            ent.units_indicator(),
            ent.character_set()
        )?;
        iges_dump_string(s, &ent.format_string())?;
        write!(s, "\nFraction Flag   : {}", ent.fraction_flag())?;
        if ent.fraction_flag() == 0 {
            write!(s, " Decimal  , Precision   : ")?;
        } else {
            write!(s, " Fraction , Denominator : ")?;
        }
        writeln!(s, "{}", ent.precision_or_denominator())?;
        Ok(())
    }
}
