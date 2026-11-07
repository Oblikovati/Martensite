//! The values of the header editor and the members of the global section that store them.
use crate::iges_data::{IgesDataBasicEditor, IgesDataGlobalSection, iges_live};
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::if_select::IFSelectEditForm;

pub(super) fn int_text(value: i32) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::from_integer(value))
}

pub(super) fn real_text(value: f64) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::from_real(value))
}

pub(super) fn name_text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The integer of an edited value.
pub(super) fn edited_int(form: &IFSelectEditForm, num: i32) -> Result<i32, Failure> {
    let edited = form.edited_value(num)?;
    iges_live(&edited, "IGES edit header")?.integer_value()
}

/// The real of an edited value.
pub(super) fn edited_real(form: &IFSelectEditForm, num: i32) -> Result<f64, Failure> {
    let edited = form.edited_value(num)?;
    iges_live(&edited, "IGES edit header")?.real_value()
}

pub(super) fn load_section(form: &IFSelectEditForm, gs: &IgesDataGlobalSection) {
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
pub(super) fn apply_texts(
    form: &IFSelectEditForm,
    gs: &mut IgesDataGlobalSection,
) -> Result<(), Failure> {
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
pub(super) fn apply_numbers(
    form: &IFSelectEditForm,
    gs: &mut IgesDataGlobalSection,
) -> Result<(), Failure> {
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
pub(super) fn apply_delimiter(
    form: &IFSelectEditForm,
    num: i32,
    set: impl FnOnce(u8),
) -> Result<(), Failure> {
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
