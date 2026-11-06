//! Private pixel-quad lowering keeps line fragments in their original draw position.
use super::{diamond::exits_diamond, major_axis, stipple_on};
use crate::error::rejected;
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderThinAxisCoverage, RenderThinLineCoverage,
    RenderVertex,
};
use ash::vk;

const PIXEL_SCALE: i64 = 65_536;
const MAX_PIXELS: usize = 1_000_000;

pub(super) fn selected_draw(
    behavior: RenderThinLineCoverage,
    axes: RenderThinAxisCoverage,
    draw: &PrimitiveDraw,
    extent: vk::Extent2D,
    single_sample: bool,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if behavior == RenderThinLineCoverage::HardwareV1 || draw.topology() != PrimitiveTopology::Lines
    {
        return Ok(None);
    }
    // V4 keeps every line it cannot lower on the device rasterizer. Older versions keep refusing,
    // and a thin-axis conflict stays a configuration error for all of them.
    if let Err(refusal) =
        super::validation::validate_selection(behavior, axes, draw, extent, single_sample)
    {
        if behavior == RenderThinLineCoverage::DiamondExitFusedProjectionV4
            && axes == RenderThinAxisCoverage::HardwareV1
        {
            return Ok(None);
        }
        return Err(refusal);
    }
    expand(behavior, draw, [extent.width, extent.height]).map(Some)
}

fn expand(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    extent: [u32; 2],
) -> Result<PrimitiveDraw, RenderError> {
    let mut geometry = Geometry::default();
    for pair in draw.indices.as_chunks::<2>().0.iter() {
        append_segment(behavior, draw, [pair[0], pair[1]], extent, &mut geometry)?;
    }
    Ok(replacement(draw, geometry.vertices, geometry.indices))
}

#[derive(Default)]
struct Geometry {
    vertices: Vec<RenderVertex>,
    indices: Vec<u32>,
}

fn append_segment(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    pair: [u32; 2],
    extent: [u32; 2],
    geometry: &mut Geometry,
) -> Result<(), RenderError> {
    let source = pair.map(|index| draw.vertices[index as usize]);
    let clip = source.map(|vertex| projected_clip(behavior, draw, vertex));
    let Some(line) = clip_line(draw, clip, source)? else {
        return Ok(());
    };
    raster_segment(behavior, draw, line, extent, geometry)
}

#[derive(Clone, Copy)]
struct ClippedLine {
    clip: [[f64; 4]; 2],
    color: [[f64; 4]; 2],
    world: [[f64; 4]; 2],
}

fn clip_line(
    draw: &PrimitiveDraw,
    clip: [[f32; 4]; 2],
    source: [RenderVertex; 2],
) -> Result<Option<ClippedLine>, RenderError> {
    let color = source.map(|vertex| vertex.color);
    let clip = clip.map(|p| p.map(f64::from));
    if clip.iter().flatten().any(|v| !v.is_finite())
        || color.iter().flatten().any(|v| !v.is_finite())
    {
        return Err(rejected(
            (clip, color),
            "finite line clip coordinates and RGBA attributes",
        ));
    }
    let Some([first, last]) = clip_interval(clip, draw.depth_clamp) else {
        return Ok(None);
    };
    let color = color.map(|channels| channels.map(f64::from));
    let world = source.map(|vertex| super::user_clipping::world_vertex(draw, vertex));
    Ok(Some(ClippedLine {
        clip: [
            interpolate(clip[0], clip[1], first),
            interpolate(clip[0], clip[1], last),
        ],
        color: [
            interpolate(color[0], color[1], first),
            interpolate(color[0], color[1], last),
        ],
        world: [
            interpolate(world[0], world[1], first),
            interpolate(world[0], world[1], last),
        ],
    }))
}

fn clip_interval(clip: [[f64; 4]; 2], depth_clamp: bool) -> Option<[f64; 2]> {
    let mut interval = [0., 1.];
    for plane in 0..if depth_clamp { 4 } else { 6 } {
        let values = clip.map(|p| plane_value(p, plane));
        if !clip_plane(values, &mut interval) {
            return None;
        }
    }
    Some(interval)
}

fn plane_value(p: [f64; 4], plane: usize) -> f64 {
    match plane {
        0 => p[0] + p[3],
        1 => p[3] - p[0],
        2 => p[1] + p[3],
        3 => p[3] - p[1],
        4 => p[2] + p[3],
        _ => p[3] - p[2],
    }
}

fn clip_plane(values: [f64; 2], interval: &mut [f64; 2]) -> bool {
    if values[0] < 0. && values[1] < 0. {
        return false;
    }
    if values[0] >= 0. && values[1] >= 0. {
        return true;
    }
    let crossing = values[0] / (values[0] - values[1]);
    if values[0] < 0. {
        interval[0] = interval[0].max(crossing);
    } else {
        interval[1] = interval[1].min(crossing);
    }
    interval[0] <= interval[1]
}

