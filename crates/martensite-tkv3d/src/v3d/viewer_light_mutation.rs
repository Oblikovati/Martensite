//! Light-node mutation preserves native handle identity and iterator links.
use martensite_tkernel::{
    ncollection::AllocatorList,
    standard::{Handle, raise_native_failure},
};
use martensite_tkservice::graphic3d::Graphic3dCLightSource;
pub(super) fn remove_light_node(
    chain: &mut AllocatorList<Handle<dyn Graphic3dCLightSource>>,
    identity: *const (),
) {
    let mut cursor = chain.cursor();
    while cursor.more() {
        if cursor
            .value(chain)
            .is_some_and(|stored| stored.object_address() == identity)
        {
            raise_native_failure(chain.remove(&mut cursor));
            return;
        }
        cursor.next(chain);
    }
}

pub(super) fn contains_light_node(
    chain: &AllocatorList<Handle<dyn Graphic3dCLightSource>>,
    identity: *const (),
) -> bool {
    chain
        .iter()
        .any(|stored| stored.object_address() == identity)
}
