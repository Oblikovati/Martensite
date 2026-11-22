//! Query borrowed descriptors without constructing any of the 120 declared classes.
mod rtti_descriptor_handles;

#[test]
fn all_static_descriptors_preserve_declared_metadata_and_identity() {
    rtti_descriptor_handles::verify_all();
}
