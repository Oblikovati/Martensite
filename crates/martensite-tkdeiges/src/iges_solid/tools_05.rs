#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidEdgeList, IgesSolidLoop, IgesSolidToolLoop, IgesSolidVertexList};
use crate::iges_basic::entity_shape::{iges_is_kind, iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::iges_shared_052;
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicHArray1OfHArray1OfInteger,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_live, iges_msg, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolLoop {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidLoop>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolLoop::read_own_params")?;
        let mut abool: bool = false;
        let mut nbedges: i32 = 0;
        let mut anint: i32 = 0;
        let mut anent: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_types: Handle<HArray1<i32>> = Handle::default();
        let mut temp_edges: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut temp_index: Handle<HArray1<i32>> = Handle::default();
        let mut temp_orientation: Handle<HArray1<i32>> = Handle::default();
        let mut nb_parameter_curves: Handle<HArray1<i32>> = Handle::default();
        let mut isoparametric_flags: Handle<IgesBasicHArray1OfHArray1OfInteger> = Handle::default();
        let mut temp_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        let sb: bool = pr.read_integer(&pr.current(), &mut nbedges)?;
        if sb && nbedges > 0 {
            let msg180 = iges_msg("XSTEP_180");
            let msg190 = iges_msg("XSTEP_190");
            temp_types = iges_new_array1::<i32>(1, nbedges)?;
            temp_edges = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbedges)?;
            temp_index = iges_new_array1::<i32>(1, nbedges)?;
            temp_orientation = iges_new_array1::<i32>(1, nbedges)?;
            nb_parameter_curves = iges_new_array1::<i32>(1, nbedges)?;
            isoparametric_flags =
                Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbedges)?);
            temp_curves = Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(
                1, nbedges,
            )?);
            for i in 1..=nbedges {
                if pr.read_integer(&pr.current(), &mut anint)? {
                    iges_array1_set(&temp_types, i, anint, "IgesSolidToolLoop::read_own_params")?;
                } else {
                    pr.send_fail(&msg190);
                }
                if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut anent, false)? {
                    let mut msg193 = iges_msg("XSTEP_193");
                    iges_send_read_status(pr, &mut msg193, a_status, false)?;
                }
                if !iges_is_kind(
                    &anent,
                    IgesSolidVertexList::static_type(),
                    "IgesSolidToolLoop::read_own_params",
                )? && !iges_is_kind(
                    &anent,
                    IgesSolidEdgeList::static_type(),
                    "IgesSolidToolLoop::read_own_params",
                )? {
                    pr.send_fail(&msg190);
                } else {
                    iges_array1_set(
                        &temp_edges,
                        i,
                        anent.clone(),
                        "IgesSolidToolLoop::read_own_params",
                    )?;
                }
                if pr.read_integer(&pr.current(), &mut anint)? {
                    iges_array1_set(&temp_index, i, anint, "IgesSolidToolLoop::read_own_params")?;
                } else {
                    let msg191 = iges_msg("XSTEP_191");
                    pr.send_fail(&msg191);
                }
                if pr.read_boolean(&pr.current(), &msg180, &mut abool, true)? {
                    iges_array1_set(
                        &temp_orientation,
                        i,
                        if abool { 1 } else { 0 },
                        "IgesSolidToolLoop::read_own_params",
                    )?;
                }
                let st: bool = pr.read_integer(&pr.current(), &mut anint)?;
                if !st {
                    let msg192 = iges_msg("XSTEP_192");
                    pr.send_fail(&msg192);
                }
                if st && anint > 0 {
                    let msg195 = iges_msg("XSTEP_195");
                    iges_array1_set(
                        &nb_parameter_curves,
                        i,
                        anint,
                        "IgesSolidToolLoop::read_own_params",
                    )?;
                    let tmpents: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
                        iges_new_array1::<Handle<IgesDataIgesEntity>>(1, anint)?;
                    let tmpints: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, anint)?;
                    for j in 1..=anint {
                        if pr.read_boolean(&pr.current(), &msg195, &mut abool, true)? {
                            iges_array1_set(
                                &tmpints,
                                j,
                                if abool { 1 } else { 0 },
                                "IgesSolidToolLoop::read_own_params",
                            )?;
                        }
                        if pr.read_entity_status(
                            ir,
                            &pr.current(),
                            &mut a_status,
                            &mut anent,
                            false,
                        )? {
                            iges_array1_set(
                                &tmpents,
                                j,
                                anent.clone(),
                                "IgesSolidToolLoop::read_own_params",
                            )?;
                        } else {
                            let mut msg194 = iges_msg("XSTEP_194");
                            iges_send_read_status(pr, &mut msg194, a_status, false)?;
                        }
                    }
                    iges_live(&isoparametric_flags, "IgesSolidToolLoop::read_own_params")?
                        .set_value(i, &tmpints)?;
                    iges_live(&temp_curves, "IgesSolidToolLoop::read_own_params")?
                        .set_value(i, &tmpents)?;
                } else {
                    iges_array1_set(
                        &nb_parameter_curves,
                        i,
                        anint,
                        "IgesSolidToolLoop::read_own_params",
                    )?;
                }
            }
        } else {
            let msg184 = iges_msg("XSTEP_184");
            pr.send_fail(&msg184);
            return Ok(());
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            &temp_types,
            &temp_edges,
            &temp_index,
            &temp_orientation,
            &nb_parameter_curves,
            &isoparametric_flags,
            &temp_curves,
        )?;
        Ok(())
    }
}
impl IgesSolidToolLoop {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidLoop>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolLoop::write_own_params")?;
        let length: i32 = ent_live.nb_edges();
        iw.send(length)?;
        for i in 1..=length {
            iw.send(ent_live.edge_type(i)?)?;
            iw.send_entity(&ent_live.edge(i)?, false)?;
            iw.send(ent_live.list_index(i)?)?;
            iw.send_boolean(ent_live.orientation(i)?)?;
            iw.send(ent_live.nb_parameter_curves(i)?)?;
            for j in 1..=ent_live.nb_parameter_curves(i)? {
                iw.send_boolean(ent_live.is_isoparametric(i, j)?)?;
                iw.send_entity(&ent_live.parametric_curve(i, j)?, false)?;
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
        ent: &Handle<IgesSolidLoop>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolLoop::own_shared")?;
        let length: i32 = ent_live.nb_edges();
        for i in 1..=length {
            iter.get_one_item(&ent_live.edge(i)?.cast::<StandardTransient>());
            for j in 1..=ent_live.nb_parameter_curves(i)? {
                iter.get_one_item(&ent_live.parametric_curve(i, j)?.cast::<StandardTransient>());
            }
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidLoop>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(508, 0, 1);
        iges_shared_052!(dc);
        dc
    }
}
impl IgesSolidToolLoop {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidLoop>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolLoop::own_check")?;
        let upper: i32 = ent_live.nb_edges();
        for i in 1..=upper {
            if ent_live.edge_type(i)? != 0 && ent_live.edge_type(i)? != 1 {
                let msg190 = iges_msg("XSTEP_190");
                ach.send_fail(&msg190);
            }
        }
        Ok(())
    }
}
impl IgesSolidToolLoop {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidLoop>,
        ent: &Handle<IgesSolidLoop>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolLoop::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolLoop::own_copy")?;
        let nbedges: i32 = another_live.nb_edges();
        let mut anint: i32 = 0;
        let temp_types: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nbedges)?;
        let temp_edges: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbedges)?;
        let temp_index: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nbedges)?;
        let temp_orientation: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nbedges)?;
        let nb_parameter_curves: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nbedges)?;
        let isoparametric_flags: Handle<IgesBasicHArray1OfHArray1OfInteger> =
            Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbedges)?);
        let temp_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> = Handle::new(
            IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, nbedges)?,
        );
        for i in 1..=nbedges {
            iges_array1_set(
                &temp_types,
                i,
                another_live.edge_type(i)?,
                "IgesSolidToolLoop::own_copy",
            )?;
            let anent = tc
                .transferred(&another_live.edge(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(&temp_edges, i, anent.clone(), "IgesSolidToolLoop::own_copy")?;
            iges_array1_set(
                &temp_index,
                i,
                another_live.list_index(i)?,
                "IgesSolidToolLoop::own_copy",
            )?;
            iges_array1_set(
                &temp_orientation,
                i,
                if another_live.orientation(i)? { 1 } else { 0 },
                "IgesSolidToolLoop::own_copy",
            )?;
            anint = another_live.nb_parameter_curves(i)?;
            iges_array1_set(
                &nb_parameter_curves,
                i,
                anint,
                "IgesSolidToolLoop::own_copy",
            )?;
            let mut tmpents: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
            if anint > 0 {
                tmpents = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, anint)?;
            }
            let mut tmpints: Handle<HArray1<i32>> = Handle::default();
            if anint > 0 {
                tmpints = iges_new_array1::<i32>(1, anint)?;
            }
            for j in 1..=anint {
                iges_array1_set(
                    &tmpints,
                    j,
                    if another_live.is_isoparametric(i, j)? {
                        1
                    } else {
                        0
                    },
                    "IgesSolidToolLoop::own_copy",
                )?;
                let localent = tc
                    .transferred(
                        &another_live
                            .parametric_curve(i, j)?
                            .cast::<StandardTransient>(),
                    )?
                    .cast::<IgesDataIgesEntity>();
                iges_array1_set(&tmpents, j, localent.clone(), "IgesSolidToolLoop::own_copy")?;
            }
            iges_live(&isoparametric_flags, "IgesSolidToolLoop::own_copy")?
                .set_value(i, &tmpints)?;
            iges_live(&temp_curves, "IgesSolidToolLoop::own_copy")?.set_value(i, &tmpents)?;
        }
        ent_live.init(
            &temp_types,
            &temp_edges,
            &temp_index,
            &temp_orientation,
            &nb_parameter_curves,
            &isoparametric_flags,
            &temp_curves,
        )?;
        Ok(())
    }
}
impl IgesSolidToolLoop {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidLoop>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolLoop::own_dump")?;
        let nbedges: i32 = ent_live.nb_edges();
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesSolidLoop\nEdge types :\nEdges      :\nList index :\nOrientation flags :\nParametric flags  : "
        )?;
        iges_dump_entities(s, dumper, -level, 1, nbedges, |first| ent_live.edge(first))?;
        writeln!(s)?;
        if level > 4 {
            write!(s, "[ ")?;
            for i in 1..=nbedges {
                let nbc: i32 = ent_live.nb_parameter_curves(i)?;
                write!(
                    s,
                    "[{}]:  Edge type : {}  Edge : ",
                    i,
                    ent_live.edge_type(i)?
                )?;
                dumper.dump(&ent_live.edge(i)?, s, sublevel, -1)?;
                write!(
                    s,
                    "  - Index : {}, Orientation flag : {}, Number of parametric curves : {}",
                    ent_live.list_index(i)?,
                    if ent_live.orientation(i)? {
                        "Positive"
                    } else {
                        "Negative"
                    },
                    nbc
                )?;
                if nbc != 0 {
                    if level <= 5 {
                        write!(s, "[ ask level > 5 for content ]")?;
                    } else {
                        write!(s, ":\n [ ")?;
                        for j in 1..=nbc {
                            write!(
                                s,
                                "[{}]:  Isoparametric flag : {}  Parametric curve : ",
                                j,
                                if ent_live.is_isoparametric(i, j)? {
                                    "True"
                                } else {
                                    "False"
                                }
                            )?;
                            dumper.dump(&ent_live.parametric_curve(i, j)?, s, sublevel, -1)?;
                            writeln!(s)?;
                        }
                        write!(s, " ]")?;
                    }
                }
                writeln!(s)?;
            }
            write!(s, " ]")?;
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolManifoldSolid,
    "The own-parameter work on `IgesSolidManifoldSolid`."
);
