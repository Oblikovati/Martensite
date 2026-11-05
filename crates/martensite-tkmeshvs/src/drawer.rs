// SPDX-License-Identifier: GPL-2.0-only
//! MeshVSDrawer: six independent typed appearance maps.
mod channels;
mod copying;
mod source;
mod storage;
pub use copying::MeshVSDrawerBaseType;
use martensite_tkernel::standard::StandardTransient;
pub(crate) use source::registered_drawer;
pub use source::{MeshVSDrawerSource, meshvs_drawer_assign};
use storage::DrawerMaps;
/// Genuine drawer state; `let drawer=MeshVSDrawer::default(); drawer.set_integer(7,42)`.
#[derive(Default)]
pub struct MeshVSDrawer {
    transient: StandardTransient,
    maps: DrawerMaps,
}
martensite_tkernel::impl_transient!(
    MeshVSDrawer, "MeshVSDrawer", StandardTransient, transient,
    interfaces: [dyn MeshVSDrawerSource]
);
