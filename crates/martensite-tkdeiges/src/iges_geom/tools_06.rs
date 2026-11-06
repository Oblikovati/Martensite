#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesGeomCopiousData, IgesGeomCurveOnSurface, IgesGeomToolCopiousData};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_live, iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{GeneralTransform3, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesGeomToolCopiousData {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomCopiousData>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCopiousData::read_own_params")?;
        let mut a_data_type: i32 = 0;
        let mut nb_data: i32 = 0;
        let mut a_z_plane: f64 = 0.0;
        let mut all_data: Handle<HArray1<f64>> = Handle::default();
        let mut upper: i32 = 0;
        let mut data: bool = false;
        if !pr.read_integer(&pr.current(), &mut a_data_type)? {
            let msg85 = iges_msg("XSTEP_85");
            pr.send_fail(&msg85);
        }
        let st: bool = pr.read_integer(&pr.current(), &mut nb_data)?;
        if st && nb_data > 0 {
            data = true;
        } else {
            let msg86 = iges_msg("XSTEP_86");
            pr.send_fail(&msg86);
        }
        if a_data_type == 1 {
            if !pr.read_real(&pr.current(), &mut a_z_plane)? {
                let msg87 = iges_msg("XSTEP_87");
                pr.send_fail(&msg87);
            }
        }
        if data {
            if a_data_type == 1 {
                upper = 2 * nb_data;
            } else if a_data_type == 2 {
                upper = 3 * nb_data;
            } else {
                upper = 6 * nb_data;
            }
            let msg88 = iges_msg("XSTEP_88");
            pr.read_reals(&pr.current_list(upper, 1), &msg88, &mut all_data, 1)?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_data_type, a_z_plane, &all_data)?;
        Ok(())
    }
}
impl IgesGeomToolCopiousData {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomCopiousData>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCopiousData::write_own_params")?;
        let upper: i32 = ent_live.nb_points()?;
        let dtype: i32 = ent_live.data_type();
        iw.send(dtype)?;
        iw.send(upper)?;
        if ent_live.data_type() == 1 {
            iw.send(ent_live.z_plane())?;
        }
        for i in 1..=upper {
            iw.send(ent_live.data(i, 1)?)?;
            iw.send(ent_live.data(i, 2)?)?;
            if dtype > 1 {
                iw.send(ent_live.data(i, 3)?)?;
            }
            if dtype <= 2 {
                continue;
            }
            iw.send(ent_live.data(i, 4)?)?;
            iw.send(ent_live.data(i, 5)?)?;
            iw.send(ent_live.data(i, 6)?)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesGeomCopiousData>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, ent: &Handle<IgesGeomCopiousData>) -> IgesDataDirChecker {
        let Some(ent_live) = ent.get() else {
            return IgesDataDirChecker::new();
        };
        let mut dc = IgesDataDirChecker::with_forms(106, 1, 63);
        dc.structure(IgesDataDefType::DefVoid);
        if ent_live.form_number() <= 3 {
            dc.line_font(IgesDataDefType::DefVoid);
            dc.line_weight(IgesDataDefType::DefVoid);
        } else {
            dc.line_font(IgesDataDefType::DefAny);
            dc.line_weight(IgesDataDefType::DefValue);
        }
        dc.color(IgesDataDefType::DefAny);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesGeomToolCopiousData {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGeomCopiousData>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCopiousData::own_check")?;
        let msg71 = iges_msg("XSTEP_71");
        let fn_: i32 = ent_live.form_number();
        if fn_ > 3 && fn_ < 11 || fn_ > 14 && fn_ < 63 {
            ach.send_fail(&msg71);
        }
        let dt: i32 = ent_live.data_type();
        if dt < 1 || dt > 3 {
            let msg85 = iges_msg("XSTEP_85");
            ach.send_fail(&msg85);
        }
        if dt == 1 && (fn_ != 1 && fn_ != 11 && fn_ != 63)
            || dt == 2 && (fn_ != 2 && fn_ != 12)
            || dt == 3 && (fn_ != 3 && fn_ != 13)
        {
            ach.send_fail(&msg71);
        }
        Ok(())
    }
}
impl IgesGeomToolCopiousData {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGeomCopiousData>,
        ent: &Handle<IgesGeomCopiousData>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesGeomToolCopiousData::own_copy")?;
        let ent_live = iges_live(ent, "IgesGeomToolCopiousData::own_copy")?;
        let mut upper: i32 = 0;
        let mut a_z_plane: f64 = 0.0;
        let nb_tuples: i32 = another_live.nb_points()?;
        let a_data_type: i32 = another_live.data_type();
        let mut all_data: Handle<HArray1<f64>> = Handle::default();
        if a_data_type == 1 {
            upper = 2 * nb_tuples;
        } else if a_data_type == 2 {
            upper = 3 * nb_tuples;
        } else {
            upper = 6 * nb_tuples;
        }
        all_data = iges_new_array1::<f64>(1, upper)?;
        if a_data_type == 1 {
            a_z_plane = another_live.z_plane();
        }
        for i in 1..=nb_tuples {
            match a_data_type {
                1 => {
                    iges_array1_set(
                        &all_data,
                        2 * i - 1,
                        another_live.data(i, 1)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        2 * i,
                        another_live.data(i, 2)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                }
                2 => {
                    iges_array1_set(
                        &all_data,
                        3 * i - 2,
                        another_live.data(i, 1)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        3 * i - 1,
                        another_live.data(i, 2)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        3 * i,
                        another_live.data(i, 3)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                }
                3 => {
                    iges_array1_set(
                        &all_data,
                        6 * i - 5,
                        another_live.data(i, 1)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        6 * i - 4,
                        another_live.data(i, 2)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        6 * i - 3,
                        another_live.data(i, 3)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        6 * i - 2,
                        another_live.data(i, 4)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        6 * i - 1,
                        another_live.data(i, 5)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                    iges_array1_set(
                        &all_data,
                        6 * i,
                        another_live.data(i, 6)?,
                        "IgesGeomToolCopiousData::own_copy",
                    )?;
                }
                _ => {}
            }
        }
        ent_live.init(a_data_type, a_z_plane, &all_data)?;
        if another_live.is_closed_path2d() {
            ent_live.set_closed_path2d();
        } else {
            ent_live.set_polyline(another_live.is_polyline());
        }
        Ok(())
    }
}
impl IgesGeomToolCopiousData {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGeomCopiousData>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesGeomToolCopiousData::own_dump")?;
        let nb_pnts: i32 = ent_live.nb_points()?;
        let dtype: i32 = ent_live.data_type();
        let loca: GeneralTransform3 = ent_live.location()?;
        let locv: GeneralTransform3 = ent_live.vector_location()?;
        let yatr: bool = level > 5 && ent_live.has_transf();
        writeln!(s, "IgesGeomCopiousData")?;
        if ent_live.is_point_set() {
            write!(s, "Point Set  ")?;
        } else if ent_live.is_polyline() {
            write!(s, "Polyline  ")?;
        } else if ent_live.is_closed_path2d() {
            write!(s, "Closed Path 2D  ")?;
        }
        write!(
            s,
            "DataType {}  Number of T-uples = {}  ",
            ent_live.data_type(),
            nb_pnts
        )?;
        if dtype == 1 {
            writeln!(
                s,
                "(Points 2D)  ZPlane = {}",
                iges_real_text(ent_live.z_plane())
            )?;
        } else if dtype == 2 {
            writeln!(s, "(Points 3D)")?;
        } else if dtype == 3 {
            writeln!(s, "(Points 3D + Vectors 3D)")?;
        }
        if level > 4 {
            for i in 1..=nb_pnts {
                let mut t: Xyz = Xyz::from(ent_live.point(i)?);
                if dtype == 1 {
                    write!(
                        s,
                        "[{}] ({},{})",
                        i,
                        iges_real_text(t.x()),
                        iges_real_text(t.y())
                    )?;
                } else {
                    write!(
                        s,
                        "[{}] ({},{},{})",
                        i,
                        iges_real_text(t.x()),
                        iges_real_text(t.y()),
                        iges_real_text(t.z())
                    )?;
                }
                if yatr {
                    loca.transforms(&mut t);
                    write!(
                        s,
                        " Transformed ({},{},{})",
                        iges_real_text(t.x()),
                        iges_real_text(t.y()),
                        iges_real_text(t.z())
                    )?;
                }
                if dtype == 3 {
                    write!(
                        s,
                        "\n   Vector ({},{},{})",
                        iges_real_text(t.x()),
                        iges_real_text(t.y()),
                        iges_real_text(t.z())
                    )?;
                    if yatr {
                        locv.transforms(&mut t);
                        write!(
                            s,
                            " Transformed ({},{},{})",
                            iges_real_text(t.x()),
                            iges_real_text(t.y()),
                            iges_real_text(t.z())
                        )?;
                    }
                }
                writeln!(s)?;
            }
        } else {
            write!(s, " [ for content, ask level > 4 ]")?;
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesGeomToolCurveOnSurface,
    "The own-parameter work on `IgesGeomCurveOnSurface`."
);
impl IgesGeomToolCurveOnSurface {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGeomCurveOnSurface>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::read_own_params")?;
        let mut a_mode: i32 = 0;
        let mut a_preference: i32 = 0;
        let mut a_surface: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_curve_uv: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_curve3d: Handle<IgesDataIgesEntity> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_integer(&pr.current(), &mut a_mode)? {
            let msg276 = iges_msg("XSTEP_276");
            pr.send_fail(&msg276);
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_surface, false)? {
            let mut msg131 = iges_msg("XSTEP_131");
            iges_send_read_status(pr, &mut msg131, a_status, false)?;
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_curve_uv, true)? {
            let mut msg132 = iges_msg("XSTEP_132");
            iges_send_read_status(pr, &mut msg132, a_status, false)?;
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut a_curve3d, true)? {
            let mut msg133 = iges_msg("XSTEP_133");
            iges_send_read_status(pr, &mut msg133, a_status, false)?;
        }
        if !pr.read_integer(&pr.current(), &mut a_preference)? {
            let msg277 = iges_msg("XSTEP_277");
            pr.send_fail(&msg277);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(a_mode, &a_surface, &a_curve_uv, &a_curve3d, a_preference);
        Ok(())
    }
}
impl IgesGeomToolCurveOnSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGeomCurveOnSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::write_own_params")?;
        iw.send(ent_live.creation_mode())?;
        iw.send_entity(&ent_live.surface(), false)?;
        iw.send_entity(&ent_live.curve_uv(), false)?;
        iw.send_entity(&ent_live.curve3d(), false)?;
        iw.send(ent_live.preference_mode())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGeomCurveOnSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesGeomToolCurveOnSurface::own_shared")?;
        iter.get_one_item(&ent_live.surface().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.curve_uv().cast::<StandardTransient>());
        iter.get_one_item(&ent_live.curve3d().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesGeomCurveOnSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(142, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(5);
        dc.hierarchy_status_ignored();
        dc
    }
}
