//! IgesGraphToolDrawingSize.
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
use crate::iges_graph::IgesGraphDrawingSize;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolDrawingSize,
    "The own-parameter work on `IgesGraphDrawingSize`."
);

impl IgesGraphToolDrawingSize {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphDrawingSize>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_property_values: i32 = Default::default();
        let mut x_size: f64 = Default::default();
        let mut y_size: f64 = Default::default();
        pr.read_integer_text(
            &pr.current(),
            "No. of property values",
            &mut nb_property_values,
        )?;
        if nb_property_values != 2 {
            pr.add_fail("No. of Property values : Value is not 2", "");
        }
        pr.read_real_text(
            &pr.current(),
            "Drawing extent along +ve XD axis",
            &mut x_size,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Drawing extent along +ve YD axis",
            &mut y_size,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(nb_property_values, x_size, y_size);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphDrawingSize>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.x_size())?;
        iw.send(ent.y_size())?;
        Ok(())
    }
    /// Lists the Entities shared by a DrawingSize `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphDrawingSize>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DrawingSize
    /// (NbPropertyValues forced to 2)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesGraphDrawingSize>) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 2;
        if res {
            ent.init(2, ent.x_size(), ent.y_size());
        }
        Ok(res)
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphDrawingSize>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 16);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphDrawingSize>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 2 {
            ach.add_fail_text("No. of Property values : Value != 2", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphDrawingSize>,
        ent: &Handle<IgesGraphDrawingSize>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(2, another.x_size(), another.y_size());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphDrawingSize>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(
            s,
            "IgesGraphDrawingSize\nNo. of property values : {}\nDrawing extent along positive X-axis : {}\nDrawing extent along positive Y-axis : {}\n",
            ent.nb_property_values(),
            iges_real_text(ent.x_size()),
            iges_real_text(ent.y_size())
        )?;
        Ok(())
    }
}
