#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomCurveOnSurface, IgesGeomDirection, IgesGeomFlash, IgesGeomToolCurveOnSurface};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::iges_shared_047;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLineFontEntity, IgesDataParamReader,
};
use crate::iges_data::{IgesDataDumpResult, iges_dump_xyz_l, iges_live, iges_msg};
use martensite_tkernel::message::Msg;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Xy, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolCurveOnSurface {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomCurveOnSurface>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::own_correct")?;
        let c2d: Handle<IgesDataIgesEntity> = ent_live.curve_uv();
        if c2d.is_null() {
            return Ok(false);
        }
        let uf: i32 = iges_live(&c2d, "IgesGeomToolCurveOnSurface::own_correct")?.use_flag();
        if uf == 5 {
            return Ok(false);
        }
        iges_live(&c2d, "IgesGeomToolCurveOnSurface::own_correct")?.init_status(
            iges_live(&c2d, "IgesGeomToolCurveOnSurface::own_correct")?.blank_status(),
            iges_live(&c2d, "IgesGeomToolCurveOnSurface::own_correct")?.subordinate_status(),
            5,
            iges_live(&c2d, "IgesGeomToolCurveOnSurface::own_correct")?.hierarchy_status(),
        );
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomCurveOnSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::own_check")?;
        if ent_live.curve_uv().is_null() && ent_live.curve3d().is_null() {
            let msg134: Msg = iges_msg("XSTEP_134");
            ach.send_fail(&msg134);
        }
        Ok(())
    }
}
impl IgesGeomToolCurveOnSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomCurveOnSurface>,
        ent: &Handle<IgesGeomCurveOnSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolCurveOnSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::own_copy")?;
        let a_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_curve_uv = tc
            .transferred(&another_live.curve_uv().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_curve3d = tc
            .transferred(&another_live.curve3d().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_mode: i32 = another_live.creation_mode();
        let a_preference: i32 = another_live.preference_mode();
        ent_live.init(a_mode, &a_surface, &a_curve_uv, &a_curve3d, a_preference);
        Ok(())
    }
}
impl IgesGeomToolCurveOnSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomCurveOnSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::own_dump")?;
        write!(s, "IgesGeomCurveOnSurface\n\n")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        let crem: i32 = ent_live.creation_mode();
        write!(s, "Creation Mode : {}  i.e. ", crem)?;
        if crem == 0 {
            writeln!(s, " <Unspecified>")?;
        } else if crem == 1 {
            writeln!(s, " Projection of a Curve on a Surface")?;
        } else if crem == 2 {
            writeln!(s, " Intersection of two Surfaces")?;
        } else if crem == 3 {
            writeln!(s, " Isoparametric Curve (either U or V)")?;
        } else {
            writeln!(s, " <Incorrect Value>")?;
        }
        write!(s, "The Surface on which the curve lies : ")?;
        dumper.dump(&ent_live.surface(), s, sublevel, -1)?;
        write!(s, "\nThe curve B (in the parametric space (u, v))  : ")?;
        dumper.dump(&ent_live.curve_uv(), s, sublevel, -1)?;
        write!(s, "\nThe curve C (in the 3D Space) : ")?;
        dumper.dump(&ent_live.curve3d(), s, sublevel, -1)?;
        writeln!(s)?;
        let pref: i32 = ent_live.preference_mode();
        write!(s, "Preferred representation mode : {}  i.e. ", pref)?;
        if pref == 0 {
            writeln!(s, " <Unspecified>")?;
        } else if pref == 1 {
            writeln!(s, " Curve B on Surface")?;
        } else if pref == 2 {
            writeln!(s, " Curve C in 3D Space")?;
        } else if pref == 3 {
            writeln!(s, " Curves B & C equally preferred")?;
        } else {
            writeln!(s, " <Incorrect Value>")?;
        }
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolDirection,
    "The own-parameter work on `IgesGeomDirection`."
);
impl IgesGeomToolDirection {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomDirection>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolDirection::read_own_params")?;
        let mut a_direction: Xyz = Xyz::default();
        let mut tmp_xy: Xy = Xy::default();
        let mut tmp_real: f64 = 0.0;
        if pr.read_xy(&pr.current_list(1, 2), "Direction", &mut tmp_xy)? {
            a_direction.set_x(tmp_xy.x());
            a_direction.set_y(tmp_xy.y());
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Direction", &mut tmp_real)? {
                a_direction.set_z(tmp_real);
            }
        } else {
            a_direction.set_z(0.0);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_direction);
        Ok(())
    }
}
impl IgesGeomToolDirection {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomDirection>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolDirection::write_own_params")?;
        iges_shared_047!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomDirection>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomDirection>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(123, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.subordinate_status_required(1);
        dc.use_flag_required(2);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolDirection {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomDirection>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolDirection::own_check")?;
        if (*ent_live.value().xyz()).square_modulus() <= 0.0 {
            ach.add_fail_text("Direction : The values indicate no direction", "");
        }
        Ok(())
    }
}
impl IgesGeomToolDirection {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomDirection>,
        ent: &Handle<IgesGeomDirection>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolDirection::own_copy")?;
        let another_live = iges_live(another, "IgesGeomToolDirection::own_copy")?;
        ent_live.init(*another_live.value().xyz());
        Ok(())
    }
}
impl IgesGeomToolDirection {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomDirection>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolDirection::own_dump")?;
        write!(s, "IgesGeomDirection\n\nValue : ")?;
        iges_dump_xyz_l(
            s,
            level,
            ent_live.value().xyz(),
            &ent_live.vector_location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolFlash,
    "The own-parameter work on `IgesGeomFlash`."
);
impl IgesGeomToolFlash {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomFlash>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::read_own_params")?;
        let mut a_point: Xy = Xy::default();
        let mut a_dim1: f64 = 0.0;
        let mut a_dim2: f64 = 0.0;
        let mut a_rotation: f64 = 0.0;
        let mut a_reference: Handle<IgesDataIgesEntity> = Handle::default();
        let fn_: i32 = ent_live.form_number();
        a_rotation = 0.0;
        a_dim2 = a_rotation;
        a_dim1 = a_dim2;
        pr.read_xy(&pr.current_list(1, 2), "Reference of Flash", &mut a_point)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "First Flash sizing parameter", &mut a_dim1)?;
        } else if fn_ > 0 {
            pr.add_fail("Fist Flash sizing parameter : undefined", "");
        }
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Second Flash sizing parameter", &mut a_dim2)?;
        } else if fn_ > 1 {
            pr.add_fail("Second Flash sizing parameter : not defined", "");
        }
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Rotation about ref. point", &mut a_rotation)?;
        } else if fn_ == 2 || fn_ == 4 {
            pr.add_fail("Rotation about ref. point : not defined", "");
        }
        if pr.is_param_entity(pr.current_number())? {
            pr.read_entity(
                ir,
                &pr.current(),
                "Referenced entity",
                &mut a_reference,
                false,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_point, a_dim1, a_dim2, a_rotation, &a_reference);
        Ok(())
    }
}
impl IgesGeomToolFlash {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomFlash>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::write_own_params")?;
        iw.send(ent_live.reference_point().x())?;
        iw.send(ent_live.reference_point().y())?;
        iw.send(ent_live.dimension1())?;
        iw.send(ent_live.dimension2())?;
        iw.send(ent_live.rotation())?;
        iw.send_entity(&ent_live.reference_entity(), false)?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomFlash>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::own_shared")?;
        iter.get_one_item(&ent_live.reference_entity().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomFlash>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(125, 0, 4);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefValue);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_required(0);
        dc
    }
}
impl IgesGeomToolFlash {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomFlash>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::own_correct")?;
        let fn_: i32 = ent_live.form_number();
        let res0: bool = ent_live.rank_line_font() != 1;
        if res0 {
            let nulfont: Handle<dyn IgesDataLineFontEntity> = Handle::default();
            ent_live.init_line_font(&nulfont, 1);
        }
        let mut res1: bool = false;
        let mut ref_: Handle<IgesDataIgesEntity> = ent_live.reference_entity();
        if fn_ != 0 && !ref_.is_null() {
            ref_.nullify();
            res1 = true;
        }
        let mut d1: f64 = ent_live.dimension1();
        let mut d2: f64 = ent_live.dimension2();
        let mut rt: f64 = ent_live.rotation();
        if fn_ == 0 && d1 != 0.0 {
            d1 = 0.0;
            res1 = true;
        }
        if fn_ <= 1 && d2 != 0.0 {
            d2 = 0.0;
            res1 = true;
        }
        if (fn_ <= 1 || fn_ == 3) && rt != 0.0 {
            rt = 0.0;
            res1 = true;
        }
        if res1 {
            ent_live.init(ent_live.reference_point().xy(), d1, d2, rt, &ref_);
        }
        Ok(res0 || res1)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomFlash>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolFlash::own_check")?;
        let fn_: i32 = ent_live.form_number();
        if ent_live.rank_line_font() != 1 {
            ach.add_fail_text("LineFontPattern : Value != 1", "");
        }
        if ent_live.reference_entity().is_null() {
            if fn_ == 0 {
                ach.add_fail_text("Flash defined by a Reference Entity, which is absent", "");
            }
        } else if fn_ != 0 {
            ach.add_warning_text("Reference Entity present though useless", "");
        }
        if fn_ == 1 && ent_live.dimension2() != 0.0 {
            ach.add_warning_text("Dimension 2 present though useless", "");
        }
        if (fn_ == 1 || fn_ == 3) && ent_live.rotation() != 0.0 {
            ach.add_warning_text("Rotation present though useless", "");
        }
        Ok(())
    }
}
impl IgesGeomToolFlash {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomFlash>,
        ent: &Handle<IgesGeomFlash>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolFlash::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolFlash::own_copy")?;
        let a_point: Xy = another_live.reference_point().xy();
        let a_dim1: f64 = another_live.dimension1();
        let a_dim2: f64 = another_live.dimension2();
        let a_rotation: f64 = another_live.rotation();
        let a_reference = tc
            .transferred(&another_live.reference_entity().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        ent_live.init(a_point, a_dim1, a_dim2, a_rotation, &a_reference);
        Ok(())
    }
}
