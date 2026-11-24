//! Rendering-only mesh levels selected by a conservative projected error bound (#594).
use crate::{PrimitiveDraw, RenderError, RenderVertex, draw::DrawStream, error::rejected};
use crate::{primitive::PrimitiveSubmission, primitive_cache::GeometryStamp};
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfTriangles, Graphic3dTriangleMeshLevel, Graphic3dTypeOfPrimitiveArray,
};
use std::sync::Arc;

/// An alternate triangle mesh with a declared maximum object-space deviation.
/// Its vertices, normals, and indices describe rendering geometry only.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderMeshLevel {
    vertices: DrawStream<RenderVertex>,
    indices: DrawStream<u32>,
    normals: Option<DrawStream<[f32; 3]>>,
    maximum_deviation: f64,
}

impl RenderMeshLevel {
    /// Construct an alternate mesh with a bound on the two-way distance to the primary mesh
    /// in the part's coordinates.
    /// Example: `RenderMeshLevel::new(vertices, indices, Some(normals), 0.02)?`.
    pub fn new(
        vertices: Vec<RenderVertex>,
        indices: Vec<u32>,
        normals: Option<Vec<[f32; 3]>>,
        maximum_deviation: f64,
    ) -> Result<Self, RenderError> {
        let draw = PrimitiveDraw::new(vertices, indices, crate::identity_clip())?;
        if !maximum_deviation.is_finite() || maximum_deviation <= 0. {
            return Err(rejected(
                maximum_deviation,
                "a positive finite object-space deviation",
            ));
        }
        validate_level(&draw, normals.as_deref())?;
        Ok(Self {
            vertices: draw.vertices.clone(),
            indices: draw.indices.clone(),
            normals: normals.map(Into::into),
            maximum_deviation,
        })
    }

    /// Snapshot a TKService shaded tessellation and a supplied object-space deviation bound.
    /// Example: `RenderMeshLevel::from_triangle_array(&coarse_triangles, 0.1)?`.
    pub fn from_triangle_array(
        triangles: &Graphic3dArrayOfTriangles,
        maximum_deviation: f64,
    ) -> Result<Self, RenderError> {
        if !triangles.bounds().is_null()
            || triangles.has_vertex_texels()
            || triangles.has_vertex_colors()
        {
            return Err(rejected(
                triangles.vertex_number(),
                "a plain shaded triangle array without bounds, UVs or colors",
            ));
        }
        let primitive = PrimitiveSubmission {
            kind: Graphic3dTypeOfPrimitiveArray::Triangles,
            indices: triangles.indices(),
            attributes: triangles.attributes(),
            bounds: triangles.bounds(),
            upload: Default::default(),
            uid: 0,
            retained: None,
            mesh_levels: None,
        };
        let stamp = GeometryStamp::capture(&primitive);
        let vertices: Vec<_> = primitive
            .positions()?
            .into_iter()
            .map(|position| RenderVertex::homogeneous(position, [1.; 4]))
            .collect();
        let indices = primitive.read_indices(vertices.len())?;
        let normals = primitive.shared_normals()?.map(|values| values.to_vec());
        if !stamp.is_some_and(|stamp| stamp.matches(&primitive)) {
            return Err(rejected(
                triangles.vertex_number(),
                "a stable triangle level revision",
            ));
        }
        Self::new(vertices, indices, normals, maximum_deviation)
    }
}

