//! The modules of the undefined entities, which are the ones of the basic protocol.
use super::{
    IgesDataDirChecker, IgesDataDumpResult, IgesDataFreeFormatEntity, IgesDataGeneralModule,
    IgesDataIgesDumper, IgesDataIgesEntity, IgesDataSpecificModule, IgesDataUndefinedEntity,
    iges_live,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceGeneralModule,
    InterfaceParamType, InterfaceShareTool, InterfaceUndefinedContent,
};
use std::io::Write;

/// The general services on the undefined entities.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataDefaultGeneral, IgesDataGeneralModule};
/// let mut made = Default::default();
/// assert!(IgesDataDefaultGeneral::default().iges_new_void(1, &mut made));
/// assert!(!IgesDataDefaultGeneral::default().iges_new_void(0, &mut made));
/// ```
#[derive(Default)]
pub struct IgesDataDefaultGeneral {
    transient: StandardTransient,
}
impl_transient!(
    IgesDataDefaultGeneral,
    "IgesDataDefaultGeneral",
    dyn IgesDataGeneralModule,
    transient,
    interfaces: [dyn InterfaceGeneralModule, dyn IgesDataGeneralModule]
);
crate::impl_iges_general_module!(IgesDataDefaultGeneral);

impl IgesDataGeneralModule for IgesDataDefaultGeneral {
    fn own_shared_case(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if cn == 0 {
            return Ok(());
        }
        let undefined = ent.cast::<IgesDataUndefinedEntity>();
        let Some(live) = undefined.get() else {
            return Ok(());
        };
        let content = live.undefined_content();
        let cont = iges_live(&content, "undefined entity")?;
        for i in 1..=cont.nb_params() {
            if cont.is_param_entity(i)? {
                iter.get_one_item(&cont.param_entity(i)?);
            }
        }
        Ok(())
    }
    fn dir_checker(&self, _cn: i32, _ent: &Handle<IgesDataIgesEntity>) -> IgesDataDirChecker {
        IgesDataDirChecker::new()
    }
    fn own_check_case(
        &self,
        _cn: i32,
        _ent: &Handle<IgesDataIgesEntity>,
        _shares: &InterfaceShareTool,
        _ach: &mut Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    fn iges_new_void(&self, cn: i32, entto: &mut Handle<StandardTransient>) -> bool {
        *entto = match cn {
            1 => Handle::new(IgesDataUndefinedEntity::new()).cast(),
            2 => Handle::new(IgesDataFreeFormatEntity::new()).cast(),
            _ => Handle::default(),
        };
        entto.is_some()
    }
    fn own_copy_case(
        &self,
        cn: i32,
        entfrom: &Handle<IgesDataIgesEntity>,
        entto: &Handle<IgesDataIgesEntity>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        if cn == 0 {
            return Ok(());
        }
        let from = entfrom.cast::<IgesDataUndefinedEntity>();
        let to = entto.cast::<IgesDataUndefinedEntity>();
        let source = iges_live(&from, "undefined entity: copy")?;
        let target = iges_live(&to, "undefined entity: copy")?;
        let cont = Handle::new(InterfaceUndefinedContent::new());
        iges_live(&cont, "undefined entity: copy")?
            .get_from_another(&source.undefined_content(), tc)?;
        target.set_new_content(&cont);
        let free_from = entfrom.cast::<IgesDataFreeFormatEntity>();
        let free_to = entto.cast::<IgesDataFreeFormatEntity>();
        if let (Some(source), Some(target)) = (free_from.get(), free_to.get()) {
            target.clear_negative_pointers();
            target.add_negative_pointers(&source.negative_pointers());
        }
        Ok(())
    }
}

/// The dump of the undefined entities.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataDefaultSpecific;
/// let _module = IgesDataDefaultSpecific::default();
/// ```
#[derive(Default)]
pub struct IgesDataDefaultSpecific {
    transient: StandardTransient,
}
impl_transient!(
    IgesDataDefaultSpecific,
    "IgesDataDefaultSpecific",
    dyn IgesDataSpecificModule,
    transient,
    interfaces: [dyn IgesDataSpecificModule]
);

impl IgesDataSpecificModule for IgesDataDefaultSpecific {
    fn own_dump(
        &self,
        _cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let undefined = ent.cast::<IgesDataUndefinedEntity>();
        let Some(live) = undefined.get() else {
            return Ok(());
        };
        let dstat = live.dir_status();
        if dstat != 0 {
            writeln!(s, " --  Directory Entry Error Status = {dstat}  --")?;
        }
        let content = live.undefined_content();
        let cont = iges_live(&content, "undefined entity: dump")?;
        let nb = cont.nb_params();
        writeln!(s, " UNDEFINED ENTITY ...")?;
        writeln!(
            s,
            "{nb} Parameters (WARNING : Odd Integer Values Interpreted as Entities)"
        )?;
        for i in 1..=nb {
            if cont.param_type(i)? == InterfaceParamType::ParamVoid {
                write!(s, "\t[{i}:Void]")?;
            } else if cont.is_param_entity(i)? {
                write!(s, "\t[{i}:IGES]=")?;
                dumper.print_d_num(&cont.param_entity(i)?, s)?;
            } else {
                write!(s, "\t[{i}]=")?;
                if let Some(text) = cont.param_value(i)?.get() {
                    s.write_all(text.string().as_bytes())?;
                }
            }
            if i == (i % 5) * 5 {
                writeln!(s)?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
