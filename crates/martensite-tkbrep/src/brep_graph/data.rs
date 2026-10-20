//! `BRepGraph_Data`: what a graph owns.
use super::cache_registry::CacheRegistry;
use super::layer_registry::LayerRegistry;
use crate::brep_graph_inc::Storage;
use std::sync::atomic::{AtomicU64, Ordering};

/// `BRepGraph_Data`: the incidence storage (the only source of truth) and the layer and
/// cache registries. The native view members are not stored: the Rust views borrow the
/// graph when asked for.
#[derive(Debug)]
pub struct Data {
    /// `myIncStorage`.
    pub(crate) inc_storage: Storage,
    /// `myLayerRegistry`, attached to this graph's token.
    pub(crate) layers: LayerRegistry,
    /// `myCacheRegistry`, attached to this graph's token.
    pub(crate) caches: CacheRegistry,
}

impl Default for Data {
    // `Attach(this)` of both registries: a fresh attachment token names this graph.
    fn default() -> Self {
        let token = next_attachment_token();
        Self {
            inc_storage: Storage::default(),
            layers: LayerRegistry::for_graph(token),
            caches: CacheRegistry::for_graph(token),
        }
    }
}

// The process-unique, non-zero identity a graph gives its registries (the native
// `BRepGraph*` a layer or cache stores).
fn next_attachment_token() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
