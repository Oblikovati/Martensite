//! Alternate presentation meshes are tessellated on private copies (#594).
use super::{StdPrsShadedShape, StdPrsToolTriangulatedShape};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions::construction_error};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkservice::graphic3d::{Graphic3dArrayOfTriangles, Graphic3dTriangleMeshLevel};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use std::sync::Arc;

/// A meshing request and independently approved deviation for one rendering level.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StdPrsMeshLevelRequest {
    /// Requested linear tessellation deflection in part coordinates.
    pub linear_deflection: f64,
    /// Requested angular tessellation deflection in radians.
    pub angular_deflection: f64,
    /// Approved maximum two-way distance between the meshes in object space.
    pub maximum_deviation: f64,
}

/// Screen-space LOD approval for one part's current topology and rendering mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct StdPrsMeshLod {
    levels: Arc<[StdPrsMeshLevelRequest]>,
    pixel_error: f64,
}

impl StdPrsMeshLod {
    /// Approve alternate meshes with a maximum screen-space deviation in pixels.
    /// The caller supplies each deviation bound independently of the meshing request.
    /// Example: `StdPrsMeshLod::new(vec![request], 0.5)?`.
    pub fn new(levels: Vec<StdPrsMeshLevelRequest>, pixel_error: f64) -> Result<Self, Failure> {
        if levels.is_empty() || !positive_finite(pixel_error) {
            return Err(construction_error(&format!(
                "Expected at least one mesh level with positive finite pixel error, got {} levels and {pixel_error}",
                levels.len(),
            )));
        }
        if let Some(level) = levels.iter().find(|level| !valid_request(level)) {
            return Err(construction_error(&format!(
                "Expected positive finite meshing deflections and approved deviation, got {level:?}",
            )));
        }
        Ok(Self {
            levels: levels.into(),
            pixel_error,
        })
    }

    pub(super) fn pixel_error(&self) -> f64 {
        self.pixel_error
    }

    pub(super) fn tessellate(&self, shape: &Shape) -> Vec<Graphic3dTriangleMeshLevel> {
        self.levels
            .iter()
            .filter_map(|level| tessellate_approved_level(shape, *level))
            .collect()
    }
}

fn valid_request(level: &StdPrsMeshLevelRequest) -> bool {
    positive_finite(level.linear_deflection)
        && positive_finite(level.angular_deflection)
        && positive_finite(level.maximum_deviation)
}

fn positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.
}

fn tessellate_approved_level(
    shape: &Shape,
    level: StdPrsMeshLevelRequest,
) -> Option<Graphic3dTriangleMeshLevel> {
    let triangles = StdPrsShadedShape::tessellate_level(
        shape,
        level.linear_deflection,
        level.angular_deflection,
    )
    .ok()
    .filter(|triangles| !triangles.is_null())?;
    Some(Graphic3dTriangleMeshLevel {
        triangles,
        maximum_deviation: level.maximum_deviation,
    })
}

impl StdPrsShadedShape {
    /// Build a presentation-only mesh on a fresh topology and reuse the standard lit triangle
    /// assembly. The linear and angular deflections are meshing requests.
    /// Example: `StdPrsShadedShape::tessellate_level(&part, 0.1, 0.5)?`.
    pub fn tessellate_level(
        shape: &Shape,
        linear_deflection: f64,
        angular_deflection: f64,
    ) -> Result<Handle<Graphic3dArrayOfTriangles>, Failure> {
        let mut copy = BRepBuilderAPICopy::with_shape(shape, true, false)?;
        let copy = copy.shape()?;
        let mesh = BRepMeshIncrementalMesh::with_shape(
            copy,
            linear_deflection,
            false,
            angular_deflection,
            false,
        )?;
        if !StdPrsToolTriangulatedShape::is_triangulated(copy)? {
            return Err(construction_error(&format!(
                "Presentation mesher returned {} with deflections ({linear_deflection}, {angular_deflection}); expected a mesh on every face",
                mesh.get_status_flags(),
            )));
        }
        Self::fill_triangles(copy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;

    #[test]
    fn deflection_levels_leave_the_original_part_unmeshed() {
        let mut cylinder = BRepPrimAPIMakeCylinder::new(5., 3.);
        let part = cylinder.shape().unwrap();
        assert!(!StdPrsToolTriangulatedShape::is_triangulated(part).unwrap());
        let fine = StdPrsShadedShape::tessellate_level(part, 0.01, 0.15).unwrap();
        let coarse = StdPrsShadedShape::tessellate_level(part, 0.3, 0.8).unwrap();
        assert!(!fine.is_null() && !coarse.is_null());
        assert!(fine.edge_number() > coarse.edge_number());
        assert!(!StdPrsToolTriangulatedShape::is_triangulated(part).unwrap());
    }

    #[test]
    fn approved_error_is_positive_and_survives_private_tessellation() {
        let request = StdPrsMeshLevelRequest {
            linear_deflection: 0.3,
            angular_deflection: 0.8,
            maximum_deviation: 0.5,
        };
        let invalid = StdPrsMeshLevelRequest {
            maximum_deviation: f64::NAN,
            ..request
        };
        assert!(StdPrsMeshLod::new(vec![invalid], 0.5).is_err());
        assert!(StdPrsMeshLod::new(vec![request], 0.).is_err());
        let approval = StdPrsMeshLod::new(vec![request], 0.75).unwrap();
        let mut cylinder = BRepPrimAPIMakeCylinder::new(5., 3.);
        let part = cylinder.shape().unwrap();
        let levels = approval.tessellate(part);
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].maximum_deviation, 0.5);
        assert_eq!(approval.pixel_error(), 0.75);
        assert!(!StdPrsToolTriangulatedShape::is_triangulated(part).unwrap());
    }
}
