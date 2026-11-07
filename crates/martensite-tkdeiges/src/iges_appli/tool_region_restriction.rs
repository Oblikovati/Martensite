//! IgesAppliToolRegionRestriction.
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
use crate::iges_appli::IgesAppliRegionRestriction;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefList, IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLevelListEntity, IgesDataParamReader,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolRegionRestriction,
    "The own-parameter work on `IgesAppliRegionRestriction`."
);

impl IgesAppliToolRegionRestriction {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliRegionRestriction>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_elect_vias_restrict: i32 = Default::default();
        let mut temp_elect_comp_restrict: i32 = Default::default();
        let mut temp_elect_ckt_restrict: i32 = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of Property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Electrical vias restriction",
            &mut temp_elect_vias_restrict,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Electrical components restriction",
            &mut temp_elect_comp_restrict,
        )?;
        pr.read_integer_text(
            &pr.current(),
            " Electrical circuitary restriction",
            &mut temp_elect_ckt_restrict,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_property_values,
            temp_elect_vias_restrict,
            temp_elect_comp_restrict,
            temp_elect_ckt_restrict,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliRegionRestriction>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.electrical_vias_restriction())?;
        iw.send(ent.electrical_component_restriction())?;
        iw.send(ent.electrical_ckt_restriction())?;
        Ok(())
    }
    /// Lists the Entities shared by a RegionRestriction `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliRegionRestriction>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a RegionRestriction
    /// (NbPropertyValues forced to 3, Level cleared if Subordinate != 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliRegionRestriction>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 3;
        if res {
            ent.init(
                3,
                ent.electrical_vias_restriction(),
                ent.electrical_component_restriction(),
                ent.electrical_ckt_restriction(),
            );
        }
        if ent.subordinate_status() != 0 {
            let mut nulevel: Handle<dyn IgesDataLevelListEntity> = Default::default();
            ent.init_level(&nulevel, 0);
            res = true;
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesAppliRegionRestriction>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 2);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliRegionRestriction>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0073!(ach, ent);
        if ent.nb_property_values() != 3 {
            ach.add_fail_text("Number of Property Values != 3", "");
        }
        if (ent.electrical_vias_restriction() < 0) || (ent.electrical_vias_restriction() > 2) {
            ach.add_fail_text("Incorrect value for Electrical Vias Restriction", "");
        }
        if (ent.electrical_component_restriction() < 0)
            || (ent.electrical_component_restriction() > 2)
        {
            ach.add_fail_text("Incorrect value for Electrical Component Restriction", "");
        }
        if (ent.electrical_ckt_restriction() < 0) || (ent.electrical_ckt_restriction() > 2) {
            ach.add_fail_text("Incorrect value for Electrical Circuit Restriction", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliRegionRestriction>,
        ent: &Handle<IgesAppliRegionRestriction>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            3,
            another.electrical_vias_restriction(),
            another.electrical_component_restriction(),
            another.electrical_ckt_restriction(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliRegionRestriction>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesAppliRegionRestriction\nNumber of property values : {}\nElectrical vias restriction       : {}\nElectrical components restriction : {}\nElectrical circuitary restriction : {}",
            ent.nb_property_values(),
            ent.electrical_vias_restriction(),
            ent.electrical_component_restriction(),
            ent.electrical_ckt_restriction()
        )?;
        Ok(())
    }
}
