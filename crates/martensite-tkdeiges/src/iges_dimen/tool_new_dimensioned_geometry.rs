//! IgesDimenToolNewDimensionedGeometry.
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
    IgesDataTransfEntity, iges_array1_set, iges_dump_entities, iges_dump_xyz_l, iges_new_array1,
    iges_real_text,
};
use crate::iges_dimen::IgesDimenNewDimensionedGeometry;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolNewDimensionedGeometry,
    "The own-parameter work on `IgesDimenNewDimensionedGeometry`."
);

impl IgesDimenToolNewDimensionedGeometry {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        let mut temp_nb_dimens: i32 = Default::default();
        let mut temp_dim_orient_flag: i32 = Default::default();
        let mut temp_angle: f64 = Default::default();
        let mut temp_dimen: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_geom_ents: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut temp_dim_loc_flags: Handle<HArray1<i32>> = Default::default();
        let mut temp_points: Handle<HArray1<Xyz>> = Default::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Number of Dimensions", &mut temp_nb_dimens)?;
        } else {
            temp_nb_dimens = 1;
        }
        let mut st: bool = pr.read_integer_text(&pr.current(), "Number of Geometries", &mut num)?;
        if st && (num > 0) {
            temp_geom_ents = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
            temp_dim_loc_flags = iges_new_array1::<i32>(1, num)?;
            temp_points = iges_new_array1::<Xyz>(1, num)?;
        } else {
            pr.add_fail("Number of Geometries: Not Positive", "");
        }
        pr.read_entity(
            ir,
            &pr.current(),
            "Dimension Entity",
            &mut temp_dimen,
            false,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Dimension Orientation Flag",
            &mut temp_dim_orient_flag,
        )?;
        pr.read_real_text(&pr.current(), "Angle Value", &mut temp_angle)?;
        if !temp_geom_ents.is_null() {
            for i in 1..=num {
                let mut temp_ent: Handle<IgesDataIgesEntity> = Default::default();
                pr.read_entity(
                    ir,
                    &pr.current(),
                    "Geometry Entity",
                    &mut temp_ent,
                    i == num,
                )?;
                iges_array1_set(
                    &temp_geom_ents,
                    i,
                    temp_ent.clone(),
                    "IgesDimenToolNewDimensionedGeometry::read_own_params",
                )?;
                let mut temp_int: i32 = Default::default();
                pr.read_integer_text(&pr.current(), "Dimension Location Flag", &mut temp_int)?;
                iges_array1_set(
                    &temp_dim_loc_flags,
                    i,
                    temp_int,
                    "IgesDimenToolNewDimensionedGeometry::read_own_params",
                )?;
                let mut temp_pnt: Xyz = Default::default();
                pr.read_xyz(&pr.current_list(1, 3), "Point", &mut temp_pnt)?;
                iges_array1_set(
                    &temp_points,
                    i,
                    temp_pnt,
                    "IgesDimenToolNewDimensionedGeometry::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_dimens,
            &temp_dimen,
            temp_dim_orient_flag,
            temp_angle,
            &temp_geom_ents,
            &temp_dim_loc_flags,
            &temp_points,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send(ent.nb_dimensions())?;
        iw.send(ent.nb_geometries()?)?;
        iw.send_entity(&ent.dimension_entity(), false)?;
        iw.send(ent.dimension_orientation_flag())?;
        iw.send(ent.angle_value())?;
        {
            num = ent.nb_geometries()?;
            i = 1;
            while i <= num {
                iw.send_entity(&ent.geometry_entity(i)?, false)?;
                iw.send(ent.dimension_location_flag(i)?)?;
                iw.send(ent.point(i)?.x())?;
                iw.send(ent.point(i)?.y())?;
                iw.send(ent.point(i)?.z())?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a NewDimensionedGeometry `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iter.get_one_item(&ent.dimension_entity().cast());
        {
            num = ent.nb_geometries()?;
            i = 1;
            while i <= num {
                iter.get_one_item(&ent.geometry_entity(i)?.cast());
                i += 1;
            }
        }
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a NewDimensionedGeometry
    /// (NbDimensions forced to 1, Transf Nullified in D.E.)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
    ) -> Result<bool, Failure> {
        let mut res: bool = ent.has_transf();
        if res {
            let mut nultransf: Handle<dyn IgesDataTransfEntity> = Default::default();
            ent.init_transf(&nultransf);
        }
        if ent.nb_dimensions() == 1 {
            return Ok(res);
        }
        let mut nb: i32 = ent.nb_geometries()?;
        let mut temp_geom_ents: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nb)?;
        let mut temp_dim_loc_flags: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nb)?;
        let mut temp_points: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(
                &temp_geom_ents,
                i,
                ent.geometry_entity(i)?,
                "IgesDimenToolNewDimensionedGeometry::own_correct",
            )?;
            iges_array1_set(
                &temp_dim_loc_flags,
                i,
                ent.dimension_location_flag(i)?,
                "IgesDimenToolNewDimensionedGeometry::own_correct",
            )?;
            iges_array1_set(
                &temp_points,
                i,
                ent.point(i)?.xyz(),
                "IgesDimenToolNewDimensionedGeometry::own_correct",
            )?;
        }
        ent.init(
            1,
            &ent.dimension_entity(),
            ent.dimension_orientation_flag(),
            ent.angle_value(),
            &temp_geom_ents,
            &temp_dim_loc_flags,
            &temp_points,
        )?;
        return Ok(true);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenNewDimensionedGeometry>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 21);
        crate::iges_shared_blocks::iges_block_0006!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_dimensions() != 1 {
            ach.add_fail_text("Number of Dimensions != 1", "");
        }
        if ent.has_transf() {
            ach.add_warning_text("Transformation Matrix exists, ignored", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenNewDimensionedGeometry>,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut num: i32 = another.nb_geometries()?;
        let mut temp_nb_dimens: i32 = another.nb_dimensions();
        let mut temp_dim_orient_flag: i32 = another.dimension_orientation_flag();
        let mut temp_angle: f64 = another.angle_value();
        let mut temp_dimen: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.dimension_entity().cast())?.cast();
        let mut temp_geom_ents: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        let mut temp_dim_loc_flags: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, num)?;
        let mut temp_points: Handle<HArray1<Xyz>> = iges_new_array1::<Xyz>(1, num)?;
        for i in 1..=num {
            let mut temp_ent: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.geometry_entity(i)?.cast())?.cast();
            iges_array1_set(
                &temp_geom_ents,
                i,
                temp_ent.clone(),
                "IgesDimenToolNewDimensionedGeometry::own_copy",
            )?;
            iges_array1_set(
                &temp_dim_loc_flags,
                i,
                another.dimension_location_flag(i)?,
                "IgesDimenToolNewDimensionedGeometry::own_copy",
            )?;
            iges_array1_set(
                &temp_points,
                i,
                another.point(i)?.xyz(),
                "IgesDimenToolNewDimensionedGeometry::own_copy",
            )?;
        }
        ent.init(
            temp_nb_dimens,
            &temp_dimen,
            temp_dim_orient_flag,
            temp_angle,
            &temp_geom_ents,
            &temp_dim_loc_flags,
            &temp_points,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenNewDimensionedGeometry>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(
            s,
            "IgesDimenNewDimensionedGeometry\nNumber of Dimensions : {}\nDimension Entity : ",
            ent.nb_dimensions()
        )?;
        dumper.dump(&ent.dimension_entity(), s, sublevel, -1)?;
        write!(
            s,
            "\nDimension Orientation Flag : {}\nAngle Value Flag : {}\nGeometry Entities :\nDimension Location Flags :\nPoints : ",
            ent.dimension_orientation_flag(),
            iges_real_text(ent.angle_value())
        )?;
        iges_dump_entities(s, dumper, -level, 1, ent.nb_geometries()?, |first| {
            ent.geometry_entity(first)
        })?;
        writeln!(s)?;
        if level > 4 {
            {
                num = ent.nb_geometries()?;
                i = 1;
                while i <= num {
                    write!(s, "[{}]:\nGeometry Entity : ", i)?;
                    dumper.dump(&ent.geometry_entity(i)?, s, 1, -1)?;
                    write!(
                        s,
                        "\nDimension Location Flag : {}\nPoint : ",
                        ent.dimension_location_flag(i)?
                    )?;
                    iges_dump_xyz_l(s, level, &ent.point(i)?.xyz(), &ent.location()?)?;
                    i += 1;
                }
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
