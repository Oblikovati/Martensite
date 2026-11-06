//! The reading, writing and dumping of files of the norm by a work session.
use super::IgesSelectFileModifier;
use crate::iges_appli::IgesAppli;
use crate::iges_data::{
    IgesDataFileProtocol, IgesDataIgesDumper, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataIgesWriter, IgesDataProtocolOps,
};
use crate::iges_defs::IgesDefs;
use crate::iges_file::iges_file_read_stream;
use crate::iges_solid::IgesSolid;
use martensite_tkernel::{
    impl_transient,
    message::Message,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkxsbase::{
    if_select::{IFSelectContextWrite, IFSelectWorkLibrary, IFSelectWorkLibraryBase},
    interface::{
        InterfaceCheckIterator, InterfaceInterfaceModel, InterfaceModelEntities,
        InterfaceModelReports, InterfaceProtocol,
    },
};
use std::fmt::Write as _;
use std::io::{Read, Write};
use std::sync::OnceLock;

/// The work library of the norm; `modefnes` writes the enciphered form.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectWorkLibrary;
/// use martensite_tkxsbase::if_select::IFSelectWorkLibrary;
/// assert_eq!(IgesSelectWorkLibrary::new(false).dump_levels(), (4, 6));
/// ```
pub struct IgesSelectWorkLibrary {
    transient: StandardTransient,
    base: IFSelectWorkLibraryBase,
    themodefnes: bool,
}
impl_transient!(
    IgesSelectWorkLibrary,
    "IgesSelectWorkLibrary",
    dyn IFSelectWorkLibrary,
    transient,
    interfaces: [dyn IFSelectWorkLibrary]
);

impl IgesSelectWorkLibrary {
    /// The library; it sets up the packages of the protocol.
    pub fn new(modefnes: bool) -> Self {
        IgesSolid::init();
        IgesAppli::init();
        IgesDefs::init();
        let library = Self {
            transient: StandardTransient::default(),
            base: IFSelectWorkLibraryBase::new(),
            themodefnes: modefnes,
        };
        library.set_dump_levels(4, 6);
        library.set_dump_help(0, "Only DNum");
        library.set_dump_help(1, "DNum, IGES Type & Form");
        library.set_dump_help(2, "Main Directory Information");
        library.set_dump_help(3, "Complete Directory Part");
        library.set_dump_help(4, "Directory + Fields (except list contents)");
        library.set_dump_help(5, "Complete (with list contents)");
        library.set_dump_help(6, "Complete + Transformed data");
        library
    }
    /// The protocol of the whole norm.
    pub fn define_protocol() -> Handle<dyn IgesDataProtocolOps> {
        static PROTOCOL: OnceLock<Handle<dyn IgesDataProtocolOps>> = OnceLock::new();
        PROTOCOL
            .get_or_init(|| {
                let file = IgesDataFileProtocol::default();
                file.add(&IgesSolid::protocol().cast());
                file.add(&IgesAppli::protocol().cast());
                Handle::new(file).cast()
            })
            .clone()
    }
    fn apply_file_modifiers(
        &self,
        ctx: &mut IFSelectContextWrite,
        writer: &mut IgesDataIgesWriter,
    ) -> Result<bool, Failure> {
        for rank in 1..=ctx.nb_modifiers() {
            ctx.set_modifier(rank)?;
            let modifier = ctx.file_modifier();
            if modifier.is_null() {
                continue;
            }
            let file_modifier = modifier.cast::<dyn IgesSelectFileModifier>();
            let Some(live) = file_modifier.get() else {
                if let Some(check) = ctx.c_check(0)?.get() {
                    check.add_fail_text("IGES file modifier has an incompatible type", "");
                }
                return Ok(false);
            };
            live.perform(ctx, writer)?;
        }
        Ok(true)
    }
}

impl IFSelectWorkLibrary for IgesSelectWorkLibrary {
    fn work_library_base(&self) -> &IFSelectWorkLibraryBase {
        &self.base
    }
    fn read_file(
        &self,
        name: &str,
        model: &mut Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<i32, Failure> {
        let Ok(mut file) = std::fs::File::open(name) else {
            let _ = writeln!(Message::send_info(), "File not found : {name}");
            model.nullify();
            return Ok(-1);
        };
        self.read_stream(name, &mut file, model, protocol)
    }
    fn read_stream(
        &self,
        name: &str,
        istream: &mut dyn Read,
        model: &mut Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<i32, Failure> {
        let target = IgesDataIgesModel::new();
        let iges = protocol.cast::<dyn IgesDataProtocolOps>();
        let status = iges_file_read_stream(name, istream, &target, &iges)?;
        if status < 0 {
            let _ = writeln!(Message::send_info(), "Stream read failed : {name}");
        } else if status > 0 {
            let _ = writeln!(Message::send_info(), "Error when reading stream : {name}");
        }
        *model = if status == 0 {
            target.cast()
        } else {
            Handle::default()
        };
        Ok(status)
    }
    fn write_file(&self, ctx: &mut IFSelectContextWrite) -> Result<bool, Failure> {
        if ctx.model().cast::<IgesDataIgesModel>().is_null()
            || ctx.protocol().cast::<dyn IgesDataProtocolOps>().is_null()
        {
            return Ok(false);
        }
        let name = String::from_utf8_lossy(ctx.file_name().as_bytes()).into_owned();
        let Ok(mut file) = std::fs::File::create(&name) else {
            if let Some(check) = ctx.c_check(0)?.get() {
                check.add_fail_text("IGES File could not be created", "");
            }
            let _ = writeln!(
                Message::send_info(),
                " - IGES File could not be created : {name}"
            );
            return Ok(false);
        };
        let written = self.write_stream(ctx, &mut file)?;
        let flushed = file.flush();
        if let Err(error) = &flushed {
            let _ = writeln!(Message::send_info(), "{error}");
        }
        Ok(written && flushed.is_ok())
    }
    fn write_stream(
        &self,
        ctx: &mut IFSelectContextWrite,
        ostream: &mut dyn Write,
    ) -> Result<bool, Failure> {
        let model = ctx.model().cast::<IgesDataIgesModel>();
        let protocol = ctx.protocol().cast::<dyn IgesDataProtocolOps>();
        if model.is_null() || protocol.is_null() {
            return Ok(false);
        }
        let mut writer = IgesDataIgesWriter::new(&model);
        if !self.apply_file_modifiers(ctx, &mut writer)? {
            return Ok(false);
        }
        writer.send_model(&protocol)?;
        if self.themodefnes {
            *writer.write_mode() = 10;
        }
        writer.print(ostream)
    }
    fn dump_entity(
        &self,
        model: &Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
        entity: &Handle<StandardTransient>,
        s: &mut dyn Write,
        level: i32,
    ) -> Result<(), Failure> {
        let igesmod = model.cast::<IgesDataIgesModel>();
        let igespro = protocol.cast::<dyn IgesDataProtocolOps>();
        let mut igesent = entity.cast::<IgesDataIgesEntity>();
        let Some(live) = igesmod.get() else {
            return Ok(());
        };
        if igespro.is_null() || igesent.is_null() {
            return Ok(());
        }
        let num = live.number(entity);
        if num == 0 {
            return Ok(());
        }
        let io = |error: std::io::Error| {
            Failure::new(&format!("IGES work library: the stream failed: {error}"))
        };
        write!(s, " --- Entity {num}").map_err(io)?;
        if live.is_redefined_content(num) {
            let report = live.report_entity(num, false);
            let content = report
                .get()
                .map_or_else(Handle::default, |rep| rep.content());
            write!(s, " ERRONEOUS, Content, Type cdl : ").map_err(io)?;
            match content.dynamic_type() {
                Some(kind) => write!(s, "{}", kind.name()).map_err(io)?,
                None => writeln!(s, "(undefined)").map_err(io)?,
            }
            igesent = content.cast();
            let mut checks = InterfaceCheckIterator::new();
            if let Some(rep) = report.get() {
                checks.add(&rep.check(), num);
            }
            checks.print_with_model(s, model, false, 0).map_err(io)?;
            if igesent.is_null() {
                return Ok(());
            }
        } else {
            let name = igesent.dynamic_type().map_or("", |kind| kind.name());
            write!(s, " Type cdl : {name}").map_err(io)?;
        }
        let dumper = IgesDataIgesDumper::new(&igesmod, &igespro);
        if dumper.dump(&igesent, s, level, (level - 1) / 3).is_err() {
            writeln!(s, " **  Dump Interrupt **").map_err(io)?;
        }
        Ok(())
    }
}
