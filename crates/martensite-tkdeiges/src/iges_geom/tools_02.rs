#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomBSplineSurface, IgesGeomToolBSplineSurface};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesReaderData, IgesDataIgesWriter,
    IgesDataParamReader,
};
use crate::iges_data::{
    iges_array1_set, iges_array2_set, iges_live, iges_msg, iges_new_array1, iges_new_array2,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceParamType,
    InterfaceShareTool,
};

impl IgesGeomToolBSplineSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomBSplineSurface>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineSurface::read_own_params")?;
        let msg100 = iges_msg("XSTEP_100");
        let msg101 = iges_msg("XSTEP_101");
        let msg102 = iges_msg("XSTEP_102");
        let msg103 = iges_msg("XSTEP_103");
        let msg159 = iges_msg("XSTEP_159");
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut an_index_u: i32 = 0;
        let mut an_index_v: i32 = 0;
        let mut a_deg_u: i32 = 0;
        let mut a_deg_v: i32 = 0;
        let mut a_close_u: bool = false;
        let mut a_close_v: bool = false;
        let mut a_polynom: bool = false;
        let mut a_period_u: bool = false;
        let mut a_period_v: bool = false;
        let mut a_umin: f64 = 0.0;
        let mut a_umax: f64 = 0.0;
        let mut a_vmin: f64 = 0.0;
        let mut a_vmax: f64 = 0.0;
        let mut temp_val: f64 = 0.0;
        let mut temp_xyz: Xyz = Xyz::default();
        let mut all_knots_u: Handle<HArray1<f64>> = Handle::default();
        let mut all_knots_v: Handle<HArray1<f64>> = Handle::default();
        let mut all_weights: Handle<HArray2<f64>> = Handle::default();
        let mut all_poles: Handle<HArray2<Xyz>> = Handle::default();
        let flagindex_u: bool = pr.read_integer(&pr.current(), &mut an_index_u)?;
        let flagindex_v: bool = pr.read_integer(&pr.current(), &mut an_index_v)?;
        if !flagindex_u || !flagindex_v {
            let msg97 = iges_msg("XSTEP_97");
            pr.send_fail(&msg97);
        }
        let flagdeg_u: bool = pr.read_integer(&pr.current(), &mut a_deg_u)?;
        let flagdeg_v: bool = pr.read_integer(&pr.current(), &mut a_deg_v)?;
        if !flagdeg_u || !flagdeg_v {
            let msg98 = iges_msg("XSTEP_98");
            pr.send_fail(&msg98);
        }
        pr.read_boolean(&pr.current(), &msg100, &mut a_close_u, true)?;
        pr.read_boolean(&pr.current(), &msg100, &mut a_close_v, true)?;
        pr.read_boolean(&pr.current(), &msg101, &mut a_polynom, true)?;
        pr.read_boolean(&pr.current(), &msg102, &mut a_period_u, true)?;
        pr.read_boolean(&pr.current(), &msg102, &mut a_period_v, true)?;
        if flagdeg_u && flagindex_u {
            let tempind: i32 = an_index_u + a_deg_u + 2;
            pr.read_reals(
                &pr.current_list(tempind, 1),
                &msg103,
                &mut all_knots_u,
                -a_deg_u,
            )?;
        }
        if flagdeg_v && flagindex_v {
            let tempind: i32 = an_index_v + a_deg_v + 2;
            pr.read_reals(
                &pr.current_list(tempind, 1),
                &msg103,
                &mut all_knots_v,
                -a_deg_v,
            )?;
        }
        if flagindex_u && flagindex_v {
            all_weights = iges_new_array2::<f64>(0, an_index_u, 0, an_index_v)?;
            all_poles = iges_new_array2::<Xyz>(0, an_index_u, 0, an_index_v)?;
            let mut bad_weigth: bool = false;
            let msg105 = iges_msg("XSTEP_105");
            j = 0;
            while j <= an_index_v {
                i = 0;
                while i <= an_index_u {
                    if pr.read_real(&pr.current(), &mut temp_val)? {
                        if temp_val < precision::p_confusion() {
                            bad_weigth = true;
                        }
                        iges_array2_set(
                            &all_weights,
                            i,
                            j,
                            temp_val,
                            "IgesGeomToolBSplineSurface::read_own_params",
                        )?;
                    } else {
                        let msg104 = iges_msg("XSTEP_104");
                        pr.send_fail(&msg104);
                    }
                    i += 1;
                }
                j += 1;
            }
            if bad_weigth {
                for j in 0..=an_index_v {
                    i = 0;
                    while i <= an_index_u {
                        iges_array2_set(
                            &all_weights,
                            i,
                            j,
                            1.0,
                            "IgesGeomToolBSplineSurface::read_own_params",
                        )?;
                        i += 1;
                    }
                }
            }
            for j in 0..=an_index_v {
                for i in 0..=an_index_u {
                    if pr.read_xyz(&pr.current_list(1, 3), &msg105, &mut temp_xyz)? {
                        iges_array2_set(
                            &all_poles,
                            i,
                            j,
                            temp_xyz,
                            "IgesGeomToolBSplineSurface::read_own_params",
                        )?;
                    }
                }
            }
        }
        if !pr.read_real(&pr.current(), &mut a_umin)?
            || !pr.read_real(&pr.current(), &mut a_umax)?
        {
            let msg106 = iges_msg("XSTEP_106");
            pr.send_fail(&msg106);
        }
        if !pr.read_real(&pr.current(), &mut a_vmin)?
            || !pr.read_real(&pr.current(), &mut a_vmax)?
        {
            let msg107 = iges_msg("XSTEP_107");
            pr.send_fail(&msg107);
        }
        let mut icur: i32 = pr.current_number();
        let imax: i32 = pr.nb_params();
        let mut bid: f64 = 0.0;
        let mut pbfin: i32 = 0;
        while imax >= icur {
            let pt: InterfaceParamType = pr.param_type(icur)?;
            if pt == InterfaceParamType::ParamReal {
                if !pr.read_real(&pr.current(), &mut bid)? {
                    pr.send_fail(&msg159);
                }
                if pbfin == 0 {
                    pbfin = 1;
                }
            } else if pt == InterfaceParamType::ParamInteger || pt == InterfaceParamType::ParamVoid
            {
                break;
            } else {
                pbfin = -1;
            }
            icur += 1;
        }
        if pbfin > 0 {
            pr.send_warning(&msg159);
        }
        if pbfin < 0 {
            let msg158 = iges_msg("XSTEP_158");
            pr.send_fail(&msg158);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            an_index_u,
            an_index_v,
            a_deg_u,
            a_deg_v,
            a_close_u,
            a_close_v,
            a_polynom,
            a_period_u,
            a_period_v,
            &all_knots_u,
            &all_knots_v,
            &all_weights,
            &all_poles,
            a_umin,
            a_umax,
            a_vmin,
            a_vmax,
        )?;
        Ok(())
    }
}
impl IgesGeomToolBSplineSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomBSplineSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineSurface::write_own_params")?;
        let ind_u: i32 = ent_live.upper_index_u();
        let ind_v: i32 = ent_live.upper_index_v();
        let deg_u: i32 = ent_live.degree_u();
        let deg_v: i32 = ent_live.degree_v();
        let mut i: i32 = 0;
        iw.send(ind_u)?;
        iw.send(ind_v)?;
        iw.send(deg_u)?;
        iw.send(deg_v)?;
        iw.send_boolean(ent_live.is_closed_u())?;
        iw.send_boolean(ent_live.is_closed_v())?;
        iw.send_boolean(ent_live.is_polynomial(false)?)?;
        iw.send_boolean(ent_live.is_periodic_u())?;
        iw.send_boolean(ent_live.is_periodic_v())?;
        i = -deg_u;
        while i <= ind_u + 1 {
            iw.send(ent_live.knot_u(i)?)?;
            i += 1;
        }
        i = -deg_v;
        while i <= ind_v + 1 {
            iw.send(ent_live.knot_v(i)?)?;
            i += 1;
        }
        for j in 0..=ind_v {
            i = 0;
            while i <= ind_u {
                iw.send(ent_live.weight(i, j)?)?;
                i += 1;
            }
        }
        for j in 0..=ind_v {
            for i in 0..=ind_u {
                let temp_xyz: Xyz = Xyz::from(ent_live.pole(i, j)?);
                iw.send(temp_xyz.x())?;
                iw.send(temp_xyz.y())?;
                iw.send(temp_xyz.z())?;
            }
        }
        iw.send(ent_live.u_min())?;
        iw.send(ent_live.u_max())?;
        iw.send(ent_live.v_min())?;
        iw.send(ent_live.v_max())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomBSplineSurface>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomBSplineSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(128, 0, 9);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolBSplineSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomBSplineSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineSurface::own_check")?;
        let mut flag: bool = true;
        let ind_u: i32 = ent_live.upper_index_u();
        let ind_v: i32 = ent_live.upper_index_v();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if ent_live.nb_knots_u()? != ind_u + ent_live.degree_u() + 2 {
            let mut msg160 = iges_msg("XSTEP_160");
            msg160.arg_integer(ind_u + ent_live.degree_u() + 2)?;
            ach.send_fail(&msg160);
        }
        if ent_live.nb_knots_v()? != ind_v + ent_live.degree_v() + 2 {
            let mut msg161 = iges_msg("XSTEP_161");
            msg161.arg_integer(ind_v + ent_live.degree_v() + 2)?;
            ach.send_fail(&msg161);
        }
        j = 0;
        while j < ind_v && flag {
            i = 0;
            while i < ind_u && flag {
                flag &= ent_live.weight(i, j)? > 0.0;
                i += 1;
            }
            j += 1;
        }
        if !flag {
            let msg104 = iges_msg("XSTEP_104");
            ach.send_fail(&msg104);
        }
        flag = true;
        let temp_val: f64 = ent_live.weight(0, 0)?;
        j = 0;
        while j < ind_v && flag {
            i = 0;
            while i < ind_u && flag {
                flag &= ent_live.weight(i, j)? == temp_val;
                i += 1;
            }
            j += 1;
        }
        Ok(())
    }
}
impl IgesGeomToolBSplineSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomBSplineSurface>,
        ent: &Handle<IgesGeomBSplineSurface>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolBSplineSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolBSplineSurface::own_copy")?;
        let mut i: i32 = 0;
        let an_index_u: i32 = another_live.upper_index_u();
        let an_index_v: i32 = another_live.upper_index_v();
        let a_deg_u: i32 = another_live.degree_u();
        let a_deg_v: i32 = another_live.degree_v();
        let a_close_u: bool = another_live.is_closed_u();
        let a_close_v: bool = another_live.is_closed_v();
        let a_polynom: bool = another_live.is_polynomial(false)?;
        let a_period_u: bool = another_live.is_periodic_u();
        let a_period_v: bool = another_live.is_periodic_v();
        let mut a_umin: f64 = 0.0;
        let mut a_umax: f64 = 0.0;
        let mut a_vmin: f64 = 0.0;
        let mut a_vmax: f64 = 0.0;
        let all_knots_u: Handle<HArray1<f64>> = iges_new_array1::<f64>(-a_deg_u, an_index_u + 1)?;
        let all_knots_v: Handle<HArray1<f64>> = iges_new_array1::<f64>(-a_deg_v, an_index_v + 1)?;
        i = -a_deg_u;
        while i <= an_index_u + 1 {
            iges_array1_set(
                &all_knots_u,
                i,
                another_live.knot_u(i)?,
                "IgesGeomToolBSplineSurface::own_copy",
            )?;
            i += 1;
        }
        i = -a_deg_v;
        while i <= an_index_v + 1 {
            iges_array1_set(
                &all_knots_v,
                i,
                another_live.knot_v(i)?,
                "IgesGeomToolBSplineSurface::own_copy",
            )?;
            i += 1;
        }
        let all_weights: Handle<HArray2<f64>> =
            iges_new_array2::<f64>(0, an_index_u, 0, an_index_v)?;
        let all_poles: Handle<HArray2<Xyz>> = iges_new_array2::<Xyz>(0, an_index_u, 0, an_index_v)?;
        for j in 0..=an_index_v {
            i = 0;
            while i <= an_index_u {
                iges_array2_set(
                    &all_weights,
                    i,
                    j,
                    another_live.weight(i, j)?,
                    "IgesGeomToolBSplineSurface::own_copy",
                )?;
                i += 1;
            }
        }
        for j in 0..=an_index_v {
            for i in 0..=an_index_u {
                iges_array2_set(
                    &all_poles,
                    i,
                    j,
                    Xyz::from(another_live.pole(i, j)?),
                    "IgesGeomToolBSplineSurface::own_copy",
                )?;
            }
        }
        a_umin = another_live.u_min();
        a_umax = another_live.u_max();
        a_vmin = another_live.v_min();
        a_vmax = another_live.v_max();
        ent_live.init(
            an_index_u,
            an_index_v,
            a_deg_u,
            a_deg_v,
            a_close_u,
            a_close_v,
            a_polynom,
            a_period_u,
            a_period_v,
            &all_knots_u,
            &all_knots_v,
            &all_weights,
            &all_poles,
            a_umin,
            a_umax,
            a_vmin,
            a_vmax,
        )?;
        ent_live.set_form_number(another_live.form_number())?;
        Ok(())
    }
}
