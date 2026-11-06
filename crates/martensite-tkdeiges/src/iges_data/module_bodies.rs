//! What every general module does with the directory part, around the own members of its package.
use super::{IgesDataDefList, IgesDataDefType, IgesDataGeneralModule, IgesDataIgesEntity};
use martensite_tkernel::{
    standard::{Failure, Handle, RuntimeTyped, StandardTransient, exceptions},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};

/// Adds the entities of the directory part, the own ones, and the properties.
pub fn iges_fill_shared_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    ent: &Handle<StandardTransient>,
    iter: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    let anent = ent.cast::<IgesDataIgesEntity>();
    let Some(live) = anent.get() else {
        return Ok(());
    };
    for field in [3, 4, 5, 6, 7, 8, 13] {
        iter.add_item(&live.dir_field_entity(field).cast());
    }
    module.own_shared_case(cn, &anent, iter)?;
    add_all(&live.properties(), iter)
}

fn add_all(
    from: &InterfaceEntityIterator,
    iter: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    from.start();
    while from.more() {
        iter.add_item(&from.value()?);
        from.next();
    }
    Ok(())
}

/// Adds the entities the own parameters imply, and the associativities.
pub fn iges_list_implied_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    ent: &Handle<StandardTransient>,
    iter: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    let anent = ent.cast::<IgesDataIgesEntity>();
    let Some(live) = anent.get() else {
        return Ok(());
    };
    module.own_implied_case(cn, &anent, iter)?;
    add_all(&live.associativities(), iter)
}

/// Checks the directory entry, then the own parameters.
pub fn iges_check_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    ent: &Handle<StandardTransient>,
    shares: &InterfaceShareTool,
    ach: &mut Handle<InterfaceCheck>,
) -> Result<(), Failure> {
    let anent = ent.cast::<IgesDataIgesEntity>();
    module.dir_checker(cn, &anent).check(ach, &anent);
    module.own_check_case(cn, &anent, shares, ach)
}

/// The entity another one was copied to, null for null.
fn copied_kind<T: ?Sized + RuntimeTyped>(
    tc: &mut InterfaceCopyTool,
    from: &Handle<T>,
) -> Result<Handle<T>, Failure> {
    if from.is_null() {
        return Ok(Handle::default());
    }
    Ok(tc.transferred(&from.cast())?.cast())
}

/// Copies the directory part, the own parameters and the properties.
pub fn iges_copy_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    entfrom: &Handle<StandardTransient>,
    entto: &Handle<StandardTransient>,
    tc: &mut InterfaceCopyTool,
) -> Result<(), Failure> {
    let enfr = entfrom.cast::<IgesDataIgesEntity>();
    let ento = entto.cast::<IgesDataIgesEntity>();
    let (Some(from), Some(to)) = (enfr.get(), ento.get()) else {
        return Err(exceptions::null_object(
            "IGES copy: the entities must be IGES entities and not null",
        ));
    };
    copy_directory(from, to, tc)?;
    module.own_copy_case(cn, &enfr, &ento, tc)?;
    if from.nb_properties() != 0 {
        let iter = from.properties();
        iter.start();
        while iter.more() {
            let property = copied_kind(tc, &iter.value()?)?.cast::<IgesDataIgesEntity>();
            to.add_property(&property)?;
            iter.next();
        }
    }
    Ok(())
}

fn copy_directory(
    from: &IgesDataIgesEntity,
    to: &IgesDataIgesEntity,
    tc: &mut InterfaceCopyTool,
) -> Result<(), Failure> {
    if from.def_line_font() == IgesDataDefType::DefReference {
        to.init_line_font(&copied_kind(tc, &from.line_font())?, 0);
    } else {
        to.init_line_font(&from.line_font(), from.rank_line_font());
    }
    if from.def_level() == IgesDataDefList::DefSeveral {
        to.init_level(&copied_kind(tc, &from.level_list())?, -1);
    } else {
        to.init_level(&from.level_list(), from.level());
    }
    if from.def_view() != IgesDataDefList::DefNone {
        to.init_view(&copied_kind(tc, &from.view())?);
    }
    if from.has_transf() {
        to.init_transf(&copied_kind(tc, &from.transf())?);
    }
    to.init_status(
        from.blank_status(),
        from.subordinate_status(),
        from.use_flag(),
        from.hierarchy_status(),
    );
    if from.def_color() == IgesDataDefType::DefReference {
        to.init_color(&copied_kind(tc, &from.color())?, 0);
    } else {
        to.init_color(&from.color(), from.rank_color());
    }
    let label = from.short_label();
    let copied_label = label.get().map_or_else(Handle::default, |text| {
        Handle::new(HAsciiString::new(text.string().as_bytes()))
    });
    to.set_label(&copied_label, from.sub_script_number());
    let structure = if from.has_structure() {
        copied_kind(tc, &from.structure())?
    } else {
        Handle::default()
    };
    let label_display = if from.has_label_display() {
        copied_kind(tc, &from.label_display())?
    } else {
        from.label_display()
    };
    to.init_misc(&structure, &label_display, from.line_weight_number());
    Ok(())
}

/// Renews the own references, then the associativities whose entity was copied.
pub fn iges_renew_implied_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    entfrom: &Handle<StandardTransient>,
    entto: &Handle<StandardTransient>,
    tc: &InterfaceCopyTool,
) -> Result<(), Failure> {
    let enfr = entfrom.cast::<IgesDataIgesEntity>();
    let ento = entto.cast::<IgesDataIgesEntity>();
    module.own_renew_case(cn, &enfr, &ento, tc)?;
    let (Some(from), Some(to)) = (enfr.get(), ento.get()) else {
        return Ok(());
    };
    if from.nb_associativities() == 0 {
        return Ok(());
    }
    let iter = from.associativities();
    iter.start();
    while iter.more() {
        let anent = iter.value()?;
        let mut newent = Handle::default();
        if tc.search(&anent, &mut newent)? {
            to.add_associativity(&newent)?;
        }
        iter.next();
    }
    Ok(())
}

/// Forgets the content of the entity, then what the package adds.
pub fn iges_when_delete_case<M: IgesDataGeneralModule + ?Sized>(
    module: &M,
    cn: i32,
    ent: &Handle<StandardTransient>,
) {
    let anent = ent.cast::<IgesDataIgesEntity>();
    let Some(live) = anent.get() else {
        return;
    };
    live.clear();
    module.own_delete_case(cn, &anent);
}

/// The name of an entity.
pub fn iges_name_of(ent: &Handle<StandardTransient>) -> Handle<HAsciiString> {
    ent.cast::<IgesDataIgesEntity>()
        .get()
        .and_then(|live| live.name_value().ok())
        .unwrap_or_default()
}
