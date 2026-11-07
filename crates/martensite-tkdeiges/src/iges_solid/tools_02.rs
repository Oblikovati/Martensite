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
    IgesSolidConeFrustum, IgesSolidConicalSurface, IgesSolidCylinder, IgesSolidCylindricalSurface,
    IgesSolidToolConeFrustum,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{
    iges_shared_002, iges_shared_007, iges_shared_011, iges_shared_013, iges_shared_014,
    iges_shared_016, iges_shared_028, iges_shared_032, iges_shared_037, iges_shared_039,
    iges_shared_042,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{IgesDataDumpResult, iges_dump_xyz_l, iges_live, iges_real_text};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolConeFrustum {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidConeFrustum>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConeFrustum::write_own_params")?;
        iw.send(ent_live.height())?;
        iw.send(ent_live.larger_radius())?;
        iw.send(ent_live.smaller_radius())?;
        iges_shared_007!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidConeFrustum>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidConeFrustum>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(156, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolConeFrustum {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidConeFrustum>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConeFrustum::own_check")?;
        if ent_live.height() <= 0.0 {
            ach.add_fail_text("Height : Value Not Positive", "");
        }
        if ent_live.larger_radius() <= 0.0 {
            ach.add_fail_text("Larger face radius : Value Not Positive", "");
        }
        if ent_live.smaller_radius() < 0.0 {
            ach.add_fail_text("Smaller face radius : Value Not Positive", "");
        }
        if ent_live.smaller_radius() > ent_live.larger_radius() {
            ach.add_fail_text(
                "Smaller face radius : is greater than Larger face radius",
                "",
            );
        }
        Ok(())
    }
}
impl IgesSolidToolConeFrustum {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidConeFrustum>,
        ent: &Handle<IgesSolidConeFrustum>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConeFrustum::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolConeFrustum::own_copy")?;
        ent_live.init(
            another_live.height(),
            another_live.larger_radius(),
            another_live.smaller_radius(),
            Xyz::from(another_live.face_center()),
            Xyz::from(another_live.axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolConeFrustum {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidConeFrustum>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolConeFrustum::own_dump")?;
        write!(
            s,
            "IgesSolidConeFrustum\nHeight : {}  Larger face Radius : {}  Smaller face Radius : {}\nFace Center : ",
            iges_real_text(ent_live.height()),
            iges_real_text(ent_live.larger_radius()),
            iges_real_text(ent_live.smaller_radius())
        )?;
        iges_shared_016!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolConicalSurface,
    "The own-parameter work on `IgesSolidConicalSurface`."
);
impl IgesSolidToolConicalSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidConicalSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::read_own_params")?;
        iges_shared_037!(temp_axis, temp_location, temp_radius, temp_refdir);
        let mut temp_angle: f64 = 0.0;
        iges_shared_011!(ir, pr, temp_axis, temp_location, temp_radius);
        pr.read_real_text(&pr.current(), "Semi-angle", &mut temp_angle)?;
        iges_shared_032!(ent, ent_live, ir, pr, self, temp_refdir);
        ent_live.init(
            &temp_location,
            &temp_axis,
            temp_radius,
            temp_angle,
            &temp_refdir,
        );
        Ok(())
    }
}
impl IgesSolidToolConicalSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidConicalSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::write_own_params")?;
        iw.send_entity(&ent_live.location_point(), false)?;
        iw.send_entity(&ent_live.axis(), false)?;
        iw.send(ent_live.radius())?;
        iw.send(ent_live.semi_angle())?;
        if ent_live.is_parametrised() {
            iw.send_entity(&ent_live.reference_dir(), false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidConicalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::own_shared")?;
        iges_shared_028!(ent_live, iter);
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidConicalSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(194, 0, 1);
        iges_shared_042!(dc);
        dc
    }
}
impl IgesSolidToolConicalSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidConicalSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::own_check")?;
        if ent_live.radius() < 0.0 {
            ach.add_fail_text("Radius : Value Negative", "");
        }
        if ent_live.semi_angle() < 0.0 || ent_live.semi_angle() > 90.0 {
            ach.add_fail_text("Semi-angle : Value not in the range [0 - 90]", "");
        }
        let mut fn_: i32 = 0;
        if ent_live.is_parametrised() {
            fn_ = 1;
        }
        if fn_ != ent_live.form_number() {
            ach.add_fail_text("Parametrised Status Mismatches with Form Number", "");
        }
        Ok(())
    }
}
impl IgesSolidToolConicalSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidConicalSurface>,
        ent: &Handle<IgesSolidConicalSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolConicalSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::own_copy")?;
        let mut temp_radius: f64 = 0.0;
        let mut temp_angle: f64 = 0.0;
        iges_shared_014!(another_live, tc, temp_axis, temp_location, temp_radius);
        temp_angle = another_live.semi_angle();
        if another_live.is_parametrised() {
            let temp_refdir = tc
                .transferred(&another_live.reference_dir().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            ent_live.init(
                &temp_location,
                &temp_axis,
                temp_radius,
                temp_angle,
                &temp_refdir,
            );
        } else {
            let temp_refdir: Handle<IgesGeomDirection> = Handle::default();
            ent_live.init(
                &temp_location,
                &temp_axis,
                temp_radius,
                temp_angle,
                &temp_refdir,
            );
        }
        Ok(())
    }
}
impl IgesSolidToolConicalSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidConicalSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolConicalSurface::own_dump")?;
        writeln!(s, "IgesSolidConicalSurface")?;
        iges_shared_013!(dumper, ent_live, level, s, sublevel);
        write!(
            s,
            "\nRadius         : {}  Semi-angle     : {}\n",
            iges_real_text(ent_live.radius()),
            iges_real_text(ent_live.semi_angle())
        )?;
        if ent_live.is_parametrised() {
            writeln!(s, "Surface is Parametrised  -  Reference direction :")?;
            dumper.dump(&ent_live.reference_dir(), s, sublevel, -1)?;
            writeln!(s)?;
        } else {
            writeln!(s, "Surface is UnParametrised")?;
        }
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolCylinder,
    "The own-parameter work on `IgesSolidCylinder`."
);
impl IgesSolidToolCylinder {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidCylinder>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylinder::read_own_params")?;
        let mut temp_height: f64 = 0.0;
        let mut temp_radius: f64 = 0.0;
        iges_shared_039!(pr, temp_axis, temp_center, temp_height, tempreal);
        pr.read_real_text(&pr.current(), "Radius", &mut temp_radius)?;
        iges_shared_002!(ent, pr, self, temp_axis, temp_center, tempreal);
        ent_live.init(temp_height, temp_radius, temp_center, temp_axis);
        let eps: f64 = 1.0e-05;
        if !temp_axis.is_equal(&Xyz::from(ent_live.axis()?), eps) {
            pr.add_warning("Axis poorly unitary, normalized", "");
        }
        Ok(())
    }
}
impl IgesSolidToolCylinder {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidCylinder>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylinder::write_own_params")?;
        iw.send(ent_live.height())?;
        iw.send(ent_live.radius())?;
        iges_shared_007!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidCylinder>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidCylinder>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(154, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolCylinder {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidCylinder>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylinder::own_check")?;
        if ent_live.height() <= 0.0 {
            ach.add_fail_text("Height : Value < 0", "");
        }
        if ent_live.radius() <= 0.0 {
            ach.add_fail_text("Radius : Value < 0", "");
        }
        Ok(())
    }
}
impl IgesSolidToolCylinder {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidCylinder>,
        ent: &Handle<IgesSolidCylinder>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylinder::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolCylinder::own_copy")?;
        ent_live.init(
            another_live.height(),
            another_live.radius(),
            Xyz::from(another_live.face_center()),
            Xyz::from(another_live.axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolCylinder {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidCylinder>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolCylinder::own_dump")?;
        write!(
            s,
            "IgesSolidCylinder\nHeight : {}  Radius : {}\nCenter : ",
            iges_real_text(ent_live.height()),
            iges_real_text(ent_live.radius())
        )?;
        iges_shared_016!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolCylindricalSurface,
    "The own-parameter work on `IgesSolidCylindricalSurface`."
);
impl IgesSolidToolCylindricalSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidCylindricalSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::read_own_params")?;
        iges_shared_037!(temp_axis, temp_location, temp_radius, temp_refdir);
        iges_shared_011!(ir, pr, temp_axis, temp_location, temp_radius);
        iges_shared_032!(ent, ent_live, ir, pr, self, temp_refdir);
        ent_live.init(&temp_location, &temp_axis, temp_radius, &temp_refdir);
        Ok(())
    }
}
