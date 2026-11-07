//! IgesAppliToolReferenceDesignator.
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
use crate::iges_appli::IgesAppliReferenceDesignator;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefList, IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLevelListEntity, IgesDataParamReader,
    iges_dump_string, iges_live,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesAppliToolReferenceDesignator,
    "The own-parameter work on `IgesAppliReferenceDesignator`."
);

impl IgesAppliToolReferenceDesignator {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliReferenceDesignator>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_property_values: i32 = Default::default();
        let mut temp_reference_designator: Handle<HAsciiString> = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_text(
            &pr.current(),
            "ReferenceDesignator",
            &mut temp_reference_designator,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_nb_property_values, &temp_reference_designator);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliReferenceDesignator>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.ref_designator_text())?;
        Ok(())
    }
    /// Lists the Entities shared by a ReferenceDesignator `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesAppliReferenceDesignator>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a ReferenceDesignator
    /// (NbPropertyValues forced to 1, Level cleared if Subordinate != 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesAppliReferenceDesignator>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 1;
        if res {
            ent.init(1, &ent.ref_designator_text());
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
        _unused_0: &Handle<IgesAppliReferenceDesignator>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 7);
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
        ent: &Handle<IgesAppliReferenceDesignator>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0073!(ach, ent);
        if ent.nb_property_values() != 1 {
            ach.add_fail_text("Number of Property Values != 1", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesAppliReferenceDesignator>,
        ent: &Handle<IgesAppliReferenceDesignator>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut a_nb_property_values: i32 = Default::default();
        let mut a_reference_designator: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.ref_designator_text(),
                "IgesAppliToolReferenceDesignator::own_copy",
            )?));
        a_nb_property_values = another.nb_property_values();
        ent.init(a_nb_property_values, &a_reference_designator);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesAppliReferenceDesignator>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesAppliReferenceDesignator")?;
        writeln!(
            s,
            "Number of Property Values : {}",
            ent.nb_property_values()
        )?;
        write!(s, "ReferenceDesignator : ")?;
        iges_dump_string(s, &ent.ref_designator_text())?;
        writeln!(s)?;
        Ok(())
    }
}
