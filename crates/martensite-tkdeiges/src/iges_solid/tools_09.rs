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
    IgesSolidSolidOfLinearExtrusion, IgesSolidSolidOfRevolution, IgesSolidSphere,
    IgesSolidToolSolidOfLinearExtrusion,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{
    iges_shared_003, iges_shared_006, iges_shared_015, iges_shared_044,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{IgesDataDumpResult, iges_dump_xyz_l, iges_live, iges_real_text};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolSolidOfLinearExtrusion {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::write_own_params")?;
        iw.send_entity(&ent_live.curve(), false)?;
        iw.send(ent_live.extrusion_length())?;
        iw.send(ent_live.extrusion_direction()?.x())?;
        iw.send(ent_live.extrusion_direction()?.y())?;
        iw.send(ent_live.extrusion_direction()?.z())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::own_shared")?;
        iter.get_one_item(&ent_live.curve().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(
        &self,
        _ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
    ) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(164, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolSolidOfLinearExtrusion {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::own_check")?;
        if ent_live.extrusion_length() <= 0.0 {
            ach.add_fail_text("Length of extrusion : Not Positive", "");
        }
        Ok(())
    }
}
impl IgesSolidToolSolidOfLinearExtrusion {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSolidOfLinearExtrusion>,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSolidOfLinearExtrusion::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::own_copy")?;
        let temp_entity = tc
            .transferred(&another_live.curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let temp_length: f64 = another_live.extrusion_length();
        let temp_direction: Xyz = Xyz::from(another_live.extrusion_direction()?);
        ent_live.init(&temp_entity, temp_length, temp_direction);
        Ok(())
    }
}
impl IgesSolidToolSolidOfLinearExtrusion {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::own_dump")?;
        write!(s, "IgesSolidSolidOfLinearExtrusion\nCurve entity        : ")?;
        dumper.dump(&ent_live.curve(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        write!(
            s,
            "\nExtrusion length    : {}\nExtrusion direction : ",
            iges_real_text(ent_live.extrusion_length())
        )?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.extrusion_direction()?),
            &ent_live.vector_location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSolidOfRevolution,
    "The own-parameter work on `IgesSolidSolidOfRevolution`."
);
impl IgesSolidToolSolidOfRevolution {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::read_own_params")?;
        let mut temp_entity: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_axis_point: Xyz = Xyz::default();
        let mut temp_axis: Xyz = Xyz::default();
        let mut temp_fraction: f64 = 0.0;
        let mut tempreal: f64 = 0.0;
        pr.read_entity(ir, &pr.current(), "Curve Entity", &mut temp_entity, false)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Fraction of rotation", &mut temp_fraction)?;
        } else {
            temp_fraction = 1.0;
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Axis Point (X)", &mut tempreal)? {
                temp_axis_point.set_x(tempreal);
            }
        } else {
            temp_axis_point.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Axis Point (Y)", &mut tempreal)? {
                temp_axis_point.set_y(tempreal);
            }
        } else {
            temp_axis_point.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Axis Point (Z)", &mut tempreal)? {
                temp_axis_point.set_z(tempreal);
            }
        } else {
            temp_axis_point.set_z(0.0);
        }
        iges_shared_003!(ent, pr, self, temp_axis, tempreal);
        ent_live.init(&temp_entity, temp_fraction, temp_axis_point, temp_axis);
        let eps: f64 = 1.0e-05;
        if !temp_axis.is_equal(&Xyz::from(ent_live.axis()?), eps) {
            pr.add_warning("Axis poorly unitary, normalized", "");
        }
        Ok(())
    }
}
impl IgesSolidToolSolidOfRevolution {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::write_own_params")?;
        iw.send_entity(&ent_live.curve(), false)?;
        iw.send(ent_live.fraction())?;
        iges_shared_006!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::own_shared")?;
        iter.get_one_item(&ent_live.curve().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSolidOfRevolution>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(162, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolSolidOfRevolution {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::own_check")?;
        if ent_live.fraction() <= 0.0 || ent_live.fraction() > 1.0 {
            ach.add_fail_text("Fraction of rotation : Incorrect value", "");
        }
        Ok(())
    }
}
impl IgesSolidToolSolidOfRevolution {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSolidOfRevolution>,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSolidOfRevolution::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::own_copy")?;
        let temp_entity = tc
            .transferred(&another_live.curve().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let temp_fraction: f64 = another_live.fraction();
        let temp_axis_point: Xyz = Xyz::from(another_live.axis_point());
        let temp_axis: Xyz = Xyz::from(another_live.axis()?);
        ent_live.init(&temp_entity, temp_fraction, temp_axis_point, temp_axis);
        Ok(())
    }
}
impl IgesSolidToolSolidOfRevolution {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSolidOfRevolution>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfRevolution::own_dump")?;
        write!(s, "IgesSolidSolidOfRevolution\nCurve entity   :")?;
        dumper.dump(&ent_live.curve(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        write!(
            s,
            "\nFraction of rotation : {}\nAxis Point     : ",
            iges_real_text(ent_live.fraction())
        )?;
        iges_shared_015!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSphere,
    "The own-parameter work on `IgesSolidSphere`."
);
impl IgesSolidToolSphere {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSphere>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphere::read_own_params")?;
        let mut temp_radius: f64 = 0.0;
        let mut tempreal: f64 = 0.0;
        let mut temp_center: Xyz = Xyz::default();
        pr.read_real_text(&pr.current(), "Radius", &mut temp_radius)?;
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center (X)", &mut tempreal)? {
                temp_center.set_x(tempreal);
            }
        } else {
            temp_center.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center (Y)", &mut tempreal)? {
                temp_center.set_y(tempreal);
            }
        } else {
            temp_center.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Center (Z)", &mut tempreal)? {
                temp_center.set_z(tempreal);
            }
        } else {
            temp_center.set_z(0.0);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_radius, temp_center);
        Ok(())
    }
}
impl IgesSolidToolSphere {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSphere>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphere::write_own_params")?;
        iw.send(ent_live.radius())?;
        iges_shared_044!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidSphere>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSphere>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(158, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolSphere {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidSphere>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphere::own_check")?;
        if ent_live.radius() <= 0.0 {
            ach.add_fail_text("Radius : Not Positive", "");
        }
        Ok(())
    }
}
impl IgesSolidToolSphere {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSphere>,
        ent: &Handle<IgesSolidSphere>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSphere::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolSphere::own_copy")?;
        ent_live.init(another_live.radius(), Xyz::from(another_live.center()));
        Ok(())
    }
}
impl IgesSolidToolSphere {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSphere>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSphere::own_dump")?;
        write!(
            s,
            "IgesSolidSphere\nRadius : {}\nCenter : ",
            iges_real_text(ent_live.radius())
        )?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.center()),
            &ent_live.location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSphericalSurface,
    "The own-parameter work on `IgesSolidSphericalSurface`."
);
