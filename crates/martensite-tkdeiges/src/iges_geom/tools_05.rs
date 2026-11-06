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
    IgesGeomCircularArc, IgesGeomCompositeCurve, IgesGeomConicArc, IgesGeomToolCircularArc,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{
    iges_shared_012, iges_shared_026, iges_shared_049, iges_shared_054,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_dump_xy_lz, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Direction3, Point3, Xy, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolCircularArc {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomCircularArc>,
        ent: &Handle<IgesGeomCircularArc>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCircularArc::own_copy")?;
        let another_live = iges_live(another, "IgesGeomToolCircularArc::own_copy")?;
        ent_live.init(
            another_live.z_plane(),
            another_live.center().xy(),
            another_live.start_point().xy(),
            another_live.end_point().xy(),
        );
        Ok(())
    }
}
impl IgesGeomToolCircularArc {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomCircularArc>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolCircularArc::own_dump")?;
        write!(
            s,
            "IgesGeomCircularArc]\nZ-Plane Displacement : {}\nCenter      : ",
            iges_real_text(ent_live.z_plane())
        )?;
        iges_dump_xy_lz(
            s,
            level,
            &ent_live.center().xy(),
            &ent_live.location()?,
            ent_live.z_plane(),
        )?;
        write!(s, "\nStart Point : ")?;
        iges_dump_xy_lz(
            s,
            level,
            &ent_live.start_point().xy(),
            &ent_live.location()?,
            ent_live.z_plane(),
        )?;
        write!(s, "\nEnd Point   : ")?;
        iges_dump_xy_lz(
            s,
            level,
            &ent_live.end_point().xy(),
            &ent_live.location()?,
            ent_live.z_plane(),
        )?;
        writeln!(s)?;
        if level <= 5 {
            return Ok(());
        }
        iges_shared_054!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolCompositeCurve,
    "The own-parameter work on `IgesGeomCompositeCurve`."
);
impl IgesGeomToolCompositeCurve {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomCompositeCurve>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCompositeCurve::read_own_params")?;
        let mut temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut num: i32 = 0;
        let st: bool = pr.read_integer(&pr.current(), &mut num)?;
        if st && num > 0 {
            let msg80 = iges_msg("XSTEP_80");
            pr.read_ents(ir, &pr.current_list(num, 1), &msg80, &mut temp_entities, 1)?;
        } else {
            let msg79 = iges_msg("XSTEP_79");
            pr.send_fail(&msg79);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_entities)?;
        Ok(())
    }
}
impl IgesGeomToolCompositeCurve {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomCompositeCurve>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCompositeCurve::write_own_params")?;
        let mut num: i32 = ent_live.nb_curves();
        let mut i: i32 = 0;
        iw.send(num)?;
        num = ent_live.nb_curves();
        i = 1;
        while i <= num {
            iw.send_entity(&ent_live.curve(i)?, false)?;
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
        ent: &Handle<IgesGeomCompositeCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCompositeCurve::own_shared")?;
        let num: i32 = ent_live.nb_curves();
        for i in 1..=num {
            iter.get_one_item(&ent_live.curve(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomCompositeCurve>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(102, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc
    }
}
impl IgesGeomToolCompositeCurve {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomCompositeCurve>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolCompositeCurve {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomCompositeCurve>,
        ent: &Handle<IgesGeomCompositeCurve>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolCompositeCurve::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolCompositeCurve::own_copy")?;
        let num: i32 = another_live.nb_curves();
        let temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        for i in 1..=num {
            let new_ent = tc
                .transferred(&another_live.curve(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &temp_entities,
                i,
                new_ent.clone(),
                "IgesGeomToolCompositeCurve::own_copy",
            )?;
        }
        ent_live.init(&temp_entities)?;
        Ok(())
    }
}
impl IgesGeomToolCompositeCurve {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomCompositeCurve>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolCompositeCurve::own_dump")?;
        write!(s, "IgesGeomCompositeCurve\nCurve Entities :\n")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_curves(), |first| {
            ent_live.curve(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolConicArc,
    "The own-parameter work on `IgesGeomConicArc`."
);
impl IgesGeomToolConicArc {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomConicArc>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::read_own_params")?;
        let msg83 = iges_msg("XSTEP_83");
        let msg84 = iges_msg("XSTEP_84");
        iges_shared_049!(a, b, c, d, e, f);
        let mut zt: f64 = 0.0;
        let mut temp_start: Xy = Xy::default();
        let mut temp_end: Xy = Xy::default();
        if !pr.read_real(&pr.current(), &mut a)?
            || !pr.read_real(&pr.current(), &mut b)?
            || !pr.read_real(&pr.current(), &mut c)?
            || !pr.read_real(&pr.current(), &mut d)?
            || !pr.read_real(&pr.current(), &mut e)?
            || !pr.read_real(&pr.current(), &mut f)?
        {
            let msg81 = iges_msg("XSTEP_81");
            pr.send_fail(&msg81);
        }
        if !pr.read_real(&pr.current(), &mut zt)? {
            let msg82 = iges_msg("XSTEP_82");
            pr.send_fail(&msg82);
        }
        pr.read_xy(&pr.current_list(1, 2), &msg83, &mut temp_start)?;
        pr.read_xy(&pr.current_list(1, 2), &msg84, &mut temp_end)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a, b, c, d, e, f, zt, temp_start, temp_end);
        Ok(())
    }
}
impl IgesGeomToolConicArc {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomConicArc>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::write_own_params")?;
        iges_shared_012!(a, b, c, d, e, ent_live, f);
        iw.send(a)?;
        iw.send(b)?;
        iw.send(c)?;
        iw.send(d)?;
        iw.send(e)?;
        iw.send(f)?;
        iw.send(ent_live.z_plane())?;
        iges_shared_026!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomConicArc>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomConicArc>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(104, 0, 3);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolConicArc {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomConicArc>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::own_correct")?;
        Ok(ent_live.own_correct())
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomConicArc>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::own_check")?;
        let cfn: i32 = ent_live.computed_form_number();
        let fn_: i32 = ent_live.form_number();
        if cfn == 0 {
        } else if fn_ != 0 && fn_ != cfn {
            let msg71 = iges_msg("XSTEP_71");
            ach.send_fail(&msg71);
        }
        Ok(())
    }
}
impl IgesGeomToolConicArc {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomConicArc>,
        ent: &Handle<IgesGeomConicArc>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolConicArc::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::own_copy")?;
        iges_shared_049!(a, b, c, d, e, f);
        another_live.equation(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
        ent_live.init(
            a,
            b,
            c,
            d,
            e,
            f,
            another_live.z_plane(),
            another_live.start_point().xy(),
            another_live.end_point().xy(),
        );
        Ok(())
    }
}
impl IgesGeomToolConicArc {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomConicArc>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolConicArc::own_dump")?;
        iges_shared_012!(a, b, c, d, e, ent_live, f);
        writeln!(s, "IgesGeomConicArc")?;
        let mut cf: i32 = ent_live.form_number();
        if cf == 0 {
            cf = ent_live.computed_form_number();
        }
        if cf == 1 {
            writeln!(s, " --     Ellipse     --")?;
        } else if cf == 2 {
            writeln!(s, " --    Hyperbola    --")?;
        } else if cf == 3 {
            writeln!(s, " --    Parabola    --")?;
        } else {
            writeln!(s, " --    (Undetermined type of Conic)    --")?;
        }
        write!(
            s,
            "Conic Coefficient A : {}\nConic Coefficient B : {}\nConic Coefficient C : {}\nConic Coefficient D : {}\nConic Coefficient E : {}\nConic Coefficient F : {}\nZ-Plane shift : {}\nStart Point : ",
            iges_real_text(a),
            iges_real_text(b),
            iges_real_text(c),
            iges_real_text(d),
            iges_real_text(e),
            iges_real_text(f),
            iges_real_text(ent_live.z_plane())
        )?;
        iges_dump_xy_lz(
            s,
            level,
            &ent_live.start_point().xy(),
            &ent_live.location()?,
            ent_live.z_plane(),
        )?;
        write!(s, "\nEnd   Point : ")?;
        iges_dump_xy_lz(
            s,
            level,
            &ent_live.end_point().xy(),
            &ent_live.location()?,
            ent_live.z_plane(),
        )?;
        writeln!(s)?;
        if level <= 4 {
            writeln!(s, " -- Computed Definition : ask level > 4")?;
        } else {
            let mut cen: Point3 = Point3::default();
            let mut ax: Direction3 = Direction3::default();
            let mut rmin: f64 = 0.0;
            let mut rmax: f64 = 0.0;
            ent_live.definition(&mut cen, &mut ax, &mut rmin, &mut rmax)?;
            writeln!(s, " -- Computed Definition (and Transformed if level > 5)")?;
            if cf != 3 {
                write!(s, " Center        : ")?;
                iges_dump_xyz_l(s, level, &Xyz::from(cen), &ent_live.location()?)?;
                writeln!(s)?;
            }
            write!(s, " Main Axis   : ")?;
            iges_dump_xyz_l(s, level, &Xyz::from(ax), &ent_live.vector_location()?)?;
            writeln!(s)?;
            if cf == 3 {
                writeln!(s, " Focal : {}", iges_real_text(rmin))?;
            } else if rmin == rmax {
                writeln!(s, " Radius (Major = Minor) : {}", iges_real_text(rmin))?;
            } else {
                writeln!(
                    s,
                    " Major Radius : {}  Minor Radius : {}",
                    iges_real_text(rmax),
                    iges_real_text(rmin)
                )?;
            }
            iges_shared_054!(ent_live, level, s);
        }
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolCopiousData,
    "The own-parameter work on `IgesGeomCopiousData`."
);
