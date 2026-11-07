#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomBSplineSurface, IgesGeomBoundary, IgesGeomToolBSplineSurface};
use crate::iges_basic::IgesBasicHArray1OfHArray1OfIgesEntity;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_send_read_status, iges_tool};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_item, iges_array1_set, iges_dump_rect_vals, iges_dump_vals,
    iges_dump_xyz_l, iges_live, iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{GeneralTransform3, Xyz};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceEntityIterator, InterfaceShareTool};
use std::io::Write;

impl IgesGeomToolBSplineSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomBSplineSurface>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolBSplineSurface::own_dump")?;
        write!(s, "IgesGeomBSplineSurface\n\n")?;
        let ind_u: i32 = ent_live.upper_index_u();
        let ind_v: i32 = ent_live.upper_index_v();
        let deg_u: i32 = ent_live.degree_u();
        let deg_v: i32 = ent_live.degree_v();
        write!(
            s,
            "In U : Upper Knot Index : {}   Degree : {}  {}  {}\nIn V : Upper Knot Index : {}   Degree : {}  {}  {}  {}\n",
            ind_u,
            deg_u,
            if ent_live.is_closed_u() {
                "Closed"
            } else {
                "Open"
            },
            if ent_live.is_periodic_u() {
                "Periodic"
            } else {
                "Non Periodic"
            },
            ind_v,
            deg_v,
            if ent_live.is_closed_v() {
                "Closed"
            } else {
                "Open"
            },
            if ent_live.is_periodic_v() {
                "Periodic"
            } else {
                "Non Periodic"
            },
            if ent_live.is_polynomial(true)? {
                "Polynomial"
            } else {
                "Rational"
            }
        )?;
        write!(
            s,
            "Starting Parameter for U direction : {}\nEnding   Parameter for U direction : {}\nStarting Parameter for V direction : {}\nEnding   Parameter for V direction : {}\n",
            iges_real_text(ent_live.u_min()),
            iges_real_text(ent_live.u_max()),
            iges_real_text(ent_live.v_min()),
            iges_real_text(ent_live.v_max())
        )?;
        write!(s, " Knots in  U : ")?;
        iges_dump_vals(s, level, -deg_u, ind_u + 1, |first| ent_live.knot_u(first))?;
        write!(s, "\n Knots in  V : ")?;
        iges_dump_vals(s, level, -deg_v, ind_v + 1, |first| ent_live.knot_v(first))?;
        write!(s, "\nWeights : (Row -> IndexU, Col -> IndexV)")?;
        iges_dump_rect_vals(s, level, (0, ind_u), (0, ind_v), |first, second| {
            ent_live.weight(first, second)
        })?;
        write!(s, "\nControl Points (Poles) : ")?;
        if level < 5 {
            writeln!(s, " [ content : ask level > 4 ]")?;
            return Ok(());
        }
        let loca: GeneralTransform3 = ent_live.location()?;
        for jp in 0..=ind_v {
            for ip in 0..=ind_u {
                if ip == 0 {
                    writeln!(s)?;
                }
                write!(s, "  -  [{},{}]: ", ip, jp)?;
                iges_dump_xyz_l(s, level, &Xyz::from(ent_live.pole(ip, jp)?), &loca)?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolBoundary,
    "The own-parameter work on `IgesGeomBoundary`."
);
impl IgesGeomToolBoundary {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomBoundary>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::read_own_params")?;
        let mut num: i32 = 0;
        let mut temp_type: i32 = 0;
        let mut temp_preference: i32 = 0;
        let mut temp_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_senses: Handle<HArray1<i32>> = Handle::default();
        let mut temp_model_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut temp_parameter_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> =
            Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_integer(&pr.current(), &mut temp_type)? {
            let msg122 = iges_msg("XTSEP_122");
            pr.send_fail(&msg122);
        }
        if !pr.read_integer(&pr.current(), &mut temp_preference)? {
            let msg123 = iges_msg("XTSEP_123");
            pr.send_fail(&msg123);
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_surface, false)? {
            let mut msg124 = iges_msg("XTSEP_124");
            iges_send_read_status(pr, &mut msg124, a_status, false)?;
        }
        if pr.read_integer(&pr.current(), &mut num)? && num > 0 {
            temp_senses = iges_new_array1::<i32>(1, num)?;
            temp_model_curves = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
            temp_parameter_curves =
                Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, num)?);
        } else {
            let msg126 = iges_msg("XTSEP_126");
            pr.send_fail(&msg126);
        }
        if !temp_senses.is_null()
            && !temp_model_curves.is_null()
            && !temp_parameter_curves.is_null()
        {
            for i in 1..=num {
                let mut temp_ent: Handle<IgesDataIgesEntity> = Handle::default();
                if pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_ent, false)? {
                    iges_array1_set(
                        &temp_model_curves,
                        i,
                        temp_ent.clone(),
                        "IgesGeomToolBoundary::read_own_params",
                    )?;
                } else {
                    let mut msg127 = iges_msg("XTSEP_127");
                    iges_send_read_status(pr, &mut msg127, a_status, false)?;
                }
                let mut temp_sense: i32 = 0;
                if pr.read_integer(&pr.current(), &mut temp_sense)? {
                    iges_array1_set(
                        &temp_senses,
                        i,
                        temp_sense,
                        "IgesGeomToolBoundary::read_own_params",
                    )?;
                } else {
                    let msg128 = iges_msg("XTSEP_128");
                    pr.send_fail(&msg128);
                }
                let mut temp_count: i32 = 0;
                if pr.read_integer(&pr.current(), &mut temp_count)? && temp_count >= 0 {
                    let mut temp_par_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                        Handle::default();
                    if temp_count > 0 {
                        let msg130 = iges_msg("XTSEP_130");
                        pr.read_ents(
                            ir,
                            &pr.current_list(temp_count, 1),
                            &msg130,
                            &mut temp_par_curves,
                            1,
                        )?;
                    }
                    iges_live(
                        &temp_parameter_curves,
                        "IgesGeomToolBoundary::read_own_params",
                    )?
                    .set_value(i, &temp_par_curves)?;
                } else {
                    let msg129 = iges_msg("XTSEP_129");
                    pr.send_fail(&msg129);
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            temp_type,
            temp_preference,
            &temp_surface,
            &temp_model_curves,
            &temp_senses,
            &temp_parameter_curves,
        )?;
        Ok(())
    }
}
impl IgesGeomToolBoundary {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomBoundary>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::write_own_params")?;
        let mut i: i32 = 0;
        let mut num1: i32 = 0;
        iw.send(ent_live.boundary_type())?;
        iw.send(ent_live.preference_type())?;
        iw.send_entity(&ent_live.surface(), false)?;
        iw.send(ent_live.nb_model_space_curves()?)?;
        num1 = ent_live.nb_model_space_curves()?;
        i = 1;
        while i <= num1 {
            iw.send_entity(&ent_live.model_space_curve(i)?, false)?;
            iw.send(ent_live.sense(i)?)?;
            let curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                ent_live.parameter_curves(i)?;
            let nbc: i32 = ent_live.nb_parameter_curves(i)?;
            iw.send(nbc)?;
            if nbc > 0 {
                for j in 1..=nbc {
                    iw.send_entity(
                        &iges_array1_item(&curves, j, "IgesGeomToolBoundary::write_own_params")?,
                        false,
                    )?;
                }
            }
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
        ent: &Handle<IgesGeomBoundary>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::own_shared")?;
        let mut i: i32 = 0;
        let mut num1: i32 = 0;
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        num1 = ent_live.nb_model_space_curves()?;
        i = 1;
        while i <= num1 {
            iter.get_one_item(&ent_live.model_space_curve(i)?.cast::<StandardTransient>());
            let curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                ent_live.parameter_curves(i)?;
            if !curves.is_null() {
                let nbc: i32 = iges_array1_length(&curves, "IgesGeomToolBoundary::own_shared")?;
                for j in 1..=nbc {
                    iter.get_one_item(
                        &iges_array1_item(&curves, j, "IgesGeomToolBoundary::own_shared")?
                            .cast::<StandardTransient>(),
                    );
                }
            }
            i += 1;
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomBoundary>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(141, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.subordinate_status_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolBoundary {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomBoundary>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::own_correct")?;
        let mut res: bool = false;
        let mut r2d: bool = false;
        let nb: i32 = ent_live.nb_model_space_curves()?;
        for i in 1..=nb {
            let nbi: i32 = ent_live.nb_parameter_curves(i)?;
            if nbi == 0 {
                continue;
            }
            for j in 1..=nbi {
                let c2d: Handle<IgesDataIgesEntity> = ent_live.parameter_curve(i, j)?;
                if c2d.is_null() {
                    continue;
                }
                iges_live(&c2d, "IgesGeomToolBoundary::own_correct")?.init_status(
                    iges_live(&c2d, "IgesGeomToolBoundary::own_correct")?.blank_status(),
                    iges_live(&c2d, "IgesGeomToolBoundary::own_correct")?.subordinate_status(),
                    5,
                    iges_live(&c2d, "IgesGeomToolBoundary::own_correct")?.hierarchy_status(),
                );
                res = true;
            }
            r2d = true;
        }
        if !r2d {
            return Ok(res);
        }
        if ent_live.boundary_type() != 0 {
            return Ok(res);
        }
        let cv2d: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> =
            Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, nb)?);
        let modcv: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        let senses: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(
                &senses,
                i,
                ent_live.sense(i)?,
                "IgesGeomToolBoundary::own_correct",
            )?;
            iges_array1_set(
                &modcv,
                i,
                ent_live.model_space_curve(i)?,
                "IgesGeomToolBoundary::own_correct",
            )?;
            iges_live(&cv2d, "IgesGeomToolBoundary::own_correct")?
                .set_value(i, &ent_live.parameter_curves(i)?)?;
        }
        ent_live.init(
            1,
            ent_live.preference_type(),
            &ent_live.surface(),
            &modcv,
            &senses,
            &cv2d,
        )?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomBoundary>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::own_check")?;
        if ent_live.boundary_type() != 0 && ent_live.boundary_type() != 1 {
            let msg122 = iges_msg("XTSEP_122");
            ach.send_fail(&msg122);
        }
        if ent_live.preference_type() < 0 || ent_live.preference_type() > 3 {
            let msg123 = iges_msg("XTSEP_123");
            ach.send_fail(&msg123);
        }
        if ent_live.boundary_type() == 1 {
            if iges_live(&ent_live.surface(), "IgesGeomToolBoundary::own_check")?.type_number()
                == 108
            {
                let msg125 = iges_msg("XTSEP_125");
                ach.send_fail(&msg125);
            }
        }
        let mut i: i32 = 0;
        let num: i32 = ent_live.nb_model_space_curves()?;
        i = 1;
        while i <= num {
            if ent_live.sense(i)? != 1 && ent_live.sense(i)? != 2 {
                let msg128 = iges_msg("XTSEP_128");
                ach.send_fail(&msg128);
            }
            i += 1;
        }
        Ok(())
    }
}
