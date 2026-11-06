//! Supporting CPU controls use real null-driver contexts, never original Vulkan entrypoints.
use super::*;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{ais::AisPoint, prs_mgr::PrsMgrPresentableObjectMembers as _};
mod geometry;
mod names;
mod rows;

fn frustum_cpu_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("frustum CPU session; expected an actual null-driver409view")
}

fn frustum_point_position(object: &Handle<dyn AisInteractiveObject>) -> [f64; 3] {
    let point = object.cast::<AisPoint>();
    let position = point.component().pnt();
    [position.x(), position.y(), position.z()]
}
pub(super) mod selected_boundary;