fn interpolate(a: [f64; 4], b: [f64; 4], t: f64) -> [f64; 4] {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t)
}

fn projected_clip(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    vertex: RenderVertex,
) -> [f32; 4] {
    if behavior == RenderThinLineCoverage::DiamondExitFusedProjectionV4 {
        return super::fused_projection::clip(draw.clip_transform, vertex);
    }
    clip_vertex(draw, vertex)
}

fn clip_vertex(draw: &PrimitiveDraw, vertex: RenderVertex) -> [f32; 4] {
    let source = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    std::array::from_fn(|r| {
        ((draw.clip_transform[4 + r] * source[1] + draw.clip_transform[r] * source[0])
            + draw.clip_transform[8 + r] * source[2])
            + draw.clip_transform[12 + r] * source[3]
    })
}

fn raster_segment(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    line: ClippedLine,
    extent: [u32; 2],
    geometry: &mut Geometry,
) -> Result<(), RenderError> {
    let [start, end] = line.clip.map(|point| window_point(point, extent));
    if start.iter().chain(&end).any(|v| !v.is_finite()) {
        return Err(rejected((start, end), "finite projected line endpoints"));
    }
    let fixed = [fixed_coordinate(start[0])?, fixed_coordinate(start[1])?];
    let fixed_end = [fixed_coordinate(end[0])?, fixed_coordinate(end[1])?];
    let fixed = [fixed, fixed_end];
    let style = draw.line_style.expect("selected line style");
    let axis = major_axis(fixed[0], fixed[1]);
    for major_pixel in candidate_range(start[axis], end[axis], extent[axis]) {
        let minors = minor_candidates(start, end, axis, major_pixel, extent[1 - axis]);
        for minor_pixel in minors {
            let pixel = pixel_coordinates(axis, major_pixel, minor_pixel);
            if covered(fixed, pixel) && stipple_on(style.pattern, style.factor, pixel[axis]) {
                append_pixel(behavior, draw, line, pixel, extent, geometry)?;
            }
        }
    }
    Ok(())
}

fn window_point(clip: [f64; 4], extent: [u32; 2]) -> [f64; 2] {
    [0, 1].map(|axis| (clip[axis] / clip[3] + 1.) * f64::from(extent[axis]) / 2.)
}

pub(super) fn fixed_coordinate(value: f64) -> Result<i64, RenderError> {
    let scaled = (value * PIXEL_SCALE as f64).round_ties_even();
    if !scaled.is_finite() || scaled < i64::MIN as f64 || scaled > i64::MAX as f64 {
        return Err(rejected(
            value,
            "finite projected coordinate representable on Q16 grid",
        ));
    }
    Ok(scaled as i64)
}

fn candidate_range(a: f64, b: f64, extent: u32) -> std::ops::Range<u32> {
    let first = (a.min(b) - 1.).floor().max(0.) as u32;
    let last = (a.max(b) + 1.).ceil().min(f64::from(extent)) as u32;
    first..last
}

fn minor_candidates(
    start: [f64; 2],
    end: [f64; 2],
    axis: usize,
    major_pixel: u32,
    extent: u32,
) -> std::ops::Range<u32> {
    // A unit diamond reaches at most one pixel beyond the line's minor coordinate.
    let major_delta = end[axis] - start[axis];
    let t = if major_delta == 0. {
        0.
    } else {
        (f64::from(major_pixel) + 0.5 - start[axis]) / major_delta
    };
    let minor = start[1 - axis] + (end[1 - axis] - start[1 - axis]) * t;
    candidate_range(minor, minor, extent)
}

fn pixel_coordinates(axis: usize, major: u32, minor: u32) -> [u32; 2] {
    if axis == 0 {
        [major, minor]
    } else {
        [minor, major]
    }
}

#[cfg(test)]
pub(super) fn candidate_work(start: [f64; 2], end: [f64; 2], extent: [u32; 2]) -> usize {
    let axis = major_axis(
        [
            fixed_coordinate(start[0]).unwrap(),
            fixed_coordinate(start[1]).unwrap(),
        ],
        [
            fixed_coordinate(end[0]).unwrap(),
            fixed_coordinate(end[1]).unwrap(),
        ],
    );
    candidate_range(start[axis], end[axis], extent[axis])
        .map(|pixel| minor_candidates(start, end, axis, pixel, extent[1 - axis]).len())
        .sum()
}

fn covered(endpoints: [[i64; 2]; 2], pixel: [u32; 2]) -> bool {
    let center = pixel.map(|coordinate| i64::from(coordinate) * PIXEL_SCALE + PIXEL_SCALE / 2);
    exits_diamond(endpoints[0], endpoints[1], center, PIXEL_SCALE)
}

