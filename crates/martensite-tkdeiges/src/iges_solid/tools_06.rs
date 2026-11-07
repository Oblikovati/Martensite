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
    IgesSolidManifoldSolid, IgesSolidPlaneSurface, IgesSolidShell, IgesSolidToolManifoldSolid,
};
use crate::iges_basic::entity_shape::{iges_array1_fill, iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::iges_shared_035;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_live, iges_msg, iges_new_array1,
};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolManifoldSolid {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidManifoldSolid>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolManifoldSolid::read_own_params")?;
        let msg180 = iges_msg("XSTEP_180");
        let mut abool: bool = false;
        let mut shell_flag: bool = false;
        let mut nbshells: i32 = 0;
        let mut void_shell_flags: Handle<HArray1<i32>> = Handle::default();
        let mut shell: Handle<IgesDataIgesEntity> = Handle::default();
        let mut ashell: Handle<IgesSolidShell> = Handle::default();
        let mut void_shells: Handle<HArray1<Handle<IgesSolidShell>>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut shell, false)? {
            let mut msg178 = iges_msg("XSTEP_178");
            iges_send_read_status(pr, &mut msg178, a_status, false)?;
        }
        pr.read_boolean(&pr.current(), &msg180, &mut shell_flag, true)?;
        let st: bool = pr.read_integer(&pr.current(), &mut nbshells)?;
        if !st {
            let msg181 = iges_msg("XSTEP_181");
            pr.send_fail(&msg181);
        }
        if st && nbshells > 0 {
            void_shells = iges_new_array1::<Handle<IgesSolidShell>>(1, nbshells)?;
            void_shell_flags = iges_new_array1::<i32>(1, nbshells)?;
            iges_array1_fill(
                &void_shell_flags,
                0,
                "IgesSolidToolManifoldSolid::read_own_params",
            )?;
            for i in 1..=nbshells {
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesSolidShell::static_type(),
                    &mut ashell,
                    false,
                )? {
                    iges_array1_set(
                        &void_shells,
                        i,
                        ashell.clone(),
                        "IgesSolidToolManifoldSolid::read_own_params",
                    )?;
                } else {
                    let mut msg179 = iges_msg("XSTEP_179");
                    iges_send_read_status(pr, &mut msg179, a_status, true)?;
                }
                pr.read_boolean(&pr.current(), &msg180, &mut abool, true)?;
                if abool {
                    iges_array1_set(
                        &void_shell_flags,
                        i,
                        1,
                        "IgesSolidToolManifoldSolid::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &shell.cast::<IgesSolidShell>(),
            shell_flag,
            &void_shells,
            &void_shell_flags,
        )?;
        Ok(())
    }
}
impl IgesSolidToolManifoldSolid {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidManifoldSolid>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolManifoldSolid::write_own_params")?;
        let nbshells: i32 = ent_live.nb_void_shells();
        iw.send_entity(&ent_live.shell(), false)?;
        iw.send_boolean(ent_live.orientation_flag())?;
        iw.send(nbshells)?;
        for i in 1..=nbshells {
            iw.send_entity(&ent_live.void_shell(i)?, false)?;
            iw.send_boolean(ent_live.void_orientation_flag(i)?)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidManifoldSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolManifoldSolid::own_shared")?;
        let nbshells: i32 = ent_live.nb_void_shells();
        iter.get_one_item(&ent_live.shell().cast::<StandardTransient>());
        for i in 1..=nbshells {
            iter.get_one_item(&ent_live.void_shell(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidManifoldSolid>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(186, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefVoid);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc
    }
}
impl IgesSolidToolManifoldSolid {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesSolidManifoldSolid>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesSolidToolManifoldSolid {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidManifoldSolid>,
        ent: &Handle<IgesSolidManifoldSolid>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolManifoldSolid::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolManifoldSolid::own_copy")?;
        let shell = tc
            .transferred(&another_live.shell().cast::<StandardTransient>())?
            .cast::<IgesSolidShell>();
        let shell_flag: bool = another_live.orientation_flag();
        let nbshells: i32 = another_live.nb_void_shells();
        let mut void_shells: Handle<HArray1<Handle<IgesSolidShell>>> = Handle::default();
        let mut void_flags: Handle<HArray1<i32>> = Handle::default();
        if nbshells > 0 {
            void_shells = iges_new_array1::<Handle<IgesSolidShell>>(1, nbshells)?;
            void_flags = iges_new_array1::<i32>(1, nbshells)?;
            for i in 1..=nbshells {
                let voidshell = tc
                    .transferred(&another_live.void_shell(i)?.cast::<StandardTransient>())?
                    .cast::<IgesSolidShell>();
                iges_array1_set(
                    &void_shells,
                    i,
                    voidshell.clone(),
                    "IgesSolidToolManifoldSolid::own_copy",
                )?;
                iges_array1_set(
                    &void_flags,
                    i,
                    if another_live.void_orientation_flag(i)? {
                        1
                    } else {
                        0
                    },
                    "IgesSolidToolManifoldSolid::own_copy",
                )?;
            }
        }
        ent_live.init(&shell, shell_flag, &void_shells, &void_flags)?;
        Ok(())
    }
}
impl IgesSolidToolManifoldSolid {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidManifoldSolid>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolManifoldSolid::own_dump")?;
        writeln!(s, "IgesSolidManifoldSolid")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "Shell : ")?;
        dumper.dump(&ent_live.shell(), s, sublevel, -1)?;
        writeln!(s)?;
        if ent_live.orientation_flag() {
            writeln!(s, "Orientation agrees with the underlying surface")?;
        } else {
            writeln!(s, "Orientation does not agrees with the underlying surface")?;
        }
        write!(s, "Void shells :\nOrientation flags : ")?;
        iges_dump_entities(s, dumper, -level, 1, ent_live.nb_void_shells(), |first| {
            ent_live.void_shell(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, "[")?;
            if ent_live.nb_void_shells() > 0 {
                let upper: i32 = ent_live.nb_void_shells();
                for i in 1..=upper {
                    write!(s, "[{}]:  Void shell : ", i)?;
                    dumper.dump(&ent_live.void_shell(i)?, s, sublevel, -1)?;
                    write!(s, "  - Orientation flag : ")?;
                    if ent_live.void_orientation_flag(i)? {
                        writeln!(s, "True")?;
                    } else {
                        writeln!(s, "False")?;
                    }
                }
            }
            writeln!(s, " ]")?;
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolPlaneSurface,
    "The own-parameter work on `IgesSolidPlaneSurface`."
);
impl IgesSolidToolPlaneSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidPlaneSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::read_own_params")?;
        let mut temp_location: Handle<IgesGeomPoint> = Handle::default();
        let mut temp_normal: Handle<IgesGeomDirection> = Handle::default();
        let mut temp_refdir: Handle<IgesGeomDirection> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_typed_status(
            ir,
            &pr.current(),
            &mut a_status,
            IgesGeomPoint::static_type(),
            &mut temp_location,
            false,
        )? {
            let mut msg174 = iges_msg("XSTEP_174");
            iges_send_read_status(pr, &mut msg174, a_status, true)?;
        }
        if !pr.read_entity_typed_status(
            ir,
            &pr.current(),
            &mut a_status,
            IgesGeomDirection::static_type(),
            &mut temp_normal,
            false,
        )? {
            let mut msg175 = iges_msg("XSTEP_175");
            iges_send_read_status(pr, &mut msg175, a_status, true)?;
        }
        if ent_live.form_number() == 1 {
            if !pr.read_entity_typed_status(
                ir,
                &pr.current(),
                &mut a_status,
                IgesGeomDirection::static_type(),
                &mut temp_refdir,
                false,
            )? {
                let mut msg176 = iges_msg("XSTEP_176");
                iges_send_read_status(pr, &mut msg176, a_status, true)?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_location, &temp_normal, &temp_refdir);
        Ok(())
    }
}
impl IgesSolidToolPlaneSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidPlaneSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::write_own_params")?;
        iw.send_entity(&ent_live.location_point(), false)?;
        iw.send_entity(&ent_live.normal(), false)?;
        if ent_live.is_parametrised() {
            iw.send_entity(&ent_live.reference_dir(), false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidPlaneSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::own_shared")?;
        iter.get_one_item(&ent_live.location_point().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.normal().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.reference_dir().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidPlaneSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(190, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolPlaneSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidPlaneSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::own_check")?;
        let mut fn_: i32 = 0;
        if ent_live.is_parametrised() {
            fn_ = 1;
        }
        if fn_ != ent_live.form_number() {
            let msg177 = iges_msg("XSTEP_177");
            ach.send_fail(&msg177);
        }
        Ok(())
    }
}
impl IgesSolidToolPlaneSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidPlaneSurface>,
        ent: &Handle<IgesSolidPlaneSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolPlaneSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::own_copy")?;
        let temp_location = tc
            .transferred(&another_live.location_point().cast::<StandardTransient>())?
            .cast::<IgesGeomPoint>();
        let temp_normal = tc
            .transferred(&another_live.normal().cast::<StandardTransient>())?
            .cast::<IgesGeomDirection>();
        if another_live.is_parametrised() {
            let temp_refdir = tc
                .transferred(&another_live.reference_dir().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            ent_live.init(&temp_location, &temp_normal, &temp_refdir);
        } else {
            let temp_refdir: Handle<IgesGeomDirection> = Handle::default();
            ent_live.init(&temp_location, &temp_normal, &temp_refdir);
        }
        Ok(())
    }
}
impl IgesSolidToolPlaneSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidPlaneSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolPlaneSurface::own_dump")?;
        writeln!(s, "IgesSolidPlaneSurface")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "Point on axis    : ")?;
        dumper.dump(&ent_live.location_point(), s, sublevel, -1)?;
        write!(s, "\nNormal direction : ")?;
        dumper.dump(&ent_live.normal(), s, sublevel, -1)?;
        writeln!(s)?;
        iges_shared_035!(dumper, ent_live, s, sublevel);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolRightAngularWedge,
    "The own-parameter work on `IgesSolidRightAngularWedge`."
);
