//! IgesDefsToolMacroDef.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_strings, iges_live, iges_new_array1,
};
use crate::iges_defs::IgesDefsMacroDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDefsToolMacroDef,
    "The own-parameter work on `IgesDefsMacroDef`."
);

impl IgesDefsToolMacroDef {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsMacroDef>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut macro_: Handle<HAsciiString> = Default::default();
        let mut entity_type_id: i32 = Default::default();
        let mut lang_statements: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut end_macro: Handle<HAsciiString> = Default::default();
        pr.read_text(&pr.current(), "MACRO", &mut macro_)?;
        pr.read_integer_text(&pr.current(), "Entity Type ID", &mut entity_type_id)?;
        let mut temp_current: i32 = pr.current_number();
        let mut nbval: i32 = Default::default();
        {
            nbval = 0;
            while pr.current_number() < pr.nb_params() {
                nbval += 1;
                pr.set_current_number(pr.current_number() + 1);
            }
        }
        pr.set_current_number(temp_current);
        if nbval > 0 {
            lang_statements = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        } else {
            pr.add_fail("Number of Lang. Stats. : Not Positive", "");
        }
        if !lang_statements.is_null() {
            for i in 1..=nbval {
                let mut lang_stat: Handle<HAsciiString> = Default::default();
                if pr.read_text(&pr.current(), "Language Statement", &mut lang_stat)? {
                    iges_array1_set(
                        &lang_statements,
                        i,
                        lang_stat.clone(),
                        "IgesDefsToolMacroDef::read_own_params",
                    )?;
                }
            }
        }
        pr.read_text(&pr.current(), "END MACRO", &mut end_macro)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&macro_, entity_type_id, &lang_statements, &end_macro)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsMacroDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.macro_())?;
        iw.send(ent.entity_type_id())?;
        let mut upper: i32 = ent.nb_statements()?;
        for i in 1..=upper {
            iw.send(ent.language_statement(i)?)?;
        }
        iw.send(ent.endmacro())?;
        Ok(())
    }
    /// Lists the Entities shared by a MacroDef `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDefsMacroDef>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsMacroDef>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(306, 0);
        crate::iges_shared_blocks::iges_block_0014!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDefsMacroDef>,
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
        another: &Handle<IgesDefsMacroDef>,
        ent: &Handle<IgesDefsMacroDef>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut macro_: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.macro_(),
            "IgesDefsToolMacroDef::own_copy",
        )?));
        let mut entity_type_id: i32 = another.entity_type_id();
        let mut end_macro: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.endmacro(),
            "IgesDefsToolMacroDef::own_copy",
        )?));
        let mut lang_statements: Handle<HArray1<Handle<HAsciiString>>> = Default::default();
        let mut nbval: i32 = another.nb_statements()?;
        lang_statements = iges_new_array1::<Handle<HAsciiString>>(1, nbval)?;
        for i in 1..=nbval {
            let mut lang_stat: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
                &another.language_statement(i)?,
                "IgesDefsToolMacroDef::own_copy",
            )?));
            iges_array1_set(
                &lang_statements,
                i,
                lang_stat.clone(),
                "IgesDefsToolMacroDef::own_copy",
            )?;
        }
        ent.init(&macro_, entity_type_id, &lang_statements, &end_macro)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsMacroDef>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(s, "IgesDefsMacroDef\nMACRO : ")?;
        iges_dump_string(s, &ent.macro_())?;
        write!(
            s,
            "\nEntity Type ID : {}\nLanguage Statement : ",
            ent.entity_type_id()
        )?;
        iges_dump_strings(s, level, 1, ent.nb_statements()?, |first| {
            ent.language_statement(first)
        })?;
        write!(s, "END MACRO : ")?;
        iges_dump_string(s, &ent.endmacro())?;
        writeln!(s)?;
        Ok(())
    }
}
