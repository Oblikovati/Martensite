// SPDX-License-Identifier: GPL-2.0-only
//! Mesh reuse gated on geometry revision and deflection policy, plus measured
//! achieved deflection reporting (#32).
use crate::brep_mesh::{BRepMeshDiscretRoot, BRepMeshIncrementalMesh};
use crate::imesh_tools::MeshParameters;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::brep_tools::{self, BRepWriteOptions};
use martensite_tkbrep::operation::{GeometryRevision, OperationKind};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use std::collections::BTreeMap;
use std::io::{Read, Seek, SeekFrom};

/// Cache key requiring exact geometry revision, orientation, and deflection policy match (#32).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshReuseKey {
    revision: GeometryRevision,
    lin_deflection_bits: u64,
    ang_deflection_bits: u64,
    relative: bool,
    orientation: u8,
}

impl MeshReuseKey {
    /// Derives a deterministic reuse key from `shape` and `parameters`.
    pub fn from_shape_and_parameters(shape: &Shape, parameters: &MeshParameters) -> Self {
        let mut opts = Vec::with_capacity(18);
        opts.extend_from_slice(&parameters.deflection.to_bits().to_le_bytes());
        opts.extend_from_slice(&parameters.angle.to_bits().to_le_bytes());
        opts.push(u8::from(parameters.relative));
        let orientation = shape.orientation() as u8;
        opts.push(orientation);
        // Hash pure B-rep geometry/topology without attached triangulations so meshing
        // does not alter the reuse key (#32).
        let mut stream = IoStream::from_bytes(Vec::new());
        let write_opts = BRepWriteOptions {
            with_triangles: false,
            with_normals: false,
            ..BRepWriteOptions::default()
        };
        let _ = brep_tools::write_with_options(
            shape,
            &mut stream,
            write_opts,
            &ProgressRange::default(),
        );
        let mut brep_bytes = Vec::new();
        if let Some(mut device) = stream.into_device()
            && device.seek(SeekFrom::Start(0)).is_ok()
        {
            let _ = device.read_to_end(&mut brep_bytes);
        }
        let revision =
            GeometryRevision::from_canonical_bytes(OperationKind::Copy, &brep_bytes, &opts);
        Self {
            revision,
            lin_deflection_bits: parameters.deflection.to_bits(),
            ang_deflection_bits: parameters.angle.to_bits(),
            relative: parameters.relative,
            orientation,
        }
    }
}

/// Measured deflection quality actually achieved on a meshed shape (#32).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AchievedDeflectionReport {
    face_count: usize,
    triangle_count: usize,
    max_chordal_deflection: f64,
}

impl AchievedDeflectionReport {
    /// Number of triangulated faces measured.
    pub fn face_count(&self) -> usize {
        self.face_count
    }
    /// Total triangles across all measured faces.
    pub fn triangle_count(&self) -> usize {
        self.triangle_count
    }
    /// Maximum measured chordal deviation of triangle vertices/centroids from analytic surfaces.
    pub fn max_chordal_deflection(&self) -> f64 {
        self.max_chordal_deflection
    }
}

