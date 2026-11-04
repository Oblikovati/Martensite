//! Driver, view and structure members that the frame tests reach only indirectly.
//!
//! Expected values come from the pinned native GlGraphicDriver/GlView/GlStructure
//! bodies: RemoveView forgets one view, RemoveStructure releases the groups and
//! nulls the caller handle, DefaultTextHeight is 16, TextSize falls back to it below a height of
//! 2, ZLayerMax starts from the default layer id, IsDefined means a bound window, and the
//! driver layer list starts BotOSD, Default, Top, Topmost, TopOSD.
// Shared fixtures; this file uses only part of each.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::AspectWindow, graphic3d::*};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::*;
use recording_device::RecordingDevice;

fn driver() -> Handle<Graphic3dRenderDriver> {
    Graphic3dRenderDriver::with_backend(RecordingDevice::default(), 128).unwrap()
}

fn bound_view(viewer: &V3dViewer) -> (Handle<dyn Graphic3dCView>, Handle<dyn AspectWindow>) {
    let view = viewer.create_view().unwrap();
    let window = view_setup::configure_view(&view);
    (view.get().unwrap().view(), window.cast())
}

fn exists(driver: &Graphic3dRenderDriver, window: &Handle<dyn AspectWindow>) -> bool {
    driver.view_exists(window, &mut Handle::default())
}

#[test]
fn remove_view_forgets_only_the_removed_view() {
    let driver = driver();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let (first, first_window) = bound_view(&viewer);
    let (_, second_window) = bound_view(&viewer);
    assert!(exists(&driver, &first_window) && exists(&driver, &second_window));
    driver.remove_view(&first);
    assert!(!exists(&driver, &first_window));
    assert!(exists(&driver, &second_window));
}

fn structure_with_group(
    driver: &Handle<Graphic3dRenderDriver>,
) -> (Handle<dyn Graphic3dCStructure>, Handle<dyn Graphic3dGroup>) {
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    let owner = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let structure = driver.create_structure(&manager).unwrap();
    let group = structure.new_group(&owner.cast()).unwrap();
    (structure, group)
}

fn group_count(structure: &Handle<dyn Graphic3dCStructure>) -> usize {
    structure.cstructure_base().fields.read().groups.len()
}

#[test]
fn remove_structure_releases_groups_and_nulls_the_caller_handle() {
    let driver = driver();
    let (mut structure, group) = structure_with_group(&driver);
    let retained = structure.clone();
    assert!(!structure.cast::<Graphic3dRenderStructure>().is_null());
    assert_eq!(group_count(&retained), 1);
    driver.remove_structure(&mut structure);
    assert!(structure.is_null());
    assert_eq!(group_count(&retained), 0);
    assert!(group.is_empty());
}

#[test]
fn structure_reports_the_driver_that_created_it() {
    // GlStructure::GlDriver: groups reach the driver's text height and shared context.
    let driver = driver();
    let (structure, _) = structure_with_group(&driver);
    let owner = structure.graphic_driver().cast::<Graphic3dRenderDriver>();
    assert!(owner == driver);
    let other = self::driver();
    assert!(structure.graphic_driver().cast::<Graphic3dRenderDriver>() != other);
}

#[test]
fn structure_group_highlight_and_clear_members_update_driver_state() {
    let driver = driver();
    let (structure, group) = structure_with_group(&driver);
    assert!(!group.cast::<Graphic3dDrawGroup>().is_null());
    let style = Handle::new(Graphic3dPresentationAttributes::default());
    structure.graphic_highlight(&style.cast());
    assert!(structure.cstructure_base().fields.read().highlight);
    structure.graphic_unhighlight();
    let fields = structure.cstructure_base().fields.read().clone();
    assert!(!fields.highlight && fields.highlight_style.is_null());
    structure.remove_group(&group);
    assert_eq!(group_count(&structure), 0);
    let (structure, _) = structure_with_group(&driver);
    structure.clear();
    assert_eq!(group_count(&structure), 0);
}

#[test]
fn default_text_height_and_text_size_follow_native_measurement_rules() {
    let driver = driver();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let (view, _) = bound_view(&viewer);
    assert_eq!(driver.default_text_height(), 16.);
    let fallback = driver.text_size(&view, "MM", 1.);
    assert_eq!(fallback, driver.text_size(&view, "MM", 16.));
    let [pair, ascent, descent] = driver.text_size(&view, "MM", 32.);
    assert!(pair > 0. && ascent > 0. && descent < ascent);
    // Courier is monospaced: equal characters advance equally and the widest line wins.
    assert_eq!(driver.text_size(&view, "MMMM", 32.)[0], 2. * pair);
    assert_eq!(driver.text_size(&view, "MM\nMMMM\nM", 32.)[0], 2. * pair);
    assert_eq!(driver.text_size(&Handle::default(), "MM", 32.), [0.; 3]);
}

#[test]
fn vertical_sync_starts_enabled_and_accessors_round_trip() {
    let driver = driver();
    assert!(driver.is_vertical_sync());
    driver.set_vertical_sync(true);
    assert!(driver.is_vertical_sync());
    driver.set_vertical_sync(false);
    assert!(!driver.is_vertical_sync());
}

#[test]
fn view_definition_layers_and_maximum_layer_follow_the_driver() {
    let driver = driver();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let unbound = viewer.create_view().unwrap().get().unwrap().view();
    assert!(!unbound.is_defined());
    let (view, _) = bound_view(&viewer);
    assert!(view.is_defined());
    let ids: Vec<i32> = view.layers().iter().map(|layer| layer.layer_id()).collect();
    let native = [
        GRAPHIC3D_ZLAYER_ID_BOT_OSD,
        GRAPHIC3D_ZLAYER_ID_DEFAULT,
        GRAPHIC3D_ZLAYER_ID_TOP,
        GRAPHIC3D_ZLAYER_ID_TOPMOST,
        GRAPHIC3D_ZLAYER_ID_TOP_OSD,
    ];
    assert_eq!(ids, native);
    assert_eq!(view.z_layer_max(), GRAPHIC3D_ZLAYER_ID_DEFAULT);
    let settings = Graphic3dZLayerSettings::default();
    driver
        .insert_layer_after(7, &settings, GRAPHIC3D_ZLAYER_ID_DEFAULT)
        .unwrap();
    assert_eq!(view.z_layer_max(), 7);
    assert_eq!(view.layers()[2].layer_id(), 7);
}
