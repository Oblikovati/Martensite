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
    IgesSolidSphericalSurface, IgesSolidToolSphericalSurface, IgesSolidToroidalSurface,
    IgesSolidTorus,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{
    iges_shared_003, iges_shared_027, iges_shared_035, iges_shared_042, iges_shared_053,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{IgesDataDumpResult, iges_live, iges_real_text};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolSphericalSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSphericalSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::read_own_params")?;
        let mut temp_center: Handle<IgesGeomPoint> = Handle::default();
        let mut temp_radius: f64 = 0.0;
        let mut temp_axis: Handle<IgesGeomDirection> = Handle::default();
        let mut temp_refdir: Handle<IgesGeomDirection> = Handle::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Center point",
            IgesGeomPoint::static_type(),
            &mut temp_center,
            false,
        )?;
        pr.read_real_text(&pr.current(), "Radius", &mut temp_radius)?;
        if ent_live.form_number() == 1 {
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Axis direction",
                IgesGeomDirection::static_type(),
                &mut temp_axis,
                false,
            )?;
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Reference direction",
                IgesGeomDirection::static_type(),
                &mut temp_refdir,
                false,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_center, temp_radius, &temp_axis, &temp_refdir);
        Ok(())
    }
}
impl IgesSolidToolSphericalSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSphericalSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::write_own_params")?;
        iw.send_entity(&ent_live.center(), false)?;
        iw.send(ent_live.radius())?;
        if ent_live.is_parametrised() {
            iw.send_entity(&ent_live.axis(), false)?;
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
        ent: &Handle<IgesSolidSphericalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::own_shared")?;
        iges_shared_027!(ent_live, iter);
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSphericalSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(196, 0, 1);
        iges_shared_042!(dc);
        dc
    }
}
impl IgesSolidToolSphericalSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidSphericalSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::own_check")?;
        if ent_live.radius() <= 0.0 {
            ach.add_fail_text("Radius : Not Positive", "");
        }
        let mut fn_: i32 = 0;
        if ent_live.is_parametrised() {
            fn_ = 1;
        }
        if fn_ != ent_live.form_number() {
            ach.add_fail_text("Parametrised Status Mismatches with Form Number", "");
        }
        if ent_live.axis().is_null() {
            if ent_live.is_parametrised() {
                ach.add_fail_text("Parametrised Spherical Surface : no Axis is defined", "");
            }
        }
        Ok(())
    }
}
impl IgesSolidToolSphericalSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSphericalSurface>,
        ent: &Handle<IgesSolidSphericalSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSphericalSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::own_copy")?;
        let temp_center = tc
            .transferred(&another_live.center().cast::<StandardTransient>())?
            .cast::<IgesGeomPoint>();
        let temp_radius: f64 = another_live.radius();
        if another_live.is_parametrised() {
            let temp_axis = tc
                .transferred(&another_live.axis().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            let temp_refdir = tc
                .transferred(&another_live.reference_dir().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            ent_live.init(&temp_center, temp_radius, &temp_axis, &temp_refdir);
        } else {
            let temp_axis: Handle<IgesGeomDirection> = Handle::default();
            let temp_refdir: Handle<IgesGeomDirection> = Handle::default();
            ent_live.init(&temp_center, temp_radius, &temp_axis, &temp_refdir);
        }
        Ok(())
    }
}
impl IgesSolidToolSphericalSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSphericalSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSphericalSurface::own_dump")?;
        writeln!(s, "IgesSolidSphericalSurface")?;
        iges_shared_053!(dumper, ent_live, level, s, sublevel);
        write!(s, "\nRadius : {}\n", iges_real_text(ent_live.radius()))?;
        if ent_live.is_parametrised() {
            write!(s, "Surface is Parametrised\nAxis direction      : ")?;
            dumper.dump(&ent_live.axis(), s, sublevel, -1)?;
            write!(s, "\nReference direction : ")?;
            dumper.dump(&ent_live.reference_dir(), s, sublevel, -1)?;
            writeln!(s)?;
        } else {
            writeln!(s, "Surface is UnParametrised")?;
        }
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolToroidalSurface,
    "The own-parameter work on `IgesSolidToroidalSurface`."
);
impl IgesSolidToolToroidalSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidToroidalSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::read_own_params")?;
        let mut temp_center: Handle<IgesGeomPoint> = Handle::default();
        let mut maj_rad: f64 = 0.0;
        let mut min_rad: f64 = 0.0;
        let mut temp_axis: Handle<IgesGeomDirection> = Handle::default();
        let mut temp_refdir: Handle<IgesDataIgesEntity> = Handle::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Center point",
            IgesGeomPoint::static_type(),
            &mut temp_center,
            false,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Axis direction",
            IgesGeomDirection::static_type(),
            &mut temp_axis,
            false,
        )?;
        pr.read_real_text(&pr.current(), "Major Radius", &mut maj_rad)?;
        pr.read_real_text(&pr.current(), "Minor Radius", &mut min_rad)?;
        if ent_live.form_number() == 1 {
            pr.read_entity(
                ir,
                &pr.current(),
                "Reference direction",
                &mut temp_refdir,
                false,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &temp_center,
            &temp_axis,
            maj_rad,
            min_rad,
            &temp_refdir.cast::<IgesGeomDirection>(),
        );
        Ok(())
    }
}
impl IgesSolidToolToroidalSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidToroidalSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::write_own_params")?;
        iw.send_entity(&ent_live.center(), false)?;
        iw.send_entity(&ent_live.axis(), false)?;
        iw.send(ent_live.major_radius())?;
        iw.send(ent_live.minor_radius())?;
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
        ent: &Handle<IgesSolidToroidalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::own_shared")?;
        iges_shared_027!(ent_live, iter);
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidToroidalSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(198, 0, 1);
        iges_shared_042!(dc);
        dc
    }
}
impl IgesSolidToolToroidalSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidToroidalSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::own_check")?;
        if ent_live.major_radius() <= 0.0 {
            ach.add_fail_text("Major Radius : Not Positive", "");
        }
        if ent_live.minor_radius() <= 0.0 {
            ach.add_fail_text("Minor Radius : Not Positive", "");
        }
        if ent_live.minor_radius() >= ent_live.major_radius() {
            ach.add_fail_text("Minor Radius : Value not < Major radius", "");
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
impl IgesSolidToolToroidalSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidToroidalSurface>,
        ent: &Handle<IgesSolidToroidalSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolToroidalSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::own_copy")?;
        let temp_center = tc
            .transferred(&another_live.center().cast::<StandardTransient>())?
            .cast::<IgesGeomPoint>();
        let temp_axis = tc
            .transferred(&another_live.axis().cast::<StandardTransient>())?
            .cast::<IgesGeomDirection>();
        let maj_rad: f64 = another_live.major_radius();
        let min_rad: f64 = another_live.minor_radius();
        if another_live.is_parametrised() {
            let temp_refdir = tc
                .transferred(&another_live.reference_dir().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            ent_live.init(&temp_center, &temp_axis, maj_rad, min_rad, &temp_refdir);
        } else {
            let temp_refdir: Handle<IgesGeomDirection> = Handle::default();
            ent_live.init(&temp_center, &temp_axis, maj_rad, min_rad, &temp_refdir);
        }
        Ok(())
    }
}
impl IgesSolidToolToroidalSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidToroidalSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolToroidalSurface::own_dump")?;
        writeln!(s, "IgesSolidToroidalSurface")?;
        iges_shared_053!(dumper, ent_live, level, s, sublevel);
        write!(s, "\nAxis direction : ")?;
        dumper.dump(&ent_live.axis(), s, sublevel, -1)?;
        write!(
            s,
            "\nMajor Radius : {}  Minor Radius : {}\n",
            iges_real_text(ent_live.major_radius()),
            iges_real_text(ent_live.minor_radius())
        )?;
        iges_shared_035!(dumper, ent_live, s, sublevel);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolTorus,
    "The own-parameter work on `IgesSolidTorus`."
);
impl IgesSolidToolTorus {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidTorus>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolTorus::read_own_params")?;
        let mut r1: f64 = 0.0;
        let mut r2: f64 = 0.0;
        let mut tempreal: f64 = 0.0;
        let mut temp_point: Xyz = Xyz::default();
        let mut temp_axis: Xyz = Xyz::default();
        pr.read_real_text(&pr.current(), "Radius of revolution", &mut r1)?;
        pr.read_real_text(&pr.current(), "Radius of disc", &mut r2)?;
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (X)", &mut tempreal)? {
                temp_point.set_x(tempreal);
            }
        } else {
            temp_point.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (Y)", &mut tempreal)? {
                temp_point.set_y(tempreal);
            }
        } else {
            temp_point.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center Point (Z)", &mut tempreal)? {
                temp_point.set_z(tempreal);
            }
        } else {
            temp_point.set_z(0.0);
        }
        iges_shared_003!(ent, pr, self, temp_axis, tempreal);
        ent_live.init(r1, r2, temp_point, temp_axis);
        let eps: f64 = 1.0e-05;
        if !temp_axis.is_equal(&Xyz::from(ent_live.axis()?), eps) {
            pr.add_warning("Axis poorly unitary, normalized", "");
        }
        Ok(())
    }
}