fn append_pixel(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    line: ClippedLine,
    pixel: [u32; 2],
    extent: [u32; 2],
    geometry: &mut Geometry,
) -> Result<(), RenderError> {
    if geometry.indices.len() / 6 >= MAX_PIXELS {
        return Err(rejected(
            geometry.indices.len() / 6 + 1,
            "at most one million pixel quads per draw",
        ));
    }
    let [start, end] = line.clip.map(|point| window_point(point, extent));
    let sample = [f64::from(pixel[0]) + 0.5, f64::from(pixel[1]) + 0.5];
    let t = line_parameter(behavior, start, end, sample);
    let s = crate::wide_line_columns::perspective_parameter(
        t as f32,
        line.clip[0][3] as f32,
        line.clip[1][3] as f32,
    ) as f64;
    if !super::user_clipping::accepts(draw, interpolate(line.world[0], line.world[1], s))? {
        return Ok(());
    }
    let color = interpolate(line.color[0], line.color[1], s).map(|v| v as f32);
    let depth = window_depth(line, t);
    append_quad(pixel, extent, depth, color, geometry)
}

pub(super) fn line_parameter(
    behavior: RenderThinLineCoverage,
    start: [f64; 2],
    end: [f64; 2],
    sample: [f64; 2],
) -> f64 {
    let delta = [end[0] - start[0], end[1] - start[1]];
    if behavior == RenderThinLineCoverage::DiamondExitV2 {
        return orthogonal_parameter(delta, start, sample);
    }
    dominant_axis_parameter(delta, start, sample)
}

fn orthogonal_parameter(delta: [f64; 2], start: [f64; 2], sample: [f64; 2]) -> f64 {
    let length = delta[0] * delta[0] + delta[1] * delta[1];
    if length == 0. {
        return 0.;
    }
    let offset = [sample[0] - start[0], sample[1] - start[1]];
    (offset[0] * delta[0] + offset[1] * delta[1]) / length
}

fn dominant_axis_parameter(delta: [f64; 2], start: [f64; 2], sample: [f64; 2]) -> f64 {
    let axis = usize::from(delta[1].abs() > delta[0].abs());
    if delta[axis] == 0. {
        return 0.;
    }
    (sample[axis] - start[axis]) / delta[axis]
}

fn window_depth(line: ClippedLine, t: f64) -> f32 {
    // OpenGL 4.6 Eq. 14.7 interpolates window depth linearly in the same parameter.
    let z = line.clip.map(|point| point[2] / point[3]);
    let depth = z[0] + (z[1] - z[0]) * t;
    // DiamondExitV2 and V3 follow the captured GL_DEPTH24_STENCIL8 logical depth contract; keeping an
    // extrapolated value here would clip the private Vulkan quad before fixed-point conversion.
    depth.clamp(-1., 1.) as f32
}

fn append_quad(
    pixel: [u32; 2],
    extent: [u32; 2],
    depth: f32,
    color: [f32; 4],
    geometry: &mut Geometry,
) -> Result<(), RenderError> {
    let [x, y] = pixel.map(f64::from);
    let bounds = [x, y, x + 1., y + 1.];
    let corners = [
        [bounds[0], bounds[1]],
        [bounds[2], bounds[1]],
        [bounds[2], bounds[3]],
        [bounds[0], bounds[3]],
    ];
    let base = u32::try_from(geometry.vertices.len())
        .ok()
        .filter(|count| *count <= u32::MAX - 4)
        .ok_or_else(|| {
            rejected(
                geometry.vertices.len(),
                "quad vertex base below u32::MAX - 4",
            )
        })?;
    for point in corners {
        geometry
            .vertices
            .push(pixel_vertex(point, extent, depth, color));
    }
    geometry
        .indices
        .extend([0, 1, 2, 0, 2, 3].map(|offset| base + offset));
    Ok(())
}

pub(super) fn pixel_vertex(
    point: [f64; 2],
    extent: [u32; 2],
    depth: f32,
    color: [f32; 4],
) -> RenderVertex {
    // GL bottom-up window Y maps to logical rows through the negative-height viewport.
    let ndc = [0, 1].map(|axis| (2. * point[axis] / f64::from(extent[axis]) - 1.) as f32);
    RenderVertex {
        position: [ndc[0], ndc[1], depth],
        weight: 1.,
        color,
    }
}

fn replacement(
    draw: &PrimitiveDraw,
    vertices: Vec<RenderVertex>,
    indices: Vec<u32>,
) -> PrimitiveDraw {
    let mut selected = draw.with_replacement_geometry(vertices, indices);
    selected.clipping = None;
    selected.clip_transform = crate::identity_clip();
    selected.line_style = None;
    selected.projected_dash = None;
    selected.source_line_strip = false;
    selected.quad_corners.clear();
    selected
}
