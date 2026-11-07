#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomSplineSurface, IgesGeomToolSplineSurface};
use crate::iges_basic::entity_shape::{iges_array1_fill, iges_array1_length};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesReaderData, IgesDataIgesWriter,
    IgesDataParamReader,
};
use crate::iges_data::{
    iges_array1_item, iges_array1_set, iges_array2_set, iges_live, iges_msg, iges_new_array1,
    iges_new_array2,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};

impl IgesGeomToolSplineSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomSplineSurface>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineSurface::read_own_params")?;
        let mut a_boundary_type: i32 = 0;
        let mut a_patch_type: i32 = 0;
        let mut all_nb_u_segments: i32 = 0;
        let mut all_nb_v_segments: i32 = 0;
        let mut k: i32 = 0;
        let mut ubreak: bool = false;
        let mut vbreak: bool = false;
        let mut all_u_break_points: Handle<HArray1<f64>> = Handle::default();
        let mut all_v_break_points: Handle<HArray1<f64>> = Handle::default();
        let mut all_x_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = Handle::default();
        let mut all_y_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = Handle::default();
        let mut all_z_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> = Handle::default();
        if !pr.read_integer(&pr.current(), &mut a_boundary_type)? {
            let msg140 = iges_msg("XSTEP_140");
            pr.send_fail(&msg140);
        }
        if !pr.read_integer(&pr.current(), &mut a_patch_type)? {
            let msg278 = iges_msg("XSTEP_278");
            pr.send_fail(&msg278);
        }
        if pr.read_integer(&pr.current(), &mut all_nb_u_segments)? {
            ubreak = true;
            all_u_break_points = iges_new_array1::<f64>(1, all_nb_u_segments + 1)?;
        } else {
            let msg141 = iges_msg("XSTEP_141");
            pr.send_fail(&msg141);
        }
        if pr.read_integer(&pr.current(), &mut all_nb_v_segments)? {
            vbreak = true;
            all_v_break_points = iges_new_array1::<f64>(1, all_nb_v_segments + 1)?;
        } else {
            let msg142 = iges_msg("XSTEP_142");
            pr.send_fail(&msg142);
        }
        if !all_u_break_points.is_null() {
            let msg143 = iges_msg("XSTEP_143");
            pr.read_reals(
                &pr.current_list(all_nb_u_segments + 1, 1),
                &msg143,
                &mut all_u_break_points,
                1,
            )?;
        }
        if !all_v_break_points.is_null() {
            let msg144 = iges_msg("XSTEP_144");
            pr.read_reals(
                &pr.current_list(all_nb_v_segments + 1, 1),
                &msg144,
                &mut all_v_break_points,
                1,
            )?;
        }
        if ubreak && vbreak {
            all_x_coeffs = iges_new_array2::<Handle<HArray1<f64>>>(
                1,
                all_nb_u_segments,
                1,
                all_nb_v_segments,
            )?;
            all_y_coeffs = iges_new_array2::<Handle<HArray1<f64>>>(
                1,
                all_nb_u_segments,
                1,
                all_nb_v_segments,
            )?;
            all_z_coeffs = iges_new_array2::<Handle<HArray1<f64>>>(
                1,
                all_nb_u_segments,
                1,
                all_nb_v_segments,
            )?;
        }
        let mut temp: Handle<HArray1<f64>> = Handle::default();
        if !all_x_coeffs.is_null() {
            let mut st: bool = false;
            let mut msg145_x = iges_msg("XSTEP_145");
            msg145_x.arg_text("X".as_bytes())?;
            let mut msg145_y = iges_msg("XSTEP_145");
            msg145_y.arg_text("Y".as_bytes())?;
            let mut msg145_z = iges_msg("XSTEP_145");
            msg145_z.arg_text("Z".as_bytes())?;
            for i in 1..=all_nb_u_segments {
                for j in 1..=all_nb_v_segments {
                    st = pr.read_reals(&pr.current_list(16, 1), &msg145_x, &mut temp, 1)?;
                    if st
                        && iges_array1_length(&temp, "IgesGeomToolSplineSurface::read_own_params")?
                            == 16
                    {
                        iges_array2_set(
                            &all_x_coeffs,
                            i,
                            j,
                            temp.clone(),
                            "IgesGeomToolSplineSurface::read_own_params",
                        )?;
                    } else {
                        let mut msg147_x = iges_msg("XSTEP_147");
                        msg147_x.arg_text("X".as_bytes())?;
                        pr.send_fail(&msg147_x);
                    }
                    st = pr.read_reals(&pr.current_list(16, 1), &msg145_y, &mut temp, 1)?;
                    if st
                        && iges_array1_length(&temp, "IgesGeomToolSplineSurface::read_own_params")?
                            == 16
                    {
                        iges_array2_set(
                            &all_y_coeffs,
                            i,
                            j,
                            temp.clone(),
                            "IgesGeomToolSplineSurface::read_own_params",
                        )?;
                    } else {
                        let mut msg147_y = iges_msg("XSTEP_147");
                        msg147_y.arg_text("Y".as_bytes())?;
                        pr.send_fail(&msg147_y);
                    }
                    st = pr.read_reals(&pr.current_list(16, 1), &msg145_z, &mut temp, 1)?;
                    if st
                        && iges_array1_length(&temp, "IgesGeomToolSplineSurface::read_own_params")?
                            == 16
                    {
                        iges_array2_set(
                            &all_z_coeffs,
                            i,
                            j,
                            temp.clone(),
                            "IgesGeomToolSplineSurface::read_own_params",
                        )?;
                    } else if i < all_nb_u_segments || j < all_nb_v_segments {
                        let mut msg147_z = iges_msg("XSTEP_147");
                        msg147_z.arg_text("Z".as_bytes())?;
                        pr.send_fail(&msg147_z);
                    } else {
                        temp = iges_new_array1::<f64>(1, 16)?;
                        iges_array1_fill(&temp, 0.0, "IgesGeomToolSplineSurface::read_own_params")?;
                        k = 1;
                        while k <= 16 {
                            let mut vl: f64 = 0.0;
                            if !pr.read_real(&pr.current(), &mut vl)? {
                                let msg146 = iges_msg("XSTEP_146");
                                pr.send_fail(&msg146);
                                break;
                            }
                            iges_array1_set(
                                &temp,
                                k,
                                vl,
                                "IgesGeomToolSplineSurface::read_own_params",
                            )?;
                            k += 1;
                        }
                        iges_array2_set(
                            &all_z_coeffs,
                            i,
                            j,
                            temp.clone(),
                            "IgesGeomToolSplineSurface::read_own_params",
                        )?;
                        pr.mend("Last patch incomplete, defaulted")?;
                    }
                }
                for _kk in 1..=48 {
                    pr.set_current_number(pr.current_number() + 1);
                }
            }
        }
        if vbreak {
            for _k in 1..=48 * (all_nb_v_segments + 1) {
                pr.set_current_number(pr.current_number() + 1);
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            a_boundary_type,
            a_patch_type,
            &all_u_break_points,
            &all_v_break_points,
            &all_x_coeffs,
            &all_y_coeffs,
            &all_z_coeffs,
        )?;
        Ok(())
    }
}
impl IgesGeomToolSplineSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomSplineSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineSurface::write_own_params")?;
        let mut k: i32 = 0;
        iw.send(ent_live.boundary_type())?;
        iw.send(ent_live.patch_type())?;
        let nb_u_segs: i32 = ent_live.nb_u_segments()?;
        let nb_v_segs: i32 = ent_live.nb_v_segments()?;
        iw.send(nb_u_segs)?;
        iw.send(nb_v_segs)?;
        for i in 1..=nb_u_segs + 1 {
            iw.send(ent_live.u_break_point(i)?)?;
        }
        for i in 1..=nb_v_segs + 1 {
            iw.send(ent_live.v_break_point(i)?)?;
        }
        for i in 1..=nb_u_segs {
            for j in 1..=nb_v_segs {
                k = 1;
                while k <= 16 {
                    iw.send(iges_array1_item(
                        &ent_live.x_polynomial(i, j)?,
                        k,
                        "IgesGeomToolSplineSurface::write_own_params",
                    )?)?;
                    k += 1;
                }
                k = 1;
                while k <= 16 {
                    iw.send(iges_array1_item(
                        &ent_live.y_polynomial(i, j)?,
                        k,
                        "IgesGeomToolSplineSurface::write_own_params",
                    )?)?;
                    k += 1;
                }
                k = 1;
                while k <= 16 {
                    iw.send(iges_array1_item(
                        &ent_live.z_polynomial(i, j)?,
                        k,
                        "IgesGeomToolSplineSurface::write_own_params",
                    )?)?;
                    k += 1;
                }
            }
            k = 1;
            while k <= 48 {
                iw.send(0.0)?;
                k += 1;
            }
        }
        for _j in 1..=(nb_v_segs + 1) * 48 {
            iw.send(0.0)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomSplineSurface>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomSplineSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(114, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolSplineSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomSplineSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineSurface::own_check")?;
        if ent_live.boundary_type() < 1 || ent_live.boundary_type() > 6 {
            let msg140 = iges_msg("XSTEP_140");
            ach.send_fail(&msg140);
        }
        Ok(())
    }
}
impl IgesGeomToolSplineSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomSplineSurface>,
        ent: &Handle<IgesGeomSplineSurface>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolSplineSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolSplineSurface::own_copy")?;
        let a_boundary_type: i32 = another_live.boundary_type();
        let a_patch_type: i32 = another_live.patch_type();
        let all_nb_u_segments: i32 = another_live.nb_u_segments()?;
        let all_nb_v_segments: i32 = another_live.nb_v_segments()?;
        let all_u_break_points: Handle<HArray1<f64>> =
            iges_new_array1::<f64>(1, all_nb_u_segments + 1)?;
        let all_v_break_points: Handle<HArray1<f64>> =
            iges_new_array1::<f64>(1, all_nb_v_segments + 1)?;
        for i in 1..=all_nb_u_segments + 1 {
            iges_array1_set(
                &all_u_break_points,
                i,
                another_live.u_break_point(i)?,
                "IgesGeomToolSplineSurface::own_copy",
            )?;
        }
        for i in 1..=all_nb_v_segments + 1 {
            iges_array1_set(
                &all_v_break_points,
                i,
                another_live.v_break_point(i)?,
                "IgesGeomToolSplineSurface::own_copy",
            )?;
        }
        let all_x_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> =
            iges_new_array2::<Handle<HArray1<f64>>>(1, all_nb_u_segments, 1, all_nb_v_segments)?;
        let all_y_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> =
            iges_new_array2::<Handle<HArray1<f64>>>(1, all_nb_u_segments, 1, all_nb_v_segments)?;
        let all_z_coeffs: Handle<HArray2<Handle<HArray1<f64>>>> =
            iges_new_array2::<Handle<HArray1<f64>>>(1, all_nb_u_segments, 1, all_nb_v_segments)?;
        let mut temp: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 16)?;
        for i in 1..=all_nb_u_segments {
            for j in 1..=all_nb_v_segments {
                temp = another_live.x_polynomial(i, j)?;
                iges_array2_set(
                    &all_x_coeffs,
                    i,
                    j,
                    temp.clone(),
                    "IgesGeomToolSplineSurface::own_copy",
                )?;
                temp = another_live.y_polynomial(i, j)?;
                iges_array2_set(
                    &all_y_coeffs,
                    i,
                    j,
                    temp.clone(),
                    "IgesGeomToolSplineSurface::own_copy",
                )?;
                temp = another_live.z_polynomial(i, j)?;
                iges_array2_set(
                    &all_z_coeffs,
                    i,
                    j,
                    temp.clone(),
                    "IgesGeomToolSplineSurface::own_copy",
                )?;
            }
        }
        ent_live.init(
            a_boundary_type,
            a_patch_type,
            &all_u_break_points,
            &all_v_break_points,
            &all_x_coeffs,
            &all_y_coeffs,
            &all_z_coeffs,
        )?;
        Ok(())
    }
}
