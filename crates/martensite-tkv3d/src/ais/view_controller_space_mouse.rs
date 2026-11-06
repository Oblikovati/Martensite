//! Native controller HID dispatch using Martensite's fallible keyboard seam.
use super::AisViewController;
use martensite_tkernel::standard::Failure;
use martensite_tkservice::wnt::WntHidSpaceMouse;
impl AisViewController {
    /// Queue translation, allowed rotation and HID buttons in native order; `controller.update_3d_mouse(&report)?`.
    pub fn update_3d_mouse_base(&mut self, event: &WntHidSpaceMouse<'_>) -> Result<bool, Failure> {
        // The inherited handlers are virtual too: an installed override intercepts each one.
        let translation = self.update_3d_mouse_translation(event)?;
        let rotation = self.to_allow_rotation() && self.update_3d_mouse_rotation(event)?;
        let keys = self.update_3d_mouse_keys(event)?;
        Ok(translation || rotation || keys)
    }
}
