#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidToolTorus, IgesSolidTorus, IgesSolidVertexList};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_006, iges_shared_015, iges_shared_051};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_list_xyz_l, iges_dump_xyz_l, iges_live,
    iges_msg, iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolTorus {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidTorus>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolTorus::write_own_params")?;
        iw.send(ent_live.major_radius())?;
        iw.send(ent_live.disc_radius())?;
        iges_shared_006!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidTorus>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidTorus>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(160, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolTorus {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidTorus>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolTorus::own_check")?;
        if ent_live.major_radius() <= 0.0 {
            ach.add_fail_text("Radius of revolution : Not Positive", "");
        }
        if ent_live.disc_radius() <= 0.0 {
            ach.add_fail_text("Radius of disc : Not Positive", "");
        }
        if ent_live.disc_radius() >= ent_live.major_radius() {
            ach.add_fail_text("Radius of disc : is not Less than Radius of revolution", "");
        }
        Ok(())
    }
}
impl IgesSolidToolTorus {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidTorus>,
        ent: &Handle<IgesSolidTorus>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolTorus::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolTorus::own_copy")?;
        ent_live.init(
            another_live.major_radius(),
            another_live.disc_radius(),
            Xyz::from(another_live.axis_point()),
            Xyz::from(another_live.axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolTorus {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidTorus>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolTorus::own_dump")?;
        write!(
            s,
            "IgesSolidTorus\nRadius of revolution : {}  Radius of the disc   : {}\nCenter Point   : ",
            iges_real_text(ent_live.major_radius()),
            iges_real_text(ent_live.disc_radius())
        )?;
        iges_shared_015!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolVertexList,
    "The own-parameter work on `IgesSolidVertexList`."
);
impl IgesSolidToolVertexList {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidVertexList>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolVertexList::read_own_params")?;
        let mut nbitems: i32 = 0;
        let mut temp_vertices: Handle<HArray1<Xyz>> = Handle::default();
        let sb: bool = pr.read_integer(&pr.current(), &mut nbitems)?;
        if sb && nbitems > 0 {
            let msg183 = iges_msg("XSTEP_183");
            temp_vertices = iges_new_array1::<Xyz>(1, nbitems)?;
            let mut an_xyz: Xyz = Xyz::default();
            for i in 1..=nbitems {
                if pr.read_xyz(&pr.current_list(1, 3), &msg183, &mut an_xyz)? {
                    iges_array1_set(
                        &temp_vertices,
                        i,
                        an_xyz,
                        "IgesSolidToolVertexList::read_own_params",
                    )?;
                }
            }
        } else {
            let msg182 = iges_msg("XSTEP_182");
            pr.send_fail(&msg182);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        if nbitems > 0 {
            ent_live.init(&temp_vertices)?;
        }
        Ok(())
    }
}
impl IgesSolidToolVertexList {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidVertexList>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolVertexList::write_own_params")?;
        let nbitems: i32 = ent_live.nb_vertices();
        iw.send(nbitems)?;
        for i in 1..=nbitems {
            iw.send(ent_live.vertex(i)?.x())?;
            iw.send(ent_live.vertex(i)?.y())?;
            iw.send(ent_live.vertex(i)?.z())?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidVertexList>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidVertexList>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(502, 1);
        iges_shared_051!(dc);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolVertexList {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidVertexList>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolVertexList::own_check")?;
        if ent_live.nb_vertices() <= 0 {
            let msg182 = iges_msg("XSTEP_182");
            ach.send_fail(&msg182);
        }
        Ok(())
    }
}
impl IgesSolidToolVertexList {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidVertexList>,
        ent: &Handle<IgesSolidVertexList>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolVertexList::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolVertexList::own_copy")?;
        let nbitems: i32 = another_live.nb_vertices();
        let temp_vertices: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(1, nbitems)?;
        for i in 1..=nbitems {
            iges_array1_set(
                &temp_vertices,
                i,
                Xyz::from(another_live.vertex(i)?),
                "IgesSolidToolVertexList::own_copy",
            )?;
        }
        ent_live.init(&temp_vertices)?;
        Ok(())
    }
}
impl IgesSolidToolVertexList {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidVertexList>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolVertexList::own_dump")?;
        write!(s, "IgesSolidVertexList\nVertices : ")?;
        iges_dump_list_xyz_l(
            s,
            level,
            1,
            ent_live.nb_vertices(),
            |first| Ok(Xyz::from(ent_live.vertex(first)?)),
            &ent_live.location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
