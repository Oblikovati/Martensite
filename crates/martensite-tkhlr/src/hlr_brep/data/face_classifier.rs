//! #592: The hiding walk retains the exact cached classifier allocation, without a parent projection.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;
use std::sync::Arc;

pub(in crate::hlr_brep) trait FaceClassifier: Send + Sync {
    fn classify(&self, point: Point2, tolerance: f64, recadre: bool) -> Result<State, Failure>;
}

impl FaceClassifier for BRepTopAdaptorTopolTool {
    fn classify(&self, point: Point2, tolerance: f64, recadre: bool) -> Result<State, Failure> {
        BRepTopAdaptorTopolTool::classify(self, point, tolerance, recadre)
    }
}

pub(super) fn retain_face_classifier(
    owner: Handle<BRepTopAdaptorTopolTool>,
) -> Option<Arc<dyn FaceClassifier>> {
    owner
        .into_arc()
        .map(|owner| owner as Arc<dyn FaceClassifier>)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTool;

    #[test]
    fn retained_classifier_keeps_cached_identity_and_exact_owner_count() {
        let cache = BRepTopAdaptorTool::new();
        let owner = cache.get_topol_tool();
        let identity = owner.object_address();
        let weak = owner.downgrade();
        assert_eq!(owner.ref_count(), 2);
        let retained = retain_face_classifier(owner).unwrap();
        assert_eq!(Arc::as_ptr(&retained).cast::<()>(), identity);
        assert_eq!(Arc::strong_count(&retained), 2);
        drop(cache);
        assert_eq!(Arc::strong_count(&retained), 1);
        assert!(!weak.upgrade().is_null());
        drop(retained);
        assert!(weak.upgrade().is_null());
        assert!(retain_face_classifier(Handle::default()).is_none());
    }
}
