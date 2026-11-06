#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomSplineCurve, IgesGeomToolSplineCurve};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{
    iges_shared_008, iges_shared_009, iges_shared_020, iges_shared_021, iges_shared_024,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesWriter,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_array2_set, iges_dump_vals, iges_live, iges_msg,
    iges_new_array1, iges_new_array2, iges_real_text,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolSplineCurve {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomSplineCurve>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineCurve::write_own_params")?;
        iw.send(ent_live.spline_type())?;
        iw.send(ent_live.degree())?;
        iw.send(ent_live.nb_dimensions())?;
        let nb_segments: i32 = ent_live.nb_segments()?;
        iw.send(nb_segments)?;
        for i in 1..=nb_segments + 1 {
            iw.send(ent_live.break_point(i)?)?;
        }
        iges_shared_009!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz);
        for i in 1..=nb_segments {
            iges_shared_020!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ent_live, i);
            iges_shared_008!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, iw);
        }
        iges_shared_024!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ent_live);
        iges_shared_008!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomSplineCurve>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomSplineCurve>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(112, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolSplineCurve {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomSplineCurve>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolSplineCurve::own_check")?;
        if ent_live.spline_type() < 1 || ent_live.spline_type() > 6 {
            let msg91 = iges_msg("XSTEP_91");
            ach.send_fail(&msg91);
        }
        if ent_live.nb_dimensions() == 2 {
            let nb_segments: i32 = ent_live.nb_segments()?;
            let mut az: f64 = 0.0;
            let mut bz: f64 = 0.0;
            let mut cz: f64 = 0.0;
            let mut dz: f64 = 0.0;
            let msg96 = iges_msg("XSTEP_96");
            for i in 1..=nb_segments {
                ent_live.z_coord_polynomial(i, &mut az, &mut bz, &mut cz, &mut dz)?;
                if bz != 0.0 || cz != 0.0 || dz != 0.0 {
                    ach.send_fail(&msg96);
                }
            }
        } else if ent_live.nb_dimensions() != 3 {
            let msg93 = iges_msg("XSTEP_93");
            ach.send_fail(&msg93);
        }
        Ok(())
    }
}
impl IgesGeomToolSplineCurve {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomSplineCurve>,
        ent: &Handle<IgesGeomSplineCurve>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolSplineCurve::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolSplineCurve::own_copy")?;
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut c: f64 = 0.0;
        let mut d: f64 = 0.0;
        let a_type: i32 = another_live.spline_type();
        let a_degree: i32 = another_live.degree();
        let nb_dimensions: i32 = another_live.nb_dimensions();
        let nb_segments: i32 = another_live.nb_segments()?;
        let all_break_points: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, nb_segments + 1)?;
        for i in 1..=nb_segments + 1 {
            iges_array1_set(
                &all_break_points,
                i,
                another_live.break_point(i)?,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
        }
        let all_x_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
        let all_y_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
        let all_z_polynomials: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, nb_segments, 1, 4)?;
        for i in 1..=nb_segments {
            another_live.x_coord_polynomial(i, &mut a, &mut b, &mut c, &mut d)?;
            iges_array2_set(
                &all_x_polynomials,
                i,
                1,
                a,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_x_polynomials,
                i,
                2,
                b,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_x_polynomials,
                i,
                3,
                c,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_x_polynomials,
                i,
                4,
                d,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            another_live.y_coord_polynomial(i, &mut a, &mut b, &mut c, &mut d)?;
            iges_array2_set(
                &all_y_polynomials,
                i,
                1,
                a,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_y_polynomials,
                i,
                2,
                b,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_y_polynomials,
                i,
                3,
                c,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_y_polynomials,
                i,
                4,
                d,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            another_live.z_coord_polynomial(i, &mut a, &mut b, &mut c, &mut d)?;
            iges_array2_set(
                &all_z_polynomials,
                i,
                1,
                a,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_z_polynomials,
                i,
                2,
                b,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_z_polynomials,
                i,
                3,
                c,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
            iges_array2_set(
                &all_z_polynomials,
                i,
                4,
                d,
                "IgesGeomToolSplineCurve::own_copy",
            )?;
        }
        iges_shared_021!(all_xvalues, all_yvalues, all_zvalues);
        another_live.x_values(&mut a, &mut b, &mut c, &mut d)?;
        iges_array1_set(&all_xvalues, 1, a, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_xvalues, 2, b, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_xvalues, 3, c, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_xvalues, 4, d, "IgesGeomToolSplineCurve::own_copy")?;
        another_live.y_values(&mut a, &mut b, &mut c, &mut d)?;
        iges_array1_set(&all_yvalues, 1, a, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_yvalues, 2, b, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_yvalues, 3, c, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_yvalues, 4, d, "IgesGeomToolSplineCurve::own_copy")?;
        another_live.z_values(&mut a, &mut b, &mut c, &mut d)?;
        iges_array1_set(&all_zvalues, 1, a, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_zvalues, 2, b, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_zvalues, 3, c, "IgesGeomToolSplineCurve::own_copy")?;
        iges_array1_set(&all_zvalues, 4, d, "IgesGeomToolSplineCurve::own_copy")?;
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
        Ok(())
    }
}
impl IgesGeomToolSplineCurve {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomSplineCurve>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolSplineCurve::own_dump")?;
        writeln!(s, "IgesGeomSplineCurve")?;
        let nb_segments: i32 = ent_live.nb_segments()?;
        write!(s, "Spline Type          : {}  ", ent_live.spline_type())?;
        match ent_live.spline_type() {
            1 => {
                write!(s, "(Linear)")?;
            }
            2 => {
                write!(s, "(Quadratic)")?;
            }
            3 => {
                write!(s, "(Cubic)")?;
            }
            4 => {
                write!(s, "(Wilson-Fowler)")?;
            }
            5 => {
                write!(s, "(Modified Wison-Fowler)")?;
            }
            6 => {
                write!(s, "(B-Spline)")?;
            }
            _ => {
                write!(s, "(Invalid value)")?;
            }
        }
        write!(
            s,
            "\nDegree Of Continuity : {}\nNumber Of Dimensions : {}\nNumber Of Segments   : {}\nSegment Break Points : ",
            ent_live.degree(),
            ent_live.nb_dimensions(),
            ent_live.nb_segments()?
        )?;
        iges_dump_vals(s, level, 1, nb_segments + 1, |first| {
            ent_live.break_point(first)
        })?;
        if level <= 4 {
            writeln!(s, " [ also ask level > 4 for X-Y-Z Polynomials ]")?;
            return Ok(());
        }
        writeln!(s, "  --  Polynomial  Values  --")?;
        iges_shared_009!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz);
        for i in 1..=nb_segments {
            iges_shared_020!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ent_live, i);
            write!(
                s,
                "Segment {} :	    X		   Y		Z\n A ...	{}	{}	{}\n B ...	{}	{}	{}\n C ...	{}	{}	{}\n D ...	{}	{}	{}\n",
                i,
                iges_real_text(ax),
                iges_real_text(ay),
                iges_real_text(az),
                iges_real_text(bx),
                iges_real_text(by),
                iges_real_text(bz),
                iges_real_text(cx),
                iges_real_text(cy),
                iges_real_text(cz),
                iges_real_text(dx),
                iges_real_text(dy),
                iges_real_text(dz)
            )?;
        }
        iges_shared_024!(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ent_live);
        write!(
            s,
            "Terminate Point :	    X		   Y		Z\n Value        	{}	{}	{}\n 1st Derivative	{}	{}	{}\n 2nd Der./2!   	{}	{}	{}\n 3rd Der./3!   	{}	{}	{}\n",
            iges_real_text(ax),
            iges_real_text(ay),
            iges_real_text(az),
            iges_real_text(bx),
            iges_real_text(by),
            iges_real_text(bz),
            iges_real_text(cx),
            iges_real_text(cy),
            iges_real_text(cz),
            iges_real_text(dx),
            iges_real_text(dy),
            iges_real_text(dz)
        )?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolSplineSurface,
    "The own-parameter work on `IgesGeomSplineSurface`."
);
