//! Exact CPU reference for a versioned single-sample triangle coverage rule.
//!
//! The reference is independent of interpolation and culling: `TriangleGridCoverage::covers` tests
//! one pixel center after nearest-even 12-bit snapping. Example: `triangle.covers(pixel, origin)`.

const FRACTIONAL_SCALE: f64 = 4096.0;
#[cfg(test)]
const SAMPLE_CENTER: i64 = 2048;
const MAX_EDGE_COORDINATE: f64 = 1_073_741_823.0;
/// Leave one pixel of margin below the signed-64-bit determinant coordinate bound.
pub(crate) const MAX_SAFE_VIEWPORT_PIXELS: f32 = 262_142.0;

/// Versioned triangle coverage; `RenderTriangleCoverageBehavior::HardwareV1` preserves replay.
///
/// Fixed-grid behaviors require opaque single-sample programmable Phong and UnsnappedV2.
/// Uncertain clipping or raster-grid degeneracy returns a typed error.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderTriangleCoverageBehavior {
    /// Preserve the device's fixed-function coverage.
    #[default]
    HardwareV1,
    /// Snap projected coordinates to 12 bits and apply explicit edge ownership; geometry, depth,
    /// and color interpolation still use the submitted triangle.
    FixedGrid12V2 {
        /// Framebuffer direction that defines the half-open top and left edges.
        origin: RenderTriangleGridOrigin,
    },
    /// Preserve V2 coverage while replacing only certified sample-empty refusals with private
    /// zero-area carriers. Original primitive ordinals and semantic geometry remain recorded.
    FixedGrid12V3 {
        /// Framebuffer direction defining half-open edge ownership.
        origin: RenderTriangleGridOrigin,
    },
    /// Restore Q12 logical coverage, facing, culling, and depth on conservative candidates.
    /// V4 saturates biased fragment depth to the declared viewport depth range.
    NativeGrid12V4 {
        /// Framebuffer direction defining half-open edge ownership.
        origin: RenderTriangleGridOrigin,
    },
}

impl RenderTriangleCoverageBehavior {
    /// Return the stable replay identifier; `behavior.behavior_version()`.
    /// Example: `RenderTriangleCoverageBehavior::default().behavior_version()` returns `1`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::FixedGrid12V2 { .. } => 2,
            Self::FixedGrid12V3 { .. } => 3,
            Self::NativeGrid12V4 { .. } => 4,
        }
    }
}

/// Typed refusal for unsupported triangle-coverage capabilities or draw domains.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderTriangleCoverageFailure {
    /// The selected device does not expose conservative rasterization.
    MissingConservativeRasterization,
    /// The selected device cannot execute the exact 64-bit fragment edge predicate.
    MissingShaderInt64,
    /// Conservative rasterization may cull a triangle collapsed by its fixed-point raster grid.
    MissingDegenerateTriangleRasterization,
    /// The conservative-raster overestimate property is not a finite nonnegative pixel size.
    InvalidOverestimateSize {
        /// The reported IEEE Float32 bits from `primitiveOverestimationSize`.
        size_bits: u32,
    },
    /// The draw does not use the registered unsnapped built-in Phong interface.
    UnsupportedInterface {
        /// Draw interface or state that prevented exact coverage selection.
        actual: String,
    },
    /// Projected vertices are nonfinite, clipped, or not positive-W.
    InvalidTriangle {
        /// Direct triangle offset in the selected draw.
        triangle: usize,
        /// Screened clip values as IEEE Float32 bits.
        clip_bits: [[u32; 4]; 3],
    },
    /// The screen triangle may collapse on the reported device raster grid.
    RasterGridDegenerate {
        /// Direct triangle offset in the selected draw.
        triangle: usize,
        /// Device subpixel precision advertised for framebuffer coordinates.
        subpixel_precision_bits: u32,
        /// Screened clip values as IEEE Float32 bits.
        clip_bits: [[u32; 4]; 3],
    },
}

impl std::fmt::Display for RenderTriangleCoverageFailure {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOverestimateSize { size_bits } => write_overestimate(output, *size_bits),
            Self::MissingDegenerateTriangleRasterization => write_degenerate_property(output),
            _ => write_general_failure(output, self),
        }
    }
}

fn write_overestimate(output: &mut std::fmt::Formatter<'_>, size_bits: u32) -> std::fmt::Result {
    write!(
        output,
        "primitiveOverestimationSize bits {size_bits:#010x}; expected finite nonnegative pixels"
    )
}

fn write_degenerate_property(output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
        output,
        "degenerateTrianglesRasterized=false; expected true for NativeGrid12V4"
    )
}

fn write_general_failure(
    output: &mut std::fmt::Formatter<'_>,
    failure: &RenderTriangleCoverageFailure,
) -> std::fmt::Result {
    write!(
        output,
        "triangle coverage rejected {failure:?}; expected supported single-sample fill triangles"
    )
}

