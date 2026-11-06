//! The base of every IGES entity: the directory part, and the lists of associativities and
//! properties.
//!
//! Members of the directory part live in this file; the derived notions (location, name) and the
//! optional lists are in `entity_lists`.
use super::{
    IgesDataColorEntity, IgesDataDefList, IgesDataDefSwitch, IgesDataDefType, IgesDataIgesType,
    IgesDataLabelDisplayEntity, IgesDataLevelListEntity, IgesDataLineFontEntity,
    IgesDataTransfEntity, IgesDataViewKindEntity,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, RuntimeTyped, SharedState, StandardTransient, Transient},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::InterfaceEntityList;

pub(super) const IGES_FLAG_ASSOCS: i32 = 131_072;
pub(super) const IGES_FLAG_PROPS: i32 = 262_144;
const IGES_STATUS_FIELD: i32 = 15;
const IGES_SHIFT_SUBORD: i32 = 4;
const IGES_SHIFT_USE: i32 = 8;
const IGES_SHIFT_HIER: i32 = 12;

/// What every entity class gives: the directory part it embeds.
///
/// `iges_entity!` implements it for each class. It is also the mark that lets
/// `IgesDataIgesWriter::send` take a handle of any entity class.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataEntityKind, IgesDataIgesEntity};
/// let entity = IgesDataIgesEntity::default();
/// assert_eq!(entity.iges_entity().type_number(), 0);
/// ```
pub trait IgesDataEntityKind: Transient {
    /// The embedded directory part.
    fn iges_entity(&self) -> &IgesDataIgesEntity;
}

/// The fields of the directory part.
#[derive(Default)]
pub(super) struct IgesEntityFields {
    pub(super) the_type: i32,
    pub(super) the_form: i32,
    pub(super) the_structure: Handle<IgesDataIgesEntity>,
    pub(super) the_def_line_font: IgesDataDefSwitch,
    pub(super) the_line_font: Handle<IgesDataIgesEntity>,
    pub(super) the_def_level: i32,
    pub(super) the_level_list: Handle<IgesDataIgesEntity>,
    pub(super) the_view: Handle<IgesDataIgesEntity>,
    pub(super) the_transf: Handle<IgesDataIgesEntity>,
    pub(super) the_lab_display: Handle<IgesDataIgesEntity>,
    pub(super) the_status_num: i32,
    pub(super) the_l_weight_num: i32,
    pub(super) the_l_weight_val: f64,
    pub(super) the_def_color: IgesDataDefSwitch,
    pub(super) the_color: Handle<IgesDataIgesEntity>,
    pub(super) the_res1: [u8; 9],
    pub(super) the_res2: [u8; 9],
    pub(super) the_short_label: Handle<HAsciiString>,
    pub(super) the_sub_script_n: i32,
    pub(super) the_assocs: InterfaceEntityList,
    pub(super) the_props: InterfaceEntityList,
    /// The directory errors of an undefined entity, by bits; 0 for every other class.
    pub(super) the_dir_status: i32,
}

/// The directory part of an entity, with its optional lists.
///
/// The classes embed it through `iges_entity!`, so its members apply to every class by `deref`.
/// A setter takes `&self`: a change is seen through every handle.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataDefType, IgesDataIgesEntity};
/// let entity = IgesDataIgesEntity::default();
/// entity.init_type_and_form(110, 0);
/// assert_eq!(entity.type_number(), 110);
/// assert_eq!(entity.def_line_font(), IgesDataDefType::DefVoid);
/// ```
#[derive(Default)]
pub struct IgesDataIgesEntity {
    transient: StandardTransient,
    pub(super) state: SharedState<IgesEntityFields>,
}
impl_transient!(
    IgesDataIgesEntity,
    "IgesDataIgesEntity",
    StandardTransient,
    transient
);
impl IgesDataEntityKind for IgesDataIgesEntity {
    fn iges_entity(&self) -> &IgesDataIgesEntity {
        self
    }
}

/// The directory part of an entity of any class, as a handle of its base.
///
/// ```
/// use martensite_tkdeiges::iges_data::{iges_entity_of, IgesDataIgesEntity};
/// use martensite_tkernel::standard::Handle;
/// let entity = Handle::new(IgesDataIgesEntity::default());
/// assert!(iges_entity_of(&entity).is_some());
/// ```
pub fn iges_entity_of<T: ?Sized + RuntimeTyped>(handle: &Handle<T>) -> Handle<IgesDataIgesEntity> {
    handle.cast()
}

