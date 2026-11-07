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
    IgesBasicExternalReferenceFile, IgesBasicGroup, IgesBasicGroupWithoutBackP,
    IgesBasicToolExternalReferenceFile,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_023, iges_shared_030, iges_shared_048};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_dump_strings, iges_live,
    iges_msg, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesBasicToolExternalReferenceFile {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicExternalReferenceFile>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolExternalReferenceFile::write_own_params")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        iw.send(ent_live.nb_list_entries()?)?;
        num = ent_live.nb_list_entries()?;
        i = 1;
        while i <= num {
            iw.send(ent_live.name(i)?)?;
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
        _ent: &Handle<IgesBasicExternalReferenceFile>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicExternalReferenceFile>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(406, 12);
        iges_shared_023!(dc);
        dc
    }
}
impl IgesBasicToolExternalReferenceFile {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicExternalReferenceFile>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolExternalReferenceFile {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicExternalReferenceFile>,
        ent: &Handle<IgesBasicExternalReferenceFile>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolExternalReferenceFile::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolExternalReferenceFile::own_copy")?;
        let num: i32 = another_live.nb_list_entries()?;
        let temp_names: Handle<HArray1<Handle<HAsciiString>>> =
            iges_new_array1::<Handle<HAsciiString>>(1, num)?;
        for i in 1..=num {
            iges_array1_set(
                &temp_names,
                i,
                Handle::new(HAsciiString::copied(iges_live(
                    &another_live.name(i)?,
                    "IgesBasicToolExternalReferenceFile::own_copy",
                )?)),
                "IgesBasicToolExternalReferenceFile::own_copy",
            )?;
        }
        ent_live.init(&temp_names)?;
        Ok(())
    }
}
impl IgesBasicToolExternalReferenceFile {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicExternalReferenceFile>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolExternalReferenceFile::own_dump")?;
        write!(
            s,
            "IgesBasicExternalReferenceFile\nExternal Reference Names : "
        )?;
        iges_dump_strings(s, level, 1, ent_live.nb_list_entries()?, |first| {
            ent_live.name(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolGroup,
    "The own-parameter work on `IgesBasicGroup`."
);
impl IgesBasicToolGroup {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicGroup>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::read_own_params")?;
        let mut nbval: i32 = 0;
        let mut ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        if pr.read_integer(&pr.current(), &mut nbval)? {
            let mut msg203 = iges_msg("XSTEP_203");
            msg203.arg_integer(1)?;
            pr.read_ents(ir, &pr.current_list(nbval, 1), &msg203, &mut ent_array, 1)?;
        } else {
            let mut msg202 = iges_msg("XSTEP_202");
            msg202.arg_integer(1)?;
            pr.send_fail(&msg202);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolGroup {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicGroup>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::write_own_params")?;
        let upper: i32 = ent_live.nb_entities();
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicGroup>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::own_shared")?;
        let upper: i32 = ent_live.nb_entities();
        for i in 1..=upper {
            iter.get_one_item(&ent_live.entity(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicGroup>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesBasicToolGroup {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicGroup>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::own_correct")?;
        let mut ianul: i32 = 0;
        let mut nbtrue: i32 = 0;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul += 1;
            } else if iges_live(&val, "IgesBasicToolGroup::own_correct")?.type_number() == 0 {
                ianul += 1;
            }
        }
        iges_shared_030!(ent_array, ianul, nb);
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                continue;
            } else if iges_live(&val, "IgesBasicToolGroup::own_correct")?.type_number() == 0 {
                continue;
            }
            nbtrue += 1;
            iges_array1_set(
                &ent_array,
                nbtrue,
                ent_live.entity(i)?,
                "IgesBasicToolGroup::own_correct",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicGroup>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::own_check")?;
        let mut ianul: bool = false;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul = true;
            } else if iges_live(&val, "IgesBasicToolGroup::own_check")?.type_number() == 0 {
                ianul = true;
            }
            if ianul {
                break;
            }
        }
        Ok(())
    }
}
impl IgesBasicToolGroup {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicGroup>,
        ent: &Handle<IgesBasicGroup>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolGroup::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolGroup::own_copy")?;
        iges_shared_048!(another_live, ent_array, lower, upper);
        for i in lower..=upper {
            let myentity = tc
                .transferred(&another_live.entity(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolGroup::own_copy",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolGroup {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicGroup>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolGroup::own_dump")?;
        write!(s, "IgesBasicGroup\nEntries in the Group : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_entities(), |first| {
            ent_live.entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolGroupWithoutBackP,
    "The own-parameter work on `IgesBasicGroupWithoutBackP`."
);
impl IgesBasicToolGroupWithoutBackP {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::read_own_params")?;
        let mut nbval: i32 = 0;
        let mut ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        if pr.read_integer(&pr.current(), &mut nbval)? {
            let mut msg203 = iges_msg("XSTEP_203");
            msg203.arg_integer(7)?;
            pr.read_ents(ir, &pr.current_list(nbval, 1), &msg203, &mut ent_array, 1)?;
        } else {
            let mut msg202 = iges_msg("XSTEP_202");
            msg202.arg_integer(7)?;
            pr.send_fail(&msg202);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolGroupWithoutBackP {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::write_own_params")?;
        let upper: i32 = ent_live.nb_entities();
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::own_shared")?;
        let upper: i32 = ent_live.nb_entities();
        for i in 1..=upper {
            iter.get_one_item(&ent_live.entity(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicGroupWithoutBackP>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 7);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesBasicToolGroupWithoutBackP {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicGroupWithoutBackP>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::own_correct")?;
        let mut ianul: i32 = 0;
        let mut nbtrue: i32 = 0;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul += 1;
            } else if iges_live(&val, "IgesBasicToolGroupWithoutBackP::own_correct")?.type_number()
                == 0
            {
                ianul += 1;
            }
        }
        iges_shared_030!(ent_array, ianul, nb);
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                continue;
            } else if iges_live(&val, "IgesBasicToolGroupWithoutBackP::own_correct")?.type_number()
                == 0
            {
                continue;
            }
            nbtrue += 1;
            iges_array1_set(
                &ent_array,
                nbtrue,
                ent_live.entity(i)?,
                "IgesBasicToolGroupWithoutBackP::own_correct",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::own_check")?;
        let mut ianul: bool = false;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul = true;
            } else if iges_live(&val, "IgesBasicToolGroupWithoutBackP::own_check")?.type_number()
                == 0
            {
                ianul = true;
            }
            if ianul {
                break;
            }
        }
        Ok(())
    }
}
