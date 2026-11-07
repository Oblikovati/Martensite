#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomFlash, IgesGeomLine, IgesGeomOffsetCurve, IgesGeomToolFlash};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{iges_shared_036, iges_shared_045};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_dump_xy_l, iges_dump_xyz_l, iges_live, iges_msg, iges_real_text,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolFlash {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomFlash>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::own_dump")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        let fn_: i32 = ent_live.form_number();
        writeln!(s, "IgesGeomFlash")?;
        match fn_ {
            0 => {
                writeln!(s, " --    Form defined by reference entity   --")?;
            }
            1 => {
                write!(s, " --    Circular    --  ")?;
            }
            2 => {
                write!(s, " --    Rectangle   --  ")?;
            }
            3 => {
                write!(s, " --    Donut    --  ")?;
            }
            4 => {
                write!(s, " --    Canoe    --  ")?;
            }
            _ => {}
        }
        write!(s, "Flash reference point    : ")?;
        iges_dump_xy_l(
            s,
            level,
            &ent_live.reference_point().xy(),
            &ent_live.location()?,
        )?;
        write!(
            s,
            " First sizing parameter  : {}   Second sizing parameter : {}\n Rotation about reference entity : {}\nReference Entity         : ",
            iges_real_text(ent_live.dimension1()),
            iges_real_text(ent_live.dimension2()),
            iges_real_text(ent_live.rotation())
        )?;
        dumper.dump(&ent_live.reference_entity(), s, sublevel, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolLine,
    "The own-parameter work on `IgesGeomLine`."
);
impl IgesGeomToolLine {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomLine>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolLine::read_own_params")?;
        let msg89 = iges_msg("XSTEP_89");
        let msg90 = iges_msg("XSTEP_90");
        let mut a_start: Xyz = Xyz::default();
        let mut an_end: Xyz = Xyz::default();
        pr.read_xyz(&pr.current_list(1, 3), &msg89, &mut a_start)?;
        pr.read_xyz(&pr.current_list(1, 3), &msg90, &mut an_end)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_start, an_end);
        Ok(())
    }
}
impl IgesGeomToolLine {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomLine>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolLine::write_own_params")?;
        iw.send(ent_live.start_point().x())?;
        iw.send(ent_live.start_point().y())?;
        iw.send(ent_live.start_point().z())?;
        iges_shared_045!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomLine>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomLine>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(110, 0, 2);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolLine {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomLine>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolLine {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomLine>,
        ent: &Handle<IgesGeomLine>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolLine::own_copy")?;
        let another_live = iges_live(another, "IgesGeomToolLine::own_copy")?;
        ent_live.init(
            Xyz::from(another_live.start_point()),
            Xyz::from(another_live.end_point()),
        );
        Ok(())
    }
}
impl IgesGeomToolLine {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomLine>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolLine::own_dump")?;
        let infin: i32 = ent_live.infinite();
        match infin {
            1 => {
                writeln!(s, "Semi-Infinite Line")?;
            }
            2 => {
                writeln!(s, "Infinite Line")?;
            }
            _ => {
                writeln!(s, "Bounded Line")?;
            }
        }
        write!(s, "IgesGeomLine\nStarting Point : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.start_point()),
            &ent_live.location()?,
        )?;
        write!(s, "\nEnd Point : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.end_point()),
            &ent_live.location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolOffsetCurve,
    "The own-parameter work on `IgesGeomOffsetCurve`."
);
impl IgesGeomToolOffsetCurve {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomOffsetCurve>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::read_own_params")?;
        let msg121 = iges_msg("XSTEP_121");
        iges_shared_036!(
            a_function_coord,
            a_tapered_offset_type,
            an_offset_type,
            arc_length1,
            arc_length2,
            off_distance1,
            off_distance2
        );
        let mut an_offset_param: f64 = 0.0;
        let mut another_offset_param: f64 = 0.0;
        let mut a_normal_vec: Xyz = Xyz::default();
        let mut a_base_curve: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_function: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_base_curve, false)? {
            let mut msg110 = iges_msg("XSTEP_110");
            iges_send_read_status(pr, &mut msg110, a_status, false)?;
        }
        if !pr.read_integer(&pr.current(), &mut an_offset_type)? {
            let msg111 = iges_msg("XSTEP_111");
            pr.send_fail(&msg111);
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_function, true)? {
            let mut msg112 = iges_msg("XSTEP_112");
            iges_send_read_status(pr, &mut msg112, a_status, false)?;
        }
        if !pr.read_integer(&pr.current(), &mut a_function_coord)? {
            let msg113 = iges_msg("XSTEP_113");
            pr.send_fail(&msg113);
        }
        if !pr.read_integer(&pr.current(), &mut a_tapered_offset_type)? {
            let msg114 = iges_msg("XSTEP_114");
            pr.send_fail(&msg114);
        }
        if !pr.read_real(&pr.current(), &mut off_distance1)? {
            let msg115 = iges_msg("XSTEP_115");
            pr.send_fail(&msg115);
        }
        if !pr.read_real(&pr.current(), &mut arc_length1)? {
            let msg116 = iges_msg("XSTEP_116");
            pr.send_fail(&msg116);
        }
        if !pr.read_real(&pr.current(), &mut off_distance2)? {
            let msg117 = iges_msg("XSTEP_117");
            pr.send_fail(&msg117);
        }
        if !pr.read_real(&pr.current(), &mut arc_length2)? {
            let msg118 = iges_msg("XSTEP_118");
            pr.send_fail(&msg118);
        }
        pr.read_xyz(&pr.current_list(1, 3), &msg121, &mut a_normal_vec)?;
        if !pr.read_real(&pr.current(), &mut an_offset_param)? {
            let msg119 = iges_msg("XSTEP_119");
            pr.send_fail(&msg119);
        }
        if !pr.read_real(&pr.current(), &mut another_offset_param)? {
            let msg120 = iges_msg("XSTEP_120");
            pr.send_fail(&msg120);
        }
        pr.read_xyz(&pr.current_list(1, 3), &msg121, &mut a_normal_vec)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &a_base_curve,
            an_offset_type,
            &a_function,
            a_function_coord,
            a_tapered_offset_type,
            off_distance1,
            arc_length1,
            off_distance2,
            arc_length2,
            a_normal_vec,
            an_offset_param,
            another_offset_param,
        );
        Ok(())
    }
}
impl IgesGeomToolOffsetCurve {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomOffsetCurve>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::write_own_params")?;
        iw.send_entity(&ent_live.base_curve(), false)?;
        iw.send(ent_live.offset_type())?;
        iw.send_entity(&ent_live.function(), false)?;
        iw.send(ent_live.function_parameter())?;
        iw.send(ent_live.tapered_offset_type())?;
        iw.send(ent_live.first_offset_distance())?;
        iw.send(ent_live.arc_length1())?;
        iw.send(ent_live.second_offset_distance())?;
        iw.send(ent_live.arc_length2())?;
        iw.send(ent_live.normal_vector().x())?;
        iw.send(ent_live.normal_vector().y())?;
        iw.send(ent_live.normal_vector().z())?;
        iw.send(ent_live.start_parameter())?;
        iw.send(ent_live.end_parameter())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomOffsetCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::own_shared")?;
        iter.get_one_item(&ent_live.base_curve().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.function().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomOffsetCurve>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(130, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolOffsetCurve {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomOffsetCurve>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::own_correct")?;
        if ent_live.offset_type() == 3 {
            return Ok(false);
        }
        let mut func: Handle<IgesDataIgesEntity> = ent_live.function();
        if func.is_null() {
            return Ok(false);
        }
        func.nullify();
        ent_live.init(
            &ent_live.base_curve(),
            ent_live.offset_type(),
            &func,
            0,
            ent_live.tapered_offset_type(),
            ent_live.first_offset_distance(),
            ent_live.arc_length1(),
            ent_live.second_offset_distance(),
            ent_live.arc_length2(),
            *ent_live.normal_vector().xyz(),
            ent_live.start_parameter(),
            ent_live.end_parameter(),
        );
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomOffsetCurve>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::own_check")?;
        let ot: i32 = ent_live.offset_type();
        if ot < 1 || ot > 3 {
            let msg111 = iges_msg("XSTEP_111");
            ach.send_fail(&msg111);
        }
        if ot != 1 && (ent_live.tapered_offset_type() < 1 || ent_live.tapered_offset_type() > 2) {
            let msg114 = iges_msg("XSTEP_114");
            ach.send_fail(&msg114);
        }
        Ok(())
    }
}
impl IgesGeomToolOffsetCurve {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomOffsetCurve>,
        ent: &Handle<IgesGeomOffsetCurve>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolOffsetCurve::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::own_copy")?;
        iges_shared_036!(
            a_function_coord,
            a_tapered_offset_type,
            an_offset_type,
            arc_length1,
            arc_length2,
            off_distance1,
            off_distance2
        );
        let mut an_offset_param1: f64 = 0.0;
        let mut an_offset_param2: f64 = 0.0;
        let a_base_curve = tc
            .transferred(&another_live.base_curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        an_offset_type = another_live.offset_type();
        let a_function = tc
            .transferred(&another_live.function().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        a_function_coord = another_live.function_parameter();
        a_tapered_offset_type = another_live.tapered_offset_type();
        off_distance1 = another_live.first_offset_distance();
        arc_length1 = another_live.arc_length1();
        off_distance2 = another_live.second_offset_distance();
        arc_length2 = another_live.arc_length2();
        let a_normal_vec: Xyz = *another_live.normal_vector().xyz();
        an_offset_param1 = another_live.start_parameter();
        an_offset_param2 = another_live.end_parameter();
        ent_live.init(
            &a_base_curve,
            an_offset_type,
            &a_function,
            a_function_coord,
            a_tapered_offset_type,
            off_distance1,
            arc_length1,
            off_distance2,
            arc_length2,
            a_normal_vec,
            an_offset_param1,
            an_offset_param2,
        );
        Ok(())
    }
}
