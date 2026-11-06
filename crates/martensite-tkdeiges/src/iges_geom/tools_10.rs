#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomPoint, IgesGeomRuledSurface, IgesGeomSplineCurve, IgesGeomToolPoint};
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{iges_shared_021, iges_shared_047};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_item, iges_array1_set, iges_array2_set, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array1, iges_new_array2,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolPoint {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomPoint>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPoint::write_own_params")?;
        iges_shared_047!(ent_live, iw);
        iw.send_entity(&ent_live.display_symbol(), false)?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomPoint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPoint::own_shared")?;
        iter.get_one_item(&ent_live.display_symbol().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, ent: &Handle<IgesGeomPoint>) -> IgesDataDirChecker {
        let Some(ent_live) = ent.get() else {
            return IgesDataDirChecker::new();
        };
        let mut dc = IgesDataDirChecker::with_form(116, 0);
        dc.structure(IgesDataDefType::DefVoid);
        if ent_live.has_display_symbol() {
            dc.line_font(IgesDataDefType::DefAny);
            dc.line_weight(IgesDataDefType::DefValue);
        }
        dc.color(IgesDataDefType::DefAny);
        dc
    }
}
impl IgesGeomToolPoint {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomPoint>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolPoint {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomPoint>,
        ent: &Handle<IgesGeomPoint>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolPoint::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolPoint::own_copy")?;
        let a_point: Xyz = Xyz::from(another_live.value());
        let a_symbol = tc
            .transferred(&another_live.display_symbol().cast::<StandardTransient>())?
            .cast::<IgesBasicSubfigureDef>();
        ent_live.init(a_point, &a_symbol);
        Ok(())
    }
}
impl IgesGeomToolPoint {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomPoint>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolPoint::own_dump")?;
        write!(s, "IgesGeomPoint\n Value         : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.value()),
            &ent_live.location()?,
        )?;
        write!(s, "\n Display Symbol : ")?;
        dumper.dump(
            &ent_live.display_symbol(),
            s,
            if level <= 4 { 0 } else { 1 },
            -1,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolRuledSurface,
    "The own-parameter work on `IgesGeomRuledSurface`."
);
impl IgesGeomToolRuledSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomRuledSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolRuledSurface::read_own_params")?;
        let mut a_dir_flag: i32 = 0;
        let mut a_dev_flag: i32 = 0;
        let mut a_curve: Handle<IgesDataIgesEntity> = Handle::default();
        let mut another_curve: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_curve, false)? {
            let mut msg148 = iges_msg("XSTEP_148");
            iges_send_read_status(pr, &mut msg148, a_status, false)?;
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut another_curve, false)? {
            let mut msg149 = iges_msg("XSTEP_149");
            iges_send_read_status(pr, &mut msg149, a_status, false)?;
        }
        if !pr.read_integer(&pr.current(), &mut a_dir_flag)? {
            let msg150 = iges_msg("XSTEP_150");
            pr.send_fail(&msg150);
        }
        if !pr.read_integer(&pr.current(), &mut a_dev_flag)? {
            let msg151 = iges_msg("XSTEP_151");
            pr.send_fail(&msg151);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&a_curve, &another_curve, a_dir_flag, a_dev_flag);
        Ok(())
    }
}
impl IgesGeomToolRuledSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomRuledSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolRuledSurface::write_own_params")?;
        iw.send_entity(&ent_live.first_curve(), false)?;
        iw.send_entity(&ent_live.second_curve(), false)?;
        iw.send(ent_live.direction_flag())?;
        iw.send_boolean(ent_live.is_developable())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomRuledSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolRuledSurface::own_shared")?;
        iter.get_one_item(&ent_live.first_curve().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.second_curve().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomRuledSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(118, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolRuledSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomRuledSurface>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolRuledSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomRuledSurface>,
        ent: &Handle<IgesGeomRuledSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolRuledSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolRuledSurface::own_copy")?;
        let a_curve = tc
            .transferred(&another_live.first_curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let another_curve = tc
            .transferred(&another_live.second_curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_dir_flag: i32 = another_live.direction_flag();
        let a_dev_flag: i32 = if another_live.is_developable() { 1 } else { 0 };
        ent_live.init(&a_curve, &another_curve, a_dir_flag, a_dev_flag);
        Ok(())
    }
}
impl IgesGeomToolRuledSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomRuledSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolRuledSurface::own_dump")?;
        let temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesGeomRuledSurface\nFirst  Curve   : ")?;
        dumper.dump(&ent_live.first_curve(), s, temp_sub_level, -1)?;
        write!(s, "\nSecond Curve   : ")?;
        dumper.dump(&ent_live.second_curve(), s, temp_sub_level, -1)?;
        write!(s, "\nDirection Flag : {}  i.e.", ent_live.direction_flag())?;
        if ent_live.direction_flag() == 0 {
            writeln!(s, "Join First to First, Last to Last")?;
        } else {
            writeln!(s, "Join First to Last, Last to First")?;
        }
        if ent_live.is_developable() {
            writeln!(s, " .. Is Developable")?;
        } else {
            writeln!(s, " .. Is possibly not developable ..")?;
        }
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolSplineCurve,
    "The own-parameter work on `IgesGeomSplineCurve`."
);
impl IgesGeomToolSplineCurve {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomSplineCurve>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineCurve::read_own_params")?;
        let mut nb_segments: i32 = 0;
        let mut a_type: i32 = 0;
        let mut a_degree: i32 = 0;
        let mut nb_dimensions: i32 = 0;
        let mut all_break_points: Handle<HArray1<f64>> = Handle::default();
        let mut all_x_polynomials: Handle<HArray2<f64>> = Handle::default();
        let mut all_y_polynomials: Handle<HArray2<f64>> = Handle::default();
        let mut all_z_polynomials: Handle<HArray2<f64>> = Handle::default();
        iges_shared_021!(all_xvalues, all_yvalues, all_zvalues);
        if !pr.read_integer(&pr.current(), &mut a_type)? {
            let msg91 = iges_msg("XSTEP_91");
            pr.send_fail(&msg91);
        }
        if !pr.read_integer(&pr.current(), &mut a_degree)? {
            let msg92 = iges_msg("XSTEP_92");
            pr.send_fail(&msg92);
        }
        if !pr.read_integer(&pr.current(), &mut nb_dimensions)? {
            let msg93 = iges_msg("XSTEP_93");
            pr.send_fail(&msg93);
        }
        if pr.read_integer(&pr.current(), &mut nb_segments)? {
            if nb_segments <= 0 {
                let msg94 = iges_msg("XSTEP_94");
                pr.send_fail(&msg94);
            } else {
                all_x_polynomials = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
                all_y_polynomials = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
                all_z_polynomials = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
                all_break_points = iges_new_array1::<f64>(1, nb_segments + 1)?;
            }
            let msg95 = iges_msg("XSTEP_95");
            pr.read_reals(
                &pr.current_list(nb_segments + 1, 1),
                &msg95,
                &mut all_break_points,
                1,
            )?;
        } else {
            let msg94 = iges_msg("XSTEP_94");
            pr.send_fail(&msg94);
        }
        let mut temp: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, 4)?;
        if !all_x_polynomials.is_null() {
            for i in 1..=nb_segments {
                if pr.read_reals(
                    &pr.current_list(4, 1),
                    "X-Coordinate Polynomial",
                    &mut temp,
                    1,
                )? {
                    let mut j: i32 = 1;
                    while j <= 4 {
                        iges_array2_set(
                            &all_x_polynomials,
                            i,
                            j,
                            iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                            "IgesGeomToolSplineCurve::read_own_params",
                        )?;
                        j += 1;
                    }
                }
                if pr.read_reals(
                    &pr.current_list(4, 1),
                    "Y-Coordinate Polynomial",
                    &mut temp,
                    1,
                )? {
                    let mut j: i32 = 1;
                    while j <= 4 {
                        iges_array2_set(
                            &all_y_polynomials,
                            i,
                            j,
                            iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                            "IgesGeomToolSplineCurve::read_own_params",
                        )?;
                        j += 1;
                    }
                }
                if pr.read_reals(
                    &pr.current_list(4, 1),
                    "Z-Coordinate Polynomial",
                    &mut temp,
                    1,
                )? {
                    let mut j: i32 = 1;
                    while j <= 4 {
                        iges_array2_set(
                            &all_z_polynomials,
                            i,
                            j,
                            iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                            "IgesGeomToolSplineCurve::read_own_params",
                        )?;
                        j += 1;
                    }
                }
            }
        }
        if pr.read_reals(
            &pr.current_list(4, 1),
            "TerminatePoint X-Values",
            &mut temp,
            1,
        )? {
            let mut j: i32 = 1;
            while j <= 4 {
                iges_array1_set(
                    &all_xvalues,
                    j,
                    iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                    "IgesGeomToolSplineCurve::read_own_params",
                )?;
                j += 1;
            }
        }
        if pr.read_reals(
            &pr.current_list(4, 1),
            "TerminatePoint Y-Values",
            &mut temp,
            1,
        )? {
            let mut j: i32 = 1;
            while j <= 4 {
                iges_array1_set(
                    &all_yvalues,
                    j,
                    iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                    "IgesGeomToolSplineCurve::read_own_params",
                )?;
                j += 1;
            }
        }
        if pr.read_reals(
            &pr.current_list(4, 1),
            "TerminatePoint Z-Values",
            &mut temp,
            1,
        )? {
            for j in 1..=4 {
                iges_array1_set(
                    &all_zvalues,
                    j,
                    iges_array1_item(&temp, j, "IgesGeomToolSplineCurve::read_own_params")?,
                    "IgesGeomToolSplineCurve::read_own_params",
                )?;
            }
        }
        if all_break_points.is_null()
            || all_x_polynomials.is_null()
            || all_y_polynomials.is_null()
            || all_z_polynomials.is_null()
        {
            return Ok(());
        } else {
            self.dir_checker(ent).check_type_and_form(pr.check(), ent);
            ent_live.init(
                a_type,
                a_degree,
                nb_dimensions,
                &all_break_points,
                &all_x_polynomials,
                &all_y_polynomials,
                &all_z_polynomials,
                &all_xvalues,
                &all_yvalues,
                &all_zvalues,
            )?;
        }
        Ok(())
    }
}
