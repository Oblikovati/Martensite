//! One rectangular frustum's box test, read once for a traversal that does not change it.
use super::{
    SelectMgrFrustumCore, SelectMgrRectangularFrustum, SelectMgrSelectingVolumeManager,
    base_intersector::SelectMgrBaseIntersectorMembers,
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::graphic3d::Graphic3dBndBox3d;

/// The frustum geometry and projection kind of the active rectangular volume. A box test
/// against the probe gives the result of `overlaps_box_inside(min, max, None)` without the
/// four lock acquisitions that each live test takes; a picking walk tests every node of the
/// object tree against the same volume (#594).
pub(crate) struct SelectMgrBoxProbe {
    core: SelectMgrFrustumCore<4>,
    ortho: bool,
    /// The intersector the probe was read from, compared by identity only.
    source: *const (),
}

impl SelectMgrSelectingVolumeManager {
    /// The probe of an initialized rectangular active volume; `None` for any other volume,
    /// whose box tests stay on `overlaps_box_inside`. Example: `volume.box_probe()`.
    pub(crate) fn box_probe(&self) -> Option<SelectMgrBoxProbe> {
        let rectangular = self.active.cast::<SelectMgrRectangularFrustum>();
        let frustum = rectangular.get()?;
        // An uninitialized frustum keeps the live path, which raises its error.
        frustum.rectangular_initialized("OverlapsBox").ok()?;
        Some(SelectMgrBoxProbe {
            core: frustum.frustum_core.read().clone(),
            ortho: frustum.is_orthographic(),
            source: self.active.object_address(),
        })
    }
}

impl SelectMgrBoxProbe {
    /// Whether `volume` tests boxes with the intersector this probe was read from;
    /// a copied volume shares it. Example: `probe.reads(&local)`.
    pub(crate) fn reads(&self, volume: &SelectMgrSelectingVolumeManager) -> bool {
        std::ptr::eq(volume.active.object_address(), self.source)
    }
    /// `overlaps_box_inside(min, max, None)` of the volume read; `probe.overlaps(&bounds)?`.
    pub(crate) fn overlaps(&self, bounds: &Graphic3dBndBox3d) -> Result<bool, Failure> {
        self.core
            .has_box_overlap(bounds.corner_min(), bounds.corner_max(), self.ortho, None)
    }
}

#[cfg(test)]
#[path = "frustum_box_probe_tests.rs"]
mod tests;
