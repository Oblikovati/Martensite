//! The packed position and normal stream of the opt-in mesh raster version (#594).
use crate::RenderVertex;
use std::sync::Arc;

pub(super) const PACKED_STRIDE: u32 = 28;
pub(super) const DECODE_PUSH_BYTES: u32 = 32;
const POSITION_LEVELS: f64 = u16::MAX as f64;
const OCTAHEDRAL_LEVELS: f64 = i16::MAX as f64;

/// The immutable position decode owned with the packed vertex stream.
pub(super) struct MeshPositionDecode {
    origin: [f32; 3],
    step: [f32; 3],
}

impl MeshPositionDecode {
    /// Match the two vec4 push members following the clip transform.
    pub(super) fn append_push_bytes(&self, output: &mut Vec<u8>) {
        for vector in [self.origin, self.step] {
            output.extend_from_slice(
                [vector[0], vector[1], vector[2], 0.]
                    .map(f32::to_ne_bytes)
                    .as_flattened(),
            );
        }
    }
}

pub(super) fn pack_mesh_vertices(
    vertices: &[RenderVertex],
    normals: &[[f32; 3]],
) -> Option<(Vec<u8>, Arc<MeshPositionDecode>)> {
    if vertices.len() != normals.len() {
        return None;
    }
    let axes = position_axes(vertices)?;
    let mut bytes = Vec::with_capacity(vertices.len().checked_mul(PACKED_STRIDE as usize)?);
    for (vertex, normal) in vertices.iter().zip(normals) {
        let oct = encode_octahedral(*normal)?;
        append_vertex(&mut bytes, vertex, &axes, oct);
    }
    let decode = MeshPositionDecode {
        origin: axes.map(|axis| axis.origin),
        step: axes.map(|axis| axis.step),
    };
    Some((bytes, Arc::new(decode)))
}

#[derive(Clone, Copy)]
struct PositionAxis {
    origin: f32,
    /// The binary64 quantizer determines the nearest integer code.
    units_per_step: f64,
    /// The finite binary32 step is the value loaded in the shader.
    step: f32,
}

impl PositionAxis {
    fn new(low: f32, high: f32) -> Option<Self> {
        let span = f64::from(high) - f64::from(low);
        let units_per_step = span / POSITION_LEVELS;
        let step = units_per_step as f32;
        let last = step * POSITION_LEVELS as f32 + low;
        if !last.is_finite() || (span != 0. && (step == 0. || step.is_subnormal())) {
            return None;
        }
        Some(Self {
            origin: low,
            units_per_step,
            step,
        })
    }

    fn quantize(self, coordinate: f32) -> u16 {
        if self.units_per_step == 0. {
            return 0;
        }
        ((f64::from(coordinate) - f64::from(self.origin)) / self.units_per_step).round() as u16
    }
}

fn position_axes(vertices: &[RenderVertex]) -> Option<[PositionAxis; 3]> {
    let first = vertices.first()?.position;
    let (mut low, mut high) = (first, first);
    for vertex in vertices {
        if vertex.weight != 1.
            || vertex
                .position
                .iter()
                .any(|value| !value.is_finite() || value.is_subnormal())
        {
            return None;
        }
        for axis in 0..3 {
            low[axis] = low[axis].min(vertex.position[axis]);
            high[axis] = high[axis].max(vertex.position[axis]);
        }
    }
    Some([
        PositionAxis::new(low[0], high[0])?,
        PositionAxis::new(low[1], high[1])?,
        PositionAxis::new(low[2], high[2])?,
    ])
}

fn append_vertex(
    output: &mut Vec<u8>,
    vertex: &RenderVertex,
    axes: &[PositionAxis; 3],
    [normal_x, normal_y]: [i16; 2],
) {
    let [x, y, z] = std::array::from_fn(|i| u32::from(axes[i].quantize(vertex.position[i])));
    let normal = u32::from(normal_x as u16) | (u32::from(normal_y as u16) << 16);
    output.extend_from_slice(
        [x | (y << 16), z, normal]
            .map(u32::to_ne_bytes)
            .as_flattened(),
    );
    output.extend_from_slice(vertex.color.map(f32::to_ne_bytes).as_flattened());
}

/// Project the unit direction onto the octahedron, then fold its back hemisphere.
/// Cigolle et al., JCGT 3(2), 2014, section 3.1.
fn encode_octahedral(normal: [f32; 3]) -> Option<[i16; 2]> {
    let [x, y, z] = normal.map(f64::from);
    let scale = x.abs() + y.abs() + z.abs();
    if !scale.is_finite() || scale == 0. || normal.iter().all(|value| !value.is_normal()) {
        return None;
    }
    let [x, y] = [x / scale, y / scale];
    let projected = if z < 0. {
        [(1. - y.abs()).copysign(x), (1. - x.abs()).copysign(y)]
    } else {
        [x, y]
    };
    Some(projected.map(|value| (value * OCTAHEDRAL_LEVELS).round() as i16))
}

#[cfg(test)]
#[path = "mesh_vertex_codec_tests.rs"]
mod tests;
