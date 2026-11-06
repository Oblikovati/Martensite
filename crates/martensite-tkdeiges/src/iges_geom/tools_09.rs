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
    IgesGeomOffsetCurve, IgesGeomOffsetSurface, IgesGeomPlane, IgesGeomPoint,
    IgesGeomToolOffsetCurve,
};
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::iges_shared_038;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{IgesDataDumpResult, iges_dump_xyz_l, iges_live, iges_msg, iges_real_text};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolOffsetCurve {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomOffsetCurve>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetCurve::own_dump")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesGeomOffsetCurve\nThe curve to be offset     :\n")?;
        dumper.dump(&ent_live.base_curve(), s, sublevel, -1)?;
        write!(
            s,
            "Offset Distance Flag       : {}\nCurve entity whose coordinate defines the offset : ",
            ent_live.offset_type()
        )?;
        dumper.dump(&ent_live.function(), s, sublevel, -1)?;
        write!(
            s,
            "\nIn which Coordinate to use : {}\nTapered Offset Type Flag   : {}\nFirst Offset Distance      : {}  Arc Length : {}\nSecond Offset Distance     : {}  Arc Length : {}\nNormal Vector : ",
            ent_live.function_parameter(),
            ent_live.tapered_offset_type(),
            iges_real_text(ent_live.first_offset_distance()),
            iges_real_text(ent_live.arc_length1()),
            iges_real_text(ent_live.second_offset_distance()),
            iges_real_text(ent_live.arc_length2())
        )?;
        iges_dump_xyz_l(
            s,
            level,
            ent_live.normal_vector().xyz(),
            &ent_live.vector_location()?,
        )?;
        writeln!(s)?;
        writeln!(
            s,
            "Offset curve Parameters. Starting : {}  Ending : {}",
            iges_real_text(ent_live.start_parameter()),
            iges_real_text(ent_live.end_parameter())
        )?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolOffsetSurface,
    "The own-parameter work on `IgesGeomOffsetSurface`."
);
impl IgesGeomToolOffsetSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomOffsetSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetSurface::read_own_params")?;
        let msg162 = iges_msg("XSTEP_162");
        let mut an_indicator: Xyz = Xyz::default();
        let mut a_distance: f64 = 0.0;
        let mut a_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        pr.read_xyz(&pr.current_list(1, 3), &msg162, &mut an_indicator)?;
        if !pr.read_real(&pr.current(), &mut a_distance)? {
            let msg163 = iges_msg("XSTEP_163");
            pr.send_fail(&msg163);
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_surface, false)? {
            let mut msg164 = iges_msg("XSTEP_164");
            iges_send_read_status(pr, &mut msg164, a_status, false)?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(an_indicator, a_distance, &a_surface);
        Ok(())
    }
}
impl IgesGeomToolOffsetSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomOffsetSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetSurface::write_own_params")?;
        iw.send(ent_live.offset_indicator().x())?;
        iw.send(ent_live.offset_indicator().y())?;
        iw.send(ent_live.offset_indicator().z())?;
        iw.send(ent_live.distance())?;
        iw.send_entity(&ent_live.surface(), false)?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomOffsetSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetSurface::own_shared")?;
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomOffsetSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(140, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolOffsetSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomOffsetSurface>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolOffsetSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomOffsetSurface>,
        ent: &Handle<IgesGeomOffsetSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolOffsetSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolOffsetSurface::own_copy")?;
        let a_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let an_indicator: Xyz = *another_live.offset_indicator().xyz();
        let a_distance: f64 = another_live.distance();
        ent_live.init(an_indicator, a_distance, &a_surface);
        Ok(())
    }
}
impl IgesGeomToolOffsetSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomOffsetSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolOffsetSurface::own_dump")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesGeomOffsetSurface\nOffset Indicator     : ")?;
        iges_dump_xyz_l(
            s,
            level,
            ent_live.offset_indicator().xyz(),
            &ent_live.vector_location()?,
        )?;
        write!(
            s,
            "\nOffset Distance      : {}  Surface to be offset : ",
            iges_real_text(ent_live.distance())
        )?;
        dumper.dump(&ent_live.surface(), s, sublevel, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolPlane,
    "The own-parameter work on `IgesGeomPlane`."
);
impl IgesGeomToolPlane {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomPlane>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPlane::read_own_params")?;
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut c: f64 = 0.0;
        let mut d: f64 = 0.0;
        let mut a_size: f64 = 0.0;
        let mut a_curve: Handle<IgesDataIgesEntity> = Handle::default();
        let mut attach: Xyz = Xyz::new(0.0, 0.0, 0.0);
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_real(&pr.current(), &mut a)?
            || !pr.read_real(&pr.current(), &mut b)?
            || !pr.read_real(&pr.current(), &mut c)?
            || !pr.read_real(&pr.current(), &mut d)?
        {
            let msg135 = iges_msg("XSTEP_135");
            pr.send_fail(&msg135);
        }
        if pr.is_param_defined(pr.current_number())? {
            if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_curve, true)? {
                let mut msg136 = iges_msg("XSTEP_136");
                iges_send_read_status(pr, &mut msg136, a_status, false)?;
            }
        }
        if pr.is_param_defined(pr.current_number())? {
            let msg139 = iges_msg("XSTEP_139");
            pr.read_xyz(&pr.current_list(1, 3), &msg139, &mut attach)?;
            if !pr.read_real(&pr.current(), &mut a_size)? {
                let msg138 = iges_msg("XSTEP_138");
                pr.send_fail(&msg138);
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a, b, c, d, &a_curve, attach, a_size);
        Ok(())
    }
}
impl IgesGeomToolPlane {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomPlane>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPlane::write_own_params")?;
        iges_shared_038!(a, b, c, d, ent_live);
        iw.send(a)?;
        iw.send(b)?;
        iw.send(c)?;
        iw.send(d)?;
        iw.send_entity(&ent_live.bounding_curve(), false)?;
        iw.send(ent_live.symbol_attach().x())?;
        iw.send(ent_live.symbol_attach().y())?;
        iw.send(ent_live.symbol_attach().z())?;
        iw.send(ent_live.symbol_size())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomPlane>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPlane::own_shared")?;
        iter.get_one_item(&ent_live.bounding_curve().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, ent: &Handle<IgesGeomPlane>) -> IgesDataDirChecker {
        let Some(ent_live) = ent.get() else {
            return IgesDataDirChecker::new();
        };
        let mut dc = IgesDataDirChecker::with_forms(108, -1, 1);
        dc.structure(IgesDataDefType::DefVoid);
        if ent_live.form_number() != 0 {
            dc.line_font(IgesDataDefType::DefAny);
        } else {
            dc.line_font(IgesDataDefType::DefVoid);
            dc.line_weight(IgesDataDefType::DefVoid);
            dc.hierarchy_status_ignored();
        }
        dc.color(IgesDataDefType::DefAny);
        dc
    }
}
impl IgesGeomToolPlane {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomPlane>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPlane::own_check")?;
        if ent_live.form_number() < -1 || ent_live.form_number() > 1 {
            let msg71 = iges_msg("XSTEP_71");
            ach.send_fail(&msg71);
        }
        let unbounded1: bool = ent_live.bounding_curve().is_null();
        let unbounded2: bool = ent_live.form_number() == 0;
        if unbounded1 && !unbounded2 || !unbounded1 && unbounded2 {
            let msg137 = iges_msg("XSTEP_137");
            ach.send_fail(&msg137);
        }
        if !ent_live.has_bounding_curve() {
            return Ok(());
        }
        Ok(())
    }
}
impl IgesGeomToolPlane {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomPlane>,
        ent: &Handle<IgesGeomPlane>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolPlane::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolPlane::own_copy")?;
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut c: f64 = 0.0;
        let mut d: f64 = 0.0;
        another_live.equation(&mut a, &mut b, &mut c, &mut d);
        let attach: Xyz = Xyz::from(another_live.symbol_attach());
        let a_size: f64 = another_live.symbol_size();
        let a_curve = tc
            .transferred(&another_live.bounding_curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        ent_live.init(a, b, c, d, &a_curve, attach, a_size);
        ent_live.set_form_number(another_live.form_number());
        Ok(())
    }
}
impl IgesGeomToolPlane {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomPlane>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolPlane::own_dump")?;
        writeln!(s, "IgesGeomPlane")?;
        iges_shared_038!(a, b, c, d, ent_live);
        write!(
            s,
            "Plane Coefficient A : {}\nPlane Coefficient B : {}\nPlane Coefficient C : {}\nPlane Coefficient D : {}\nThe Bounding Curve  : ",
            iges_real_text(a),
            iges_real_text(b),
            iges_real_text(c),
            iges_real_text(d)
        )?;
        dumper.dump(
            &ent_live.bounding_curve(),
            s,
            if level <= 4 { 0 } else { 1 },
            -1,
        )?;
        write!(s, "\nDisplay Symbol Location : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.symbol_attach()),
            &ent_live.location()?,
        )?;
        writeln!(s, "  Size  : {}", iges_real_text(ent_live.symbol_size()))?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolPoint,
    "The own-parameter work on `IgesGeomPoint`."
);
impl IgesGeomToolPoint {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomPoint>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolPoint::read_own_params")?;
        let msg73 = iges_msg("XSTEP_73");
        let mut a_point: Xyz = Xyz::default();
        let mut a_symbol: Handle<IgesBasicSubfigureDef> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        pr.read_xyz(&pr.current_list(1, 3), &msg73, &mut a_point)?;
        if pr.defined_else_skip()? {
            if !pr.read_entity_typed_status(
                ir,
                &pr.current(),
                &mut a_status,
                IgesBasicSubfigureDef::static_type(),
                &mut a_symbol,
                true,
            )? {
                let mut msg74 = iges_msg("XSTEP_74");
                iges_send_read_status(pr, &mut msg74, a_status, true)?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_point, &a_symbol);
        Ok(())
    }
}
