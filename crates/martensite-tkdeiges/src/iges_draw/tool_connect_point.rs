//! IgesDrawToolConnectPoint.
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
    iges_dump_string, iges_dump_xyz_l, iges_live,
};
use crate::iges_draw::IgesDrawConnectPoint;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolConnectPoint,
    "The own-parameter work on `IgesDrawConnectPoint`."
);

impl IgesDrawToolConnectPoint {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawConnectPoint>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_point: Xyz = Default::default();
        let mut temp_type_flag: i32 = Default::default();
        let mut temp_function_flag: i32 = Default::default();
        let mut temp_point_identifier: i32 = Default::default();
        let mut temp_function_code: i32 = Default::default();
        let mut temp_swap_flag: i32 = Default::default();
        let mut temp_display_symbol: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_owner_subfigure: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_function_identifier: Handle<HAsciiString> = Default::default();
        let mut temp_function_name: Handle<HAsciiString> = Default::default();
        let mut temp_function_template: Handle<IgesGraphTextDisplayTemplate> = Default::default();
        let mut temp_identifier_template: Handle<IgesGraphTextDisplayTemplate> = Default::default();
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Connect Point Coordinate",
            &mut temp_point,
        )?;
        pr.read_entity(
            ir,
            &pr.current(),
            "Display Symbol Geometry Entity",
            &mut temp_display_symbol,
            true,
        )?;
        pr.read_integer_text(&pr.current(), "Type Flag", &mut temp_type_flag)?;
        pr.read_integer_text(&pr.current(), "Function Flag", &mut temp_function_flag)?;
        pr.read_text(
            &pr.current(),
            "Function Identifier",
            &mut temp_function_identifier,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Text Display Identifier Template",
            <IgesGraphTextDisplayTemplate>::static_type(),
            &mut temp_identifier_template,
            true,
        )?;
        pr.read_text(
            &pr.current(),
            "Connect Point Function Name",
            &mut temp_function_name,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Text Display Function Template",
            <IgesGraphTextDisplayTemplate>::static_type(),
            &mut temp_function_template,
            true,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Unique Connect Point Identifier",
            &mut temp_point_identifier,
        )?;
        pr.read_integer_text(
            &pr.current(),
            "Connect Point Function Code",
            &mut temp_function_code,
        )?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Swap Flag", &mut temp_swap_flag)?;
        } else {
            temp_swap_flag = 0;
        }
        pr.read_entity(
            ir,
            &pr.current(),
            "Owner Network Subfigure Entity",
            &mut temp_owner_subfigure,
            true,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_point,
            &temp_display_symbol,
            temp_type_flag,
            temp_function_flag,
            &temp_function_identifier,
            &temp_identifier_template,
            &temp_function_name,
            &temp_function_template,
            temp_point_identifier,
            temp_function_code,
            temp_swap_flag,
            &temp_owner_subfigure,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawConnectPoint>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.point().x())?;
        iw.send(ent.point().y())?;
        iw.send(ent.point().z())?;
        iw.send_entity(&ent.display_symbol(), false)?;
        iw.send(ent.type_flag())?;
        iw.send(ent.function_flag())?;
        iw.send(ent.function_identifier())?;
        iw.send_entity(&ent.identifier_template(), false)?;
        iw.send(ent.function_name())?;
        iw.send_entity(&ent.function_template(), false)?;
        iw.send(ent.point_identifier())?;
        iw.send(ent.function_code())?;
        iw.send_boolean(ent.swap_flag())?;
        iw.send_entity(&ent.owner_subfigure(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a ConnectPoint `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawConnectPoint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.display_symbol().cast());
        iter.get_one_item(&ent.identifier_template().cast());
        iter.get_one_item(&ent.function_template().cast());
        iter.get_one_item(&ent.owner_subfigure().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, ent: &Handle<IgesDrawConnectPoint>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(132, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(4);
        if !(ent.display_symbol().is_null()) {
            dc.line_font(IgesDataDefType::DefAny);
            dc.line_weight(IgesDataDefType::DefAny);
        } else {
            dc.line_font(IgesDataDefType::DefVoid);
            dc.line_weight(IgesDataDefType::DefVoid);
            dc.hierarchy_status_ignored();
        }
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawConnectPoint>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ((((ent.type_flag() < 0) || (ent.type_flag() > 9999))
            || ((ent.type_flag() > 2) && (ent.type_flag() < 101)))
            || ((ent.type_flag() > 104) && (ent.type_flag() < 201)))
            || ((ent.type_flag() > 203) && (ent.type_flag() < 5001))
        {
            ach.add_fail_text("TypeFlag has Invalid value", "");
        }
        if (ent.function_flag() < 0) || (ent.function_flag() > 2) {
            ach.add_fail_text("FunctionFlag has Invalid value", "");
        }
        if (((ent.function_code() < 0) || (ent.function_code() > 9999))
            || ((ent.function_code() > 49) && (ent.function_code() < 98)))
            || ((ent.function_code() > 99) && (ent.function_code() < 5001))
        {
            ach.add_fail_text("FunctionCode has Invalid value", "");
        }
        if (i32::from(ent.swap_flag()) != 0) && (i32::from(ent.swap_flag()) != 1) {
            ach.add_fail_text("SwapFlag has Invalid value", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawConnectPoint>,
        ent: &Handle<IgesDrawConnectPoint>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_point: Xyz = another.point().xyz();
        let mut temp_display_symbol: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.display_symbol().cast())?.cast();
        let mut temp_type_flag: i32 = another.type_flag();
        let mut temp_function_flag: i32 = another.function_flag();
        let mut temp_function_identifier: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.function_identifier(),
                "IgesDrawToolConnectPoint::own_copy",
            )?));
        let mut temp_identifier_template: Handle<IgesGraphTextDisplayTemplate> = tc
            .transferred(&another.identifier_template().cast())?
            .cast();
        let mut temp_function_name: Handle<HAsciiString> =
            Handle::new(HAsciiString::copied(iges_live(
                &another.function_name(),
                "IgesDrawToolConnectPoint::own_copy",
            )?));
        let mut temp_function_template: Handle<IgesGraphTextDisplayTemplate> =
            tc.transferred(&another.function_template().cast())?.cast();
        let mut temp_point_identifier: i32 = another.point_identifier();
        let mut temp_function_code: i32 = another.function_code();
        let mut temp_swap_flag: i32 = if another.swap_flag() { 1 } else { 0 };
        let mut temp_owner_subfigure: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.owner_subfigure().cast())?.cast();
        ent.init(
            temp_point,
            &temp_display_symbol,
            temp_type_flag,
            temp_function_flag,
            &temp_function_identifier,
            &temp_identifier_template,
            &temp_function_name,
            &temp_function_template,
            temp_point_identifier,
            temp_function_code,
            temp_swap_flag,
            &temp_owner_subfigure,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawConnectPoint>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesDrawConnectPoint\nConnection Point Coordinate : ")?;
        iges_dump_xyz_l(s, level, &ent.point().xyz(), &ent.location()?)?;
        write!(s, "Display Symbol Geometry Entity : ")?;
        dumper.dump(&ent.display_symbol(), s, temp_sub_level, -1)?;
        write!(
            s,
            "\nType Flag : {}  Function Flag : {}\nFunction Identifier : ",
            ent.type_flag(),
            ent.function_flag()
        )?;
        iges_dump_string(s, &ent.function_identifier())?;
        write!(s, "\nText Display Template Entity for CID : ")?;
        dumper.dump(&ent.identifier_template(), s, temp_sub_level, -1)?;
        write!(s, "\nFunction Name : ")?;
        iges_dump_string(s, &ent.function_name())?;
        write!(s, "\nText Display Template Entity for CFN : ")?;
        dumper.dump(&ent.function_template(), s, temp_sub_level, -1)?;
        write!(
            s,
            "\nPoint Identifier : {}\nFunction Code : {}Swap Flag : {}\nOwner Subfigure Entity : ",
            ent.point_identifier(),
            ent.function_code(),
            if ent.swap_flag() { "True" } else { "False" }
        )?;
        dumper.dump(&ent.owner_subfigure(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
