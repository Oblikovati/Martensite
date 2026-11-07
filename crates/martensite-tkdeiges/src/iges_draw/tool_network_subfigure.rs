//! IgesDrawToolNetworkSubfigure.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_entities, iges_dump_string, iges_dump_xyz, iges_dump_xyz_l, iges_live,
    iges_new_array1,
};
use crate::iges_draw::{
    IgesDrawConnectPoint, IgesDrawNetworkSubfigure, IgesDrawNetworkSubfigureDef,
};
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolNetworkSubfigure,
    "The own-parameter work on `IgesDrawNetworkSubfigure`."
);

impl IgesDrawToolNetworkSubfigure {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut definition: Handle<IgesDrawNetworkSubfigureDef> = Default::default();
        crate::iges_shared_blocks::iges_block_0049!(designator, scale, translation, typeflag);
        let mut text_template: Handle<IgesGraphTextDisplayTemplate> = Default::default();
        let mut connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>> = Default::default();
        let mut scale_x: f64 = Default::default();
        let mut scale_y: f64 = Default::default();
        let mut scale_z: f64 = Default::default();
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Instance of NetworkSubfigureDef",
            <IgesDrawNetworkSubfigureDef>::static_type(),
            &mut definition,
            false,
        )?;
        pr.read_xyz(&pr.current_list(1, 3), "Translation data", &mut translation)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Scale factors(X)", &mut scale_x)?;
        } else {
            scale_x = 1.0;
        }
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Scale factors(Y)", &mut scale_y)?;
        } else {
            scale_y = scale_x;
        }
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Scale factors(Z)", &mut scale_z)?;
        } else {
            scale_z = scale_x;
        }
        scale = Xyz::new(scale_x, scale_y, scale_z);
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Type flag", &mut typeflag)?;
        } else {
            typeflag = 0;
        }
        if pr.defined_else_skip()? {
            pr.read_text(
                &pr.current(),
                "Primary reference designator",
                &mut designator,
            )?;
        } else {
            pr.add_warning("Primary reference designator : Null definition", "");
        }
        let mut st: bool = pr.read_entity_typed(
            ir,
            &pr.current(),
            "Instance of TextDisplayTemplate",
            <IgesGraphTextDisplayTemplate>::static_type(),
            &mut text_template,
            true,
        )?;
        if pr.defined_else_skip()? {
            st = pr.read_integer_text(&pr.current(), "Count of Connect Points", &mut nbval)?;
        } else {
            nbval = 0;
        }
        if st && (nbval > 0) {
            connect_points = iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, nbval)?;
            let mut temp_connect_point: Handle<IgesDrawConnectPoint> = Default::default();
            for i in 1..=nbval {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "ConnectPoint entity",
                    <IgesDrawConnectPoint>::static_type(),
                    &mut temp_connect_point,
                    true,
                )? {
                    iges_array1_set(
                        &connect_points,
                        i,
                        temp_connect_point.clone(),
                        "IgesDrawToolNetworkSubfigure::read_own_params",
                    )?;
                }
            }
        } else {
            if nbval < 0 {
                pr.add_fail("Count of Connect point entities : Less than Zero", "");
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &definition,
            translation,
            scale,
            typeflag,
            &designator,
            &text_template,
            &connect_points,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.subfigure_definition(), false)?;
        iw.send(ent.translation().x())?;
        iw.send(ent.translation().y())?;
        iw.send(ent.translation().z())?;
        iw.send(ent.scale_factors().x())?;
        iw.send(ent.scale_factors().y())?;
        iw.send(ent.scale_factors().z())?;
        iw.send(ent.type_flag())?;
        iw.send(ent.reference_designator())?;
        iw.send_entity(&ent.designator_template(), false)?;
        iw.send(ent.nb_connect_points()?)?;
        let mut up: i32 = ent.nb_connect_points()?;
        for i in 1..=up {
            iw.send_entity(&ent.connect_point(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a NetworkSubfigure `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.subfigure_definition().cast());
        iter.get_one_item(&ent.designator_template().cast());
        let mut up: i32 = ent.nb_connect_points()?;
        for i in 1..=up {
            iter.get_one_item(&ent.connect_point(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawNetworkSubfigure>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(420, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.line_weight(IgesDataDefType::DefValue);
        dc.color(IgesDataDefType::DefAny);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ((ent.type_flag() != 0) && (ent.type_flag() != 1)) && (ent.type_flag() != 2) {
            ach.add_fail_text("Type Flag : Value != 0/1/2", "");
        }
        if ent.nb_connect_points()?
            != (iges_live(
                &ent.subfigure_definition(),
                "IgesDrawToolNetworkSubfigure::own_check",
            )?
            .nb_point_entities()?)
        {
            ach.add_fail_text(
                "Count of associated Connect Points inconsistent with Definition",
                "",
            );
        }
        if ent.reference_designator().is_null() {
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
        another: &Handle<IgesDrawNetworkSubfigure>,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0049!(designator, scale, translation, typeflag);
        let mut connect_points: Handle<HArray1<Handle<IgesDrawConnectPoint>>> = Default::default();
        nbval = another.nb_connect_points()?;
        let mut definition: Handle<IgesDrawNetworkSubfigureDef> = tc
            .transferred(&another.subfigure_definition().cast())?
            .cast();
        translation = another.translation();
        scale = another.scale_factors();
        typeflag = another.type_flag();
        if !(another.reference_designator().is_null()) {
            designator = Handle::new(HAsciiString::copied(iges_live(
                &another.reference_designator(),
                "IgesDrawToolNetworkSubfigure::own_copy",
            )?));
        }
        let mut text_template: Handle<IgesGraphTextDisplayTemplate> = tc
            .transferred(&another.designator_template().cast())?
            .cast();
        if nbval > 0 {
            connect_points = iges_new_array1::<Handle<IgesDrawConnectPoint>>(1, nbval)?;
        }
        for i in 1..=nbval {
            let mut tempconnect_point: Handle<IgesDrawConnectPoint> =
                tc.transferred(&another.connect_point(i)?.cast())?.cast();
            iges_array1_set(
                &connect_points,
                i,
                tempconnect_point.clone(),
                "IgesDrawToolNetworkSubfigure::own_copy",
            )?;
        }
        ent.init(
            &definition,
            translation,
            scale,
            typeflag,
            &designator,
            &text_template,
            &connect_points,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawNetworkSubfigure>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDrawNetworkSubfigure\nNetwork Subfigure Definition Entity : "
        )?;
        dumper.dump(&ent.subfigure_definition(), s, sublevel, -1)?;
        write!(s, "\nTranslation Data : ")?;
        iges_dump_xyz_l(s, level, &ent.translation(), &ent.location()?)?;
        write!(s, "\nScale Factors    : ")?;
        iges_dump_xyz(s, &ent.scale_factors())?;
        write!(
            s,
            "\nType Flag : {}\nPrimary Reference Designator : ",
            ent.type_flag()
        )?;
        iges_dump_string(s, &ent.reference_designator())?;
        write!(s, "\nText Display Template Entity : ")?;
        dumper.dump(&ent.designator_template(), s, sublevel, -1)?;
        write!(s, "\nConnect Points  : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_connect_points()?, |first| {
            ent.connect_point(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
