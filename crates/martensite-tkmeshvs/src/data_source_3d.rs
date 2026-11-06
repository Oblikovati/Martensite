// SPDX-License-Identifier: GPL-2.0-only
//! `MeshVSDataSource3D`: a data source with cached prism and pyramid face
//! topologies. The class stays abstract (it implements no pure virtual member), so a data
//! source embeds [`MeshVSDataSource3DBase`] and implements [`MeshVSDataSource3D`].
//!
//! ```
//! use martensite_tkmeshvs::MeshVSDataSource3DBase;
//! let prism = MeshVSDataSource3DBase::create_prism_topology(3);
//! assert_eq!(prism.get().unwrap().array1().length(), 5);
//! assert!(MeshVSDataSource3DBase::create_pyramid_topology(2).is_null());
//! ```
use crate::data_source::{MeshVSDataSource, MeshVSTopologyHandle};
use martensite_tkernel::ncollection::{DataMap, HArray1, Sequence};
use martensite_tkernel::standard::{Handle, SharedState};

/// The private topology caches of `MeshVSDataSource3D`, keyed by the number of base points.
#[derive(Default)]
pub struct MeshVSDataSource3DBase {
    /// The prism topologies.
    pub prism_topos: SharedState<DataMap<i32, MeshVSTopologyHandle>>,
    /// The pyramid topologies.
    pub pyramid_topos: SharedState<DataMap<i32, MeshVSTopologyHandle>>,
}

impl MeshVSDataSource3DBase {
    /// `create_prism_topology`: the two bases (the first reversed) and the side quadrangles of a
    /// prism, null below three base points. Example: `MeshVSDataSource3DBase::create_prism_topology(4)`.
    pub fn create_prism_topology(base_points: i32) -> MeshVSTopologyHandle {
        let Some(result) = new_faces(base_points, base_points + 2) else {
            return Handle::default();
        };
        {
            let mut faces = result.change_array1();
            for i in 0..base_points {
                faces[1].prepend(i);
                faces[2].append(i + base_points);
                let side = &mut faces[3 + i];
                let next = (i + 1) % base_points;
                for node in [i, i + base_points, next + base_points, next] {
                    side.prepend(node);
                }
            }
        }
        Handle::new(result)
    }
    /// `create_pyramid_topology`: the reversed base and the side triangles over apex 0, null
    /// below three base points. Example: `MeshVSDataSource3DBase::create_pyramid_topology(4)`.
    pub fn create_pyramid_topology(base_points: i32) -> MeshVSTopologyHandle {
        let Some(result) = new_faces(base_points, base_points + 1) else {
            return Handle::default();
        };
        {
            let mut faces = result.change_array1();
            for i in 1..=base_points {
                faces[1].prepend(i);
                let side = &mut faces[1 + i];
                for node in [0, i, i % base_points + 1] {
                    side.append(node);
                }
            }
        }
        Handle::new(result)
    }
}

/// `HArray1<Sequence<int>>(1, faces)` when a solid exists.
fn new_faces(base_points: i32, faces: i32) -> Option<HArray1<Sequence<i32>>> {
    if base_points < 3 {
        return None;
    }
    HArray1::with_bounds(1, faces).ok()
}

/// A data source with the cached volume topologies of `MeshVSDataSource3D`.
pub trait MeshVSDataSource3D: MeshVSDataSource {
    /// The embedded caches; `source.data_source_3d_base()`.
    fn data_source_3d_base(&self) -> &MeshVSDataSource3DBase;
    /// `get_prism_topology`: the cached prism topology, created on the first request.
    /// Example: `source.get_prism_topology(3)`.
    fn get_prism_topology(&self, base_points: i32) -> MeshVSTopologyHandle {
        cached(
            &self.data_source_3d_base().prism_topos,
            base_points,
            MeshVSDataSource3DBase::create_prism_topology,
        )
    }
    /// `get_pyramid_topology`: the cached pyramid topology, created on the first request.
    /// Example: `source.get_pyramid_topology(4)`.
    fn get_pyramid_topology(&self, base_points: i32) -> MeshVSTopologyHandle {
        cached(
            &self.data_source_3d_base().pyramid_topos,
            base_points,
            MeshVSDataSource3DBase::create_pyramid_topology,
        )
    }
}
martensite_tkernel::impl_transient_interface!(
    dyn MeshVSDataSource3D,
    "MeshVSDataSource3D",
    dyn MeshVSDataSource
);

/// The bound topology, or a new one bound when not null (the const_cast cache).
fn cached(
    cache: &SharedState<DataMap<i32, MeshVSTopologyHandle>>,
    base_points: i32,
    create: fn(i32) -> MeshVSTopologyHandle,
) -> MeshVSTopologyHandle {
    if let Some(found) = cache.read().seek(&base_points) {
        return found.clone();
    }
    let result = create(base_points);
    if !result.is_null() {
        cache.write().bind(base_points, result.clone());
    }
    result
}
