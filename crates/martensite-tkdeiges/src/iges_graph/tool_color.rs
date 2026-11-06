//! IgesGraphToolColor.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_string, iges_live,
    iges_real_text,
};
use crate::iges_graph::IgesGraphColor;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceParamType,
    InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolColor,
    "The own-parameter work on `IgesGraphColor`."
);

impl IgesGraphToolColor {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphColor>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0048!(
            temp_blue,
            temp_color_name,
            temp_green,
            temp_red
        );
        pr.read_real_text(&pr.current(), "RED as % Of Full Intensity", &mut temp_red)?;
        pr.read_real_text(
            &pr.current(),
            "GREEN as % Of Full Intensity",
            &mut temp_green,
        )?;
        pr.read_real_text(&pr.current(), "BLUE as % Of Full Intensity", &mut temp_blue)?;
        if (pr.current_number() <= pr.nb_params())
            && ((pr.param_type(pr.current_number())?) == (InterfaceParamType::ParamText))
        {
            pr.read_text(&pr.current(), "Color Name", &mut temp_color_name)?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(temp_red, temp_green, temp_blue, &temp_color_name);
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphColor>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0047!(blue, ent, green, red);
        iw.send(red)?;
        iw.send(green)?;
        iw.send(blue)?;
        if ent.has_color_name() {
            iw.send(ent.color_name())?;
        } else {
            iw.send_void()?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a Color `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesGraphColor>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphColor>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(314, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefVoid);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.blank_status_ignored();
        dc.subordinate_status_required(0);
        dc.use_flag_required(2);
        dc.hierarchy_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesGraphColor>,
        _unused_1: &InterfaceShareTool,
        _unused_2: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphColor>,
        ent: &Handle<IgesGraphColor>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0048!(
            temp_blue,
            temp_color_name,
            temp_green,
            temp_red
        );
        another.rgb_intensity(&mut temp_red, &mut temp_green, &mut temp_blue);
        if another.has_color_name() {
            temp_color_name = Handle::new(HAsciiString::copied(iges_live(
                &another.color_name(),
                "IgesGraphToolColor::own_copy",
            )?));
        }
        ent.init(temp_red, temp_green, temp_blue, &temp_color_name);
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphColor>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _unused_3: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesGraphColor")?;
        crate::iges_shared_blocks::iges_block_0047!(blue, ent, green, red);
        write!(
            s,
            "Red   (in % Of Full Intensity) : {}\nGreen (in % Of Full Intensity) : {}\nBlue  (in % Of Full Intensity) : {}\nColor Name : ",
            iges_real_text(red),
            iges_real_text(green),
            iges_real_text(blue)
        )?;
        iges_dump_string(s, &ent.color_name())?;
        writeln!(s)?;
        Ok(())
    }
}