impl IgesDataIgesEntity {
    /// Forgets everything; the reserved texts stay. It starts the reading of an entity.
    pub(crate) fn clear(&self) {
        let mut state = self.state.write();
        state.the_structure.nullify();
        state.the_def_line_font.set_void();
        state.the_line_font.nullify();
        state.the_def_color.set_void();
        state.the_color.nullify();
        state.the_def_level = 0;
        state.the_level_list.nullify();
        state.the_view.nullify();
        state.the_transf.nullify();
        state.the_lab_display.nullify();
        state.the_sub_script_n = 0;
        state.the_short_label.nullify();
        state.the_type = 0;
        state.the_form = 0;
        state.the_status_num = 0;
        state.the_l_weight_num = 0;
        state.the_l_weight_val = 0.0;
        state.the_props.clear();
        state.the_assocs.clear();
    }
    /// The type and form numbers.
    pub fn iges_type(&self) -> IgesDataIgesType {
        let state = self.state.read();
        IgesDataIgesType::new(state.the_type, state.the_form)
    }
    /// The type number.
    pub fn type_number(&self) -> i32 {
        self.state.read().the_type
    }
    /// The form number.
    pub fn form_number(&self) -> i32 {
        self.state.read().the_form
    }
    /// The entity of a directory field, by its number in the directory entry: 3 structure,
    /// 4 line font, 5 level list, 6 view, 7 transformation, 8 label display, 13 color; null
    /// for any other number.
    pub fn dir_field_entity(&self, fieldnum: i32) -> Handle<IgesDataIgesEntity> {
        let state = self.state.read();
        match fieldnum {
            3 => state.the_structure.clone(),
            4 => state.the_line_font.clone(),
            5 => state.the_level_list.clone(),
            6 => state.the_view.clone(),
            7 => state.the_transf.clone(),
            8 => state.the_lab_display.clone(),
            13 => state.the_color.clone(),
            _ => Handle::default(),
        }
    }
    /// Whether a structure is defined.
    pub fn has_structure(&self) -> bool {
        self.state.read().the_structure.is_some()
    }
    /// The structure, null when none.
    pub fn structure(&self) -> Handle<IgesDataIgesEntity> {
        self.state.read().the_structure.clone()
    }
    /// How the line font is defined.
    pub fn def_line_font(&self) -> IgesDataDefType {
        let state = self.state.read();
        match (state.the_dir_status / 4) & 3 {
            0 => state.the_def_line_font.def_type(),
            1 => IgesDataDefType::ErrorVal,
            _ => IgesDataDefType::ErrorRef,
        }
    }
    /// The line font rank, negative for a reference.
    pub fn rank_line_font(&self) -> i32 {
        self.state.read().the_def_line_font.value()
    }
    /// The line font entity, null when none.
    pub fn line_font(&self) -> Handle<dyn IgesDataLineFontEntity> {
        self.state.read().the_line_font.cast()
    }
    /// How the level is defined.
    pub fn def_level(&self) -> IgesDataDefList {
        let state = self.state.read();
        match (state.the_dir_status / 16) & 3 {
            0 if state.the_def_level > 0 => IgesDataDefList::DefOne,
            0 if state.the_def_level < 0 => IgesDataDefList::DefSeveral,
            0 => IgesDataDefList::DefNone,
            1 => IgesDataDefList::ErrorOne,
            _ => IgesDataDefList::ErrorSeveral,
        }
    }
    /// The level, negative for a list.
    pub fn level(&self) -> i32 {
        self.state.read().the_def_level
    }
    /// The level list entity, null when none.
    pub fn level_list(&self) -> Handle<dyn IgesDataLevelListEntity> {
        self.state.read().the_level_list.cast()
    }
    /// How the view is defined.
    pub fn def_view(&self) -> IgesDataDefList {
        match (self.state.read().the_dir_status / 64) & 3 {
            0 => {}
            1 => return IgesDataDefList::ErrorOne,
            _ => return IgesDataDefList::ErrorSeveral,
        }
        match self.view().get() {
            None => IgesDataDefList::DefNone,
            Some(view) if view.is_single() => IgesDataDefList::DefOne,
            Some(_) => IgesDataDefList::DefSeveral,
        }
    }
    /// The view entity, null when none.
    pub fn view(&self) -> Handle<dyn IgesDataViewKindEntity> {
        self.state.read().the_view.cast()
    }
    /// The view when it is single, else null.
    pub fn single_view(&self) -> Handle<dyn IgesDataViewKindEntity> {
        if self.def_view() != IgesDataDefList::DefOne {
            return Handle::default();
        }
        self.view()
    }
    /// The view when it is a list, else null.
    pub fn view_list(&self) -> Handle<dyn IgesDataViewKindEntity> {
        if self.def_view() != IgesDataDefList::DefSeveral {
            return Handle::default();
        }
        self.view()
    }
    /// Whether a transformation is defined.
    pub fn has_transf(&self) -> bool {
        self.state.read().the_transf.is_some()
    }
    /// The transformation entity, null when none.
    pub fn transf(&self) -> Handle<dyn IgesDataTransfEntity> {
        self.state.read().the_transf.cast()
    }
    /// Whether a label display is defined.
    pub fn has_label_display(&self) -> bool {
        self.state.read().the_lab_display.is_some()
    }
    /// The label display entity, null when none.
    pub fn label_display(&self) -> Handle<dyn IgesDataLabelDisplayEntity> {
        self.state.read().the_lab_display.cast()
    }
    /// The blank status.
    pub fn blank_status(&self) -> i32 {
        self.state.read().the_status_num & IGES_STATUS_FIELD
    }
    /// The subordinate status.
    pub fn subordinate_status(&self) -> i32 {
        (self.state.read().the_status_num >> IGES_SHIFT_SUBORD) & IGES_STATUS_FIELD
    }
    /// The use flag.
    pub fn use_flag(&self) -> i32 {
        (self.state.read().the_status_num >> IGES_SHIFT_USE) & IGES_STATUS_FIELD
    }
    /// The hierarchy status.
    pub fn hierarchy_status(&self) -> i32 {
        (self.state.read().the_status_num >> IGES_SHIFT_HIER) & IGES_STATUS_FIELD
    }
    /// The line weight number.
    pub fn line_weight_number(&self) -> i32 {
        self.state.read().the_l_weight_num
    }
    /// The line weight value, computed by [`Self::set_line_weight`].
    pub fn line_weight(&self) -> f64 {
        self.state.read().the_l_weight_val
    }
    /// How the color is defined.
    pub fn def_color(&self) -> IgesDataDefType {
        let state = self.state.read();
        match (state.the_dir_status / 256) & 3 {
            0 => state.the_def_color.def_type(),
            1 => IgesDataDefType::ErrorVal,
            _ => IgesDataDefType::ErrorRef,
        }
    }
    /// The color rank, negative for a reference.
    pub fn rank_color(&self) -> i32 {
        self.state.read().the_def_color.value()
    }
    /// The color entity, null when none.
    pub fn color(&self) -> Handle<dyn IgesDataColorEntity> {
        self.state.read().the_color.cast()
    }
    /// Copies the reserved texts; true when one holds something above a blank.
    pub fn c_res_values(&self, res1: &mut [u8; 9], res2: &mut [u8; 9]) -> bool {
        let state = self.state.read();
        let mut res = false;
        for i in 0..8 {
            res1[i] = state.the_res1[i];
            res2[i] = state.the_res2[i];
            res |= state.the_res1[i] > b' ' || state.the_res2[i] > b' ';
        }
        res1[8] = 0;
        res2[8] = 0;
        res
    }
    /// Whether a short label is defined.
    pub fn has_short_label(&self) -> bool {
        self.state.read().the_short_label.is_some()
    }
    /// The short label, null when none.
    pub fn short_label(&self) -> Handle<HAsciiString> {
        self.state.read().the_short_label.clone()
    }
    /// Whether a subscript is defined; 0 is a defined value, negative means none.
    pub fn has_sub_script_number(&self) -> bool {
        let state = self.state.read();
        (state.the_dir_status / 1024) & 1 == 0 && state.the_sub_script_n >= 0
    }
    /// The subscript, 0 when none.
    pub fn sub_script_number(&self) -> i32 {
        self.state.read().the_sub_script_n.max(0)
    }
    /// Sets the type and form numbers.
    pub fn init_type_and_form(&self, typenum: i32, formnum: i32) {
        let mut state = self.state.write();
        state.the_type = typenum;
        state.the_form = formnum;
    }
    /// Sets the entity of a directory field, see [`Self::dir_field_entity`]; the kind of the
    /// entity is not checked.
    pub fn init_dir_field_entity(&self, fieldnum: i32, ent: &Handle<IgesDataIgesEntity>) {
        let mut state = self.state.write();
        match fieldnum {
            3 => state.the_structure = ent.clone(),
            4 => state.the_line_font = ent.clone(),
            5 => state.the_level_list = ent.clone(),
            6 => state.the_view = ent.clone(),
            7 => state.the_transf = ent.clone(),
            8 => state.the_lab_display = ent.clone(),
            13 => state.the_color = ent.clone(),
            _ => {}
        }
    }
    /// Sets the transformation.
    pub fn init_transf(&self, ent: &Handle<dyn IgesDataTransfEntity>) {
        self.state.write().the_transf = ent.cast();
    }
    /// Sets the view.
    pub fn init_view(&self, ent: &Handle<dyn IgesDataViewKindEntity>) {
        self.state.write().the_view = ent.cast();
    }
    /// Sets the line font: the entity when there is one, else the rank.
    pub fn init_line_font(&self, ent: &Handle<dyn IgesDataLineFontEntity>, rank: i32) {
        let mut state = self.state.write();
        state
            .the_def_line_font
            .set_rank(if ent.is_null() { rank } else { -1 });
        state.the_line_font = ent.cast();
    }
    /// Sets the level: the list when there is one, else the value.
    pub fn init_level(&self, ent: &Handle<dyn IgesDataLevelListEntity>, val: i32) {
        let mut state = self.state.write();
        state.the_level_list = ent.cast();
        state.the_def_level = if ent.is_null() { val } else { -1 };
    }
    /// Sets the color: the entity when there is one, else the rank.
    pub fn init_color(&self, ent: &Handle<dyn IgesDataColorEntity>, rank: i32) {
        let mut state = self.state.write();
        state
            .the_def_color
            .set_rank(if ent.is_null() { rank } else { -1 });
        state.the_color = ent.cast();
    }
    /// Sets the four statuses. It also lowers the marks that the optional lists were read.
    pub fn init_status(&self, blank: i32, subordinate: i32, useflag: i32, hierarchy: i32) {
        let mut state = self.state.write();
        state.the_status_num = (blank & IGES_STATUS_FIELD)
            | ((subordinate & IGES_STATUS_FIELD) << IGES_SHIFT_SUBORD)
            | ((useflag & IGES_STATUS_FIELD) << IGES_SHIFT_USE)
            | ((hierarchy & IGES_STATUS_FIELD) << IGES_SHIFT_HIER);
    }
    /// Sets the short label and the subscript, negative for none.
    pub fn set_label(&self, label: &Handle<HAsciiString>, sub: i32) {
        let mut state = self.state.write();
        state.the_short_label = label.clone();
        state.the_sub_script_n = sub;
    }
    /// Sets the structure, the label display and the line weight number; a line weight value
    /// already computed is scaled with the number.
    pub fn init_misc(
        &self,
        str_ent: &Handle<IgesDataIgesEntity>,
        lab: &Handle<dyn IgesDataLabelDisplayEntity>,
        weightnum: i32,
    ) {
        let mut state = self.state.write();
        state.the_structure = str_ent.clone();
        state.the_lab_display = lab.cast();
        if state.the_l_weight_num != 0 {
            state.the_l_weight_val *= f64::from(weightnum / state.the_l_weight_num);
        } else if weightnum == 0 {
            state.the_l_weight_val = 0.0;
        }
        state.the_l_weight_num = weightnum;
    }
    /// Computes the line weight value from its number: `defw` when the number is 0, else the
    /// share of `maxw` the number gives over `gradw` gradations.
    pub fn set_line_weight(&self, defw: f64, maxw: f64, gradw: i32) {
        let mut state = self.state.write();
        let num = f64::from(state.the_l_weight_num);
        state.the_l_weight_val = if state.the_l_weight_num == 0 {
            defw
        } else if gradw == 1 {
            maxw * num
        } else {
            (maxw * num) / f64::from(gradw)
        };
    }
    /// The directory errors of an undefined entity, by bits.
    pub(super) fn dir_status_bits(&self) -> i32 {
        self.state.read().the_dir_status
    }
    /// Records the directory errors of an undefined entity.
    pub(super) fn set_dir_status_bits(&self, bits: i32) {
        self.state.write().the_dir_status = bits;
    }
    /// Sets the reserved texts as read from a directory entry.
    pub(crate) fn set_res_values(&self, res1: &[u8; 9], res2: &[u8; 9]) {
        let mut state = self.state.write();
        state.the_res1 = *res1;
        state.the_res2 = *res2;
    }
}
