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
    IgesSolidCylindricalSurface, IgesSolidEdgeList, IgesSolidToolCylindricalSurface,
    IgesSolidVertexList,
};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{
    iges_shared_013, iges_shared_014, iges_shared_028, iges_shared_035, iges_shared_042,
    iges_shared_052,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_vals, iges_live, iges_msg, iges_new_array1,
    iges_real_text,
};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolCylindricalSurface {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidCylindricalSurface>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::write_own_params")?;
        iw.send_entity(&ent_live.location_point(), false)?;
        iw.send_entity(&ent_live.axis(), false)?;
        iw.send(ent_live.radius())?;
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
        ent: &Handle<IgesSolidCylindricalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::own_shared")?;
        iges_shared_028!(ent_live, iter);
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidCylindricalSurface>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(192, 0, 1);
        iges_shared_042!(dc);
        dc
    }
}
impl IgesSolidToolCylindricalSurface {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidCylindricalSurface>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::own_check")?;
        if ent_live.radius() <= 0.0 {
            ach.add_fail_text("Radius : Value <= 0.0", "");
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
impl IgesSolidToolCylindricalSurface {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidCylindricalSurface>,
        ent: &Handle<IgesSolidCylindricalSurface>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolCylindricalSurface::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::own_copy")?;
        let mut temp_radius: f64 = 0.0;
        iges_shared_014!(another_live, tc, temp_axis, temp_location, temp_radius);
        if another_live.is_parametrised() {
            let temp_refdir = tc
                .transferred(&another_live.reference_dir().cast::<StandardTransient>())?
                .cast::<IgesGeomDirection>();
            ent_live.init(&temp_location, &temp_axis, temp_radius, &temp_refdir);
        } else {
            let temp_refdir: Handle<IgesGeomDirection> = Handle::default();
            ent_live.init(&temp_location, &temp_axis, temp_radius, &temp_refdir);
        }
        Ok(())
    }
}
impl IgesSolidToolCylindricalSurface {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidCylindricalSurface>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolCylindricalSurface::own_dump")?;
        writeln!(s, "IgesSolidCylindricalSurface")?;
        iges_shared_013!(dumper, ent_live, level, s, sublevel);
        write!(
            s,
            "\nRadius         : {}\n",
            iges_real_text(ent_live.radius())
        )?;
        iges_shared_035!(dumper, ent_live, s, sublevel);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolEdgeList,
    "The own-parameter work on `IgesSolidEdgeList`."
);
impl IgesSolidToolEdgeList {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidEdgeList>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::read_own_params")?;
        let mut length: i32 = 0;
        let mut anint: i32 = 0;
        let mut anent: Handle<IgesDataIgesEntity> = Handle::default();
        let mut avert: Handle<IgesSolidVertexList> = Handle::default();
        let mut temp_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut temp_start_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> =
            Handle::default();
        let mut temp_start_vertex_index: Handle<HArray1<i32>> = Handle::default();
        let mut temp_end_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> =
            Handle::default();
        let mut temp_end_vertex_index: Handle<HArray1<i32>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        let st: bool = pr.read_integer(&pr.current(), &mut length)?;
        if !st {
            let msg184 = iges_msg("XSTEP_184");
            pr.send_fail(&msg184);
        }
        if st && length > 0 {
            temp_curves = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, length)?;
            temp_start_vertex_list = iges_new_array1::<Handle<IgesSolidVertexList>>(1, length)?;
            temp_start_vertex_index = iges_new_array1::<i32>(1, length)?;
            temp_end_vertex_list = iges_new_array1::<Handle<IgesSolidVertexList>>(1, length)?;
            temp_end_vertex_index = iges_new_array1::<i32>(1, length)?;
            for i in 1..=length {
                if pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut anent, false)? {
                    iges_array1_set(
                        &temp_curves,
                        i,
                        anent.clone(),
                        "IgesSolidToolEdgeList::read_own_params",
                    )?;
                } else {
                    let mut msg185 = iges_msg("XSTEP_185");
                    iges_send_read_status(pr, &mut msg185, a_status, false)?;
                }
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesSolidVertexList::static_type(),
                    &mut avert,
                    false,
                )? {
                    iges_array1_set(
                        &temp_start_vertex_list,
                        i,
                        avert.clone(),
                        "IgesSolidToolEdgeList::read_own_params",
                    )?;
                } else {
                    let mut msg188 = iges_msg("XSTEP_188");
                    iges_send_read_status(pr, &mut msg188, a_status, true)?;
                }
                if pr.read_integer(&pr.current(), &mut anint)? {
                    iges_array1_set(
                        &temp_start_vertex_index,
                        i,
                        anint,
                        "IgesSolidToolEdgeList::read_own_params",
                    )?;
                } else {
                    let msg186 = iges_msg("XSTEP_186");
                    pr.send_fail(&msg186);
                }
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesSolidVertexList::static_type(),
                    &mut avert,
                    false,
                )? {
                    iges_array1_set(
                        &temp_end_vertex_list,
                        i,
                        avert.clone(),
                        "IgesSolidToolEdgeList::read_own_params",
                    )?;
                } else {
                    let mut msg189 = iges_msg("XSTEP_189");
                    iges_send_read_status(pr, &mut msg189, a_status, true)?;
                }
                if pr.read_integer(&pr.current(), &mut anint)? {
                    iges_array1_set(
                        &temp_end_vertex_index,
                        i,
                        anint,
                        "IgesSolidToolEdgeList::read_own_params",
                    )?;
                } else {
                    let msg187 = iges_msg("XSTEP_187");
                    pr.send_fail(&msg187);
                }
            }
        }
        if st && length <= 0 {
            let msg184 = iges_msg("XSTEP_184");
            pr.send_fail(&msg184);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        if length > 0 {
            ent_live.init(
                &temp_curves,
                &temp_start_vertex_list,
                &temp_start_vertex_index,
                &temp_end_vertex_list,
                &temp_end_vertex_index,
            )?;
        }
        Ok(())
    }
}
impl IgesSolidToolEdgeList {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidEdgeList>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::write_own_params")?;
        let length: i32 = ent_live.nb_edges();
        iw.send(length)?;
        for i in 1..=length {
            iw.send_entity(&ent_live.curve(i)?, false)?;
            iw.send_entity(&ent_live.start_vertex_list(i)?, false)?;
            iw.send(ent_live.start_vertex_index(i)?)?;
            iw.send_entity(&ent_live.end_vertex_list(i)?, false)?;
            iw.send(ent_live.end_vertex_index(i)?)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidEdgeList>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::own_shared")?;
        let length: i32 = ent_live.nb_edges();
        for i in 1..=length {
            iter.get_one_item(&ent_live.curve(i)?.cast::<StandardTransient>());
            iter.get_one_item(&ent_live.start_vertex_list(i)?.cast::<StandardTransient>());
            iter.get_one_item(&ent_live.end_vertex_list(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidEdgeList>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(504, 1);
        iges_shared_052!(dc);
        dc.hierarchy_status_required(1);
        dc
    }
}
impl IgesSolidToolEdgeList {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidEdgeList>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::own_check")?;
        if ent_live.nb_edges() <= 0 {
            let msg184 = iges_msg("XSTEP_184");
            ach.send_fail(&msg184);
        }
        Ok(())
    }
}
impl IgesSolidToolEdgeList {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidEdgeList>,
        ent: &Handle<IgesSolidEdgeList>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolEdgeList::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::own_copy")?;
        let length: i32 = another_live.nb_edges();
        let temp_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, length)?;
        let temp_start_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> =
            iges_new_array1::<Handle<IgesSolidVertexList>>(1, length)?;
        let temp_start_vertex_index: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, length)?;
        let temp_end_vertex_list: Handle<HArray1<Handle<IgesSolidVertexList>>> =
            iges_new_array1::<Handle<IgesSolidVertexList>>(1, length)?;
        let temp_end_vertex_index: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, length)?;
        for i in 1..=length {
            let curve = tc
                .transferred(&another_live.curve(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &temp_curves,
                i,
                curve.clone(),
                "IgesSolidToolEdgeList::own_copy",
            )?;
            let start = tc
                .transferred(
                    &another_live
                        .start_vertex_list(i)?
                        .cast::<StandardTransient>(),
                )?
                .cast::<IgesSolidVertexList>();
            iges_array1_set(
                &temp_start_vertex_list,
                i,
                start.clone(),
                "IgesSolidToolEdgeList::own_copy",
            )?;
            iges_array1_set(
                &temp_start_vertex_index,
                i,
                another_live.start_vertex_index(i)?,
                "IgesSolidToolEdgeList::own_copy",
            )?;
            let end = tc
                .transferred(&another_live.end_vertex_list(i)?.cast::<StandardTransient>())?
                .cast::<IgesSolidVertexList>();
            iges_array1_set(
                &temp_end_vertex_list,
                i,
                end.clone(),
                "IgesSolidToolEdgeList::own_copy",
            )?;
            iges_array1_set(
                &temp_end_vertex_index,
                i,
                another_live.end_vertex_index(i)?,
                "IgesSolidToolEdgeList::own_copy",
            )?;
        }
        ent_live.init(
            &temp_curves,
            &temp_start_vertex_list,
            &temp_start_vertex_index,
            &temp_end_vertex_list,
            &temp_end_vertex_index,
        )?;
        Ok(())
    }
}
impl IgesSolidToolEdgeList {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidEdgeList>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolEdgeList::own_dump")?;
        let length: i32 = ent_live.nb_edges();
        write!(s, "IgesSolidEdgeList\nNumber of edge tuples : {}\n", length)?;
        match level {
            4 => {
                write!(s, "Curves : Start Vertex List : Start Vertex Index : ")?;
                iges_dump_vals(s, level, 1, length, |first| {
                    ent_live.start_vertex_index(first)
                })?;
                write!(s, "\nEnd Vertex List : End Vertex Index : ")?;
                iges_dump_vals(s, level, 1, length, |first| {
                    ent_live.end_vertex_index(first)
                })?;
                writeln!(s)?;
            }
            5 | 6 => {
                writeln!(
                    s,
                    " Curve - Vertices. Start : (VertexList,Index)  End : (VertexList,Index)"
                )?;
                for i in 1..=length {
                    write!(s, "[{}]:Curve : ", i)?;
                    dumper.dump(&ent_live.curve(i)?, s, level - 5, -1)?;
                    write!(s, " - Vertices. Start : (")?;
                    dumper.dump(&ent_live.start_vertex_list(i)?, s, 0, -1)?;
                    write!(s, " , {})  End : (", ent_live.start_vertex_index(i)?)?;
                    dumper.dump(&ent_live.end_vertex_list(i)?, s, 0, -1)?;
                    writeln!(s, " , {})", ent_live.end_vertex_index(i)?)?;
                }
            }
            _ => {}
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolEllipsoid,
    "The own-parameter work on `IgesSolidEllipsoid`."
);
