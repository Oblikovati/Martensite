//! The header and directory editors on a model made by hand.
use super::select_tests::{model_of, plain};
use super::{IgesSelectEditDirPart, IgesSelectEditHeader};
use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
use martensite_tkxsbase::{if_select::IFSelectEditor, interface::InterfaceInterfaceModel};

#[test]
fn the_editors_load_and_apply_a_model() {
    let entity = plain(110);
    let model = model_of(&[&entity]);
    model.get().unwrap().change_global_section(|header| {
        header.set_unit_flag(2);
        header.set_scale(1.0);
    });
    let editor = IgesSelectEditHeader::new().unwrap();
    let form = editor.get().unwrap().form(false, true).unwrap();
    let general: Handle<dyn InterfaceInterfaceModel> = model.cast();
    let live_form = form.get().unwrap();
    assert!(live_form.load_model(&general).unwrap());
    assert!(
        live_form
            .modify(5, &Handle::new(HAsciiString::new("Other.igs")), false)
            .unwrap()
    );
    assert!(live_form.apply().unwrap());
    let name = model.get().unwrap().global_section().file_name();
    assert_eq!(name.get().unwrap().string().as_bytes(), b"Other.igs");
    let dir = IgesSelectEditDirPart::new().unwrap();
    let dir_form = dir.get().unwrap().form(false, true).unwrap();
    let live_dir = dir_form.get().unwrap();
    assert!(live_dir.load_data(&entity.cast(), &general).unwrap());
    assert!(
        live_dir
            .modify(8, &Handle::new(HAsciiString::new("5")), false)
            .unwrap()
    );
    assert!(
        live_dir
            .modify(22, &Handle::new(HAsciiString::new("LAB")), false)
            .unwrap()
    );
    assert!(live_dir.apply().unwrap());
    let live = entity.get().unwrap();
    assert_eq!(live.level(), 5);
    assert_eq!(
        live.short_label().get().unwrap().string().as_bytes(),
        b"LAB"
    );
}
