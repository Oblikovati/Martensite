//! IgesGraphToolLineFontDefTemplate.
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
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_real_text,
};
use crate::iges_graph::IgesGraphLineFontDefTemplate;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolLineFontDefTemplate,
    "The own-parameter work on `IgesGraphLineFontDefTemplate`."
);

impl IgesGraphToolLineFontDefTemplate {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_orientation: i32 = Default::default();
        let mut temp_distance: f64 = Default::default();
        let mut temp_scale: f64 = Default::default();
        let mut temp_template_entity: Handle<IgesBasicSubfigureDef> = Default::default();
        pr.read_integer_text(&pr.current(), "Template Orientation", &mut temp_orientation)?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Subfigure Definition Entity for Template Display",
            <IgesBasicSubfigureDef>::static_type(),
            &mut temp_template_entity,
            false,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Distance between successive Template",
            &mut temp_distance,
        )?;
        pr.read_real_text(&pr.current(), "Scale Factor For Subfigure", &mut temp_scale)?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_orientation,
            &temp_template_entity,
            temp_distance,
            temp_scale,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.orientation())?;
        iw.send_entity(&ent.template_entity(), false)?;
        iw.send(ent.distance())?;
        iw.send(ent.scale())?;
        Ok(())
    }
    /// Lists the Entities shared by a LineFontDefTemplate `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.template_entity().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesGraphLineFontDefTemplate>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(304, 1);
        crate::iges_shared_blocks::iges_block_0012!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.rank_line_font() == 0 {
            ach.add_warning_text("Line Font Rank is zero", "");
        } else {
            if (ent.rank_line_font() < 1) || (ent.rank_line_font() > 5) {
                ach.add_warning_text("Invalid Value As Line Font Rank", "");
            }
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphLineFontDefTemplate>,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_orientation: i32 = another.orientation();
        let mut temp_template_subfigure: Handle<IgesBasicSubfigureDef> =
            tc.transferred(&another.template_entity().cast())?.cast();
        let mut temp_distance: f64 = another.distance();
        let mut temp_scale: f64 = another.scale();
        ent.init(
            temp_orientation,
            &temp_template_subfigure,
            temp_distance,
            temp_scale,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphLineFontDefTemplate>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesGraphLineFontDefTemplate\nOrientation : {}\nSubfigure Display Entity For Template Display : ",
            ent.orientation()
        )?;
        dumper.dump(&ent.template_entity(), s, temp_sub_level, -1)?;
        writeln!(
            s,
            "\nLength Between Successive Template Figure : {}\nScale Factor for Subfigure : {}\n",
            iges_real_text(ent.distance()),
            iges_real_text(ent.scale())
        )?;
        Ok(())
    }
}
