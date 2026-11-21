//! Concrete arguments keep their actual slots; typed reads own the current Arc snapshot (#592).
use super::{
    Graphic3dClipPlane, Graphic3dClipPlaneSequenceSource, Graphic3dClipPlaneSource,
    Graphic3dSequenceOfHClipPlane,
};
use martensite_tkernel::standard::{
    Handle, HandleFieldRef, HandleObserver, HandleReference, ProjectedHandleReference,
};

macro_rules! borrowed_clip_source_argument {
    ($concrete:ty, $source:ty) => {
        impl HandleReference<$source> for Handle<$concrete> {
            fn observe(&self) -> HandleObserver<'_, $source> {
                // The projected snapshot owns its target after the temporary adapter is released.
                HandleObserver::Snapshot(
                    ProjectedHandleReference::<$concrete, $source>::new(self)
                        .observe()
                        .into_handle(),
                )
            }
            fn reference_address(&self) -> *const () {
                std::ptr::from_ref(self).cast()
            }
        }
        impl<S> HandleReference<$source> for HandleFieldRef<'_, S, $concrete> {
            fn observe(&self) -> HandleObserver<'_, $source> {
                // Field replacement must be reread before each actual native argument use.
                HandleObserver::Snapshot(
                    ProjectedHandleReference::<$concrete, $source>::new(self)
                        .observe()
                        .into_handle(),
                )
            }
            fn reference_address(&self) -> *const () {
                // Keep actual field identity independently of its dynamically assigned target.
                HandleFieldRef::reference_address(self)
            }
        }
    };
}
borrowed_clip_source_argument!(Graphic3dClipPlane, dyn Graphic3dClipPlaneSource);
borrowed_clip_source_argument!(
    Graphic3dSequenceOfHClipPlane,
    dyn Graphic3dClipPlaneSequenceSource
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_argument_keeps_slot_identity_and_counts_owned_snapshots() {
        let plane = Handle::new(Graphic3dClipPlane::default());
        let reference: &dyn HandleReference<dyn Graphic3dClipPlaneSource> = &plane;
        let observed = reference.observe();
        assert_eq!(
            reference.reference_address(),
            std::ptr::from_ref(&plane).cast()
        );
        assert_eq!(observed.object_address(), plane.object_address());
        assert_eq!(plane.ref_count(), 2);
        drop(observed);
        assert_eq!(plane.ref_count(), 1);
        let owned = reference.clone_handle();
        assert_eq!(owned.ref_count(), 2);
        drop(owned);
        assert_eq!(plane.ref_count(), 1);
    }

    #[test]
    fn collection_argument_snapshot_keeps_slot_identity_and_preserves_null() {
        let collection = Handle::new(Graphic3dSequenceOfHClipPlane::default());
        let reference: &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource> = &collection;
        assert_eq!(
            reference.reference_address(),
            std::ptr::from_ref(&collection).cast()
        );
        assert_eq!(reference.observe().ref_count(), 2);
        assert_eq!(reference.object_address(), collection.object_address());
        let null = Handle::<Graphic3dSequenceOfHClipPlane>::default();
        let null_reference: &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource> = &null;
        assert!(null_reference.is_null());
        assert!(null_reference.observe().is_null());
    }
}
