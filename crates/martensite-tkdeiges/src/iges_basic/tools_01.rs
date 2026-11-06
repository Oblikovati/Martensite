#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesBasicAssocGroupType, IgesBasicExternalRefFile, IgesBasicExternalRefFileIndex};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_023, iges_shared_031};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_string, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesBasicToolAssocGroupType,
    "The own-parameter work on `IgesBasicAssocGroupType`."
);
impl IgesBasicToolAssocGroupType {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicAssocGroupType>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::read_own_params")?;
        let mut temp_nb_data: i32 = 0;
        let mut temp_type: i32 = 0;
        let mut temp_name: Handle<HAsciiString> = Handle::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Number of data fields", &mut temp_nb_data)?;
        } else {
            temp_nb_data = 2;
        }
        pr.read_integer_text(
            &pr.current(),
            "Type of attached associativity",
            &mut temp_type,
        )?;
        pr.read_text(
            &pr.current(),
            "Name of attached associativity",
            &mut temp_name,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_nb_data, temp_type, &temp_name);
        Ok(())
    }
}
impl IgesBasicToolAssocGroupType {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicAssocGroupType>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::write_own_params")?;
        iw.send(ent_live.nb_data())?;
        iw.send(ent_live.assoc_type())?;
        iw.send(ent_live.name())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicAssocGroupType>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicAssocGroupType>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(406, 23);
        iges_shared_023!(dc);
        dc
    }
}
impl IgesBasicToolAssocGroupType {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicAssocGroupType>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::own_correct")?;
        let res: bool = ent_live.nb_data() != 2;
        if res {
            ent_live.init(2, ent_live.assoc_type(), &ent_live.name());
        }
        Ok(res)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicAssocGroupType>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::own_check")?;
        if ent_live.nb_data() != 2 {
            ach.add_fail_text("Number of data fields != 2", "");
        }
        Ok(())
    }
}
impl IgesBasicToolAssocGroupType {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicAssocGroupType>,
        ent: &Handle<IgesBasicAssocGroupType>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolAssocGroupType::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::own_copy")?;
        let temp_nb_data: i32 = another_live.nb_data();
        let temp_type: i32 = another_live.assoc_type();
        let temp_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.name(),
            "IgesBasicToolAssocGroupType::own_copy",
        )?));
        ent_live.init(temp_nb_data, temp_type, &temp_name);
        Ok(())
    }
}
impl IgesBasicToolAssocGroupType {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicAssocGroupType>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolAssocGroupType::own_dump")?;
        write!(
            s,
            "IgesBasicAssocGroupType\nNumber of data fields : {}\nType of attached associativity : {}\nName of attached associativity : ",
            ent_live.nb_data(),
            ent_live.assoc_type()
        )?;
        iges_dump_string(s, &ent_live.name())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalRefFile,
    "The own-parameter work on `IgesBasicExternalRefFile`."
);
impl IgesBasicToolExternalRefFile {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFile>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFile::read_own_params")?;
        let mut temp_ext_ref_file_identifier: Handle<HAsciiString> = Handle::default();
        pr.read_text(
            &pr.current(),
            "External Reference File Identifier",
            &mut temp_ext_ref_file_identifier,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_ext_ref_file_identifier);
        Ok(())
    }
}
impl IgesBasicToolExternalRefFile {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFile>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFile::write_own_params")?;
        iw.send(ent_live.file_id())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicExternalRefFile>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalRefFile>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(416, 1);
        iges_shared_031!(dc);
        dc
    }
}
impl IgesBasicToolExternalRefFile {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicExternalRefFile>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolExternalRefFile {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalRefFile>,
        ent: &Handle<IgesBasicExternalRefFile>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalRefFile::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFile::own_copy")?;
        let temp_file_id: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.file_id(),
            "IgesBasicToolExternalRefFile::own_copy",
        )?));
        ent_live.init(&temp_file_id);
        Ok(())
    }
}
impl IgesBasicToolExternalRefFile {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalRefFile>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFile::own_dump")?;
        write!(
            s,
            "IgesBasicExternalRefFile\nExternal Reference File Identifier : "
        )?;
        iges_dump_string(s, &ent_live.file_id())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolExternalRefFileIndex,
    "The own-parameter work on `IgesBasicExternalRefFileIndex`."
);
impl IgesBasicToolExternalRefFileIndex {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFileIndex>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileIndex::read_own_params")?;
        let mut num: i32 = 0;
        let mut temp_names: Handle<HArray1<Handle<HAsciiString>>> = Handle::default();
        let mut temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let st: bool = pr.read_integer_text(&pr.current(), "Number of index entries", &mut num)?;
        if st && num > 0 {
            temp_names = iges_new_array1::<Handle<HAsciiString>>(1, num)?;
            temp_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        } else {
            pr.add_fail("Number of index entries: Not Positive", "");
        }
        if !temp_names.is_null() && !temp_entities.is_null() {
            for i in 1..=num {
                let mut temp_nam: Handle<HAsciiString> = Handle::default();
                if pr.read_text(&pr.current(), "External Reference Entity", &mut temp_nam)? {
                    iges_array1_set(
                        &temp_names,
                        i,
                        temp_nam.clone(),
                        "IgesBasicToolExternalRefFileIndex::read_own_params",
                    )?;
                }
                let mut temp_ent: Handle<IgesDataIgesEntity> = Handle::default();
                if pr.read_entity(ir, &pr.current(), "Internal Entity", &mut temp_ent, false)? {
                    iges_array1_set(
                        &temp_entities,
                        i,
                        temp_ent.clone(),
                        "IgesBasicToolExternalRefFileIndex::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_names, &temp_entities)?;
        Ok(())
    }
}
impl IgesBasicToolExternalRefFileIndex {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalRefFileIndex>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileIndex::write_own_params")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        iw.send(ent_live.nb_entries()?)?;
        num = ent_live.nb_entries()?;
        i = 1;
        while i <= num {
            iw.send(ent_live.name(i)?)?;
            iw.send_entity(&ent_live.entity(i)?, false)?;
            i += 1;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicExternalRefFileIndex>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileIndex::own_shared")?;
        let mut i: i32 = 0;
        let num: i32 = ent_live.nb_entries()?;
        i = 1;
        while i <= num {
            iter.get_one_item(&ent_live.entity(i)?.cast::<StandardTransient>());
            i += 1;
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalRefFileIndex>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 12);
        iges_shared_023!(dc);
        dc
    }
}
impl IgesBasicToolExternalRefFileIndex {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicExternalRefFileIndex>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolExternalRefFileIndex {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalRefFileIndex>,
        ent: &Handle<IgesBasicExternalRefFileIndex>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalRefFileIndex::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalRefFileIndex::own_copy")?;
        let num: i32 = another_live.nb_entries()?;
        let temp_names: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        let temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_names,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another_live.name(i)?,
                    "IgesBasicToolExternalRefFileIndex::own_copy",
                )?)),
                "IgesBasicToolExternalRefFileIndex::own_copy",
            )?;
            let new_item = tc
                .transferred(&another_live.entity(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &temp_entities,
                i,
                new_item.clone(),
                "IgesBasicToolExternalRefFileIndex::own_copy",
            )?;
        }
        ent_live.init(&temp_names, &temp_entities)?;
        Ok(())
    }
}