impl std::error::Error for RenderTriangleCoverageFailure {}

/// Which framebuffer direction defines the named top-left edge rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderTriangleGridOrigin {
    /// Positive Y points toward the top of the framebuffer.
    BottomUp,
    /// Positive Y points toward the bottom of the framebuffer.
    TopDown,
}

/// Cull state used by the reference's independent facing negative controls.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TriangleFaceCull {
    /// Retain either nondegenerate winding.
    None,
    /// Reject the winding named as front-facing.
    Front,
    /// Reject the winding opposite the named front-facing orientation.
    Back,
}

/// One vertex already projected into framebuffer coordinates and snapped to the 12-bit grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GridVertex {
    x: i32,
    y: i32,
}

/// One Q12 point enclosed by independent outward-rounded coordinate endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GridVertexInterval {
    coordinates: [[i32; 2]; 2],
}

impl GridVertexInterval {
    /// Refuse reversed or out-of-domain Q12 endpoints.
    pub(crate) fn new(x: [i32; 2], y: [i32; 2]) -> Option<Self> {
        [x, y].into_iter().all(valid_grid_interval).then_some(Self {
            coordinates: [x, y],
        })
    }

    /// Read the inclusive x/y endpoint pairs.
    pub(crate) const fn coordinates(self) -> [[i32; 2]; 2] {
        self.coordinates
    }

    #[cfg(test)]
    fn valid(self) -> bool {
        self.coordinates.into_iter().all(valid_grid_interval)
    }
}

impl GridVertex {
    /// Snap a finite framebuffer point with shared nearest-even precision; `GridVertex::snap([x,y])`.
    pub(crate) fn snap(point: [f64; 2]) -> Option<Self> {
        let precision = crate::RenderViewportGridPrecision::Fractional12;
        let x = snap_coordinate(precision, point[0])?;
        let y = snap_coordinate(precision, point[1])?;
        Some(Self { x, y })
    }
}

fn snap_coordinate(precision: crate::RenderViewportGridPrecision, value: f64) -> Option<i32> {
    let snapped = precision.snap(value).ok()? * FRACTIONAL_SCALE;
    (-MAX_EDGE_COORDINATE..=MAX_EDGE_COORDINATE)
        .contains(&snapped)
        .then_some(snapped as i32)
}

/// One snapped, indexed triangle in framebuffer coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GridTriangle {
    vertices: [GridVertex; 3],
}

impl GridTriangle {
    /// Keep the submitted vertex order; `GridTriangle::new([a, b, c])`.
    pub(crate) const fn new(vertices: [GridVertex; 3]) -> Self {
        Self { vertices }
    }

    /// Test pixel-center inclusion and half-open edge ownership; `triangle.covers([x, y], origin)`.
    #[cfg(test)]
    pub(crate) fn covers(self, pixel: [i32; 2], origin: RenderTriangleGridOrigin) -> bool {
        let Some(vertices) = positive_area(self.vertices) else {
            return false;
        };
        let sample = pixel_center(pixel);
        (0..3).all(|edge| owned_edge(vertices[edge], vertices[(edge + 1) % 3], sample, origin))
    }

    /// Return the exact signed area used to reject degenerate snapped triangles.
    pub(crate) fn signed_area(self) -> i128 {
        edge(self.vertices[0], self.vertices[1], point(self.vertices[2]))
    }

    /// Require area to survive device and projection rounding; `triangle.survives_grid(8, error)`.
    pub(crate) fn survives_grid(self, fractional_bits: u32, projection_error: f64) -> bool {
        let Some(radius) = grid_error_radius(fractional_bits, projection_error) else {
            return false;
        };
        let (minimum, maximum) = interval_area_range(self.vertices, radius);
        minimum > 0 || maximum < 0
    }

    /// Read logical winding after Q12 snapping; `triangle.front_facing(true)`.
    #[cfg(test)]
    pub(crate) fn front_facing(self, front_face_ccw: bool) -> Option<bool> {
        let area = self.signed_area();
        (area != 0).then_some((area < 0) == front_face_ccw)
    }

    /// Apply facing before coverage; `triangle.survives_culling(true, TriangleFaceCull::Back)`.
    #[cfg(test)]
    pub(crate) fn survives_culling(self, front_face_ccw: bool, cull: TriangleFaceCull) -> bool {
        let Some(is_front) = self.front_facing(front_face_ccw) else {
            return false;
        };
        !matches!(cull, TriangleFaceCull::Front if is_front)
            && !matches!(cull, TriangleFaceCull::Back if !is_front)
    }
}

