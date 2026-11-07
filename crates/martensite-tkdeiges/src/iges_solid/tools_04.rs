#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidEllipsoid, IgesSolidFace, IgesSolidLoop, IgesSolidToolEllipsoid};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{
    iges_shared_010, iges_shared_017, iges_shared_044, iges_shared_046, iges_shared_051,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_dump_xyz, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolEllipsoid {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidEllipsoid>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEllipsoid::read_own_params")?;
        let mut temp_size: Xyz = Xyz::default();
        let mut temp_center: Xyz = Xyz::default();
        let mut temp_x_axis: Xyz = Xyz::default();
        let mut temp_z_axis: Xyz = Xyz::default();
        let mut tempreal: f64 = 0.0;
        pr.read_xyz(&pr.current_list(1, 3), "Size", &mut temp_size)?;
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (X)", &mut tempreal)? {
                temp_center.set_x(tempreal);
            }
        } else {
            temp_center.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (Y)", &mut tempreal)? {
                temp_center.set_y(tempreal);
            }
        } else {
            temp_center.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (Z)", &mut tempreal)? {
                temp_center.set_z(tempreal);
            }
        } else {
            temp_center.set_z(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local X axis (I)", &mut tempreal)? {
                temp_x_axis.set_x(tempreal);
            }
        } else {
            temp_x_axis.set_x(1.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local X axis (J)", &mut tempreal)? {
                temp_x_axis.set_y(tempreal);
            }
        } else {
            temp_x_axis.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local X axis (K)", &mut tempreal)? {
                temp_x_axis.set_z(tempreal);
            }
        } else {
            temp_x_axis.set_z(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local Z axis (I)", &mut tempreal)? {
                temp_z_axis.set_x(tempreal);
            }
        } else {
            temp_z_axis.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local Z axis (J)", &mut tempreal)? {
                temp_z_axis.set_y(tempreal);
            }
        } else {
            temp_z_axis.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Local Z axis (K)", &mut tempreal)? {
                temp_z_axis.set_z(tempreal);
            }
        } else {
            temp_z_axis.set_z(1.0);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_size, temp_center, temp_x_axis, temp_z_axis);
        iges_shared_017!(ent_live, eps, pr, temp_x_axis, temp_z_axis);
        Ok(())
    }
}
impl IgesSolidToolEllipsoid {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidEllipsoid>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEllipsoid::write_own_params")?;
        iges_shared_046!(ent_live, iw);
        iges_shared_044!(ent_live, iw);
        iw.send(ent_live.x_axis()?.x())?;
        iw.send(ent_live.x_axis()?.y())?;
        iw.send(ent_live.x_axis()?.z())?;
        iw.send(ent_live.z_axis()?.x())?;
        iw.send(ent_live.z_axis()?.y())?;
        iw.send(ent_live.z_axis()?.z())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidEllipsoid>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidEllipsoid>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(168, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolEllipsoid {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidEllipsoid>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEllipsoid::own_check")?;
        let eps: f64 = 1.0e-04;
        let prosca: f64 = Xyz::from(ent_live.x_axis()?).dot(Xyz::from(ent_live.z_axis()?));
        if prosca < -eps || prosca > eps {
            ach.add_fail_text("Local Z axis : Not orthogonal to X axis", "");
        }
        if ent_live.size().x() < ent_live.size().y()
            || ent_live.size().y() < ent_live.size().z()
            || ent_live.size().z() <= 0.0
        {
            ach.add_fail_text("Size : The values does not satisfy LX >= LY >= LZ > 0", "");
        }
        Ok(())
    }
}
impl IgesSolidToolEllipsoid {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidEllipsoid>,
        ent: &Handle<IgesSolidEllipsoid>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEllipsoid::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolEllipsoid::own_copy")?;
        ent_live.init(
            another_live.size(),
            Xyz::from(another_live.center()),
            Xyz::from(another_live.x_axis()?),
            Xyz::from(another_live.z_axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolEllipsoid {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidEllipsoid>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolEllipsoid::own_dump")?;
        write!(s, "IgesSolidEllipsoid\nSize   : ")?;
        iges_dump_xyz(s, &ent_live.size())?;
        write!(s, "\nCenter : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.center()),
            &ent_live.location()?,
        )?;
        iges_shared_010!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolFace,
    "The own-parameter work on `IgesSolidFace`."
);
impl IgesSolidToolFace {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidFace>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolFace::read_own_params")?;
        let msg197 = iges_msg("XSTEP_197");
        let msg198 = iges_msg("XSTEP_198");
        let mut outer_loop_flag: bool = false;
        let mut aloop: Handle<IgesSolidLoop> = Handle::default();
        let mut temp_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut nbloops: i32 = 0;
        let mut temp_loops: Handle<HArray1<Handle<IgesSolidLoop>>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_surface, false)? {
            let mut msg196 = iges_msg("XSTEP_196");
            iges_send_read_status(pr, &mut msg196, a_status, false)?;
        }
        let st: bool = pr.read_integer(&pr.current(), &mut nbloops)?;
        if !st {
            pr.send_fail(&msg197);
        }
        if st && nbloops > 0 {
            temp_loops = iges_new_array1::<Handle<IgesSolidLoop>>(1, nbloops)?;
        } else {
            pr.send_fail(&msg197);
        }
        pr.read_boolean(&pr.current(), &msg198, &mut outer_loop_flag, true)?;
        if !temp_loops.is_null() {
            for i in 1..=nbloops {
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesSolidLoop::static_type(),
                    &mut aloop,
                    false,
                )? {
                    iges_array1_set(
                        &temp_loops,
                        i,
                        aloop.clone(),
                        "IgesSolidToolFace::read_own_params",
                    )?;
                } else {
                    let mut msg199 = iges_msg("XSTEP_199");
                    iges_send_read_status(pr, &mut msg199, a_status, true)?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_surface, outer_loop_flag, &temp_loops)?;
        Ok(())
    }
}
impl IgesSolidToolFace {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidFace>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolFace::write_own_params")?;
        let upper: i32 = ent_live.nb_loops()?;
        iw.send_entity(&ent_live.surface(), false)?;
        iw.send(upper)?;
        iw.send_boolean(ent_live.has_outer_loop())?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.loop_(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidFace>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolFace::own_shared")?;
        let upper: i32 = ent_live.nb_loops()?;
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        for i in 1..=upper {
            iter.get_one_item(&ent_live.loop_(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidFace>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(510, 1);
        iges_shared_051!(dc);
        dc
    }
}
impl IgesSolidToolFace {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidFace>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolFace::own_check")?;
        if ent_live.nb_loops()? <= 0 {
            let msg197 = iges_msg("XSTEP_197");
            ach.send_fail(&msg197);
        }
        Ok(())
    }
}
impl IgesSolidToolFace {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidFace>,
        ent: &Handle<IgesSolidFace>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolFace::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolFace::own_copy")?;
        let temp_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let nbloops: i32 = another_live.nb_loops()?;
        let outer_loop_flag: bool = another_live.has_outer_loop();
        let temp_loops: Handle<HArray1<Handle<IgesSolidLoop>>> =
            iges_new_array1::<Handle<IgesSolidLoop>>(1, nbloops)?;
        for i in 1..=nbloops {
            let anent = tc
                .transferred(&another_live.loop_(i)?.cast::<StandardTransient>())?
                .cast::<IgesSolidLoop>();
            iges_array1_set(&temp_loops, i, anent.clone(), "IgesSolidToolFace::own_copy")?;
        }
        ent_live.init(&temp_surface, outer_loop_flag, &temp_loops)?;
        Ok(())
    }
}
impl IgesSolidToolFace {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidFace>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolFace::own_dump")?;
        writeln!(s, "IgesSolidFace")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "Surface : ")?;
        dumper.dump(&ent_live.surface(), s, sublevel, -1)?;
        writeln!(s)?;
        if ent_live.has_outer_loop() {
            writeln!(s, "Outer loop is present (First one)")?;
        } else {
            writeln!(s, "Outer loop is not present")?;
        }
        write!(s, "Loops : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_loops()?, |first| {
            ent_live.loop_(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolLoop,
    "The own-parameter work on `IgesSolidLoop`."
);
