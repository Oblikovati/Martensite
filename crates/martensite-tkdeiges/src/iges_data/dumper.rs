//! The dump of IGES entities as text.
use super::{
    IgesDataDefList, IgesDataDefType, IgesDataDumpResult, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataProtocolOps, IgesDataSpecificLib, iges_entity_of, iges_real_text,
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceModelEntities, InterfaceMsg, InterfaceSignTypeBase,
};
use std::io::Write;

/// The dump of the entities of a model.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataIgesDumper, IgesDataIgesEntity, IgesDataIgesModel, IgesDataProtocol};
/// use martensite_tkernel::standard::Handle;
/// let protocol = Handle::new(IgesDataProtocol::default());
/// let dumper = IgesDataIgesDumper::new(&IgesDataIgesModel::new(), &protocol.cast());
/// let mut text = Vec::new();
/// dumper.print_d_num(&Handle::<IgesDataIgesEntity>::default(), &mut text).unwrap();
/// assert_eq!(text, b"  D0(Null)  ");
/// ```
pub struct IgesDataIgesDumper {
    themodel: Handle<IgesDataIgesModel>,
    thelib: IgesDataSpecificLib,
}

impl IgesDataIgesDumper {
    /// A dumper for the entities of a model, by the modules of a protocol.
    pub fn new(
        model: &Handle<IgesDataIgesModel>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
    ) -> Self {
        Self {
            themodel: model.clone(),
            thelib: IgesDataSpecificLib::new(protocol),
        }
    }
    fn number_of(&self, ent: &Handle<IgesDataIgesEntity>) -> i32 {
        self.themodel
            .get()
            .map_or(0, |model| model.number(&ent.cast()))
    }
    /// Writes a directory number, on twelve characters.
    ///
    /// # Errors
    /// A failure of the stream.
    pub fn print_d_num<T: ?Sized + RuntimeTyped>(
        &self,
        ent: &Handle<T>,
        s: &mut dyn Write,
    ) -> IgesDataDumpResult {
        let entity = iges_entity_of(ent);
        if entity.is_null() {
            write!(s, "  D0(Null)  ")?;
        } else if self.themodel.is_null() {
            write!(s, "    D???    ")?;
        } else {
            match self.number_of(&entity) {
                0 => write!(s, "    0:D?????")?,
                num => write!(
                    s,
                    "{}{num}:D{}{}",
                    InterfaceMsg::blanks_for_integer(num, 9),
                    2 * num - 1,
                    InterfaceMsg::blanks_for_integer(2 * num - 1, 9)
                )?,
            }
        }
        Ok(())
    }
    /// Writes the number, type, form and class of an entity.
    ///
    /// # Errors
    /// A failure of the stream.
    pub fn print_short<T: ?Sized + RuntimeTyped>(
        &self,
        ent: &Handle<T>,
        s: &mut dyn Write,
    ) -> IgesDataDumpResult {
        let entity = iges_entity_of(ent);
        let Some(live) = entity.get() else {
            return Ok(());
        };
        let num = self.number_of(&entity);
        if num > 0 {
            write!(s, "{num}:D{}", 2 * num - 1)?;
        }
        let form = live.form_number();
        let class = entity
            .dynamic_type()
            .map_or("", |kind| InterfaceSignTypeBase::class_name(kind.name()));
        write!(
            s,
            "  Type:{}  Form:{form}{} Class:{class}",
            live.type_number(),
            InterfaceMsg::blanks_for_integer(form, 3)
        )?;
        Ok(())
    }
    /// Writes an entity at a level `own`: the higher, the more it says. `attached` is the level of
    /// its properties and associativities, 0 for the one that goes with `own`, negative for none.
    ///
    /// # Errors
    /// A failure of the stream or of a module.
    pub fn dump<T: ?Sized + RuntimeTyped>(
        &self,
        ent: &Handle<T>,
        s: &mut dyn Write,
        own: i32,
        attached: i32,
    ) -> IgesDataDumpResult {
        let entity = iges_entity_of(ent);
        let diratt = if own < 3 { own - 1 } else { 1 };
        let att = if attached == 0 { diratt } else { attached };
        if own < 0 {
            return Ok(());
        }
        if own > 1 {
            writeln!(s)?;
        }
        let Some(live) = entity.get() else {
            write!(s, "(Null)")?;
            if own > 1 {
                writeln!(s)?;
            }
            return Ok(());
        };
        match own {
            0 => return self.print_d_num(&entity, s),
            1 => return self.print_short(&entity, s),
            _ => {}
        }
        writeln!(
            s,
            "****    Dump IGES, level {own} ( {} )    ****",
            level_name(own)
        )?;
        write!(s, "\n****    Entity ")?;
        self.print_short(&entity, s)?;
        writeln!(s)?;
        self.dump_directory(&entity, live, s, own, diratt)?;
        if att < 0 {
            return Ok(());
        }
        self.dump_attached(live, s, att)
    }
    fn dump_directory(
        &self,
        entity: &Handle<IgesDataIgesEntity>,
        live: &IgesDataIgesEntity,
        s: &mut dyn Write,
        own: i32,
        diratt: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "              Directory Part")?;
        writeln!(
            s,
            "**      Status Number :   Blank:{}    Subordinate:{}    UseFlag:{}    Hierarchy:{}",
            live.blank_status(),
            live.subordinate_status(),
            live.use_flag(),
            live.hierarchy_status()
        )?;
        self.dump_names(live, s)?;
        self.dump_ref(
            s,
            "**      Structure     :",
            live.has_structure(),
            &live.structure(),
        )?;
        writeln!(s, "\n              Graphic Attributes")?;
        match live.def_line_font() {
            IgesDataDefType::DefValue => {
                writeln!(s, "**      LineFont Value:{}", live.rank_line_font())?;
            }
            IgesDataDefType::DefReference => {
                self.dump_ref(s, "**      LineFont Ref  :", true, &live.line_font())?;
            }
            _ => {}
        }
        if live.level() > 0 {
            writeln!(s, "**      Level Value   :{}", live.level())?;
        } else if live.level() < 0 {
            self.dump_ref(s, "**      Level List    :", true, &live.level_list())?;
        }
        self.dump_ref(
            s,
            "**      Label Display :",
            live.has_label_display(),
            &live.label_display(),
        )?;
        if live.line_weight_number() != 0 {
            write!(s, "**      LineWeight Num:{}", live.line_weight_number())?;
            if diratt > 0 {
                write!(s, "    ->  Value:{}", iges_real_text(live.line_weight()))?;
            }
            writeln!(s)?;
        }
        match live.def_color() {
            IgesDataDefType::DefValue => write!(s, "**      Color Value   :{}", live.rank_color())?,
            IgesDataDefType::DefReference => {
                write!(s, "**      Color Ref     :")?;
                self.print_d_num(&live.color(), s)?;
            }
            _ => {}
        }
        writeln!(s)?;
        if own > 3 {
            write!(s, "****             Own Data             ****\n\n")?;
            self.own_dump(entity, s, own)?;
        }
        Ok(())
    }
    fn dump_names(&self, live: &IgesDataIgesEntity, s: &mut dyn Write) -> IgesDataDumpResult {
        let label = live.short_label();
        if let Some(text) = label.get() {
            write!(s, "****    Label         :")?;
            s.write_all(text.string().as_bytes())?;
        }
        if live.has_sub_script_number() {
            write!(s, "    SubScript:{}", live.sub_script_number())?;
        }
        if label.is_some() {
            writeln!(s)?;
        }
        self.dump_ref(
            s,
            "**      Transf.Matrix :",
            live.has_transf(),
            &live.transf(),
        )?;
        match live.def_view() {
            IgesDataDefList::DefOne => {
                self.dump_ref(s, "**      View          :", true, &live.view())
            }
            IgesDataDefList::DefSeveral => {
                self.dump_ref(s, "**      View (List)   :", true, &live.view_list())
            }
            _ => Ok(()),
        }
    }
    fn dump_ref<T: ?Sized + RuntimeTyped>(
        &self,
        s: &mut dyn Write,
        label: &str,
        present: bool,
        ent: &Handle<T>,
    ) -> IgesDataDumpResult {
        if present {
            write!(s, "{label}")?;
            self.print_d_num(ent, s)?;
            writeln!(s)?;
        }
        Ok(())
    }
    fn dump_attached(
        &self,
        live: &IgesDataIgesEntity,
        s: &mut dyn Write,
        att: i32,
    ) -> IgesDataDumpResult {
        let properties = live.properties();
        let nb_props = properties.nb_entities();
        if nb_props > 0 {
            write!(s, "\n****     Properties (nb:{nb_props})          ****\n")?;
            self.dump_list(&properties, s, att)?;
        }
        let associativities = live.associativities();
        let nb_assocs = associativities.nb_entities();
        if nb_assocs > 0 {
            write!(s, "\n****   Associativities (nb:{nb_assocs})        ****\n")?;
            self.dump_list(&associativities, s, att)?;
        }
        if (nb_props > 0 || nb_assocs > 0) && att <= 1 {
            writeln!(s)?;
        }
        write!(s, "\n****             End of Dump          ****\n\n")?;
        Ok(())
    }
    fn dump_list(
        &self,
        iter: &InterfaceEntityIterator,
        s: &mut dyn Write,
        att: i32,
    ) -> IgesDataDumpResult {
        iter.start();
        while iter.more() {
            self.dump(&iter.value()?, s, att, -1)?;
            iter.next();
        }
        Ok(())
    }
    /// Writes the own parameters of an entity, by the module of its class.
    ///
    /// # Errors
    /// A failure of the stream or of the module.
    pub fn own_dump<T: ?Sized + RuntimeTyped>(
        &self,
        ent: &Handle<T>,
        s: &mut dyn Write,
        own: i32,
    ) -> IgesDataDumpResult {
        let entity = iges_entity_of(ent);
        let transient: Handle<StandardTransient> = entity.cast();
        if let Some((module, cn)) = self.thelib.select(&transient)
            && let Some(live) = module.get()
        {
            return live.own_dump(cn, &entity, self, s, own);
        }
        let name = entity.dynamic_type().map_or("", |kind| kind.name());
        match self.themodel.get() {
            None => writeln!(s, "  ****  Dump impossible. Type {name}")?,
            Some(model) => writeln!(
                s,
                "  ****  Dump Impossible, n0:id:{}:D{} Type {name}",
                self.number_of(&entity),
                model.d_num(&entity),
            )?,
        }
        Ok(())
    }
}

fn level_name(own: i32) -> &'static str {
    match own {
        2 => "D.E. Number + Type, Name + Transf, View",
        3 => "Complete Directory Part",
        4 => "Directory Part + Parameters, no Arrays",
        5 => "Complete",
        _ => "Complete + Transformed Values",
    }
}
