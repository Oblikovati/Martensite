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
    IgesBasicSingleParent, IgesBasicSingularSubfigure, IgesBasicSubfigureDef,
    IgesBasicToolSingleParent,
};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_dump_string, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesBasicToolSingleParent {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicSingleParent>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::own_correct")?;
        if ent_live.nb_parent_entities() == 1 {
            return Ok(false);
        }
        let nb: i32 = ent_live.nb_children();
        let ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(
                &ent_array,
                i,
                ent_live.child(i)?,
                "IgesBasicToolSingleParent::own_correct",
            )?;
        }
        ent_live.init(1, &ent_live.single_parent(), &ent_array)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicSingleParent>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::own_check")?;
        if ent_live.nb_parent_entities() != 1 {
            let msg204 = iges_msg("XSTEP_204");
            ach.send_fail(&msg204);
        }
        Ok(())
    }
}
impl IgesBasicToolSingleParent {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicSingleParent>,
        ent: &Handle<IgesBasicSingleParent>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolSingleParent::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::own_copy")?;
        let a_nb_parent_entities: i32 = another_live.nb_parent_entities();
        let aparent = tc
            .transferred(&another_live.single_parent().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let upper: i32 = another_live.nb_children();
        let ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, upper)?;
        for i in 1..=upper {
            let myentity = tc
                .transferred(&another_live.child(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolSingleParent::own_copy",
            )?;
        }
        ent_live.init(a_nb_parent_entities, &aparent, &ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolSingleParent {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicSingleParent>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::own_dump")?;
        write!(
            s,
            "IgesBasicSingleParent\nNumber of ParentEntities : {}\nParentEntity : ",
            ent_live.nb_parent_entities()
        )?;
        dumper.dump(
            &ent_live.single_parent(),
            s,
            if level <= 4 { 0 } else { 1 },
            -1,
        )?;
        write!(s, "\nChildren : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_children(), |first| {
            ent_live.child(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolSingularSubfigure,
    "The own-parameter work on `IgesBasicSingularSubfigure`."
);
impl IgesBasicToolSingularSubfigure {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicSingularSubfigure>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingularSubfigure::read_own_params")?;
        let msg213 = iges_msg("XSTEP_213");
        let mut temphasscale: bool = false;
        let mut temp_scale_factor: f64 = 0.0;
        let mut temp_subfigure_def: Handle<IgesBasicSubfigureDef> = Handle::default();
        let mut temp_translation: Xyz = Xyz::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_typed_status(
            ir,
            &pr.current(),
            &mut a_status,
            IgesBasicSubfigureDef::static_type(),
            &mut temp_subfigure_def,
            false,
        )? {
            let mut msg212 = iges_msg("XSTEP_212");
            iges_send_read_status(pr, &mut msg212, a_status, true)?;
        }
        pr.read_xyz(&pr.current_list(1, 3), &msg213, &mut temp_translation)?;
        if pr.defined_else_skip()? {
            temphasscale = pr.read_real(&pr.current(), &mut temp_scale_factor)?;
            if !temphasscale {
                let msg214 = iges_msg("XSTEP_214");
                pr.send_fail(&msg214);
            }
        } else {
            temp_scale_factor = 1.0;
            temphasscale = false;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &temp_subfigure_def,
            temp_translation,
            temphasscale,
            temp_scale_factor,
        );
        Ok(())
    }
}
impl IgesBasicToolSingularSubfigure {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicSingularSubfigure>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingularSubfigure::write_own_params")?;
        iw.send_entity(&ent_live.subfigure(), false)?;
        iw.send(ent_live.translation().x())?;
        iw.send(ent_live.translation().y())?;
        iw.send(ent_live.translation().z())?;
        if ent_live.has_scale_factor() {
            iw.send(ent_live.scale_factor())?;
        } else {
            iw.send_void()?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicSingularSubfigure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingularSubfigure::own_shared")?;
        iter.get_one_item(&ent_live.subfigure().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, ent: &Handle<IgesBasicSingularSubfigure>) -> IgesDataDirChecker {
        let Some(ent_live) = ent.get() else {
            return IgesDataDirChecker::new();
        };
        let mut dc = IgesDataDirChecker::with_form(408, 0);
        dc.structure(IgesDataDefType::DefVoid);
        if ent_live.hierarchy_status() == 1 {
            dc.graphics_ignored(1);
        }
        dc
    }
}
impl IgesBasicToolSingularSubfigure {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicSingularSubfigure>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolSingularSubfigure {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicSingularSubfigure>,
        ent: &Handle<IgesBasicSingularSubfigure>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolSingularSubfigure::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolSingularSubfigure::own_copy")?;
        let mut a_translation: Xyz = Xyz::default();
        let mut ahas_scale: bool = false;
        let mut a_scale: f64 = 0.0;
        let a_subfigure_def = tc
            .transferred(&another_live.subfigure().cast::<StandardTransient>())?
            .cast::<IgesBasicSubfigureDef>();
        a_translation = another_live.translation();
        ahas_scale = another_live.has_scale_factor();
        a_scale = another_live.scale_factor();
        ent_live.init(&a_subfigure_def, a_translation, ahas_scale, a_scale);
        Ok(())
    }
}
impl IgesBasicToolSingularSubfigure {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicSingularSubfigure>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolSingularSubfigure::own_dump")?;
        write!(
            s,
            "IgesBasicSingularSubfigure\nSubfigure Definition Entity : "
        )?;
        dumper.dump(&ent_live.subfigure(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        write!(s, "\n Translation Data : ")?;
        iges_dump_xyz_l(s, level, &ent_live.translation(), &ent_live.location()?)?;
        write!(
            s,
            "  Scale Factors : {}\n\n",
            iges_real_text(ent_live.scale_factor())
        )?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolSubfigureDef,
    "The own-parameter work on `IgesBasicSubfigureDef`."
);
impl IgesBasicToolSubfigureDef {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicSubfigureDef>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSubfigureDef::read_own_params")?;
        let msg209 = iges_msg("XSTEP_209");
        let mut temp_depth: i32 = 0;
        let mut temp_name: Handle<HAsciiString> = Handle::default();
        let mut nbval: i32 = 0;
        let mut ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        if !pr.read_integer(&pr.current(), &mut temp_depth)? {
            let msg208 = iges_msg("XSTEP_208");
            pr.send_fail(&msg208);
        }
        pr.read_text(&pr.current(), &msg209, &mut temp_name)?;
        if pr.read_integer(&pr.current(), &mut nbval)? {
            let msg211 = iges_msg("XSTEP_211");
            pr.read_ents(ir, &pr.current_list(nbval, 1), &msg211, &mut ent_array, 1)?;
        } else {
            let msg210 = iges_msg("XSTEP_210");
            pr.send_fail(&msg210);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_depth, &temp_name, &ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolSubfigureDef {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicSubfigureDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSubfigureDef::write_own_params")?;
        let nb: i32 = ent_live.nb_entities();
        iw.send(ent_live.depth())?;
        iw.send(ent_live.name())?;
        iw.send(nb)?;
        for i in 1..=nb {
            iw.send_entity(&ent_live.associated_entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicSubfigureDef>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSubfigureDef::own_shared")?;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            iter.get_one_item(&ent_live.associated_entity(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, ent: &Handle<IgesBasicSubfigureDef>) -> IgesDataDirChecker {
        let Some(ent_live) = ent.get() else {
            return IgesDataDirChecker::new();
        };
        let mut dc = IgesDataDirChecker::with_form(308, 0);
        dc.structure(IgesDataDefType::DefVoid);
        if ent_live.hierarchy_status() == 1 {
            dc.graphics_ignored(1);
        } else {
            dc.blank_status_ignored();
        }
        dc.use_flag_required(2);
        dc
    }
}
impl IgesBasicToolSubfigureDef {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesBasicSubfigureDef>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesBasicToolSubfigureDef {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicSubfigureDef>,
        ent: &Handle<IgesBasicSubfigureDef>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolSubfigureDef::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolSubfigureDef::own_copy")?;
        let mut lower: i32 = 0;
        let mut upper: i32 = 0;
        let a_depth: i32 = another_live.depth();
        let a_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.name(),
            "IgesBasicToolSubfigureDef::own_copy",
        )?));
        lower = 1;
        upper = another_live.nb_entities();
        let ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(lower, upper)?;
        for i in lower..=upper {
            let myentity = tc
                .transferred(
                    &another_live
                        .associated_entity(i)?
                        .cast::<StandardTransient>(),
                )?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolSubfigureDef::own_copy",
            )?;
        }
        ent_live.init(a_depth, &a_name, &ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolSubfigureDef {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicSubfigureDef>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolSubfigureDef::own_dump")?;
        write!(
            s,
            "IgesBasicSubfigureDef\nDepth of the subfigure : {}\nName of subfigure : ",
            ent_live.depth()
        )?;
        iges_dump_string(s, &ent_live.name())?;
        write!(s, "\nThe Associated Entities : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_entities(), |first| {
            ent_live.associated_entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
