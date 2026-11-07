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
    IgesSolidFace, IgesSolidShell, IgesSolidSolidAssembly, IgesSolidSolidInstance,
    IgesSolidSolidOfLinearExtrusion, IgesSolidToolShell,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_live, iges_new_array1,
};
use crate::iges_geom::IgesGeomTransformationMatrix;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesSolidToolShell {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidShell>,
        ent: &Handle<IgesSolidShell>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolShell::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolShell::own_copy")?;
        let nbfaces: i32 = another_live.nb_faces()?;
        let temp_faces: Handle<HArray1<Handle<IgesSolidFace>>> =
            iges_new_array1::<Handle<IgesSolidFace>>(1, nbfaces)?;
        let temp_orientation: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nbfaces)?;
        for i in 1..=nbfaces {
            let face = tc
                .transferred(&another_live.face(i)?.cast::<StandardTransient>())?
                .cast::<IgesSolidFace>();
            iges_array1_set(&temp_faces, i, face.clone(), "IgesSolidToolShell::own_copy")?;
            iges_array1_set(
                &temp_orientation,
                i,
                if another_live.orientation(i)? { 1 } else { 0 },
                "IgesSolidToolShell::own_copy",
            )?;
        }
        ent_live.init(&temp_faces, &temp_orientation)?;
        Ok(())
    }
}
impl IgesSolidToolShell {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidShell>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolShell::own_dump")?;
        writeln!(s, "IgesSolidShell")?;
        let upper: i32 = ent_live.nb_faces()?;
        let sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "Faces :\nOrientation flags : ")?;
        iges_dump_entities(s, dumper, -level, 1, ent_live.nb_faces()?, |first| {
            ent_live.face(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            writeln!(s, "[")?;
            for i in 1..=upper {
                write!(s, "[{}]:  Face : ", i)?;
                dumper.dump(&ent_live.face(i)?, s, sublevel, -1)?;
                write!(s, "  - Orientation flag : ")?;
                if ent_live.orientation(i)? {
                    writeln!(s, "True")?;
                } else {
                    writeln!(s, "False")?;
                }
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSolidAssembly,
    "The own-parameter work on `IgesSolidSolidAssembly`."
);
impl IgesSolidToolSolidAssembly {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSolidAssembly>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidAssembly::read_own_params")?;
        let mut nbitems: i32 = 0;
        let mut temp_items: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut temp_matrices: Handle<HArray1<Handle<IgesGeomTransformationMatrix>>> =
            Handle::default();
        let st: bool = pr.read_integer_text(&pr.current(), "Number of Items", &mut nbitems)?;
        if st && nbitems > 0 {
            temp_items = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbitems)?;
            temp_matrices = iges_new_array1::<Handle<IgesGeomTransformationMatrix>>(1, nbitems)?;
            let mut anent: Handle<IgesDataIgesEntity> = Handle::default();
            for i in 1..=nbitems {
                if pr.read_entity(ir, &pr.current(), "Solid assembly items", &mut anent, false)? {
                    iges_array1_set(
                        &temp_items,
                        i,
                        anent.clone(),
                        "IgesSolidToolSolidAssembly::read_own_params",
                    )?;
                }
            }
            let mut amatr: Handle<IgesGeomTransformationMatrix> = Handle::default();
            for i in 1..=nbitems {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Matrices",
                    IgesGeomTransformationMatrix::static_type(),
                    &mut amatr,
                    true,
                )? {
                    iges_array1_set(
                        &temp_matrices,
                        i,
                        amatr.clone(),
                        "IgesSolidToolSolidAssembly::read_own_params",
                    )?;
                }
            }
        } else {
            pr.add_fail("Number of Items : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_items, &temp_matrices)?;
        Ok(())
    }
}
impl IgesSolidToolSolidAssembly {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSolidAssembly>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidAssembly::write_own_params")?;
        let nbitems: i32 = ent_live.nb_items()?;
        iw.send(nbitems)?;
        for i in 1..=nbitems {
            iw.send_entity(&ent_live.item(i)?, false)?;
        }
        for i in 1..=nbitems {
            iw.send_entity(&ent_live.transf_matrix(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidSolidAssembly>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidAssembly::own_shared")?;
        let nbitems: i32 = ent_live.nb_items()?;
        for i in 1..=nbitems {
            iter.get_one_item(&ent_live.item(i)?.cast::<StandardTransient>());
        }
        for i in 1..=nbitems {
            iter.get_one_item(&ent_live.transf_matrix(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSolidAssembly>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(184, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(2);
        dc.graphics_ignored(1);
        dc
    }
}
impl IgesSolidToolSolidAssembly {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesSolidSolidAssembly>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesSolidToolSolidAssembly {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSolidAssembly>,
        ent: &Handle<IgesSolidSolidAssembly>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSolidAssembly::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSolidAssembly::own_copy")?;
        let nbitems: i32 = another_live.nb_items()?;
        let temp_items: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbitems)?;
        let temp_matrices: Handle<HArray1<Handle<IgesGeomTransformationMatrix>>> =
            iges_new_array1::<Handle<IgesGeomTransformationMatrix>>(1, nbitems)?;
        for i in 1..=nbitems {
            let localent = tc
                .transferred(&another_live.item(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &temp_items,
                i,
                localent.clone(),
                "IgesSolidToolSolidAssembly::own_copy",
            )?;
        }
        for i in 1..=nbitems {
            let newlocalent = tc
                .transferred(&another_live.transf_matrix(i)?.cast::<StandardTransient>())?
                .cast::<IgesGeomTransformationMatrix>();
            iges_array1_set(
                &temp_matrices,
                i,
                newlocalent.clone(),
                "IgesSolidToolSolidAssembly::own_copy",
            )?;
        }
        ent_live.init(&temp_items, &temp_matrices)?;
        Ok(())
    }
}
impl IgesSolidToolSolidAssembly {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSolidAssembly>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSolidAssembly::own_dump")?;
        write!(s, "IgesSolidSolidAssembly\nItems : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_items()?, |first| {
            ent_live.item(first)
        })?;
        write!(s, "\nMatrices : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_items()?, |first| {
            ent_live.transf_matrix(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSolidInstance,
    "The own-parameter work on `IgesSolidSolidInstance`."
);
impl IgesSolidToolSolidInstance {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSolidInstance>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidInstance::read_own_params")?;
        let mut temp_entity: Handle<IgesDataIgesEntity> = Handle::default();
        pr.read_entity(ir, &pr.current(), "Solid Entity", &mut temp_entity, false)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_entity);
        Ok(())
    }
}
impl IgesSolidToolSolidInstance {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidSolidInstance>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidInstance::write_own_params")?;
        iw.send_entity(&ent_live.entity(), false)?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidSolidInstance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidInstance::own_shared")?;
        iter.get_one_item(&ent_live.entity().cast::<StandardTransient>());
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidSolidInstance>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_forms(430, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.graphics_ignored(1);
        dc
    }
}
impl IgesSolidToolSolidInstance {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        _ent: &Handle<IgesSolidSolidInstance>,
        _shares: &InterfaceShareTool,
        _ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
impl IgesSolidToolSolidInstance {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidSolidInstance>,
        ent: &Handle<IgesSolidSolidInstance>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolSolidInstance::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolSolidInstance::own_copy")?;
        let temp_entity = tc
            .transferred(&another_live.entity().cast::<StandardTransient>())?
            .cast::<IgesDataIgesEntity>();
        ent_live.init(&temp_entity);
        Ok(())
    }
}
impl IgesSolidToolSolidInstance {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidSolidInstance>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolSolidInstance::own_dump")?;
        write!(s, "IgesSolidSolidInstance\nSolid entity : ")?;
        dumper.dump(&ent_live.entity(), s, if level <= 4 { 0 } else { 1 }, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolSolidOfLinearExtrusion,
    "The own-parameter work on `IgesSolidSolidOfLinearExtrusion`."
);
impl IgesSolidToolSolidOfLinearExtrusion {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidSolidOfLinearExtrusion>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolSolidOfLinearExtrusion::read_own_params")?;
        let mut temp_entity: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_direction: Xyz = Xyz::default();
        let mut temp_length: f64 = 0.0;
        let mut tempreal: f64 = 0.0;
        pr.read_entity(ir, &pr.current(), "Curve Entity", &mut temp_entity, false)?;
        pr.read_real_text(&pr.current(), "Length of extrusion", &mut temp_length)?;
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Extrusion direction (I)", &mut tempreal)? {
                temp_direction.set_x(tempreal);
            }
        } else {
            temp_direction.set_x(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Extrusion direction (J)", &mut tempreal)? {
                temp_direction.set_y(tempreal);
            }
        } else {
            temp_direction.set_y(0.0);
        }
        if pr.defined_else_skip()? {
            if pr.read_real_text(&pr.current(), "Extrusion direction (K)", &mut tempreal)? {
                temp_direction.set_z(tempreal);
            }
        } else {
            temp_direction.set_z(1.0);
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_entity, temp_length, temp_direction);
        let eps: f64 = 1.0e-05;
        if !temp_direction.is_equal(&Xyz::from(ent_live.extrusion_direction()?), eps) {
            pr.add_warning("Extrusion Direction poorly unitary, normalized", "");
        }
        Ok(())
    }
}
