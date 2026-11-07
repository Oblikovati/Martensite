//! The directory part of an entity as an editor of twenty-three values.
use crate::iges_data::{
    IgesDataColorEntity, IgesDataDefList, IgesDataDefType, IgesDataIgesEntity, IgesDataIgesModel,
    IgesDataLevelListEntity, IgesDataLineFontEntity, IgesDataTransfEntity, IgesDataViewKindEntity,
    iges_live,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::{IFSelectEditForm, IFSelectEditValue, IFSelectEditor, IFSelectEditorBase},
    interface::{InterfaceInterfaceModel, InterfaceParamType, InterfaceTypedValue},
};

const INTEGER: InterfaceParamType = InterfaceParamType::ParamInteger;
const IDENT: InterfaceParamType = InterfaceParamType::ParamIdent;
const ENUM: InterfaceParamType = InterfaceParamType::ParamEnum;
const TEXT: InterfaceParamType = InterfaceParamType::ParamText;
const OPTIONAL: IFSelectEditValue = IFSelectEditValue::Optional;
const EDITABLE: IFSelectEditValue = IFSelectEditValue::Editable;
const PROTECTED: IFSelectEditValue = IFSelectEditValue::EditProtected;
const DYNAMIC: IFSelectEditValue = IFSelectEditValue::EditDynamic;
const READ: IFSelectEditValue = IFSelectEditValue::EditRead;

/// The values of the editor, by their numbers from 1: name, type, short name, mode.
const DIR_VALUES: [(&str, InterfaceParamType, &str, IFSelectEditValue); 23] = [
    ("Type Number", INTEGER, "D1:Type", READ),
    ("Form Number", INTEGER, "D15:Form", READ),
    ("Structure", IDENT, "D3:Struct", OPTIONAL),
    ("Line Font Pattern", ENUM, "D4:LineFont", DYNAMIC),
    ("Line Font Number", INTEGER, "N4:LineFont", OPTIONAL),
    ("Line Font Entity", IDENT, "E4:LineFont", OPTIONAL),
    ("Level", ENUM, "D5:Level", DYNAMIC),
    ("Level Number", INTEGER, "N5:Level", OPTIONAL),
    ("Level List Entity", IDENT, "L5:Level", OPTIONAL),
    ("View", ENUM, "D6:View", DYNAMIC),
    ("View Entity", IDENT, "E6:View", OPTIONAL),
    ("Transformation", IDENT, "D7:Transf", OPTIONAL),
    (
        "Label Display Associativity",
        IDENT,
        "D8:LabelDisp",
        OPTIONAL,
    ),
    ("Blank Status", INTEGER, "D9-1:Blank", EDITABLE),
    ("Subordinate Status", INTEGER, "D9-2:Subordinate", PROTECTED),
    ("Use Flag", INTEGER, "D9-3:UseFlag", EDITABLE),
    ("Hierarchy", INTEGER, "D9-4:Hierarchy", PROTECTED),
    ("Line Weight Number", INTEGER, "D12:LineWeight", EDITABLE),
    ("Color", ENUM, "D13:Color", DYNAMIC),
    ("Color Number", INTEGER, "N13:Color", OPTIONAL),
    ("Color Entity", IDENT, "E13:Color", OPTIONAL),
    ("Entity Label", TEXT, "D18:Label", EDITABLE),
    ("Subscript Number", INTEGER, "D19:Subscript", OPTIONAL),
];

/// The limits and enumerations of the values that have some.
fn complete_definition(num: i32, value: &InterfaceTypedValue) -> Result<(), Failure> {
    let enum_of = |names: [&str; 3]| -> Result<(), Failure> {
        value.start_enum(0, true)?;
        for (rank, name) in names.iter().enumerate() {
            value.add_enum_value(name, rank as i32)?;
        }
        Ok(())
    };
    match num {
        4 | 19 => enum_of(["Void", "Value", "Entity"])?,
        7 | 10 => enum_of(["None", "One", "List"])?,
        5 | 8 | 18 | 20 => value.set_integer_limit(false, 0)?,
        14..=17 => {
            value.set_integer_limit(false, 0)?;
            let max = match num {
                14 => 1,
                15 => 3,
                16 => 6,
                _ => 2,
            };
            value.set_integer_limit(true, max)?;
        }
        _ => {}
    }
    Ok(())
}

