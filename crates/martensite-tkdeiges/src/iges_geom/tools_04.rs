#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomBoundary, IgesGeomBoundedSurface, IgesGeomCircularArc, IgesGeomToolBoundary};
use crate::iges_basic::IgesBasicHArray1OfHArray1OfIgesEntity;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::iges_shared_026;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_item, iges_array1_set, iges_dump_entities, iges_live, iges_msg,
    iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolBoundary {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomBoundary>,
        ent: &Handle<IgesGeomBoundary>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolBoundary::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::own_copy")?;
        let temp_type: i32 = another_live.boundary_type();
        let temp_preference: i32 = another_live.preference_type();
        let num1: i32 = another_live.nb_model_space_curves()?;
        let temp_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let temp_senses: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, num1)?;
        let temp_model_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num1)?;
        let temp_parameter_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> =
            Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, num1)?);
        for i in 1..=num1 {
            let temp_ent = tc
                .transferred(
                    &another_live
                        .model_space_curve(i)?
                        .cast::<StandardTransient>(),
                )?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &temp_model_curves,
                i,
                temp_ent.clone(),
                "IgesGeomToolBoundary::own_copy",
            )?;
            iges_array1_set(
                &temp_senses,
                i,
                another_live.sense(i)?,
                "IgesGeomToolBoundary::own_copy",
            )?;
            let num2: i32 = another_live.nb_parameter_curves(i)?;
            let par_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                another_live.parameter_curves(i)?;
            let mut temp_par_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                Handle::default();
            if num2 > 0 {
                temp_par_curves = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num2)?;
            }
            for j in 1..=num2 {
                let temp_ent1 = tc
                    .transferred(
                        &iges_array1_item(&par_curves, j, "IgesGeomToolBoundary::own_copy")?
                            .cast::<StandardTransient>(),
                    )?
                    .cast::<IgesDataIgesEntity>();
                iges_array1_set(
                    &temp_par_curves,
                    j,
                    temp_ent1.clone(),
                    "IgesGeomToolBoundary::own_copy",
                )?;
            }
            iges_live(&temp_parameter_curves, "IgesGeomToolBoundary::own_copy")?
                .set_value(i, &temp_par_curves)?;
        }
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
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomBoundary>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolBoundary::own_dump")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        let sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(
            s,
            "IgesGeomBoundary\nBounded Surface Representation Type : {}\nTrimming Curves Representation : {}\nBounded Surface    : ",
            ent_live.boundary_type(),
            ent_live.preference_type()
        )?;
        dumper.dump(&ent_live.surface(), s, sublevel, -1)?;
        write!(
            s,
            "\nModel Space Curves :\nOrientation Flags  :\nParameter Curves Set : "
        )?;
        iges_dump_entities(
            s,
            dumper,
            -level,
            1,
            ent_live.nb_model_space_curves()?,
            |first| ent_live.model_space_curve(first),
        )?;
        writeln!(s)?;
        if level > 4 {
            num = ent_live.nb_model_space_curves()?;
            i = 1;
            while i <= num {
                write!(s, "[{}]: Model Space Curve : ", i)?;
                dumper.dump(&ent_live.model_space_curve(i)?, s, 1, -1)?;
                write!(
                    s,
                    "  Orientation Flags : {}\n  Parameter Curves : ",
                    ent_live.sense(i)?
                )?;
                let curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                    ent_live.parameter_curves(i)?;
                if !curves.is_null() {
                    iges_dump_entities(
                        s,
                        dumper,
                        level,
                        1,
                        iges_array1_length(&curves, "IgesGeomToolBoundary::own_dump")?,
                        |first| iges_array1_item(&curves, first, "IgesGeomToolBoundary::own_dump"),
                    )?;
                } else {
                    write!(s, " List Empty")?;
                }
                writeln!(s)?;
                i += 1;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolBoundedSurface,
    "The own-parameter work on `IgesGeomBoundedSurface`."
);
impl IgesGeomToolBoundedSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomBoundedSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::read_own_params")?;
        let mut num: i32 = 0;
        let mut temp_type: i32 = 0;
        let mut temp_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_bounds: Handle<HArray1<Handle<IgesGeomBoundary>>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_integer(&pr.current(), &mut temp_type)? {
            let msg165 = iges_msg("XTSEP_165");
            pr.send_fail(&msg165);
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_surface, false)? {
            let mut msg166 = iges_msg("XTSEP_166");
            iges_send_read_status(pr, &mut msg166, a_status, false)?;
        }
        if pr.read_integer(&pr.current(), &mut num)? && num > 0 {
            temp_bounds = iges_new_array1::<Handle<IgesGeomBoundary>>(1, num)?;
        } else {
            let msg167 = iges_msg("XTSEP_167");
            pr.send_fail(&msg167);
        }
        if !temp_bounds.is_null() {
            for i in 1..=num {
                let mut temp_ent: Handle<IgesDataIgesEntity> = Handle::default();
                if pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_ent, false)? {
                    iges_array1_set(
                        &temp_bounds,
                        i,
                        temp_ent.cast::<IgesGeomBoundary>(),
                        "IgesGeomToolBoundedSurface::read_own_params",
                    )?;
                } else {
                    let mut msg168 = iges_msg("XTSEP_168");
                    iges_send_read_status(pr, &mut msg168, a_status, false)?;
                }
            }
        }
        if !temp_bounds.is_null() {
            self.dir_checker(ent).check_type_and_form(pr.check(), ent);
            ent_live.init(temp_type, &temp_surface, &temp_bounds)?;
        }
        Ok(())
    }
}
impl IgesGeomToolBoundedSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomBoundedSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::write_own_params")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        iw.send(ent_live.representation_type())?;
        iw.send_entity(&ent_live.surface(), false)?;
        iw.send(ent_live.nb_boundaries())?;
        num = ent_live.nb_boundaries();
        i = 1;
        while i <= num {
            iw.send_entity(&ent_live.boundary(i)?, false)?;
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
        ent: &Handle<IgesGeomBoundedSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::own_shared")?;
        let mut i: i32 = 0;
        let mut num: i32 = 0;
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        num = ent_live.nb_boundaries();
        i = 1;
        while i <= num {
            iter.get_one_item(&ent_live.boundary(i)?.cast::<StandardTransient>());
            i += 1;
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomBoundedSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(143, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolBoundedSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomBoundedSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::own_check")?;
        if ent_live.representation_type() != 0 && ent_live.representation_type() != 1 {
            let msg165 = iges_msg("XTSEP_165");
            ach.send_fail(&msg165);
        }
        Ok(())
    }
}
impl IgesGeomToolBoundedSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomBoundedSurface>,
        ent: &Handle<IgesGeomBoundedSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolBoundedSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::own_copy")?;
        let mut num: i32 = 0;
        let temp_type: i32 = another_live.representation_type();
        let temp_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        num = another_live.nb_boundaries();
        let mut temp_bounds: Handle<HArray1<Handle<IgesGeomBoundary>>> = Handle::default();
        if num > 0 {
            temp_bounds = iges_new_array1::<Handle<IgesGeomBoundary>>(1, num)?;
        }
        for i in 1..=num {
            let temp_boundary = tc
                .transferred(&another_live.boundary(i)?.cast::<StandardTransient>())?
                .cast::<IgesGeomBoundary>();
            iges_array1_set(
                &temp_bounds,
                i,
                temp_boundary.clone(),
                "IgesGeomToolBoundedSurface::own_copy",
            )?;
        }
        ent_live.init(temp_type, &temp_surface, &temp_bounds)?;
        Ok(())
    }
}
impl IgesGeomToolBoundedSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomBoundedSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolBoundedSurface::own_dump")?;
        let sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(
            s,
            "IgesGeomBoundedSurface\nRepresentation Type   : {}\nSurface to be Bounded : ",
            ent_live.representation_type()
        )?;
        dumper.dump(&ent_live.surface(), s, sublevel, -1)?;
        write!(s, "\nBoundary Entities     : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_boundaries(), |first| {
            ent_live.boundary(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolCircularArc,
    "The own-parameter work on `IgesGeomCircularArc`."
);
impl IgesGeomToolCircularArc {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomCircularArc>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCircularArc::read_own_params")?;
        let msg76 = iges_msg("XSTEP_76");
        let msg77 = iges_msg("XSTEP_77");
        let msg78 = iges_msg("XSTEP_78");
        let mut a_zt: f64 = 0.0;
        let mut a_center: Xy = Xy::default();
        let mut a_start: Xy = Xy::default();
        let mut an_end: Xy = Xy::default();
        if !pr.read_real(&pr.current(), &mut a_zt)? {
            let msg75 = iges_msg("XSTEP_75");
            pr.send_fail(&msg75);
        }
        pr.read_xy(&pr.current_list(1, 2), &msg76, &mut a_center)?;
        pr.read_xy(&pr.current_list(1, 2), &msg77, &mut a_start)?;
        pr.read_xy(&pr.current_list(1, 2), &msg78, &mut an_end)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_zt, a_center, a_start, an_end);
        Ok(())
    }
}
impl IgesGeomToolCircularArc {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomCircularArc>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCircularArc::write_own_params")?;
        iw.send(ent_live.z_plane())?;
        iw.send(ent_live.center().x())?;
        iw.send(ent_live.center().y())?;
        iges_shared_026!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomCircularArc>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomCircularArc>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(100, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolCircularArc {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomCircularArc>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
