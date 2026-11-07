//! IgesDrawToolNetworkSubfigureDef.
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
    iges_array1_set, iges_dump_entities, iges_dump_string, iges_live, iges_new_array1,
};
use crate::iges_draw::{IgesDrawConnectPoint, IgesDrawNetworkSubfigureDef};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolNetworkSubfigureDef,
    "The own-parameter work on `IgesDrawNetworkSubfigureDef`."
);

impl IgesDrawToolNetworkSubfigureDef {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_depth: i32 = Default::default();
        let mut temp_nb_entities1: i32 = Default::default();
        let mut temp_type_flag: i32 = Default::default();
        let mut temp_nb_entities2: i32 = Default::default();
        let mut temp_name: Handle<HAsciiString> = Default::default();
        let mut temp_designator: Handle<HAsciiString> = Default::default();
        let mut temp_template: Handle<IgesGraphTextDisplayTemplate> = Default::default();
        let mut temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut temp_point_entities: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
            Default::default();
        pr.read_integer_text(&pr.current(), "Depth Of Subfigure", &mut temp_depth)?;
        pr.read_text(&pr.current(), "Subfigure Name", &mut temp_name)?;
        if pr.read_integer_text(
            &pr.current(),
            "Number Of Child Entities",
            &mut temp_nb_entities1,
        )? {
            if temp_nb_entities1 < 0 {
                pr.add_fail("Number Of Child Entities : Not Positive", "");
            } else {
                if temp_nb_entities1 > 0 {
                    pr.read_ents(
                        ir,
                        &pr.current_list(temp_nb_entities1, 1),
                        "Child Entities",
                        &mut temp_entities,
                        1,
                    )?;
                }
            }
        }
        pr.read_integer_text(&pr.current(), "Type Flag", &mut temp_type_flag)?;
        if pr.defined_else_skip()? {
            pr.read_text(
                &pr.current(),
                "Primary Reference Designator",
                &mut temp_designator,
            )?;
        } else {
            pr.add_warning("Primary Reference Designator : Null", "");
        }
        let mut st: bool = pr.read_entity_typed(
            ir,
            &pr.current(),
            "Primary Reference Designator",
            <IgesGraphTextDisplayTemplate>::static_type(),
            &mut temp_template,
            true,
        )?;
        if pr.defined_else_skip()? {
            st = pr.read_integer_text(
                &pr.current(),
                "Number Of Connect Points",
                &mut temp_nb_entities2,
            )?;
        } else {
            temp_nb_entities2 = 0;
        }
        if st {
            if temp_nb_entities2 < 0 {
                pr.add_fail("Number Of Connect Points : Less Than Zero", "");
            } else {
                if temp_nb_entities2 > 0 {
                    temp_point_entities =
                        iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, temp_nb_entities2)?;
                }
            }
        }
        if !temp_point_entities.is_null() {
            let mut temp_connect_point: Handle<IgesDrawConnectPoint> = Default::default();
            let mut i: i32 = Default::default();
            for i in 1..=temp_nb_entities2 {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Associated Connect Point Entity",
                    <IgesDrawConnectPoint>::static_type(),
                    &mut temp_connect_point,
                    true,
                )? {
                    iges_array1_set(
                        &temp_point_entities,
                        i,
                        temp_connect_point.clone(),
                        "IgesDrawToolNetworkSubfigureDef::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_depth,
            &temp_name,
            &temp_entities,
            temp_type_flag,
            &temp_designator,
            &temp_template,
            &temp_point_entities,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_entities()?;
        iw.send(ent.depth())?;
        iw.send(ent.name())?;
        iw.send(up)?;
        let mut i: i32 = Default::default();
        for i in 1..=up {
            iw.send_entity(&ent.entity(i)?, false)?;
        }
        iw.send(ent.type_flag())?;
        iw.send(ent.designator())?;
        iw.send_entity(&ent.designator_template(), false)?;
        up = ent.nb_point_entities()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send_entity(&ent.point_entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a NetworkSubfigureDef `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut up: i32 = ent.nb_entities()?;
        for i in 1..=up {
            iter.get_one_item(&ent.entity(i)?.cast());
        }
        up = ent.nb_point_entities()?;
        for i in 1..=up {
            iter.get_one_item(&ent.point_entity(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDrawNetworkSubfigureDef>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(320, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.line_weight(IgesDataDefType::DefValue);
        dc.blank_status_ignored();
        dc.use_flag_required(2);
        dc.graphics_ignored(1);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (ent.type_flag() < 0) || (ent.type_flag() > 2) {
            ach.add_fail_text("TypeFlag has Invalid value", "");
        }
        if ent.designator().is_null() {
            ach.add_fail_text("Primary Reference Designator : not defined", "");
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawNetworkSubfigureDef>,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_depth: i32 = another.depth();
        let mut temp_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.name(),
            "IgesDrawToolNetworkSubfigureDef::own_copy",
        )?));
        let mut temp_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut up: i32 = another.nb_entities()?;
        if up > 0 {
            temp_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, up)?;
        }
        let mut i: i32 = Default::default();
        for i in 1..=up {
            let mut temp_entity: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.entity(i)?.cast())?.cast();
            iges_array1_set(
                &temp_entities,
                i,
                temp_entity.clone(),
                "IgesDrawToolNetworkSubfigureDef::own_copy",
            )?;
        }
        let mut temp_type_flag: i32 = another.type_flag();
        let mut temp_designator: Handle<HAsciiString> = Default::default();
        if !(another.designator().is_null()) {
            temp_designator = Handle::new(HAsciiString::copied(iges_live(
                &another.designator(),
                "IgesDrawToolNetworkSubfigureDef::own_copy",
            )?));
        }
        up = another.nb_point_entities()?;
        let mut temp_point_entities: Handle<HArray1<Handle<IgesDrawConnectPoint>>> =
            Default::default();
        if up > 0 {
            temp_point_entities = iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, up)?;
        }
        for i in 1..=up {
            if another.has_point_entity(i)? {
                let mut temp_point_entity: Handle<IgesDrawConnectPoint> =
                    tc.transferred(&another.point_entity(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_point_entities,
                    i,
                    temp_point_entity.clone(),
                    "IgesDrawToolNetworkSubfigureDef::own_copy",
                )?;
            }
        }
        if another.has_designator_template() {
            let mut temp_designator_template: Handle<IgesGraphTextDisplayTemplate> = tc
                .transferred(&another.designator_template().cast())?
                .cast();
            ent.init(
                temp_depth,
                &temp_name,
                &temp_entities,
                temp_type_flag,
                &temp_designator,
                &temp_designator_template,
                &temp_point_entities,
            )?;
        } else {
            let mut temp_designator_template: Handle<IgesGraphTextDisplayTemplate> =
                Default::default();
            ent.init(
                temp_depth,
                &temp_name,
                &temp_entities,
                temp_type_flag,
                &temp_designator,
                &temp_designator_template,
                &temp_point_entities,
            )?;
        }
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigureDef>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDrawNetworkSubfigureDef\nDepth Of Subfigure(Nesting)  : {}\nName Of Subfigure            : ",
            ent.depth()
        )?;
        iges_dump_string(s, &ent.name())?;
        write!(s, "\nAssociated Entities          : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_entities()?, |first| {
            ent.entity(first)
        })?;
        write!(
            s,
            "\nType Flag : {}\nPrimary Reference Designator : ",
            ent.type_flag()
        )?;
        iges_dump_string(s, &ent.designator())?;
        write!(s, "\nText Display Template Entity : ")?;
        dumper.dump(&ent.designator_template(), s, temp_sub_level, -1)?;
        write!(s, "\nConnect Point Entities       : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_point_entities()?, |first| {
            ent.point_entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
