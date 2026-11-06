//! IgesDrawToolDrawing.
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
    IgesDataViewKindEntity, iges_array1_set, iges_dump_entities, iges_dump_xy, iges_live,
    iges_new_array1,
};
use crate::iges_draw::IgesDrawDrawing;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolDrawing,
    "The own-parameter work on `IgesDrawDrawing`."
);

impl IgesDrawToolDrawing {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawDrawing>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0041!(annotations, nbval, view_origins, views);
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Count of array of view entities", &mut nbval)?;
        if st && (nbval > 0) {
            views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
            view_origins = iges_new_array1::<Xy>(1, nbval)?;
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> = Default::default();
            let mut temp_xy: Xy = Default::default();
            for i in 1..=nbval {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "View Entity",
                    <dyn IgesDataViewKindEntity>::static_type(),
                    &mut temp_view,
                    true,
                )? {
                    iges_array1_set(
                        &views,
                        i,
                        temp_view.clone(),
                        "IgesDrawToolDrawing::read_own_params",
                    )?;
                }
                if pr.read_xy(&pr.current_list(1, 2), "array viewOrigins", &mut temp_xy)? {
                    iges_array1_set(
                        &view_origins,
                        i,
                        temp_xy,
                        "IgesDrawToolDrawing::read_own_params",
                    )?;
                }
            }
        } else {
            if nbval < 0 {
                pr.add_fail("Count of view entities : Less than zero", "");
            }
        }
        crate::iges_shared_blocks::iges_block_0066!(annotations, ent, ir, nbval, pr, self);
        ent.init(&views, &view_origins, &annotations)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawDrawing>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_views()?;
        iw.send(up)?;
        let mut i: i32 = Default::default();
        for i in 1..=up {
            iw.send_entity(&ent.view_item(i)?, false)?;
            iw.send(ent.view_origin(i)?.x())?;
            iw.send(ent.view_origin(i)?.y())?;
        }
        up = ent.nb_annotations()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send_entity(&ent.annotation(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a Drawing `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawDrawing>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0033!(ent, i, iter, up);
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a Drawing
    /// (Null Views are removed from list)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDrawDrawing>) -> Result<bool, Failure> {
        let mut i: i32 = Default::default();
        let mut nb: i32 = ent.nb_views()?;
        let mut nbtrue: i32 = nb;
        for i in 1..=nb {
            let mut val: Handle<dyn IgesDataViewKindEntity> = ent.view_item(i)?;
            if val.is_null() {
                nbtrue -= 1;
            } else {
                if (iges_live(&val, "IgesDrawToolDrawing::own_correct")?
                    .iges_entity()
                    .type_number())
                    == 0
                {
                    nbtrue -= 1;
                }
            }
        }
        if nbtrue == nb {
            return Ok(false);
        }
        let mut views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> = Default::default();
        let mut view_origins: Handle<HArray1<Xy>> = Default::default();
        if nbtrue > 0 {
            views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbtrue)?;
            view_origins = iges_new_array1::<Xy>(1, nbtrue)?;
        }
        nbtrue = 0;
        for i in 1..=nb {
            let mut val: Handle<dyn IgesDataViewKindEntity> = ent.view_item(i)?;
            if val.is_null() {
                continue;
            } else {
                if (iges_live(&val, "IgesDrawToolDrawing::own_correct")?
                    .iges_entity()
                    .type_number())
                    == 0
                {
                    continue;
                }
            }
            nbtrue += 1;
            iges_array1_set(
                &views,
                nbtrue,
                val.clone(),
                "IgesDrawToolDrawing::own_correct",
            )?;
            iges_array1_set(
                &view_origins,
                nbtrue,
                ent.view_origin(i)?.xy(),
                "IgesDrawToolDrawing::own_correct",
            )?;
        }
        let mut nbanot: i32 = ent.nb_annotations()?;
        let mut annotations: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbanot)?;
        for i in 1..=nbanot {
            iges_array1_set(
                &annotations,
                i,
                ent.annotation(i)?,
                "IgesDrawToolDrawing::own_correct",
            )?;
        }
        ent.init(&views, &view_origins, &annotations)?;
        return Ok(true);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawDrawing>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(404, 0);
        crate::iges_shared_blocks::iges_block_0013!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawDrawing>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut ianul: bool = false;
        let mut i: i32 = Default::default();
        let mut nb: i32 = ent.nb_views()?;
        for i in 1..=nb {
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> = ent.view_item(i)?;
            if temp_view.is_null() {
                ianul = true;
            } else {
                if (iges_live(&temp_view, "IgesDrawToolDrawing::own_check")?
                    .iges_entity()
                    .type_number())
                    == 0
                {
                    ianul = true;
                }
            }
            if ianul {
                ach.add_warning_text("At least one View is Null", "");
                break;
            }
        }
        nb = ent.nb_annotations()?;
        for i in 1..=nb {
            let mut ann: Handle<IgesDataIgesEntity> = ent.annotation(i)?;
            if ann.is_null() {
                ianul = true;
            } else {
                if (iges_live(&ann, "IgesDrawToolDrawing::own_check")?.type_number()) == 0 {
                    ianul = true;
                }
            }
            if ianul {
                ach.add_warning_text("At least one Annotation is Null", "");
                break;
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
        another: &Handle<IgesDrawDrawing>,
        ent: &Handle<IgesDrawDrawing>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbanot: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0041!(annotations, nbval, view_origins, views);
        nbanot = another.nb_annotations()?;
        nbval = another.nb_views()?;
        if nbval > 0 {
            views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
            view_origins = iges_new_array1::<Xy>(1, nbval)?;
        }
        if nbanot > 0 {
            annotations = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbanot)?;
            for i in 1..=nbanot {
                let mut temp_annotation: Handle<IgesDataIgesEntity> =
                    tc.transferred(&another.annotation(i)?.cast())?.cast();
                iges_array1_set(
                    &annotations,
                    i,
                    temp_annotation.clone(),
                    "IgesDrawToolDrawing::own_copy",
                )?;
            }
        }
        for i in 1..=nbval {
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> =
                tc.transferred(&another.view_item(i)?.cast())?.cast();
            iges_array1_set(
                &views,
                i,
                temp_view.clone(),
                "IgesDrawToolDrawing::own_copy",
            )?;
            iges_array1_set(
                &view_origins,
                i,
                another.view_origin(i)?.xy(),
                "IgesDrawToolDrawing::own_copy",
            )?;
        }
        ent.init(&views, &view_origins, &annotations)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawDrawing>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDrawDrawing\nView Entities            :\nTransformed View Origins : Count = {}",
            ent.nb_views()?
        )?;
        match level {
            4 => {
                writeln!(s, " [ ask level > 4 for content ]")?;
            }
            5 => {
                writeln!(s)?;
                {
                    let mut i: i32 = Default::default();
                    let mut up: i32 = ent.nb_views()?;
                    for i in 1..=up {
                        write!(s, "\n[{}] View Entity : ", i)?;
                        dumper.dump(&ent.view_item(i)?, s, sublevel, -1)?;
                        write!(s, "\nTransformed View Origin : ")?;
                        iges_dump_xy(s, &ent.view_origin(i)?.xy())?;
                    }
                }
            }
            6 => {
                let mut i: i32 = Default::default();
                let mut up: i32 = ent.nb_views()?;
                for i in 1..=up {
                    write!(s, "\n[{}] View Entity : ", i)?;
                    dumper.dump(&ent.view_item(i)?, s, sublevel, -1)?;
                    write!(s, "\nTransformed View Origin : ")?;
                    iges_dump_xy(s, &ent.view_origin(i)?.xy())?;
                }
            }
            _ => {}
        }
        write!(s, "\nAnnotation Entities : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_annotations()?, |first| {
            ent.annotation(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
