//! IgesDimenToolDimensionedGeometry.
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
use crate::iges_dimen::IgesDimenDimensionedGeometry;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolDimensionedGeometry,
    "The own-parameter work on `IgesDimenDimensionedGeometry`."
);

impl IgesDimenToolDimensionedGeometry {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_dimen: i32 = Default::default();
        let mut a_dim_entity: Handle<IgesDataIgesEntity> = Default::default();
        let mut nbgeom: i32 = 0;
        let mut geom_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        pr.read_integer_text(&pr.current(), "Number of Dimensions", &mut temp_nb_dimen)?;
        pr.read_integer_text(&pr.current(), "number of entities", &mut nbgeom)?;
        pr.read_entity(
            ir,
            &pr.current(),
            "Dimension Entity",
            &mut a_dim_entity,
            false,
        )?;
        if nbgeom > 0 {
            pr.read_ents(
                ir,
                &pr.current_list(nbgeom, 1),
                "Geometry Entities",
                &mut geom_entities,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_nb_dimen, &a_dim_entity, &geom_entities)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_dimensions())?;
        iw.send(ent.nb_geometry_entities()?)?;
        iw.send_entity(&ent.dimension_entity(), false)?;
        {
            let mut upper: i32 = ent.nb_geometry_entities()?;
            let mut i: i32 = 1;
            while i <= upper {
                iw.send_entity(&ent.geometry_entity(i)?, false)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a DimensionedGeometry `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimension_entity().cast());
        {
            let mut upper: i32 = ent.nb_geometry_entities()?;
            let mut i: i32 = 1;
            while i <= upper {
                iter.get_one_item(&ent.geometry_entity(i)?.cast());
                i += 1;
            }
        }
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DimensionedGeometry
    /// (NbDimensions forced to 1)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDimenDimensionedGeometry>) -> Result<bool, Failure> {
        if ent.nb_dimensions() == 1 {
            return Ok(false);
        }
        let mut nb: i32 = ent.nb_geometry_entities()?;
        let mut ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(
                &ent_array,
                i,
                ent.geometry_entity(i)?,
                "IgesDimenToolDimensionedGeometry::own_correct",
            )?;
        }
        ent.init(1, &ent.dimension_entity(), &ent_array)?;
        return Ok(true);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenDimensionedGeometry>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 13);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_dimensions() != 1 {
            ach.add_fail_text("NbDimensions != 1", "");
        }
        if ent.use_flag() > 3 {
            ach.add_fail_text("Incorrect UseFlag", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenDimensionedGeometry>,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nb_dim: i32 = another.nb_dimensions();
        let mut anentity: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.dimension_entity().cast())?.cast();
        let mut upper: i32 = another.nb_geometry_entities()?;
        let mut ent_array: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, upper)?;
        for i in 1..=upper {
            let mut myentity: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.geometry_entity(i)?.cast())?.cast();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesDimenToolDimensionedGeometry::own_copy",
            )?;
        }
        ent.init(nb_dim, &anentity, &ent_array)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenDimensionedGeometry>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDimenDimensionedGeometry")?;
        write!(
            s,
            "Number of Dimensions : {}\nDimension Entity : ",
            ent.nb_dimensions()
        )?;
        dumper.dump(
            &ent.dimension_entity(),
            s,
            if level <= 4 { 0 } else { 1 },
            -1,
        )?;
        write!(s, "\nGeometry Entities : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_geometry_entities()?, |first| {
            ent.geometry_entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
