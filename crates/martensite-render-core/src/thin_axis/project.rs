//! Homogeneous positions and line interpolation stay coupled when cells become quads.
use crate::error::rejected;
use crate::{PrimitiveDraw, RenderError, RenderVertex};
#[derive(Clone, Copy)]
pub(super) struct AxisProjection {
    pub clip: [[f32; 4]; 2],
    pub window: [[f32; 2]; 2],
    source: [RenderVertex; 2],
    extent: [u32; 2],
    depth_clamp: bool,
}
impl AxisProjection {
    pub(super) fn new(
        draw: &PrimitiveDraw,
        pair: [u32; 2],
        extent: [u32; 2],
    ) -> Result<Self, RenderError> {
        let clip = pair.map(|i| clip_vertex(draw, draw.vertices[i as usize]));
        validate_projection(clip)?;
        let window = clip.map(|p| [0, 1].map(|i| (p[i] / p[3] + 1.) * extent[i] as f32 / 2.));
        let source = pair.map(|i| draw.vertices[i as usize]);
        Ok(Self {
            clip,
            window,
            source,
            extent,
            depth_clamp: draw.depth_clamp,
        })
    }
    pub(super) fn validate_clip(self) -> Result<(), RenderError> {
        if self.clip.iter().any(|p| {
            p[..2].iter().any(|v| v.abs() > p[3]) || (!self.depth_clamp && p[2].abs() > p[3])
        }) {
            return Err(rejected(
                self.clip,
                "thin-axis endpoints inside XY planes and Z planes unless semantic depth clamp is enabled",
            ));
        }
        Ok(())
    }
    pub(super) fn axis(self) -> Option<usize> {
        let [a, b] = self.window;
        if a == b {
            return Some(0);
        }
        if a[0] == b[0] {
            return Some(1);
        }
        (a[1] == b[1]).then_some(0)
    }
}
fn validate_projection(clip: [[f32; 4]; 2]) -> Result<(), RenderError> {
    if clip
        .iter()
        .any(|p| p.iter().any(|v| !v.is_finite()) || p[3] <= 0.)
    {
        return Err(rejected(
            clip,
            "finite positive-w thin-axis projection endpoints",
        ));
    }
    Ok(())
}

fn clip_vertex(draw: &PrimitiveDraw, vertex: RenderVertex) -> [f32; 4] {
    let source = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    // V2 owns unfused f32 projection in the builtin shader's explicit column order. This is
    // not a claim about a different GPU's contraction; the original image gate remains required.
    std::array::from_fn(|r| {
        ((draw.clip_transform[4 + r] * source[1] + draw.clip_transform[r] * source[0])
            + draw.clip_transform[8 + r] * source[2])
            + draw.clip_transform[12 + r] * source[3]
    })
}
impl AxisProjection {
    pub(super) fn quad_vertices(
        self,
        bounds: [[f32; 2]; 2],
        axis: usize,
    ) -> Result<[RenderVertex; 4], RenderError> {
        let corners = quad_corners(bounds);
        let vertices = corners.map(|point| self.corner(point, axis));
        if vertices.iter().any(invalid_corner) {
            return Err(rejected(
                vertices,
                "finite positive-w thin-axis extrapolated corners",
            ));
        }
        Ok(vertices)
    }
    fn corner(self, point: [f32; 2], axis: usize) -> RenderVertex {
        let t =
            (point[axis] - self.window[0][axis]) / (self.window[1][axis] - self.window[0][axis]);
        let s =
            crate::wide_line_columns::perspective_parameter(t, self.clip[0][3], self.clip[1][3]);
        let mut vertex = crate::wide_line_columns::lerp_vertex(self.source[0], self.source[1], s);
        let clip: [f32; 4] =
            std::array::from_fn(|i| self.clip[0][i] + (self.clip[1][i] - self.clip[0][i]) * s);
        vertex.weight = clip[3];
        let xy = [0, 1].map(|i| (2. * point[i] / self.extent[i] as f32 - 1.) * clip[3]);
        vertex.position = [xy[0], xy[1], clip[2]];
        vertex
    }
}
fn invalid_corner(v: &RenderVertex) -> bool {
    v.weight <= 0.
        || !v.weight.is_finite()
        || v.position.iter().chain(&v.color).any(|x| !x.is_finite())
}

fn quad_corners([lo, hi]: [[f32; 2]; 2]) -> [[f32; 2]; 4] {
    [
        [lo[0], lo[1]],
        [hi[0], lo[1]],
        [hi[0], hi[1]],
        [lo[0], hi[1]],
    ]
}
