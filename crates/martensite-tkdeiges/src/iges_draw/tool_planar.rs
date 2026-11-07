//! IgesDrawToolPlanar.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_dump_entities, iges_new_array1,
};
use crate::iges_draw::IgesDrawPlanar;
use crate::iges_geom::IgesGeomTransformationMatrix;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolPlanar,
    "The own-parameter work on `IgesDrawPlanar`."
);

impl IgesDrawToolPlanar {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawPlanar>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut st: bool = Default::default();
        let mut nbval: i32 = Default::default();
        let mut nb_matrices: i32 = Default::default();
        let mut transformation_matrix: Handle<IgesGeomTransformationMatrix> = Default::default();
        let mut entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        st = pr.read_integer_text(
            &pr.current(),
            "No. of Transformation matrices",
            &mut nb_matrices,
        )?;
        if nb_matrices != 1 {
            pr.add_fail("No. of Transformation matrices != 1", "");
        }
        st = pr.read_integer_text(&pr.current(), "No. of Entities in this plane", &mut nbval)?;
        if !st {
            nbval = 0;
        }
        if nbval <= 0 {
            pr.add_fail("No. of Entities in this plane : Not Positive", "");
        }
        st = pr.read_entity_typed(
            ir,
            &pr.current(),
            "Instance of TransformationMatrix",
            <IgesGeomTransformationMatrix>::static_type(),
            &mut transformation_matrix,
            true,
        )?;
        if nbval > 0 {
            st = pr.read_ents(
                ir,
                &pr.current_list(nbval, 1),
                "Planar Entities",
                &mut entities,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(nb_matrices, &transformation_matrix, &entities)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawPlanar>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_entities()?;
        iw.send(ent.nb_matrices())?;
        iw.send(up)?;
        iw.send_entity(&ent.transform_matrix(), false)?;
        for i in 1..=up {
            iw.send_entity(&ent.entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a Planar `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawPlanar>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_entities()?;
        iter.get_one_item(&ent.transform_matrix().cast());
        for i in 1..=up {
            iter.get_one_item(&ent.entity(i)?.cast());
        }
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a Planar
    /// (NbMatrices forced to 1)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDrawPlanar>) -> Result<bool, Failure> {
        if ent.nb_matrices() == 1 {
            return Ok(false);
        }
        let mut nb: i32 = ent.nb_entities()?;
        let mut ents: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(&ents, i, ent.entity(i)?, "IgesDrawToolPlanar::own_correct")?;
        }
        ent.init(1, &ent.transform_matrix(), &ents)?;
        return Ok(true);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawPlanar>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 16);
        crate::iges_shared_blocks::iges_block_0040!(dc);
        dc.use_flag_required(5);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawPlanar>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_matrices() != 1 {
            ach.add_fail_text("No. of Transformation matrices : Value != 1", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawPlanar>,
        ent: &Handle<IgesDrawPlanar>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut nb_matrices: i32 = Default::default();
        let mut entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        nbval = another.nb_entities()?;
        nb_matrices = another.nb_matrices();
        let mut transformation_matrix: Handle<IgesGeomTransformationMatrix> =
            tc.transferred(&another.transform_matrix().cast())?.cast();
        entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbval)?;
        for i in 1..=nbval {
            let mut temp_entity: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.entity(i)?.cast())?.cast();
            iges_array1_set(
                &entities,
                i,
                temp_entity.clone(),
                "IgesDrawToolPlanar::own_copy",
            )?;
        }
        ent.init(nb_matrices, &transformation_matrix, &entities)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawPlanar>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDrawPlanar\nNo. of Transformation Matrices : {}  i.e. : ",
            ent.nb_matrices()
        )?;
        if ent.transform_matrix().is_null() {
            write!(s, "Null Handle")?;
        } else {
            dumper.own_dump(&ent.transform_matrix(), s, sublevel)?;
        }
        write!(s, "\nArray of Entities on the specified plane : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_entities()?, |first| {
            ent.entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
