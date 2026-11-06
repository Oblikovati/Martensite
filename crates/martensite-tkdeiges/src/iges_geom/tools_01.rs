#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::IgesGeomBSplineCurve;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_033, iges_shared_050};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_list_xyz_l, iges_dump_vals, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGeomToolBSplineCurve,
    "The own-parameter work on `IgesGeomBSplineCurve`."
);
impl IgesGeomToolBSplineCurve {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomBSplineCurve>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineCurve::read_own_params")?;
        let msg99 = iges_msg("XSTEP_99");
        let msg100 = iges_msg("XSTEP_100");
        let msg101 = iges_msg("XSTEP_101");
        let msg102 = iges_msg("XSTEP_102");
        let msg103 = iges_msg("XSTEP_103");
        iges_shared_050!(
            a_closed,
            a_degree,
            a_periodic,
            a_planar,
            a_polynomial,
            an_index
        );
        let mut a_umin: f64 = 0.0;
        let mut a_umax: f64 = 0.0;
        let mut norm_x: f64 = 0.0;
        let mut norm_y: f64 = 0.0;
        let mut norm_z: f64 = 0.0;
        let mut a_norm: Xyz = Xyz::new(0.0, 0.0, 0.0);
        iges_shared_033!(all_knots, all_poles, all_weights);
        if pr.read_integer(&pr.current(), &mut an_index)? {
            if an_index < 0 {
                let msg97 = iges_msg("XSTEP_97");
                pr.send_fail(&msg97);
                an_index = 0;
            } else {
                all_poles = iges_new_array1::<Xyz>(0, an_index)?;
            }
        } else {
            let msg97 = iges_msg("XSTEP_97");
            pr.send_fail(&msg97);
        }
        if !pr.read_integer(&pr.current(), &mut a_degree)? {
            a_degree = 0;
            let msg98 = iges_msg("XSTEP_98");
            pr.send_fail(&msg98);
        }
        pr.read_boolean(&pr.current(), &msg99, &mut a_planar, true)?;
        pr.read_boolean(&pr.current(), &msg100, &mut a_closed, true)?;
        pr.read_boolean(&pr.current(), &msg101, &mut a_polynomial, true)?;
        pr.read_boolean(&pr.current(), &msg102, &mut a_periodic, true)?;
        let nb_knots: i32 = an_index + a_degree + 2;
        pr.read_reals(
            &pr.current_list(nb_knots, 1),
            &msg103,
            &mut all_knots,
            -a_degree,
        )?;
        if !all_poles.is_null() {
            let msg104 = iges_msg("XSTEP_104");
            let msg105 = iges_msg("XSTEP_105");
            pr.read_reals(
                &pr.current_list(an_index + 1, 1),
                &msg104,
                &mut all_weights,
                0,
            )?;
            for i in 0..=an_index {
                let mut temp_pole: Xyz = Xyz::default();
                if pr.read_xyz(&pr.current_list(1, 3), &msg105, &mut temp_pole)? {
                    iges_array1_set(
                        &all_poles,
                        i,
                        temp_pole,
                        "IgesGeomToolBSplineCurve::read_own_params",
                    )?;
                }
            }
        }
        if !pr.read_real(&pr.current(), &mut a_umin)? {
            let msg106 = iges_msg("XSTEP_106");
            pr.send_fail(&msg106);
        }
        if !pr.read_real(&pr.current(), &mut a_umax)? {
            let msg107 = iges_msg("XSTEP_107");
            pr.send_fail(&msg107);
        }
        let mut st: bool = false;
        if pr.defined_else_skip()? {
            st = pr.read_real(&pr.current(), &mut norm_x)?;
            if !st {
                let msg108 = iges_msg("XSTEP_108");
                pr.send_fail(&msg108);
            }
        } else {
            norm_x = 0.0;
        }
        if pr.defined_else_skip()? {
            st = pr.read_real(&pr.current(), &mut norm_y)?;
            if !st {
                let msg108 = iges_msg("XSTEP_108");
                pr.send_fail(&msg108);
            }
        } else {
            norm_y = 0.0;
        }
        if pr.defined_else_skip()? {
            st = pr.read_real(&pr.current(), &mut norm_z)?;
            if !st {
                let msg108 = iges_msg("XSTEP_108");
                pr.send_fail(&msg108);
            }
        } else {
            norm_z = 0.0;
        }
        if st {
            a_norm.set_coordinates(norm_x, norm_y, norm_z);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            an_index,
            a_degree,
            a_planar,
            a_closed,
            a_polynomial,
            a_periodic,
            &all_knots,
            &all_weights,
            &all_poles,
            a_umin,
            a_umax,
            a_norm,
        )?;
        Ok(())
    }
}
impl IgesGeomToolBSplineCurve {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomBSplineCurve>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineCurve::write_own_params")?;
        let mut low: i32 = 0;
        let mut up: i32 = 0;
        let index: i32 = ent_live.upper_index();
        let degree: i32 = ent_live.degree();
        iw.send(index)?;
        iw.send(degree)?;
        iw.send_boolean(ent_live.is_planar())?;
        iw.send_boolean(ent_live.is_closed())?;
        iw.send_boolean(ent_live.is_polynomial(false)?)?;
        iw.send_boolean(ent_live.is_periodic())?;
        low = -degree;
        up = index + 1;
        for i in low..=up {
            iw.send(ent_live.knot(i)?)?;
        }
        low = 0;
        up = index;
        for i in low..=up {
            iw.send(ent_live.weight(i)?)?;
        }
        for i in low..=up {
            iw.send(ent_live.pole(i)?.x())?;
            iw.send(ent_live.pole(i)?.y())?;
            iw.send(ent_live.pole(i)?.z())?;
        }
        iw.send(ent_live.u_min())?;
        iw.send(ent_live.u_max())?;
        iw.send(ent_live.normal().x())?;
        iw.send(ent_live.normal().y())?;
        iw.send(ent_live.normal().z())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomBSplineCurve>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomBSplineCurve>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(126, 0, 5);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolBSplineCurve {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomBSplineCurve>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineCurve::own_check")?;
        let eps: f64 = 1.0e-04;
        let lower: i32 = 0;
        let upper: i32 = ent_live.upper_index();
        let mut flag: bool = true;
        let mut i: i32 = 0;
        while i < upper && flag {
            flag &= ent_live.weight(i)? > 0.0;
            i += 1;
        }
        if !flag {
            let msg104 = iges_msg("XSTEP_104");
            ach.send_fail(&msg104);
        }
        flag = true;
        let temp_val: f64 = ent_live.weight(lower)?;
        i = lower;
        while i < upper && flag {
            flag &= ent_live.weight(i)? == temp_val;
            i += 1;
        }
        if ent_live.is_planar() {
            let a_norm: Xyz = ent_live.normal();
            let epsn: f64 = eps * 10.0;
            let normod: f64 = a_norm.square_modulus();
            if normod < epsn {
                let msg109 = iges_msg("XSTEP_109");
                ach.add_warning_msg(&msg109);
            }
        }
        Ok(())
    }
}
impl IgesGeomToolBSplineCurve {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomBSplineCurve>,
        ent: &Handle<IgesGeomBSplineCurve>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolBSplineCurve::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolBSplineCurve::own_copy")?;
        let mut low: i32 = 0;
        let mut up: i32 = 0;
        iges_shared_050!(
            a_closed,
            a_degree,
            a_periodic,
            a_planar,
            a_polynomial,
            an_index
        );
        iges_shared_033!(all_knots, all_poles, all_weights);
        let mut a_umin: f64 = 0.0;
        let mut a_umax: f64 = 0.0;
        let mut a_norm: Xyz = Xyz::default();
        an_index = another_live.upper_index();
        a_degree = another_live.degree();
        a_planar = another_live.is_planar();
        a_closed = another_live.is_closed();
        a_polynomial = another_live.is_polynomial(false)?;
        a_periodic = another_live.is_periodic();
        all_knots = iges_new_array1::<f64>(-a_degree, an_index + 1)?;
        low = -a_degree;
        up = an_index + 1;
        for i in low..=up {
            iges_array1_set(
                &all_knots,
                i,
                another_live.knot(i)?,
                "IgesGeomToolBSplineCurve::own_copy",
            )?;
        }
        all_weights = iges_new_array1::<f64>(0, an_index)?;
        low = 0;
        up = an_index;
        for i in low..=up {
            iges_array1_set(
                &all_weights,
                i,
                another_live.weight(i)?,
                "IgesGeomToolBSplineCurve::own_copy",
            )?;
        }
        all_poles = iges_new_array1::<Xyz>(0, an_index)?;
        for i in low..=up {
            iges_array1_set(
                &all_poles,
                i,
                Xyz::from(another_live.pole(i)?),
                "IgesGeomToolBSplineCurve::own_copy",
            )?;
        }
        a_umin = another_live.u_min();
        a_umax = another_live.u_max();
        a_norm = another_live.normal();
        ent_live.init(
            an_index,
            a_degree,
            a_planar,
            a_closed,
            a_polynomial,
            a_periodic,
            &all_knots,
            &all_weights,
            &all_poles,
            a_umin,
            a_umax,
            a_norm,
        )?;
        Ok(())
    }
}
impl IgesGeomToolBSplineCurve {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomBSplineCurve>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineCurve::own_dump")?;
        let upind: i32 = ent_live.upper_index();
        write!(
            s,
            "IgesGeomBSplineCurve\nSum UpperIndex : {}   Degree : {}  {}\n{}  {}  {}\nKnots : ",
            upind,
            ent_live.degree(),
            if ent_live.is_planar() {
                "Planar"
            } else {
                "NonPlanar"
            },
            if ent_live.is_closed() {
                "Closed"
            } else {
                "Open"
            },
            if ent_live.is_periodic() {
                "Periodic"
            } else {
                "NonPeriodic"
            },
            if ent_live.is_polynomial(true)? {
                "Polynomial"
            } else {
                "Rational"
            }
        )?;
        iges_dump_vals(s, level, -ent_live.degree(), upind + 1, |first| {
            ent_live.knot(first)
        })?;
        write!(s, "\nWeights : ")?;
        iges_dump_vals(s, level, 0, upind, |first| ent_live.weight(first))?;
        write!(s, "\nControl Points (Poles) : ")?;
        iges_dump_list_xyz_l(
            s,
            level,
            0,
            upind,
            |first| Ok(Xyz::from(ent_live.pole(first)?)),
            &ent_live.location()?,
        )?;
        write!(
            s,
            "\nStarting Parameter Value : {}  Ending Parameter Value : {}\nUnit Normal : ",
            iges_real_text(ent_live.u_min()),
            iges_real_text(ent_live.u_max())
        )?;
        iges_dump_xyz_l(s, level, &ent_live.normal(), &ent_live.location()?)?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolBSplineSurface,
    "The own-parameter work on `IgesGeomBSplineSurface`."
);
