#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{
    IgesBasicExternalRefFileIndex, IgesBasicExternalRefFileName, IgesBasicExternalRefLibName,
    IgesBasicExternalRefName, IgesBasicExternalReferenceFile, IgesBasicToolExternalRefFileIndex,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_029, iges_shared_031};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_dump_entities, iges_dump_string, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesBasicToolExternalRefFileIndex {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalRefFileIndex>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileIndex::own_dump")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        write!(
            s,
            "IgesBasicExternalRefFileIndex\nExternal Reference Names :\nInternal Entities : "
        )?;
        iges_dump_entities(s, dumper, -level, 1, ent_live.nb_entries()?, |first| {
            ent_live.entity(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            num = ent_live.nb_entries()?;
            i = 1;
            while i <= num {
                write!(s, "[{}]: External Reference Name : ", i)?;
                iges_dump_string(s, &ent_live.name(i)?)?;
                write!(s, "  Internal Entity : ")?;
                dumper.dump(&ent_live.entity(i)?, s, 1, -1)?;
                writeln!(s)?;
                i += 1;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalRefFileName,
    "The own-parameter work on `IgesBasicExternalRefFileName`."
);
impl IgesBasicToolExternalRefFileName {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFileName>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileName::read_own_params")?;
        let mut temp_ext_ref_file_identifier: Handle<HAsciiString> = Handle::default();
        let mut temp_ext_ref_entity_symb_name: Handle<HAsciiString> = Handle::default();
        pr.read_text(
            &pr.current(),
            "External Reference File Identifier",
            &mut temp_ext_ref_file_identifier,
        )?;
        pr.read_text(
            &pr.current(),
            "External Reference Symbolic Name",
            &mut temp_ext_ref_entity_symb_name,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &temp_ext_ref_file_identifier,
            &temp_ext_ref_entity_symb_name,
        );
        Ok(())
    }
}
impl IgesBasicToolExternalRefFileName {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFileName>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileName::write_own_params")?;
        iw.send(ent_live.file_id())?;
        iw.send(ent_live.reference_name())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicExternalRefFileName>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalRefFileName>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(416, 0, 2);
        iges_shared_031!(dc);
        dc
    }
}
impl IgesBasicToolExternalRefFileName {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicExternalRefFileName>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileName::own_check")?;
        if ent_live.form_number() == 1 {
            ach.add_fail_text("Invalid Form Number", "");
        }
        Ok(())
    }
}
impl IgesBasicToolExternalRefFileName {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalRefFileName>,
        ent: &Handle<IgesBasicExternalRefFileName>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalRefFileName::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileName::own_copy")?;
        let temp_file_id: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.file_id(),
            "IgesBasicToolExternalRefFileName::own_copy",
        )?));
        let temp_ref_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.reference_name(),
            "IgesBasicToolExternalRefFileName::own_copy",
        )?));
        ent_live.init(&temp_file_id, &temp_ref_name);
        Ok(())
    }
}
impl IgesBasicToolExternalRefFileName {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalRefFileName>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileName::own_dump")?;
        write!(
            s,
            "IgesBasicExternalRefFileName\nExternal Reference File Identifier : "
        )?;
        iges_dump_string(s, &ent_live.file_id())?;
        write!(s, "\nExternal Reference Symbolic Name : ")?;
        iges_dump_string(s, &ent_live.reference_name())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalRefLibName,
    "The own-parameter work on `IgesBasicExternalRefLibName`."
);
impl IgesBasicToolExternalRefLibName {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefLibName>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefLibName::read_own_params")?;
        let mut temp_lib_name: Handle<HAsciiString> = Handle::default();
        let mut temp_ext_ref_entity_symb_name: Handle<HAsciiString> = Handle::default();
        pr.read_text(&pr.current(), "Name of Library", &mut temp_lib_name)?;
        pr.read_text(
            &pr.current(),
            "External Reference Symbolic Name",
            &mut temp_ext_ref_entity_symb_name,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_lib_name, &temp_ext_ref_entity_symb_name);
        Ok(())
    }
}
impl IgesBasicToolExternalRefLibName {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefLibName>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefLibName::write_own_params")?;
        iw.send(ent_live.library_name())?;
        iw.send(ent_live.reference_name())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicExternalRefLibName>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalRefLibName>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(416, 4);
        iges_shared_029!(dc);
        dc
    }
}
impl IgesBasicToolExternalRefLibName {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicExternalRefLibName>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolExternalRefLibName {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalRefLibName>,
        ent: &Handle<IgesBasicExternalRefLibName>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalRefLibName::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefLibName::own_copy")?;
        let temp_lib_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.library_name(),
            "IgesBasicToolExternalRefLibName::own_copy",
        )?));
        let temp_ref_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.reference_name(),
            "IgesBasicToolExternalRefLibName::own_copy",
        )?));
        ent_live.init(&temp_lib_name, &temp_ref_name);
        Ok(())
    }
}
impl IgesBasicToolExternalRefLibName {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalRefLibName>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefLibName::own_dump")?;
        write!(s, "IgesBasicExternalRefLibName\nName of Library : ")?;
        iges_dump_string(s, &ent_live.library_name())?;
        write!(s, "\nExternal Reference Symbolic Name : ")?;
        iges_dump_string(s, &ent_live.reference_name())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalRefName,
    "The own-parameter work on `IgesBasicExternalRefName`."
);
impl IgesBasicToolExternalRefName {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefName>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefName::read_own_params")?;
        let mut temp_ext_ref_entity_symb_name: Handle<HAsciiString> = Handle::default();
        pr.read_text(
            &pr.current(),
            "External Reference Symbolic Name",
            &mut temp_ext_ref_entity_symb_name,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_ext_ref_entity_symb_name);
        Ok(())
    }
}
impl IgesBasicToolExternalRefName {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefName>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefName::write_own_params")?;
        iw.send(ent_live.reference_name())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicExternalRefName>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalRefName>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(416, 3);
        iges_shared_029!(dc);
        dc
    }
}
impl IgesBasicToolExternalRefName {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicExternalRefName>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolExternalRefName {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalRefName>,
        ent: &Handle<IgesBasicExternalRefName>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalRefName::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefName::own_copy")?;
        let temp_ref_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.reference_name(),
            "IgesBasicToolExternalRefName::own_copy",
        )?));
        ent_live.init(&temp_ref_name);
        Ok(())
    }
}
impl IgesBasicToolExternalRefName {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalRefName>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefName::own_dump")?;
        write!(
            s,
            "IgesBasicExternalRefName\nExternal Reference Symbolic Name : "
        )?;
        iges_dump_string(s, &ent_live.reference_name())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalReferenceFile,
    "The own-parameter work on `IgesBasicExternalReferenceFile`."
);
impl IgesBasicToolExternalReferenceFile {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalReferenceFile>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalReferenceFile::read_own_params")?;
        let mut num: i32 = 0;
        let mut temp_names: Handle<HArray1<Handle<HAsciiString>>> = Handle::default();
        let st: bool = pr.read_integer_text(&pr.current(), "Number of list entries", &mut num)?;
        if st && num > 0 {
            temp_names = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        } else {
            pr.add_fail("Number of list entries: Not Positive", "");
        }
        if !temp_names.is_null() {
            pr.read_texts(
                &pr.current_list(num, 1),
                "External Reference Entity",
                &mut temp_names,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_names)?;
        Ok(())
    }
}