fn def_type_name(deftype: IgesDataDefType) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(match deftype {
        IgesDataDefType::DefVoid => "Void",
        IgesDataDefType::DefValue => "Value",
        IgesDataDefType::DefReference => "Entity",
        _ => "???",
    }))
}

fn def_list_name(deflist: IgesDataDefList) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(match deflist {
        IgesDataDefList::DefNone => "None",
        IgesDataDefList::DefOne => "One",
        IgesDataDefList::DefSeveral => "List",
        _ => "???",
    }))
}

fn int_text(value: i32) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::from_integer(value))
}

/// The editor of the directory part of an entity.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectEditDirPart;
/// use martensite_tkxsbase::if_select::IFSelectEditor;
/// let editor = IgesSelectEditDirPart::new().unwrap();
/// assert_eq!(editor.get().unwrap().label().as_bytes(), b"IGES Header");
/// ```
pub struct IgesSelectEditDirPart {
    transient: StandardTransient,
    base: IFSelectEditorBase,
}
impl_transient!(
    IgesSelectEditDirPart,
    "IgesSelectEditDirPart",
    dyn IFSelectEditor,
    transient,
    interfaces: [dyn IFSelectEditor]
);

impl IgesSelectEditDirPart {
    /// The editor, as a bound handle.
    ///
    /// # Errors
    /// A failure of a definition.
    pub fn new() -> Result<Handle<Self>, Failure> {
        let editor = Handle::new(Self {
            transient: StandardTransient::default(),
            base: IFSelectEditorBase::new(23),
        });
        let live = iges_live(&editor, "IGES edit directory part")?;
        live.base.bind_self(&editor.cast());
        for (rank, (name, kind, short, mode)) in DIR_VALUES.iter().enumerate() {
            let num = rank as i32 + 1;
            let value = InterfaceTypedValue::new(name, *kind, "")?;
            complete_definition(num, &value)?;
            live.set_value(num, &Handle::new(value), short, *mode)?;
        }
        Ok(editor.clone())
    }
}

/// The rank in a model of the entity an edited label names: -1 for no label, 0 for none found.
fn edited_rank(
    form: &IFSelectEditForm,
    modl: &IgesDataIgesModel,
    num: i32,
) -> Result<i32, Failure> {
    let edited = form.edited_value(num)?;
    Ok(edited.get().map_or(-1, |text| {
        let label = String::from_utf8_lossy(text.string().as_bytes()).into_owned();
        modl.next_number_for_label(&label, 0, true)
    }))
}

/// The integer of an edited value, 0 for no value.
fn edited_number(form: &IFSelectEditForm, num: i32) -> Result<i32, Failure> {
    let edited = form.edited_value(num)?;
    match edited.get() {
        Some(text) => text.integer_value(),
        None => Ok(0),
    }
}

/// Loads the directory fields that are references and their kinds.
fn load_references(form: &IFSelectEditForm, modl: &IgesDataIgesModel, iges: &IgesDataIgesEntity) {
    let label = |ent: &Handle<StandardTransient>| modl.string_label(ent);
    if iges.has_structure() {
        form.load_value(3, &label(&iges.structure().cast()));
    }
    form.load_value(4, &def_type_name(iges.def_line_font()));
    form.load_value(5, &int_text(iges.rank_line_font()));
    if iges.def_line_font() == IgesDataDefType::DefReference {
        form.load_value(6, &label(&iges.line_font().cast()));
    }
    form.load_value(7, &def_list_name(iges.def_level()));
    form.load_value(8, &int_text(iges.level()));
    if iges.def_level() == IgesDataDefList::DefSeveral {
        form.load_value(9, &label(&iges.level_list().cast()));
    }
    form.load_value(10, &def_list_name(iges.def_view()));
    if iges.def_view() != IgesDataDefList::DefNone {
        form.load_value(11, &label(&iges.view().cast()));
    }
    if iges.has_transf() {
        form.load_value(12, &label(&iges.transf().cast()));
    }
    if iges.has_label_display() {
        form.load_value(13, &label(&iges.label_display().cast()));
    }
    form.load_value(19, &def_type_name(iges.def_color()));
    form.load_value(20, &int_text(iges.rank_color()));
    if iges.def_color() == IgesDataDefType::DefReference {
        form.load_value(21, &label(&iges.color().cast()));
    }
}

