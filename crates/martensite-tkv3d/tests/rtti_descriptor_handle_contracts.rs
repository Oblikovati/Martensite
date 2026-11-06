//! Descriptor handle slots are queried without constructing any of the 120 declaration owners.
mod rtti_descriptor_handles;

#[test]
fn all_static_descriptor_handles_preserve_declared_metadata_and_borrowed_ownership() {
    // The single test avoids racing descriptor clone counters against another query test.
    rtti_descriptor_handles::verify_all();
}