fn grid_error_radius(fractional_bits: u32, projection_error: f64) -> Option<i128> {
    let scale = 2_i32.checked_pow(fractional_bits)?;
    if !projection_error.is_finite() || projection_error < 0.0 {
        return None;
    }
    let pixel_radius = 1.0 / f64::from(scale) + 1.0 / 8192.0 + projection_error;
    let grid_radius = (pixel_radius * FRACTIONAL_SCALE).ceil() + 1.0;
    (grid_radius.is_finite() && grid_radius <= f64::from(i32::MAX)).then_some(grid_radius as i128)
}

fn interval_area_range(vertices: [GridVertex; 3], radius: i128) -> (i128, i128) {
    // The determinant is multi-affine in all six coordinates, so its extrema lie at box corners.
    let mut range = (i128::MAX, i128::MIN);
    for corner in 0..64 {
        let points =
            std::array::from_fn(|index| interval_vertex(vertices[index], radius, corner, index));
        let area = interval_area(points);
        range.0 = range.0.min(area);
        range.1 = range.1.max(area);
    }
    range
}

/// Prove one winding for every point in three independent coordinate rectangles.
#[cfg(test)]
pub(crate) fn interval_area_excludes_zero(vertices: [GridVertexInterval; 3]) -> bool {
    if !vertices.into_iter().all(GridVertexInterval::valid) {
        return false;
    }
    let (minimum, maximum) = interval_area_range_for_vertices(vertices);
    minimum > 0 || maximum < 0
}

#[cfg(test)]
fn interval_area_range_for_vertices(vertices: [GridVertexInterval; 3]) -> (i128, i128) {
    // The determinant is multi-affine in six coordinates, so extrema occur at box corners.
    let mut range = (i128::MAX, i128::MIN);
    for corner in 0..64 {
        let points = std::array::from_fn(|index| {
            [0, 1].map(|axis| {
                let bit = 1 << (2 * index + axis);
                i128::from(vertices[index].coordinates[axis][(corner & bit != 0) as usize])
            })
        });
        let area = interval_area(points);
        range.0 = range.0.min(area);
        range.1 = range.1.max(area);
    }
    range
}

fn valid_grid_interval(interval: [i32; 2]) -> bool {
    interval[0] <= interval[1]
        && f64::from(interval[0]).abs() <= MAX_EDGE_COORDINATE
        && f64::from(interval[1]).abs() <= MAX_EDGE_COORDINATE
}

fn interval_vertex(vertex: GridVertex, radius: i128, corner: u32, index: usize) -> [i128; 2] {
    [0, 1].map(|axis| {
        let bit = 1 << (2 * index + axis);
        i128::from(if corner & bit == 0 { -1 } else { 1 }) * radius
            + i128::from(if axis == 0 { vertex.x } else { vertex.y })
    })
}

fn interval_area(points: [[i128; 2]; 3]) -> i128 {
    let [[ax, ay], [bx, by], [cx, cy]] = points;
    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}

#[cfg(test)]
fn pixel_center(pixel: [i32; 2]) -> [i64; 2] {
    [
        i64::from(pixel[0]) * 4096 + SAMPLE_CENTER,
        i64::from(pixel[1]) * 4096 + SAMPLE_CENTER,
    ]
}

fn point(vertex: GridVertex) -> [i64; 2] {
    [i64::from(vertex.x), i64::from(vertex.y)]
}

fn edge(a: GridVertex, b: GridVertex, p: [i64; 2]) -> i128 {
    let dx = i128::from(b.x) - i128::from(a.x);
    let dy = i128::from(b.y) - i128::from(a.y);
    dx * (i128::from(p[1]) - i128::from(a.y)) - dy * (i128::from(p[0]) - i128::from(a.x))
}

#[cfg(test)]
fn positive_area(mut vertices: [GridVertex; 3]) -> Option<[GridVertex; 3]> {
    let area = edge(vertices[0], vertices[1], point(vertices[2]));
    match area.cmp(&0) {
        std::cmp::Ordering::Greater => Some(vertices),
        std::cmp::Ordering::Less => {
            vertices.swap(1, 2);
            Some(vertices)
        }
        std::cmp::Ordering::Equal => None,
    }
}

#[cfg(test)]
fn owned_edge(a: GridVertex, b: GridVertex, p: [i64; 2], origin: RenderTriangleGridOrigin) -> bool {
    let value = edge(a, b, p);
    value > 0 || (value == 0 && is_top_left(a, b, origin))
}

#[cfg(test)]
fn is_top_left(a: GridVertex, b: GridVertex, origin: RenderTriangleGridOrigin) -> bool {
    let dx = i64::from(b.x) - i64::from(a.x);
    let dy = i64::from(b.y) - i64::from(a.y);
    match origin {
        RenderTriangleGridOrigin::BottomUp => dy < 0 || (dy == 0 && dx < 0),
        RenderTriangleGridOrigin::TopDown => dy < 0 || (dy == 0 && dx > 0),
    }
}

#[cfg(test)]
#[path = "triangle_grid_coverage_tests.rs"]
mod tests;