/// Applies the references that have no rank of their own.
fn apply_structure_and_view(
    form: &IFSelectEditForm,
    modl: &IgesDataIgesModel,
    iges: &IgesDataIgesEntity,
) -> Result<(), Failure> {
    let entity_at = |rank: i32| -> Handle<IgesDataIgesEntity> {
        if rank > 0 {
            modl.entity(rank)
        } else {
            Handle::default()
        }
    };
    if form.is_modified(3)? {
        let rank = edited_rank(form, modl, 3)?;
        if rank != 0 {
            iges.init_dir_field_entity(3, &entity_at(rank));
        }
    }
    if form.is_modified(11)? {
        let rank = edited_rank(form, modl, 11)?;
        let view = entity_at(rank).cast::<dyn IgesDataViewKindEntity>();
        if rank != 0 || view.is_some() {
            iges.init_view(&view);
        }
    }
    if form.is_modified(12)? {
        let rank = edited_rank(form, modl, 12)?;
        let transf = entity_at(rank).cast::<dyn IgesDataTransfEntity>();
        if rank != 0 || transf.is_some() {
            iges.init_transf(&transf);
        }
    }
    if form.is_modified(13)? {
        let rank = edited_rank(form, modl, 13)?;
        let label = entity_at(rank);
        if rank != 0 || label.is_some() {
            iges.init_dir_field_entity(8, &label);
        }
    }
    Ok(())
}

/// Applies the line font, the level and the color: each by a number or an entity.
fn apply_rank_or_entity(
    form: &IFSelectEditForm,
    modl: &IgesDataIgesModel,
    iges: &IgesDataIgesEntity,
) -> Result<(), Failure> {
    let entity_at = |rank: i32| -> Handle<IgesDataIgesEntity> {
        if rank > 0 {
            modl.entity(rank)
        } else {
            Handle::default()
        }
    };
    if form.is_modified(5)? {
        iges.init_line_font(&Handle::default(), edited_number(form, 5)?);
    }
    if form.is_modified(6)? {
        let rank = edited_rank(form, modl, 6)?;
        let font = entity_at(rank).cast::<dyn IgesDataLineFontEntity>();
        if rank < 0 || font.is_some() {
            iges.init_line_font(&font, 0);
        }
    }
    if form.is_modified(8)? {
        iges.init_level(&Handle::default(), edited_number(form, 8)?);
    }
    if form.is_modified(9)? {
        let rank = edited_rank(form, modl, 9)?;
        let levels = entity_at(rank).cast::<dyn IgesDataLevelListEntity>();
        if rank < 0 || levels.is_some() {
            iges.init_level(&levels, 0);
        }
    }
    if form.is_modified(20)? {
        iges.init_color(&Handle::default(), edited_number(form, 20)?);
    }
    if form.is_modified(21)? {
        let rank = edited_rank(form, modl, 21)?;
        let color = entity_at(rank).cast::<dyn IgesDataColorEntity>();
        if rank < 0 || color.is_some() {
            iges.init_color(&color, 0);
        }
    }
    Ok(())
}

/// Applies the status, the line weight and the label.
fn apply_values(form: &IFSelectEditForm, iges: &IgesDataIgesEntity) -> Result<(), Failure> {
    let any_status = form.is_modified(14)?
        || form.is_modified(15)?
        || form.is_modified(16)?
        || form.is_modified(17)?;
    if any_status {
        let pick = |num: i32, current: i32| -> Result<i32, Failure> {
            if !form.is_modified(num)? {
                return Ok(current);
            }
            let edited = form.edited_value(num)?;
            iges_live(&edited, "IGES edit directory part")?.integer_value()
        };
        iges.init_status(
            pick(14, iges.blank_status())?,
            pick(15, iges.subordinate_status())?,
            pick(16, iges.use_flag())?,
            pick(17, iges.hierarchy_status())?,
        );
    }
    if form.is_modified(18)? {
        iges.init_misc(
            &iges.structure(),
            &iges.label_display(),
            edited_number(form, 18)?,
        );
    }
    if form.is_modified(22)? || form.is_modified(23)? {
        let mut subscript = if iges.has_sub_script_number() {
            iges.sub_script_number()
        } else {
            -1
        };
        if form.is_modified(23)? {
            let edited = form.edited_value(23)?;
            subscript = match edited.get() {
                Some(text) => text.integer_value()?,
                None => -1,
            };
        }
        let label = if form.is_modified(22)? {
            form.edited_value(22)?
        } else {
            iges.short_label()
        };
        iges.set_label(&label, subscript);
    }
    Ok(())
}

