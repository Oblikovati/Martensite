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
    IgesSolidBooleanTree, IgesSolidFace, IgesSolidRightAngularWedge, IgesSolidSelectedComponent,
    IgesSolidShell, IgesSolidToolRightAngularWedge,
};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{
    iges_shared_004, iges_shared_017, iges_shared_025, iges_shared_040, iges_shared_043,
    iges_shared_046, iges_shared_052,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_xyz, iges_dump_xyz_l, iges_live, iges_msg,
    iges_new_array1, iges_real_text,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolRightAngularWedge {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidRightAngularWedge>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolRightAngularWedge::read_own_params")?;
        iges_shared_040!(temp_corner, temp_size, temp_x_axis, temp_z_axis);
        let mut low_x: f64 = 0.0;
        let mut tempreal: f64 = 0.0;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Size of RightAngularWedge",
            &mut temp_size,
        )?;
        pr.read_real_text(&pr.current(), "Small X length", &mut low_x)?;
        iges_shared_025!(
            ent,
            pr,
            self,
            temp_corner,
            temp_x_axis,
            temp_z_axis,
            tempreal
        );
        ent_live.init(temp_size, low_x, temp_corner, temp_x_axis, temp_z_axis);
        iges_shared_017!(ent_live, eps, pr, temp_x_axis, temp_z_axis);
        Ok(())
    }
}
impl IgesSolidToolRightAngularWedge {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidRightAngularWedge>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolRightAngularWedge::write_own_params")?;
        iges_shared_046!(ent_live, iw);
        iw.send(ent_live.x_small_length())?;
        iges_shared_004!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidRightAngularWedge>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidRightAngularWedge>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(152, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolRightAngularWedge {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidRightAngularWedge>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolRightAngularWedge::own_check")?;
        iges_shared_043!(ach, ent_live, eps, prosca);
        if ent_live.size().x() <= 0.0 || ent_live.size().y() <= 0.0 || ent_live.size().z() <= 0.0 {
            ach.add_fail_text("Size : Values are not positive", "");
        }
        if ent_live.x_small_length() <= 0.0 {
            ach.add_fail_text("Small X Length : Not Positive", "");
        }
        if ent_live.x_small_length() >= ent_live.size().x() {
            ach.add_fail_text("Small X Length : Value not < LX", "");
        }
        Ok(())
    }
}
impl IgesSolidToolRightAngularWedge {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidRightAngularWedge>,
        ent: &Handle<IgesSolidRightAngularWedge>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolRightAngularWedge::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolRightAngularWedge::own_copy")?;
        ent_live.init(
            another_live.size(),
            another_live.x_small_length(),
            Xyz::from(another_live.corner()),
            Xyz::from(another_live.x_axis()?),
            Xyz::from(another_live.z_axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolRightAngularWedge {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidRightAngularWedge>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolRightAngularWedge::own_dump")?;
        write!(s, "IgesSolidRightAngularWedge\nSize   : ")?;
        iges_dump_xyz(s, &ent_live.size())?;
        write!(s, "  ")?;
        write!(
            s,
            "XSmall : {}\nCorner : ",
            iges_real_text(ent_live.x_small_length())
        )?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.corner()),
            &ent_live.location()?,
        )?;
        write!(s, "\nXAxis : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.x_axis()?),
            &ent_live.vector_location()?,
        )?;
        write!(s, "\nZAxis : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.z_axis()?),
            &ent_live.vector_location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSelectedComponent,
    "The own-parameter work on `IgesSolidSelectedComponent`."
);
impl IgesSolidToolSelectedComponent {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSelectedComponent>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSelectedComponent::read_own_params")?;
        let mut temp_entity: Handle<IgesSolidBooleanTree> = Handle::default();
        let mut temp_select_point: Xyz = Xyz::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Boolean Tree Entity",
            IgesSolidBooleanTree::static_type(),
            &mut temp_entity,
            false,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Select Point",
            &mut temp_select_point,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_entity, temp_select_point);
        Ok(())
    }
}
impl IgesSolidToolSelectedComponent {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSelectedComponent>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSelectedComponent::write_own_params")?;
        iw.send_entity(&ent_live.component(), false)?;
        iw.send(ent_live.select_point().x())?;
        iw.send(ent_live.select_point().y())?;
        iw.send(ent_live.select_point().z())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidSelectedComponent>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSelectedComponent::own_shared")?;
        iter.get_one_item(&ent_live.component().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSelectedComponent>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(182, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefVoid);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.use_flag_required(3);
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesSolidToolSelectedComponent {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesSolidSelectedComponent>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesSolidToolSelectedComponent {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSelectedComponent>,
        ent: &Handle<IgesSolidSelectedComponent>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSelectedComponent::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSelectedComponent::own_copy")?;
        let temp_entity = tc
            .transferred(&another_live.component().cast::<StandardTransient>())?
            .cast::<IgesSolidBooleanTree>();
        let temp_select_point: Xyz = Xyz::from(another_live.select_point());
        ent_live.init(&temp_entity, temp_select_point);
        Ok(())
    }
}
impl IgesSolidToolSelectedComponent {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSelectedComponent>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSelectedComponent::own_dump")?;
        writeln!(s, "IgesSolidSelectedComponent")?;
        writeln!(s, "Boolean Tree Entity :")?;
        dumper.dump(&ent_live.component(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        write!(s, "Selected Point       : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.select_point()),
            &ent_live.location()?,
        )?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolShell,
    "The own-parameter work on `IgesSolidShell`."
);
impl IgesSolidToolShell {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidShell>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolShell::read_own_params")?;
        let mut nbfaces: i32 = 0;
        let mut temp_faces: Handle<HArray1<Handle<IgesSolidFace>>> = Handle::default();
        let mut temp_orientation: Handle<HArray1<i32>> = Handle::default();
        let sb: bool = pr.read_integer(&pr.current(), &mut nbfaces)?;
        if sb && nbfaces > 0 {
            let msg180 = iges_msg("XSTEP_180");
            let mut abool: bool = false;
            let mut aface: Handle<IgesSolidFace> = Handle::default();
            temp_faces = iges_new_array1::<Handle<IgesSolidFace>>(1, nbfaces)?;
            temp_orientation = iges_new_array1::<i32>(1, nbfaces)?;
            let mut a_status: IgesDataStatus = IgesDataStatus::default();
            for i in 1..=nbfaces {
                if pr.read_entity_typed_status(
                    ir,
                    &pr.current(),
                    &mut a_status,
                    IgesSolidFace::static_type(),
                    &mut aface,
                    false,
                )? {
                    iges_array1_set(
                        &temp_faces,
                        i,
                        aface.clone(),
                        "IgesSolidToolShell::read_own_params",
                    )?;
                } else {
                    let mut msg201 = iges_msg("XSTEP_201");
                    iges_send_read_status(pr, &mut msg201, a_status, true)?;
                }
                if pr.read_boolean(&pr.current(), &msg180, &mut abool, true)? {
                    iges_array1_set(
                        &temp_orientation,
                        i,
                        if abool { 1 } else { 0 },
                        "IgesSolidToolShell::read_own_params",
                    )?;
                }
            }
        } else {
            let msg200 = iges_msg("XSTEP_200");
            pr.send_fail(&msg200);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_faces, &temp_orientation)?;
        Ok(())
    }
}
impl IgesSolidToolShell {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidShell>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolShell::write_own_params")?;
        let nbfaces: i32 = ent_live.nb_faces()?;
        iw.send(nbfaces)?;
        for i in 1..=nbfaces {
            iw.send_entity(&ent_live.face(i)?, false)?;
            iw.send_boolean(ent_live.orientation(i)?)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidShell>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolShell::own_shared")?;
        let nbfaces: i32 = ent_live.nb_faces()?;
        for i in 1..=nbfaces {
            iter.get_one_item(&ent_live.face(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidShell>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(514, 1, 2);
        iges_shared_052!(dc);
        dc
    }
}
impl IgesSolidToolShell {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidShell>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolShell::own_check")?;
        if ent_live.nb_faces()? <= 0 {
            let msg200 = iges_msg("XSTEP_200");
            ach.send_fail(&msg200);
        }
        Ok(())
    }
}
