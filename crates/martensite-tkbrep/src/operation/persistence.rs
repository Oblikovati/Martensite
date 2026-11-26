// SPDX-License-Identifier: GPL-2.0-only
//! Identity-preserving native B-rep persistence contract for host documents (#495).
use super::{OperationError, OperationKind};
use crate::brep::BRepBuilder;
use crate::brep_tools;
use crate::topexp;
use crate::topods::Shape;
use crate::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::io::{Read, Seek, SeekFrom};

/// Stable host-assigned persistent name for a topological entity (#495).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersistentEntityName(String);

impl PersistentEntityName {
    /// Creates a new persistent entity name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrows the underlying name string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Self-contained B-rep body archive preserving exact topology, geometry, and
/// persistent entity names across save/restore round-trips (#495).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistentBodyArchive {
    format_version: u32,
    brep_bytes: Vec<u8>,
    named_entities: Vec<(PersistentEntityName, ShapeEnum, usize)>,
}

impl PersistentBodyArchive {
    /// Current archive format version.
    pub const FORMAT_VERSION: u32 = 1;

    /// Serializes `shape` and binds every `(name, entity)` pair to its deterministic
    /// 1-based `TopExp::MapShapes` index (#495).
    pub fn capture(
        shape: &Shape,
        names: &[(PersistentEntityName, Shape)],
    ) -> Result<Self, OperationError> {
        if shape.is_null() {
            return Err(OperationError::kernel_failure(
                OperationKind::Copy,
                "cannot archive a null body",
            ));
        }
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes(shape, &mut map, true, true);

        let mut named_entities = Vec::with_capacity(names.len());
        for (name, entity) in names {
            let index = map.find_index(entity);
            if index == 0 {
                return Err(OperationError::kernel_failure(
                    OperationKind::Copy,
                    format!(
                        "persistent name '{}' does not belong to the archived body",
                        name.as_str()
                    ),
                ));
            }
            let kind = entity.shape_type().map_err(|err| {
                OperationError::kernel_failure(OperationKind::Copy, err.message().to_string())
            })?;
            named_entities.push((name.clone(), kind, index));
        }

        let mut stream = IoStream::from_bytes(Vec::new());
        brep_tools::write(shape, &mut stream, &ProgressRange::default()).map_err(|err| {
            OperationError::kernel_failure(OperationKind::Copy, err.message().to_string())
        })?;
        let mut brep_bytes = Vec::new();
        if let Some(mut device) = stream.into_device()
            && device.seek(SeekFrom::Start(0)).is_ok()
        {
            let _ = device.read_to_end(&mut brep_bytes);
        }

        Ok(Self {
            format_version: Self::FORMAT_VERSION,
            brep_bytes,
            named_entities,
        })
    }

    /// Restores the B-rep body and re-resolves every persistent name to its exact
    /// restored topological entity (#495).
    pub fn restore(&self) -> Result<(Shape, Vec<(PersistentEntityName, Shape)>), OperationError> {
        let mut stream = IoStream::from_bytes(self.brep_bytes.clone());
        let mut restored = Shape::default();
        brep_tools::read(
            &mut restored,
            &mut stream,
            &BRepBuilder::default(),
            &ProgressRange::default(),
        )
        .map_err(|err| {
            OperationError::kernel_failure(OperationKind::Copy, err.message().to_string())
        })?;

        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes(&restored, &mut map, true, true);
        let mut resolved = Vec::with_capacity(self.named_entities.len());
        for (name, expected_kind, index) in &self.named_entities {
            let entity = map
                .find_key(*index)
                .map_err(|err| {
                    OperationError::kernel_failure(OperationKind::Copy, err.message().to_string())
                })?
                .clone();
            let actual_kind = entity.shape_type().map_err(|err| {
                OperationError::kernel_failure(OperationKind::Copy, err.message().to_string())
            })?;
            if actual_kind != *expected_kind {
                return Err(OperationError::kernel_failure(
                    OperationKind::Copy,
                    format!(
                        "persistent name '{}' restored kind {:?}, expected {:?}",
                        name.as_str(),
                        actual_kind,
                        expected_kind
                    ),
                ));
            }
            resolved.push((name.clone(), entity));
        }
        Ok((restored, resolved))
    }

    /// Archive format version.
    pub fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Serialized B-rep payload bytes.
    pub fn brep_bytes(&self) -> &[u8] {
        &self.brep_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::{BRepBuilder, BRepTool};
    use crate::topods::{Compound, Vertex, vertex};
    use martensite_tkmath::gp::Point3;

    #[test]
    fn round_trip_preserves_topology_geometry_and_persistent_names() {
        let builder = BRepBuilder::default();
        let mut v1 = Vertex::default();
        let mut v2 = Vertex::default();
        builder
            .make_vertex_at(&mut v1, Point3::new(1.0, 2.0, 3.0), 1e-6)
            .expect("v1");
        builder
            .make_vertex_at(&mut v2, Point3::new(4.0, 5.0, 6.0), 1e-6)
            .expect("v2");
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let s1 = v1.into_shape();
        let s2 = v2.into_shape();
        let mut body = compound.into_shape();
        builder.add(&mut body, &s1).expect("add v1");
        builder.add(&mut body, &s2).expect("add v2");

        let archive = PersistentBodyArchive::capture(
            &body,
            &[
                (PersistentEntityName::new("origin_corner"), s1),
                (PersistentEntityName::new("far_corner"), s2),
            ],
        )
        .expect("capture archive");

        let (_restored, named) = archive.restore().expect("restore archive");
        assert_eq!(named.len(), 2);
        let p1 = BRepTool::pnt(&vertex(&named[0].1).expect("v1")).expect("p1");
        let p2 = BRepTool::pnt(&vertex(&named[1].1).expect("v2")).expect("p2");
        assert_eq!(named[0].0.as_str(), "origin_corner");
        assert_eq!(named[1].0.as_str(), "far_corner");
        assert!((p1.x() - 1.0).abs() < 1e-12 && (p2.z() - 6.0).abs() < 1e-12);
    }
}
