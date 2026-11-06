//! The header of a model as an editor of thirty values.
use crate::iges_data::{
    IgesDataBasicEditor, IgesDataGlobalSection, IgesDataIgesModel, IgesDataTextLines, iges_atoi,
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

/// Whether a text is a date of the form `[YY]YYMMDD.HHNNSS`.
fn is_time_stamp(val: &Handle<HAsciiString>) -> bool {
    let Some(live) = val.get() else {
        return false;
    };
    let content = live.string();
    let text = content.as_bytes();
    let offset = match text.len() {
        13 => 0,
        15 => 2,
        _ => return false,
    };
    if offset == 2 && (!matches!(text[0], b'1' | b'2') || !text[1].is_ascii_digit()) {
        return false;
    }
    let date = &text[offset..];
    let digit = |rank: usize, low: u8, high: u8| (low..=high).contains(&date[rank]);
    digit(0, b'0', b'9')
        && digit(1, b'0', b'9')
        && digit(2, b'0', b'1')
        && digit(3, b'0', if date[2] == b'1' { b'2' } else { b'9' })
        && digit(4, b'0', b'3')
        && digit(5, b'0', if date[4] == b'3' { b'1' } else { b'9' })
        && date[6] == b'.'
        && digit(7, b'0', b'2')
        && digit(8, b'0', if date[7] == b'2' { b'3' } else { b'9' })
        && digit(9, b'0', b'5')
        && digit(10, b'0', b'9')
        && digit(11, b'0', b'5')
        && digit(12, b'0', b'9')
}

/// The definition of one value of the editor.
struct HeaderValue {
    name: &'static str,
    kind: InterfaceParamType,
    short: &'static str,
    mode: IFSelectEditValue,
}

const fn text_value(
    name: &'static str,
    short: &'static str,
    mode: IFSelectEditValue,
) -> HeaderValue {
    HeaderValue {
        name,
        kind: InterfaceParamType::ParamText,
        short,
        mode,
    }
}

const fn typed_value(
    name: &'static str,
    kind: InterfaceParamType,
    short: &'static str,
    mode: IFSelectEditValue,
) -> HeaderValue {
    HeaderValue {
        name,
        kind,
        short,
        mode,
    }
}

const OPTIONAL: IFSelectEditValue = IFSelectEditValue::Optional;
const EDITABLE: IFSelectEditValue = IFSelectEditValue::Editable;
const INTEGER: InterfaceParamType = InterfaceParamType::ParamInteger;
const REAL: InterfaceParamType = InterfaceParamType::ParamReal;
const ENUM: InterfaceParamType = InterfaceParamType::ParamEnum;

/// The values of the editor, by their numbers from 1.
const HEADER_VALUES: [HeaderValue; 30] = [
    text_value("Start Section", "Start", EDITABLE),
    text_value("Parameter Delimiter", "G1:Separator", OPTIONAL),
    text_value("Record Delimiter", "G2:EndMark", OPTIONAL),
    text_value("Sender Product Id", "G3:SendName", OPTIONAL),
    text_value("File Name", "G4:FileName", EDITABLE),
    text_value("Native System Id", "G5:SystemId", EDITABLE),
    text_value("Preprocessor Version", "G6:Version", EDITABLE),
    typed_value("Integer Binary Bits", INTEGER, "G7:IntBits", EDITABLE),
    typed_value(
        "Single Precision Magnitude",
        INTEGER,
        "G8:SingleMag",
        EDITABLE,
    ),
    typed_value(
        "Single Precision Significance",
        INTEGER,
        "G9:SingDigits",
        EDITABLE,
    ),
    typed_value(
        "Double Precision Magnitude",
        INTEGER,
        "G10:DoubleMag",
        EDITABLE,
    ),
    typed_value(
        "Double Precision Significance",
        INTEGER,
        "G11:DoubDigits",
        EDITABLE,
    ),
    text_value("Receiver Product Id", "G12:Receiver", OPTIONAL),
    typed_value("Model Space Scale", REAL, "G13:Scale", OPTIONAL),
    typed_value("Units Flag", INTEGER, "G14:UnitFlag", OPTIONAL),
    typed_value("Units Name", ENUM, "G15:UnitName", OPTIONAL),
    typed_value(
        "Computed Unit Value",
        REAL,
        "V15:UnitValue",
        IFSelectEditValue::EditDynamic,
    ),
    typed_value(
        "Max Line Weight Gradation",
        INTEGER,
        "G16:LineWGrad",
        OPTIONAL,
    ),
    typed_value("Width of Max Line Weight", REAL, "G17:MaxLineW", EDITABLE),
    text_value("Date of File Creation", "G18:FileDate", EDITABLE),
    typed_value("Max Resolution", REAL, "G19:Resolution", EDITABLE),
    typed_value("Max Coordinates", REAL, "G20:MaxCoord", OPTIONAL),
    text_value("Name of Author", "G21:Author", EDITABLE),
    text_value("Author Organization", "G22:Company", EDITABLE),
    typed_value("Version Flag", INTEGER, "G23:IGESVersion", EDITABLE),
    typed_value("IGES Version Name", ENUM, "V23:VersionName", EDITABLE),
    typed_value("Drafting Standard Flag", INTEGER, "G24:Drafting", EDITABLE),
    typed_value("Drafting Standard Name", ENUM, "V24:DraftingName", EDITABLE),
    text_value("Date of Creation/Change", "G25:ChangeDate", OPTIONAL),
    text_value("Application Protocol/Subset Id", "G26:Protocol", OPTIONAL),
];

/// The limits, enumerations and checks of the values that have some.
fn complete_definition(num: i32, value: &InterfaceTypedValue) -> Result<(), Failure> {
    match num {
        1 => value.set_max_length(72),
        2 | 3 => value.set_max_length(1),
        15 => {
            value.set_integer_limit(false, 1)?;
            value.set_integer_limit(true, 11)?;
        }
        16 => {
            value.start_enum(1, true)?;
            for flag in 1..=11 {
                value.add_enum_value(IgesDataBasicEditor::unit_flag_name(flag), flag)?;
            }
        }
        20 | 29 => value.set_satisfies(Some(is_time_stamp), "IsIGESDate"),
        25 => {
            value.set_integer_limit(false, 1)?;
            value.set_integer_limit(true, IgesDataBasicEditor::iges_version_max())?;
        }
        26 => {
            value.start_enum(0, true)?;
            for flag in 0..=IgesDataBasicEditor::iges_version_max() {
                value.add_enum_value(IgesDataBasicEditor::iges_version_name(flag), flag)?;
            }
        }
        27 => {
            value.set_integer_limit(false, 0)?;
            value.set_integer_limit(true, IgesDataBasicEditor::drafting_max())?;
        }
        28 => {
            value.start_enum(0, true)?;
            for flag in 0..=IgesDataBasicEditor::drafting_max() {
                value.add_enum_value(IgesDataBasicEditor::drafting_name(flag), flag)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// The editor of the header of a model.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectEditHeader;
/// use martensite_tkxsbase::if_select::IFSelectEditor;
/// let editor = IgesSelectEditHeader::new().unwrap();
/// assert_eq!(editor.get().unwrap().label().as_bytes(), b"IGES Header");
/// ```
pub struct IgesSelectEditHeader {
    transient: StandardTransient,
    base: IFSelectEditorBase,
}
impl_transient!(
    IgesSelectEditHeader,
    "IgesSelectEditHeader",
    dyn IFSelectEditor,
    transient,
    interfaces: [dyn IFSelectEditor]
);

impl IgesSelectEditHeader {
    /// The editor, as a bound handle.
    ///
    /// # Errors
    /// A failure of a definition.
    pub fn new() -> Result<Handle<Self>, Failure> {
        let editor = Handle::new(Self {
            transient: StandardTransient::default(),
            base: IFSelectEditorBase::new(30),
        });
        let live = iges_live(&editor, "IGES edit header")?;
        live.base.bind_self(&editor.cast());
        for (rank, definition) in HEADER_VALUES.iter().enumerate() {
            let num = rank as i32 + 1;
            let value = InterfaceTypedValue::new(definition.name, definition.kind, "")?;
            complete_definition(num, &value)?;
            live.set_value(num, &Handle::new(value), definition.short, definition.mode)?;
        }
        live.set_list(1, 0);
        Ok(editor.clone())
    }
}

fn int_text(value: i32) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::from_integer(value))
}

fn real_text(value: f64) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::from_real(value))
}

fn name_text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The integer of an edited value.
fn edited_int(form: &IFSelectEditForm, num: i32) -> Result<i32, Failure> {
    let edited = form.edited_value(num)?;
    iges_live(&edited, "IGES edit header")?.integer_value()
}

/// The real of an edited value.
fn edited_real(form: &IFSelectEditForm, num: i32) -> Result<f64, Failure> {
    let edited = form.edited_value(num)?;
    iges_live(&edited, "IGES edit header")?.real_value()
}

fn load_section(form: &IFSelectEditForm, gs: &IgesDataGlobalSection) {
    let one_char = |value: u8| Handle::new(HAsciiString::from_char(value));
    form.load_value(2, &one_char(gs.separator()));
    form.load_value(3, &one_char(gs.end_mark()));
    form.load_value(4, &gs.send_name());
    form.load_value(5, &gs.file_name());
    form.load_value(6, &gs.system_id());
    form.load_value(7, &gs.interface_version());
    form.load_value(8, &int_text(gs.integer_bits()));
    form.load_value(9, &int_text(gs.max_power10_single()));
    form.load_value(10, &int_text(gs.max_digits_single()));
    form.load_value(11, &int_text(gs.max_power10_double()));
    form.load_value(12, &int_text(gs.max_digits_double()));
    form.load_value(13, &gs.receive_name());
    form.load_value(14, &real_text(gs.scale()));
    form.load_value(15, &int_text(gs.unit_flag()));
    form.load_value(16, &gs.unit_name());
    form.load_value(17, &real_text(gs.unit_value()));
    form.load_value(18, &int_text(gs.line_weight_grad()));
    form.load_value(19, &real_text(gs.max_line_weight()));
    form.load_value(20, &gs.date());
    form.load_value(21, &real_text(gs.resolution()));
    if gs.has_max_coord() {
        form.load_value(22, &real_text(gs.max_coord()));
    }
    form.load_value(23, &gs.author_name());
    form.load_value(24, &gs.company_name());
    form.load_value(25, &int_text(gs.iges_version()));
    form.load_value(
        26,
        &name_text(IgesDataBasicEditor::iges_version_name(gs.iges_version())),
    );
    form.load_value(27, &int_text(gs.drafting_standard()));
    form.load_value(
        28,
        &name_text(IgesDataBasicEditor::drafting_name(gs.drafting_standard())),
    );
    form.load_value(29, &gs.last_change_date());
    form.load_value(30, &gs.application_protocol());
}

/// A field number of the form and the member of the global section that stores it.
type GlobalSetter<V> = (i32, fn(&mut IgesDataGlobalSection, V));
/// The same for a text, whose setter takes a borrow of any lifetime.
type GlobalTextSetter = (i32, fn(&mut IgesDataGlobalSection, &Handle<HAsciiString>));

/// Takes the modified values that are texts.
fn apply_texts(form: &IFSelectEditForm, gs: &mut IgesDataGlobalSection) -> Result<(), Failure> {
    let setters: [GlobalTextSetter; 11] = [
        (4, IgesDataGlobalSection::set_send_name),
        (5, IgesDataGlobalSection::set_file_name),
        (6, IgesDataGlobalSection::set_system_id),
        (7, IgesDataGlobalSection::set_interface_version),
        (13, IgesDataGlobalSection::set_receive_name),
        (16, IgesDataGlobalSection::set_unit_name),
        (20, IgesDataGlobalSection::set_date),
        (23, IgesDataGlobalSection::set_author_name),
        (24, IgesDataGlobalSection::set_company_name),
        (29, IgesDataGlobalSection::set_last_change_date),
        (30, IgesDataGlobalSection::set_application_protocol),
    ];
    for (num, setter) in setters {
        if form.is_modified(num)? {
            setter(gs, &form.edited_value(num)?);
        }
    }
    Ok(())
}

/// Takes the modified values that are numbers.
fn apply_numbers(form: &IFSelectEditForm, gs: &mut IgesDataGlobalSection) -> Result<(), Failure> {
    let integers: [GlobalSetter<i32>; 9] = [
        (8, IgesDataGlobalSection::set_integer_bits),
        (9, IgesDataGlobalSection::set_max_power10_single),
        (10, IgesDataGlobalSection::set_max_digits_single),
        (11, IgesDataGlobalSection::set_max_power10_double),
        (12, IgesDataGlobalSection::set_max_digits_double),
        (15, IgesDataGlobalSection::set_unit_flag),
        (18, IgesDataGlobalSection::set_line_weight_grad),
        (25, IgesDataGlobalSection::set_iges_version),
        (27, IgesDataGlobalSection::set_drafting_standard),
    ];
    for (num, setter) in integers {
        if form.is_modified(num)? {
            setter(gs, edited_int(form, num)?);
        }
    }
    let reals: [GlobalSetter<f64>; 3] = [
        (14, IgesDataGlobalSection::set_scale),
        (19, IgesDataGlobalSection::set_max_line_weight),
        (21, IgesDataGlobalSection::set_resolution),
    ];
    for (num, setter) in reals {
        if form.is_modified(num)? {
            setter(gs, edited_real(form, num)?);
        }
    }
    if form.is_modified(22)? {
        let edited = form.edited_value(22)?;
        let value = match edited.get() {
            Some(text) => text.real_value()?,
            None => 0.0,
        };
        gs.set_max_coord(value);
    }
    Ok(())
}

/// The first character of a modified delimiter.
fn apply_delimiter(form: &IFSelectEditForm, num: i32, set: impl FnOnce(u8)) -> Result<(), Failure> {
    if !form.is_modified(num)? {
        return Ok(());
    }
    let edited = form.edited_value(num)?;
    if let Some(first) = edited
        .get()
        .and_then(|text| text.string().as_bytes().first().copied())
    {
        set(first);
    }
    Ok(())
}

impl IFSelectEditor for IgesSelectEditHeader {
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
        Ok(iges_live(&value, "IGES edit header")?.h_string_value())
    }
    fn load(
        &self,
        form: &Handle<IFSelectEditForm>,
        _ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let modl = model.cast::<IgesDataIgesModel>();
        let Some(live) = modl.get() else {
            return Ok(false);
        };
        let edit = iges_live(form, "IGES edit header")?;
        let lines: Vec<Handle<HAsciiString>> =
            live.start_section().get().map_or_else(Vec::new, |list| {
                list.sequence(|items| items.iter().cloned().collect())
            });
        edit.load_list(1, Some(&lines));
        load_section(edit, &live.global_section());
        Ok(true)
    }
    fn update(
        &self,
        form: &Handle<IFSelectEditForm>,
        num: i32,
        newval: &Handle<HAsciiString>,
        enforce: bool,
    ) -> Result<bool, Failure> {
        let edit = iges_live(form, "IGES edit header")?;
        let text = newval.get().map(|live| live.string().as_bytes().to_vec());
        match num {
            15 | 16 => {
                if !enforce {
                    return Ok(false);
                }
                let content = text.unwrap_or_default();
                let flag = if num == 15 {
                    iges_atoi(&content)
                } else {
                    IgesDataBasicEditor::unit_name_flag(&content)
                };
                let name = IgesDataBasicEditor::unit_flag_name(flag);
                if name.is_empty() {
                    return Ok(false);
                }
                if num == 15 {
                    edit.touch(16, &name_text(name))?;
                } else {
                    edit.touch(15, &int_text(flag))?;
                }
                edit.touch(17, &real_text(IgesDataBasicEditor::unit_flag_value(flag)))?;
            }
            25 | 27 => {
                let default = if num == 25 { 3 } else { 0 };
                let flag = text.map_or(default, |content| iges_atoi(&content));
                // The name of a drafting standard is searched among the versions of the norm.
                let name = IgesDataBasicEditor::iges_version_name(flag);
                if name.is_empty() {
                    return Ok(false);
                }
                edit.touch(num + 1, &name_text(name))?;
            }
            _ => {}
        }
        Ok(true)
    }
    fn apply(
        &self,
        form: &Handle<IFSelectEditForm>,
        _ent: &Handle<StandardTransient>,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let modl = model.cast::<IgesDataIgesModel>();
        let Some(live) = modl.get() else {
            return Ok(false);
        };
        let edit = iges_live(form, "IGES edit header")?;
        let mut gs = live.global_section();
        if edit.is_modified(1)? {
            let lines = Handle::new(IgesDataTextLines::default());
            if let (Some(target), Some(edited)) = (lines.get(), edit.edited_list(1)?) {
                for line in edited {
                    target.append(line);
                }
            }
            live.set_start_section(&lines, true);
        }
        apply_delimiter(edit, 2, |value| gs.set_separator(value))?;
        apply_delimiter(edit, 3, |value| gs.set_end_mark(value))?;
        apply_texts(edit, &mut gs)?;
        apply_numbers(edit, &mut gs)?;
        live.set_global_section(&gs);
        if edit.is_modified(15)? || edit.is_modified(16)? {
            let mut editor = IgesDataBasicEditor::with_model(&modl, &live.protocol().cast());
            if editor.set_unit_value(gs.unit_value()) {
                return Ok(false);
            }
            editor.apply_unit(true);
        }
        Ok(true)
    }
}
