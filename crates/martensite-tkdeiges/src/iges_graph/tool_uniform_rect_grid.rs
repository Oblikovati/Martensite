//! IgesGraphToolUniformRectGrid.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xy,
};
use crate::iges_graph::IgesGraphUniformRectGrid;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolUniformRectGrid,
    "The own-parameter work on `IgesGraphUniformRectGrid`."
);

impl IgesGraphToolUniformRectGrid {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphUniformRectGrid>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_property_values: i32 = Default::default();
        let mut finite: i32 = Default::default();
        let mut line: i32 = Default::default();
        let mut weighted: i32 = Default::default();
        let mut grid_point: Xy = Default::default();
        let mut grid_spacing: Xy = Default::default();
        let mut nb_points_x: i32 = 0;
        let mut nb_points_y: i32 = 0;
        pr.read_integer_text(
            &pr.current(),
            "No. of property values",
            &mut nb_property_values,
        )?;
        if nb_property_values != 9 {
            pr.add_fail("No. of Property values : Value is not 9", "");
        }
        pr.read_integer_text(&pr.current(), "Finite/infinite grid flag", &mut finite)?;
        pr.read_integer_text(&pr.current(), "Line/point grid flag", &mut line)?;
        pr.read_integer_text(
            &pr.current(),
            "Weighted/unweighted grid flag",
            &mut weighted,
        )?;
        pr.read_xy(
            &pr.current_list(1, 2),
            "Grid point coordinates",
            &mut grid_point,
        )?;
        pr.read_xy(
            &pr.current_list(1, 2),
            "Grid Spacing coordinates",
            &mut grid_spacing,
        )?;
        if (finite != 0) || pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "No. of points/lines in X direction",
                &mut nb_points_x,
            )?;
        }
        if (finite != 0) || pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "No. of points/lines in Y direction",
                &mut nb_points_y,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            nb_property_values,
            finite,
            line,
            weighted,
            grid_point,
            grid_spacing,
            nb_points_x,
            nb_points_y,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphUniformRectGrid>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send_boolean(ent.is_finite())?;
        iw.send_boolean(ent.is_line())?;
        iw.send_boolean(!ent.is_weighted())?;
        iw.send(ent.grid_point().x())?;
        iw.send(ent.grid_point().y())?;
        iw.send(ent.grid_spacing().x())?;
        iw.send(ent.grid_spacing().y())?;
        iw.send(ent.nb_points_x())?;
        iw.send(ent.nb_points_y())?;
        Ok(())
    }
    /// Lists the Entities shared by a UniformRectGrid `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphUniformRectGrid>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a UniformRectGrid
    /// (NbPropertyValues forced to 9)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesGraphUniformRectGrid>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 9;
        if res {
            ent.init(
                9,
                if ent.is_finite() { 1 } else { 0 },
                if ent.is_line() { 1 } else { 0 },
                if ent.is_weighted() { 0 } else { 1 },
                ent.grid_point().xy(),
                ent.grid_spacing().xy(),
                ent.nb_points_x(),
                ent.nb_points_y(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphUniformRectGrid>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 22);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphUniformRectGrid>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (i32::from(ent.is_finite()) != 0) && (i32::from(ent.is_finite()) != 1) {
            ach.add_fail_text("Finite/infinite grid flag : Value != 0/1", "");
        }
        if (i32::from(ent.is_line()) != 0) && (i32::from(ent.is_line()) != 1) {
            ach.add_fail_text("Line/point grid flag : Value != 0/1", "");
        }
        if (i32::from(ent.is_weighted()) != 0) && (i32::from(ent.is_weighted()) != 1) {
            ach.add_fail_text("Weighted/unweighted grid flag : Value != 0/1", "");
        }
        if ent.nb_property_values() != 9 {
            ach.add_fail_text("No. of Property values : Value != 9", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphUniformRectGrid>,
        ent: &Handle<IgesGraphUniformRectGrid>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            9,
            if another.is_finite() { 1 } else { 0 },
            if another.is_line() { 1 } else { 0 },
            if another.is_weighted() { 0 } else { 1 },
            another.grid_point().xy(),
            another.grid_spacing().xy(),
            another.nb_points_x(),
            another.nb_points_y(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphUniformRectGrid>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesGraphUniformRectGrid\nNo. of property values : {}\nGrid         : {}  -  Composed of {}  -  {}\nGrid Point   : ",
            ent.nb_property_values(),
            if ent.is_finite() {
                "Finite"
            } else {
                "Infinite"
            },
            if ent.is_line() { "Lines" } else { "Points" },
            if ent.is_weighted() {
                "Weighted"
            } else {
                "Unweighted"
            }
        )?;
        iges_dump_xy(s, &ent.grid_point().xy())?;
        write!(s, "  Grid Spacing : ")?;
        iges_dump_xy(s, &ent.grid_spacing().xy())?;
        writeln!(s)?;
        if ent.is_finite() {
            writeln!(
                s,
                "No. of points/lines in direction :  X : {}  -  Y : {}",
                ent.nb_points_x(),
                ent.nb_points_y()
            )?;
        }
        Ok(())
    }
}