impl IFSelectEditor for IgesSelectEditDirPart {
    fn editor_base(&self) -> &IFSelectEditorBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        AsciiString::new("IGES Header")
    }
    fn recognize(&self, _form: &Handle<IFSelectEditForm>) -> Result<bool, Failure> {
        Ok(true)
    }
    fn string_value(
        &self,
        _form: &Handle<IFSelectEditForm>,
        num: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        let value = self.typed_value(num)?;
        Ok(iges_live(&value, "IGES edit directory part")?.h_string_value())
    }
    fn load(
        &self,
        form: &Handle<IFSelectEditForm>,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let modl = model.cast::<IgesDataIgesModel>();
        let entity = ent.cast::<IgesDataIgesEntity>();
        let (Some(live), Some(iges)) = (modl.get(), entity.get()) else {
            return Ok(false);
        };
        let edit = iges_live(form, "IGES edit directory part")?;
        edit.load_value(1, &int_text(iges.type_number()));
        edit.load_value(2, &int_text(iges.form_number()));
        load_references(edit, live, iges);
        edit.load_value(14, &int_text(iges.blank_status()));
        edit.load_value(15, &int_text(iges.subordinate_status()));
        edit.load_value(16, &int_text(iges.use_flag()));
        edit.load_value(17, &int_text(iges.hierarchy_status()));
        edit.load_value(18, &int_text(iges.line_weight_number()));
        edit.load_value(22, &iges.short_label());
        if iges.has_sub_script_number() {
            edit.load_value(23, &int_text(iges.sub_script_number()));
        }
        Ok(true)
    }
    fn update(
        &self,
        form: &Handle<IFSelectEditForm>,
        num: i32,
        newval: &Handle<HAsciiString>,
        _enforce: bool,
    ) -> Result<bool, Failure> {
        let edit = iges_live(form, "IGES edit directory part")?;
        let is_null = newval.is_null();
        let type_of = |value: IgesDataDefType| {
            def_type_name(if is_null {
                IgesDataDefType::DefVoid
            } else {
                value
            })
        };
        let list_of = |value: IgesDataDefList| {
            def_list_name(if is_null {
                IgesDataDefList::DefNone
            } else {
                value
            })
        };
        match num {
            5 => edit.touch(4, &type_of(IgesDataDefType::DefValue))?,
            6 => edit.touch(4, &type_of(IgesDataDefType::DefReference))?,
            8 => edit.touch(7, &list_of(IgesDataDefList::DefOne))?,
            9 => edit.touch(7, &list_of(IgesDataDefList::DefSeveral))?,
            11 => edit.touch(10, &list_of(IgesDataDefList::DefOne))?,
            20 => edit.touch(19, &type_of(IgesDataDefType::DefValue))?,
            21 => edit.touch(19, &type_of(IgesDataDefType::DefReference))?,
            _ => true,
        };
        Ok(true)
    }
    fn apply(
        &self,
        form: &Handle<IFSelectEditForm>,
        ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let modl = model.cast::<IgesDataIgesModel>();
        let entity = ent.cast::<IgesDataIgesEntity>();
        let (Some(live), Some(iges)) = (modl.get(), entity.get()) else {
            return Ok(false);
        };
        let edit = iges_live(form, "IGES edit directory part")?;
        apply_structure_and_view(edit, live, iges)?;
        apply_rank_or_entity(edit, live, iges)?;
        apply_values(edit, iges)?;
        Ok(true)
    }
}
