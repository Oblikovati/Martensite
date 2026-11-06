//! IgesDimenToolDimensionTolerance.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_real_text,
};
use crate::iges_dimen::IgesDimenDimensionTolerance;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolDimensionTolerance,
    "The own-parameter work on `IgesDimenDimensionTolerance`."
);

impl IgesDimenToolDimensionTolerance {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionTolerance>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_props: i32 = Default::default();
        let mut temp_second_tol_flag: i32 = Default::default();
        let mut temp_tol_typ: i32 = Default::default();
        let mut temp_tol_place_flag: i32 = Default::default();
        let mut temp_upper_tol: f64 = Default::default();
        let mut temp_lower_tol: f64 = Default::default();
        let mut temp_sign_sup_flag: bool = Default::default();
        let mut temp_frac_flag: i32 = Default::default();
        let mut temp_precision: i32 = Default::default();
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Number of properties", &mut temp_nb_props)?;
        } else {
            temp_nb_props = 8;
        }
        pr.read_integer_text(
            &pr.current(),
            "Secondary Tolerance Flag",
            &mut temp_second_tol_flag,
        )?;
        pr.read_integer_text(&pr.current(), "Tolerance Type", &mut temp_tol_typ)?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Tolerance Placement Flag",
                &mut temp_tol_place_flag,
            )?;
        } else {
            temp_tol_place_flag = 2;
        }
        pr.read_real_text(&pr.current(), "Upper Tolerance", &mut temp_upper_tol)?;
        pr.read_real_text(&pr.current(), "Lower Tolerance", &mut temp_lower_tol)?;
        pr.read_boolean(
            &pr.current(),
            "Sign Suppression Flag",
            &mut temp_sign_sup_flag,
            true,
        )?;
        crate::iges_shared_blocks::iges_block_0058!(ent, pr, self, temp_frac_flag, temp_precision);
        ent.init(
            temp_nb_props,
            temp_second_tol_flag,
            temp_tol_typ,
            temp_tol_place_flag,
            temp_upper_tol,
            temp_lower_tol,
            temp_sign_sup_flag,
            temp_frac_flag,
            temp_precision,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionTolerance>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.secondary_tolerance_flag())?;
        iw.send(ent.tolerance_type())?;
        iw.send(ent.tolerance_placement_flag())?;
        iw.send(ent.upper_tolerance())?;
        iw.send(ent.lower_tolerance())?;
        iw.send_boolean(ent.sign_suppression_flag())?;
        iw.send(ent.fraction_flag())?;
        iw.send(ent.precision())?;
        Ok(())
    }
    /// Lists the Entities shared by a DimensionTolerance `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenDimensionTolerance>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DimensionTolerance
    /// (NbPropertyValues forced to 8)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDimenDimensionTolerance>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 8;
        if res {
            ent.init(
                8,
                ent.secondary_tolerance_flag(),
                ent.tolerance_type(),
                ent.tolerance_placement_flag(),
                ent.upper_tolerance(),
                ent.lower_tolerance(),
                (if ent.sign_suppression_flag() { 1 } else { 0 }) != 0,
                ent.fraction_flag(),
                ent.precision(),
            );
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenDimensionTolerance>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 29);
        crate::iges_shared_blocks::iges_block_0007!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenDimensionTolerance>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 8 {
            ach.add_fail_text("Number of properties != 8", "");
        }
        if (ent.secondary_tolerance_flag() < 0) || (ent.secondary_tolerance_flag() > 2) {
            ach.add_fail_text("Secondary Tolerance Flag != 0-2", "");
        }
        if (ent.tolerance_type() < 1) || (ent.tolerance_type() > 10) {
            ach.add_fail_text("Tolerance Type != 1-10", "");
        }
        if (ent.tolerance_placement_flag() < 1) || (ent.tolerance_placement_flag() > 4) {
            ach.add_fail_text("Tolerance Placement Flag != 1-4", "");
        }
        if (ent.fraction_flag() < 0) || (ent.fraction_flag() > 2) {
            ach.add_fail_text("Fraction Flag != 0-2", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenDimensionTolerance>,
        ent: &Handle<IgesDimenDimensionTolerance>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            8,
            another.secondary_tolerance_flag(),
            another.tolerance_type(),
            another.tolerance_placement_flag(),
            another.upper_tolerance(),
            another.lower_tolerance(),
            (if another.sign_suppression_flag() {
                1
            } else {
                0
            }) != 0,
            another.fraction_flag(),
            another.precision(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenDimensionTolerance>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesDimenDimensionTolerance\nNumber of property values : {}\nSecondary Tolerance Flag : {}\nTolerance Type           : {}\nTolerance Placement Flag : {}\nUpper Tolerance          : {}\nLower Tolerance          : {}\nSign Suppression Flag    : {}\nFraction Flag            : {}\nPrecision                : {}",
            ent.nb_property_values(),
            ent.secondary_tolerance_flag(),
            ent.tolerance_type(),
            ent.tolerance_placement_flag(),
            iges_real_text(ent.upper_tolerance()),
            iges_real_text(ent.lower_tolerance()),
            if ent.sign_suppression_flag() {
                "True"
            } else {
                "False"
            },
            ent.fraction_flag(),
            ent.precision()
        )?;
        Ok(())
    }
}