/// Measures the achieved chordal deflection of all face triangulations on `shape` (#32).
pub fn measure_achieved_deflection(shape: &Shape) -> Result<AchievedDeflectionReport, Failure> {
    let mut face_count = 0usize;
    let mut triangle_count = 0usize;
    let mut max_chordal = 0.0_f64;

    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        let face = topods::face(explorer.current())?;
        if let Ok((tri, loc)) = BRepTool::triangulation(&face, mesh_purpose::NONE)
            && let Some(mesh) = tri.get()
        {
            face_count += 1;
            let nb_tris = mesh.nb_triangles();
            triangle_count += nb_tris as usize;
            let trsf = loc.transformation();
            let adaptor = BRepAdaptorSurface::with_face(&face, true)?;
            if mesh.has_uv_nodes() {
                for idx in 1..=nb_tris {
                    let t = mesh.triangle(idx)?;
                    let (n1, n2, n3) = t.get();
                    let p1 = mesh.node(n1)?.transformed(trsf);
                    let p2 = mesh.node(n2)?.transformed(trsf);
                    let p3 = mesh.node(n3)?.transformed(trsf);
                    let centroid = Point3::new(
                        (p1.x() + p2.x() + p3.x()) / 3.0,
                        (p1.y() + p2.y() + p3.y()) / 3.0,
                        (p1.z() + p2.z() + p3.z()) / 3.0,
                    );
                    let uv1 = mesh.uv_node(n1)?;
                    let uv2 = mesh.uv_node(n2)?;
                    let uv3 = mesh.uv_node(n3)?;
                    let u_mid = (uv1.x() + uv2.x() + uv3.x()) / 3.0;
                    let v_mid = (uv1.y() + uv2.y() + uv3.y()) / 3.0;
                    if let Ok(surf_pt) = adaptor.value(u_mid, v_mid) {
                        max_chordal = max_chordal.max(centroid.distance(&surf_pt));
                    }
                }
            }
        }
        explorer.next()?;
    }

    Ok(AchievedDeflectionReport {
        face_count,
        triangle_count,
        max_chordal_deflection: max_chordal,
    })
}

/// Deterministic mesh cache reusing triangulations only on exact key match (#32).
#[derive(Clone, Debug, Default)]
pub struct MeshReuseCache {
    entries: BTreeMap<MeshReuseKey, (Shape, AchievedDeflectionReport)>,
}

impl MeshReuseCache {
    /// Creates an empty mesh reuse cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a cached `(Shape, AchievedDeflectionReport, true)` on exact key match,
    /// or meshes `shape` with `parameters` and caches the result `(Shape, report, false)` (#32).
    pub fn mesh_or_reuse(
        &mut self,
        shape: &Shape,
        parameters: &MeshParameters,
    ) -> Result<(Shape, AchievedDeflectionReport, bool), Failure> {
        let key = MeshReuseKey::from_shape_and_parameters(shape, parameters);
        if let Some((cached_shape, report)) = self.entries.get(&key) {
            return Ok((cached_shape.clone(), *report, true));
        }
        // Mesh on an isolated copy so the caller's shape and its revision stay untouched (#32).
        let copy = BRepBuilderAPICopy::with_shape(shape, true, false)?
            .shape()?
            .clone();
        let mesher = BRepMeshIncrementalMesh::with_shape(
            &copy,
            parameters.deflection,
            parameters.relative,
            parameters.angle,
            parameters.in_parallel,
        )?;
        let _ = mesher.is_done();
        let report = measure_achieved_deflection(&copy)?;
        self.entries.insert(key, (copy.clone(), report));
        Ok((copy, report, false))
    }

    /// Number of cached mesh entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when the cache holds no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

    #[test]
    fn cache_reuses_identical_mesh_and_invalidates_on_deflection_change() {
        let shape = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        let mut cache = MeshReuseCache::new();
        let coarse = MeshParameters {
            deflection: 0.2,
            angle: 0.5,
            ..MeshParameters::default()
        };
        let fine = MeshParameters {
            deflection: 0.05,
            angle: 0.2,
            ..MeshParameters::default()
        };

        let (_, report1, reused1) = cache.mesh_or_reuse(&shape, &coarse).expect("first mesh");
        assert!(!reused1);
        assert_eq!(report1.face_count(), 6);
        assert!(report1.triangle_count() >= 12);
        assert!(report1.max_chordal_deflection() <= 1e-6);

        let (_, _, reused2) = cache.mesh_or_reuse(&shape, &coarse).expect("reused mesh");
        assert!(reused2);

        let (_, _, reused3) = cache.mesh_or_reuse(&shape, &fine).expect("fine mesh");
        assert!(!reused3);
        assert_eq!(cache.len(), 2);
    }
}
