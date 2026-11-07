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
    IgesGeomLine, IgesGeomSplineSurface, IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder,
    IgesGeomToolSplineSurface, IgesGeomTransformationMatrix,
};
use crate::iges_basic::entity_shape::{iges_array1_length, iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::iges_shared_045;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_item, iges_array2_set, iges_dump_vals, iges_dump_xyz_l,
    iges_live, iges_msg, iges_new_array2, iges_real_text,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolSplineSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomSplineSurface>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolSplineSurface::own_dump")?;
        writeln!(s, "IgesGeomSplineSurface")?;
        let nb_u_segs: i32 = ent_live.nb_u_segments()?;
        let nb_v_segs: i32 = ent_live.nb_v_segments()?;
        let mut temp: Handle<HArray1<f64>> = Handle::default();
        write!(
            s,
            "The  Spline Boundary Type : {}",
            ent_live.boundary_type()
        )?;
        match ent_live.boundary_type() {
            1 => {
                write!(s, "  (Linear)")?;
            }
            2 => {
                write!(s, "  (Quadratic)")?;
            }
            3 => {
                write!(s, "  (Cubic)")?;
            }
            4 => {
                write!(s, "  (Wilson-Fowler)")?;
            }
            5 => {
                write!(s, "  (Modified Wison-Fowler)")?;
            }
            6 => {
                write!(s, "  (B-Spline)")?;
            }
            _ => {
                write!(s, "  (Invalid value)")?;
            }
        }
        write!(s, "\nThe  Patch Type : {}", ent_live.patch_type())?;
        if ent_live.patch_type() == 1 {
            write!(s, "  (Cartesian Product)")?;
        } else {
            write!(s, "  (Unspecified)")?;
        }
        write!(
            s,
            "\nNumber Of Segments. In U : {}    In V : {}\nThe U Break Points : ",
            nb_u_segs, nb_v_segs
        )?;
        iges_dump_vals(s, level, 1, nb_u_segs + 1, |first| {
            ent_live.u_break_point(first)
        })?;
        write!(s, "\nThe V Break Points : ")?;
        iges_dump_vals(s, level, 1, nb_v_segs + 1, |first| {
            ent_live.v_break_point(first)
        })?;
        write!(s, "\n X-Y-Z Polynomials Of Segments :\n")?;
        if level > 4 {
            for i in 1..=nb_u_segs {
                for j in 1..=nb_v_segs {
                    write!(s, "[{},{}]:\nX Polynomial : ", i, j)?;
                    temp = ent_live.x_polynomial(i, j)?;
                    iges_dump_vals(
                        s,
                        level,
                        1,
                        iges_array1_length(&temp, "IgesGeomToolSplineSurface::own_dump")?,
                        |first| {
                            iges_array1_item(&temp, first, "IgesGeomToolSplineSurface::own_dump")
                        },
                    )?;
                    write!(s, "\nY Polynomial : ")?;
                    temp = ent_live.y_polynomial(i, j)?;
                    iges_dump_vals(
                        s,
                        level,
                        1,
                        iges_array1_length(&temp, "IgesGeomToolSplineSurface::own_dump")?,
                        |first| {
                            iges_array1_item(&temp, first, "IgesGeomToolSplineSurface::own_dump")
                        },
                    )?;
                    write!(s, "\nZ Polynomial : ")?;
                    temp = ent_live.z_polynomial(i, j)?;
                    iges_dump_vals(
                        s,
                        level,
                        1,
                        iges_array1_length(&temp, "IgesGeomToolSplineSurface::own_dump")?,
                        |first| {
                            iges_array1_item(&temp, first, "IgesGeomToolSplineSurface::own_dump")
                        },
                    )?;
                    writeln!(s)?;
                }
            }
        } else {
            writeln!(s)?;
        }
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolSurfaceOfRevolution,
    "The own-parameter work on `IgesGeomSurfaceOfRevolution`."
);
impl IgesGeomToolSurfaceOfRevolution {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomSurfaceOfRevolution>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSurfaceOfRevolution::read_own_params")?;
        let mut an_axis: Handle<IgesGeomLine> = Handle::default();
        let mut a_generatrix: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_start_angle: f64 = 0.0;
        let mut an_end_angle: f64 = 0.0;
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_typed_status(
            ir,
            &pr.current(),
            &mut a_status,
            IgesGeomLine::static_type(),
            &mut an_axis,
            false,
        )? {
            let mut msg152 = iges_msg("XSTEP_152");
            iges_send_read_status(pr, &mut msg152, a_status, true)?;
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_generatrix, false)? {
            let mut msg153 = iges_msg("XSTEP_153");
            iges_send_read_status(pr, &mut msg153, a_status, false)?;
        }
        if !pr.read_real(&pr.current(), &mut a_start_angle)? {
            let msg154 = iges_msg("XSTEP_154");
            pr.send_fail(&msg154);
        }
        if !pr.read_real(&pr.current(), &mut an_end_angle)? {
            let msg155 = iges_msg("XSTEP_155");
            pr.send_fail(&msg155);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&an_axis, &a_generatrix, a_start_angle, an_end_angle);
        Ok(())
    }
}
impl IgesGeomToolSurfaceOfRevolution {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomSurfaceOfRevolution>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSurfaceOfRevolution::write_own_params")?;
        iw.send_entity(&ent_live.axis_of_revolution(), false)?;
        iw.send_entity(&ent_live.generatrix(), false)?;
        iw.send(ent_live.start_angle())?;
        iw.send(ent_live.end_angle())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomSurfaceOfRevolution>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSurfaceOfRevolution::own_shared")?;
        iter.get_one_item(&ent_live.axis_of_revolution().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.generatrix().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomSurfaceOfRevolution>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(120, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolSurfaceOfRevolution {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomSurfaceOfRevolution>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolSurfaceOfRevolution {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomSurfaceOfRevolution>,
        ent: &Handle<IgesGeomSurfaceOfRevolution>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolSurfaceOfRevolution::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolSurfaceOfRevolution::own_copy")?;
        let an_axis = tc
            .transferred(
                &another_live
                    .axis_of_revolution()
                    .cast::<StandardTransient>(),
            )?
            .cast::<IgesGeomLine>();
        let a_generatrix = tc
            .transferred(&another_live.generatrix().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_start_angle: f64 = another_live.start_angle();
        let an_end_angle: f64 = another_live.end_angle();
        ent_live.init(&an_axis, &a_generatrix, a_start_angle, an_end_angle);
        Ok(())
    }
}
impl IgesGeomToolSurfaceOfRevolution {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomSurfaceOfRevolution>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolSurfaceOfRevolution::own_dump")?;
        write!(s, "IgesGeomSurfaceOfRevolution\n\n")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "Axis Of Revolution : ")?;
        dumper.dump(&ent_live.axis_of_revolution(), s, sublevel, -1)?;
        write!(s, "\nGeneratrix         : ")?;
        dumper.dump(&ent_live.generatrix(), s, sublevel, -1)?;
        write!(
            s,
            "\nStart Angle        : {}  End Angle   : {}\n",
            iges_real_text(ent_live.start_angle()),
            iges_real_text(ent_live.end_angle())
        )?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolTabulatedCylinder,
    "The own-parameter work on `IgesGeomTabulatedCylinder`."
);
impl IgesGeomToolTabulatedCylinder {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomTabulatedCylinder>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTabulatedCylinder::read_own_params")?;
        let msg157 = iges_msg("XSTEP_157");
        let mut a_directrix: Handle<IgesDataIgesEntity> = Handle::default();
        let mut an_end: Xyz = Xyz::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_directrix, false)? {
            let mut msg156 = iges_msg("XSTEP_156");
            iges_send_read_status(pr, &mut msg156, a_status, false)?;
        }
        pr.read_xyz(&pr.current_list(1, 3), &msg157, &mut an_end)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&a_directrix, an_end);
        Ok(())
    }
}
impl IgesGeomToolTabulatedCylinder {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomTabulatedCylinder>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTabulatedCylinder::write_own_params")?;
        iw.send_entity(&ent_live.directrix(), false)?;
        iges_shared_045!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomTabulatedCylinder>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTabulatedCylinder::own_shared")?;
        iter.get_one_item(&ent_live.directrix().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomTabulatedCylinder>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(122, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolTabulatedCylinder {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesGeomTabulatedCylinder>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesGeomToolTabulatedCylinder {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomTabulatedCylinder>,
        ent: &Handle<IgesGeomTabulatedCylinder>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolTabulatedCylinder::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolTabulatedCylinder::own_copy")?;
        let a_directrix = tc
            .transferred(&another_live.directrix().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let an_end: Xyz = Xyz::from(another_live.end_point());
        ent_live.init(&a_directrix, an_end);
        Ok(())
    }
}
impl IgesGeomToolTabulatedCylinder {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomTabulatedCylinder>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolTabulatedCylinder::own_dump")?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesGeomTabulatedCylinder\nDirectrix       : ")?;
        dumper.dump(&ent_live.directrix(), s, sublevel, -1)?;
        write!(s, "\nTerminate Point : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.end_point()),
            &ent_live.location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolTransformationMatrix,
    "The own-parameter work on `IgesGeomTransformationMatrix`."
);
impl IgesGeomToolTransformationMatrix {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomTransformationMatrix>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::read_own_params")?;
        let mut temp: f64 = 0.0;
        let a_matrix: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 3, 1, 4)?;
        for i in 1..=3 {
            for j in 1..=4 {
                if pr.read_real(&pr.current(), &mut temp)? {
                    iges_array2_set(
                        &a_matrix,
                        i,
                        j,
                        temp,
                        "IgesGeomToolTransformationMatrix::read_own_params",
                    )?;
                } else {
                    let msg215 = iges_msg("XSTEP_215");
                    pr.send_fail(&msg215);
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&a_matrix)?;
        Ok(())
    }
}