fn validate_level(draw: &PrimitiveDraw, normals: Option<&[[f32; 3]]>) -> Result<(), RenderError> {
    if draw.vertices.is_empty()
        || draw.indices.is_empty()
        || draw
            .vertices
            .iter()
            .any(|vertex| vertex.weight != 1. || vertex.color[3] != 1.)
    {
        return Err(rejected(
            (draw.vertices.len(), draw.indices.len()),
            "nonempty opaque triangle geometry with unit weights",
        ));
    }
    if let Some(normals) = normals
        && (normals.len() != draw.vertices.len()
            || normals.iter().any(|normal| {
                !normal.iter().all(|component| component.is_finite()) || *normal == [0.; 3]
            }))
    {
        return Err(rejected(
            normals.len(),
            "one nonzero finite normal per level vertex",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MeshLod {
    approved: Arc<MeshAlternates>,
    bounds: ([f32; 3], [f32; 3]),
}

#[derive(Debug, PartialEq)]
pub(crate) struct MeshAlternates {
    levels: Vec<RenderMeshLevel>,
    pixel_error: f64,
}

pub(crate) struct SubmittedMeshLevels {
    approved: Arc<MeshAlternates>,
    primary: GeometryStamp,
}

impl SubmittedMeshLevels {
    pub(crate) fn bind(
        primitive: &PrimitiveSubmission,
        approved: Arc<MeshAlternates>,
    ) -> Option<Arc<Self>> {
        Some(Arc::new(Self {
            approved,
            primary: GeometryStamp::capture(primitive)?,
        }))
    }

    pub(crate) fn attach(&self, primitive: &PrimitiveSubmission, draws: &mut [PrimitiveDraw]) {
        if let [draw] = draws
            && draw
                .lighting
                .as_ref()
                .is_some_and(|light| !light.vertex_colors)
            && !draw.source_color_array
            && self.primary.matches(primitive)
        {
            let _ = draw.attach_mesh_levels(self.approved.clone());
        }
    }
}

impl MeshAlternates {
    fn new(mut levels: Vec<RenderMeshLevel>, pixel_error: f64) -> Result<Self, RenderError> {
        if levels.is_empty() || !pixel_error.is_finite() || pixel_error <= 0. {
            return Err(rejected(
                (levels.len(), pixel_error),
                "mesh levels and a positive finite pixel error",
            ));
        }
        levels.sort_by_key(|level| level.indices.len());
        Ok(Self {
            levels,
            pixel_error,
        })
    }

    pub(crate) fn snapshot(
        levels: &[Graphic3dTriangleMeshLevel],
        pixel_error: f64,
    ) -> Option<Arc<Self>> {
        let levels = levels
            .iter()
            .filter_map(|level| {
                RenderMeshLevel::from_triangle_array(
                    level.triangles.get()?,
                    level.maximum_deviation,
                )
                .ok()
            })
            .collect();
        Self::new(levels, pixel_error).ok().map(Arc::new)
    }
}

impl PrimitiveDraw {
    /// Enable screen-space level selection for an ordinary surface.
    /// Example: `let draw = draw.with_mesh_levels(vec![coarse], 0.45)?;`.
    pub fn with_mesh_levels(
        mut self,
        levels: Vec<RenderMeshLevel>,
        pixel_error: f64,
    ) -> Result<Self, RenderError> {
        let levels = Arc::new(MeshAlternates::new(levels, pixel_error)?);
        self.attach_mesh_levels(levels)?;
        Ok(self)
    }

    fn attach_mesh_levels(&mut self, approved: Arc<MeshAlternates>) -> Result<(), RenderError> {
        let Some(bounds) = crate::draw_instances::InstanceBounds::of(self) else {
            return Err(rejected(
                self.vertices.len(),
                "a finite primary mesh with unit weights",
            ));
        };
        validate_mesh_source(self, &approved.levels)?;
        self.mesh_lod = Some(Arc::new(MeshLod {
            approved,
            bounds: (bounds.minimum, bounds.maximum),
        }));
        Ok(())
    }
}

fn validate_mesh_source(
    draw: &PrimitiveDraw,
    levels: &[RenderMeshLevel],
) -> Result<(), RenderError> {
    if !ordinary_surface(draw)
        || levels.iter().any(|level| {
            level.indices.len() >= draw.indices.len()
                || (draw.lighting.is_some() && level.normals.is_none())
        })
    {
        return Err(rejected(
            draw.indices.len(),
            "ordinary opaque triangle geometry and simpler levels with matching lighting inputs",
        ));
    }
    Ok(())
}

fn ordinary_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && !draw.is_surface_blended()
        && draw.surface_cutoff.is_none()
        && draw.surface_texture.is_none()
        && draw.mesh_edges.is_none()
        && draw.clipping.is_none()
        && draw.hatch_rows.is_none()
        && draw.user_program.is_none()
        && draw.capping_pass == crate::RenderCappingPass::None
        && draw.quad_corners.is_empty()
        && draw
            .lighting
            .as_ref()
            .is_none_or(|light| light.fixed.is_none() && !light.pbr)
}

#[derive(Clone, Copy)]
pub(crate) enum FrameMeshLod {
    Direct,
    RetainedPrecomposed,
    Disabled,
}

pub(crate) fn select_levels(
    draws: &mut [PrimitiveDraw],
    extent: [u32; 2],
    policy: FrameMeshLod,
) -> bool {
    let mut authored = false;
    for draw in draws {
        let Some(lod) = &draw.mesh_lod else {
            continue;
        };
        authored = true;
        if let Some(level) = choose_level(draw, lod, extent, policy) {
            *draw = replace_mesh(draw, level);
        }
    }
    authored
}

fn choose_level<'a>(
    draw: &PrimitiveDraw,
    lod: &'a MeshLod,
    extent: [u32; 2],
    policy: FrameMeshLod,
) -> Option<&'a RenderMeshLevel> {
    let projection_admitted = match policy {
        FrameMeshLod::Direct => {
            draw.projection_factors.is_none() && draw.fixed_function_factors.is_none()
        }
        FrameMeshLod::RetainedPrecomposed => true,
        FrameMeshLod::Disabled => false,
    };
    if !ordinary_surface(draw) || !projection_admitted {
        return None;
    }
    let projection = ProjectionBounds::of(draw.clip_transform, lod.bounds, extent)?;
    lod.approved.levels.iter().find(|level| {
        (draw.lighting.is_none() || level.normals.is_some())
            && projection.error(level.maximum_deviation) <= lod.approved.pixel_error
    })
}

fn replace_mesh(draw: &PrimitiveDraw, level: &RenderMeshLevel) -> PrimitiveDraw {
    let mut selected =
        draw.with_replacement_geometry(level.vertices.clone(), level.indices.clone());
    selected.mesh_lod = None;
    selected.geometry_revision = Default::default();
    if let (Some(lighting), Some(normals)) = (&mut selected.lighting, &level.normals) {
        lighting.normals = normals.clone();
    }
    selected
}

struct ProjectionBounds {
    extent: [f64; 2],
    maximum_numerator: [f64; 2],
    minimum_w: f64,
    gradients: [f64; 3],
}

impl ProjectionBounds {
    fn of(matrix: [f32; 16], (low, high): ([f32; 3], [f32; 3]), extent: [u32; 2]) -> Option<Self> {
        let x = affine_interval(&matrix, 0, low, high);
        let y = affine_interval(&matrix, 1, low, high);
        let w = affine_interval(&matrix, 3, low, high);
        if !w[0].is_finite() || w[0] <= 0. {
            return None;
        }
        Some(Self {
            extent: extent.map(f64::from),
            maximum_numerator: [largest_absolute(x), largest_absolute(y)],
            minimum_w: w[0],
            gradients: [0, 1, 3].map(|row| {
                let coefficients = [matrix[row], matrix[row + 4], matrix[row + 8]].map(f64::from);
                martensite_cmath::hypot(
                    martensite_cmath::hypot(coefficients[0], coefficients[1]),
                    coefficients[2],
                )
            }),
        })
    }

    fn error(&self, deviation: f64) -> f64 {
        let delta_w = deviation * self.gradients[2];
        let positive_w = self.minimum_w - delta_w;
        if positive_w <= 0. {
            return f64::INFINITY;
        }
        // Bound each projective quotient over the full mesh and its deviation neighbourhood.
        let pixel_delta = |axis: usize| {
            let delta = deviation * self.gradients[axis];
            let numerator = delta + self.maximum_numerator[axis] / self.minimum_w * delta_w;
            numerator / positive_w * self.extent[axis] * 0.5
        };
        martensite_cmath::hypot(pixel_delta(0), pixel_delta(1)) * (1. + 64. * f64::EPSILON)
    }
}

fn affine_interval(matrix: &[f32; 16], row: usize, low: [f32; 3], high: [f32; 3]) -> [f64; 2] {
    let mut result = [f64::from(matrix[12 + row]); 2];
    for axis in 0..3 {
        let coefficient = f64::from(matrix[4 * axis + row]);
        let [a, b] = [
            coefficient * f64::from(low[axis]),
            coefficient * f64::from(high[axis]),
        ];
        result[0] += a.min(b);
        result[1] += a.max(b);
    }
    let roundoff = (f64::from(matrix[12 + row]).abs()
        + (0..3)
            .map(|axis| {
                f64::from(matrix[4 * axis + row]).abs()
                    * f64::from(low[axis]).abs().max(f64::from(high[axis]).abs())
            })
            .sum::<f64>())
        * (8. * f64::EPSILON);
    [result[0] - roundoff, result[1] + roundoff]
}

fn largest_absolute([low, high]: [f64; 2]) -> f64 {
    low.abs().max(high.abs())
}

#[cfg(test)]
#[path = "mesh_lod_tests.rs"]
mod tests;
