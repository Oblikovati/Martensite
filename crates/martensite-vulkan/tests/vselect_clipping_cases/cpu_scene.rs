//! Real viewer/selector with only device I/O replaced by the named RecordingDevice.
use super::prelude::*;
#[path = "../support/recording_device.rs"]
mod recording_device;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorResultMembers;
use martensite_vulkan::Graphic3dRenderDriver;
pub(super) use recording_device::RecordingDevice;

pub(super) fn session() -> (ViewerSession, RecordingDevice) {
    // The source camera remains409x409; the fake declares a real supported extent.
    let limit = 4096;
    session_with_limit(limit)
}

pub(super) fn session_with_limit(limit: u32) -> (ViewerSession, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), limit).unwrap();
    (ViewerSession::new(&driver).unwrap(), device)
}

pub(super) fn checked_frame(
    session: &ViewerSession,
    device: &RecordingDevice,
) -> martensite_vulkan::RenderFrame {
    let view = session
        .view()
        .get()
        .unwrap()
        .view()
        .cast::<martensite_vulkan::Graphic3dRenderView>();
    view.try_redraw()
        .expect("real RecordingDevice submission must succeed");
    device.last_frame()
}

pub(super) fn axis_hits(
    session: &ViewerSession,
    origin: [f64; 3],
    direction: [f64; 3],
) -> Vec<(usize, [f64; 3])> {
    use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorPickMembers;
    let selector = session.context().main_selector();
    let [x, y, z] = origin;
    let [dx, dy, dz] = direction;
    let ray = Axis1::new(Point3::new(x, y, z), Direction3::new(dx, dy, dz).unwrap());
    selector.pick_axis(ray, session.view().view_base()).unwrap();
    (1..=selector.nb_picked())
        .map(|rank| hit_observation(selector.get().unwrap(), rank))
        .collect()
}

pub(super) fn selected_names(session: &ViewerSession, names: &[&str]) -> Vec<String> {
    let context = session.context();
    let mut found = Vec::new();
    context.init_selected();
    while context.more_selected() {
        let object = context.selected_interactive().unwrap();
        for name in names {
            if session.object(name) == Some(&object) {
                found.push((*name).to_owned());
            }
        }
        context.next_selected();
    }
    found.sort();
    found
}

fn hit_observation(
    selector: &dyn martensite_tkv3d::select_mgr::SelectMgrViewerSelectorSource,
    rank: i32,
) -> (usize, [f64; 3]) {
    let owner = selector.picked(rank).unwrap();
    let point = selector.picked_point(rank).unwrap();
    (
        owner.selectable().object_address() as usize,
        [point.x(), point.y(), point.z()],
    )
}
