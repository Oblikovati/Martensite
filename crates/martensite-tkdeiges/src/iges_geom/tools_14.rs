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
    IgesGeomCurveOnSurface, IgesGeomToolTransformationMatrix, IgesGeomTransformationMatrix,
    IgesGeomTrimmedSurface,
};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_array2_set, iges_dump_entities, iges_live, iges_msg,
    iges_new_array1, iges_new_array2, iges_real_text,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolTransformationMatrix {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomTransformationMatrix>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::write_own_params")?;
        for i in 1..=3 {
            for j in 1..=4 {
                iw.send(ent_live.data(i, j)?)?;
            }
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomTransformationMatrix>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomTransformationMatrix>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_type(124);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.subordinate_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolTransformationMatrix {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesGeomTransformationMatrix>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::own_correct")?;
        if ent_live.form_number() > 1 {
            return Ok(false);
        }
        let cfn: i32 = if ent_live.value()?.is_negative() {
            1
        } else {
            0
        };
        if cfn == ent_live.form_number() {
            return Ok(false);
        }
        ent_live.set_form_number(cfn)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomTransformationMatrix>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::own_check")?;
        let form: i32 = ent_live.form_number();
        if form != 0 && form != 1 && (form < 10 || form > 12) {
            let msg71 = iges_msg("XSTEP_71");
            ach.send_fail(&msg71);
        }
        Ok(())
    }
}
impl IgesGeomToolTransformationMatrix {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomTransformationMatrix>,
        ent: &Handle<IgesGeomTransformationMatrix>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolTransformationMatrix::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::own_copy")?;
        let data: Handle<HArray2<f64>> = iges_new_array2::<f64>(1, 3, 1, 4)?;
        for i in 1..=3 {
            for j in 1..=4 {
                iges_array2_set(
                    &data,
                    i,
                    j,
                    another_live.data(i, j)?,
                    "IgesGeomToolTransformationMatrix::own_copy",
                )?;
            }
        }
        ent_live.init(&data)?;
        ent_live.set_form_number(another_live.form_number())?;
        Ok(())
    }
}
impl IgesGeomToolTransformationMatrix {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomTransformationMatrix>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolTransformationMatrix::own_dump")?;
        write!(
            s,
            "IgesGeomTransformationMatrix\n| R11, R12, R13, T1 |       {}, {}, {}, {}\n| R21, R22, R23, T2 |       {}, {}, {}, {}\n| R31, R32, R33, T3 |       {}, {}, {}, {}\n",
            iges_real_text(ent_live.data(1, 1)?),
            iges_real_text(ent_live.data(1, 2)?),
            iges_real_text(ent_live.data(1, 3)?),
            iges_real_text(ent_live.data(1, 4)?),
            iges_real_text(ent_live.data(2, 1)?),
            iges_real_text(ent_live.data(2, 2)?),
            iges_real_text(ent_live.data(2, 3)?),
            iges_real_text(ent_live.data(2, 4)?),
            iges_real_text(ent_live.data(3, 1)?),
            iges_real_text(ent_live.data(3, 2)?),
            iges_real_text(ent_live.data(3, 3)?),
            iges_real_text(ent_live.data(3, 4)?)
        )?;
        match ent_live.form_number() {
            0 => {
                writeln!(s, "-- Direct Orthogonal Matrix")?;
            }
            1 => {
                writeln!(s, "-- Reverse Orthogonal Matrix")?;
            }
            10 => {
                writeln!(s, "-- Cartesien Coordinate System")?;
            }
            11 => {
                writeln!(s, "-- Cylindrical Coordinate System")?;
            }
            12 => {
                writeln!(s, "-- Spherical Coordinate System")?;
            }
            _ => {
                writeln!(s, "--  (Incorrect Form Number)")?;
            }
        }
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolTrimmedSurface,
    "The own-parameter work on `IgesGeomTrimmedSurface`."
);
impl IgesGeomToolTrimmedSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomTrimmedSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::read_own_params")?;
        let mut a_flag: i32 = 0;
        let mut count: i32 = 0;
        let mut a_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut an_outer: Handle<IgesGeomCurveOnSurface> = Handle::default();
        let mut an_inner: Handle<HArray1<Handle<IgesGeomCurveOnSurface>>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_surface, false)? {
            let mut msg169 = iges_msg("XSTEP_169");
            iges_send_read_status(pr, &mut msg169, a_status, false)?;
        }
        if !pr.read_integer(&pr.current(), &mut a_flag)? {
            let msg170 = iges_msg("XSTEP_170");
            pr.send_fail(&msg170);
        }
        if pr.read_integer(&pr.current(), &mut count)? {
            if count < 0 {
                let msg171 = iges_msg("XSTEP_171");
                pr.send_fail(&msg171);
            } else if count > 0 {
                an_inner = iges_new_array1::<Handle<IgesGeomCurveOnSurface>>(1, count)?;
            }
        } else {
            let msg171 = iges_msg("XSTEP_171");
            pr.send_fail(&msg171);
        }
        if !pr.read_entity_typed_status(
            ir,
            &pr.current(),
            &mut a_status,
            IgesGeomCurveOnSurface::static_type(),
            &mut an_outer,
            true,
        )? {
            let mut msg172 = iges_msg("XSTEP_172");
            iges_send_read_status(pr, &mut msg172, a_status, true)?;
        }
        if !an_inner.is_null() {
            for i in 1..=count {
                let mut temp_ent: Handle<IgesGeomCurveOnSurface> = Handle::default();
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesGeomCurveOnSurface::static_type(),
                    &mut temp_ent,
                    false,
                )? {
                    iges_array1_set(
                        &an_inner,
                        i,
                        temp_ent.clone(),
                        "IgesGeomToolTrimmedSurface::read_own_params",
                    )?;
                } else {
                    let mut msg173 = iges_msg("XSTEP_173");
                    iges_send_read_status(pr, &mut msg173, a_status, true)?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&a_surface, a_flag, &an_outer, &an_inner)?;
        Ok(())
    }
}
impl IgesGeomToolTrimmedSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomTrimmedSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::write_own_params")?;
        let up: i32 = ent_live.nb_inner_contours();
        iw.send_entity(&ent_live.surface(), false)?;
        iw.send(ent_live.outer_boundary_type())?;
        iw.send(up)?;
        if ent_live.outer_boundary_type() != 0 {
            iw.send_entity(&ent_live.outer_contour(), false)?;
        } else {
            iw.send(0)?;
        }
        for i in 1..=up {
            iw.send_entity(&ent_live.inner_contour(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomTrimmedSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::own_shared")?;
        let up: i32 = ent_live.nb_inner_contours();
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.outer_contour().cast::<StandardTransient>());
        for i in 1..=up {
            iter.get_one_item(&ent_live.inner_contour(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomTrimmedSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(144, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolTrimmedSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomTrimmedSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::own_check")?;
        if ent_live.outer_boundary_type() != 0 && ent_live.outer_boundary_type() != 1 {
            let msg172 = iges_msg("XSTEP_172");
            ach.send_fail(&msg172);
        }
        Ok(())
    }
}
impl IgesGeomToolTrimmedSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomTrimmedSurface>,
        ent: &Handle<IgesGeomTrimmedSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolTrimmedSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::own_copy")?;
        let mut an_inner: Handle<HArray1<Handle<IgesGeomCurveOnSurface>>> = Handle::default();
        let a_surface = tc
            .transferred(&another_live.surface().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        let a_flag: i32 = another_live.outer_boundary_type();
        let an_outer = tc
            .transferred(&another_live.outer_contour().cast::<StandardTransient>())?
            .cast::<IgesGeomCurveOnSurface>();
        let count: i32 = another_live.nb_inner_contours();
        if count > 0 {
            an_inner = iges_new_array1::<Handle<IgesGeomCurveOnSurface>>(1, count)?;
            for i in 1..=count {
                let temp = tc
                    .transferred(&another_live.inner_contour(i)?.cast::<StandardTransient>())?
                    .cast::<IgesGeomCurveOnSurface>();
                iges_array1_set(
                    &an_inner,
                    i,
                    temp.clone(),
                    "IgesGeomToolTrimmedSurface::own_copy",
                )?;
            }
        }
        ent_live.init(&a_surface, a_flag, &an_outer, &an_inner)?;
        Ok(())
    }
}
impl IgesGeomToolTrimmedSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomTrimmedSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolTrimmedSurface::own_dump")?;
        let temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesGeomTrimmedSurface\nSurface to be trimmed : ")?;
        dumper.dump(&ent_live.surface(), s, temp_sub_level, -1)?;
        write!(
            s,
            "\nBoundary type    : {}\nOuter Boundary   : ",
            ent_live.outer_boundary_type()
        )?;
        dumper.dump(&ent_live.outer_contour(), s, temp_sub_level, -1)?;
        write!(s, "\nInner Boundaries : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_inner_contours(), |first| {
            ent_live.inner_contour(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
