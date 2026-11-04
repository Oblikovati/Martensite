//! GlGraduatedTrihedron::getNormal, getDistanceToCorner and getGridAxes:
//! the box corner farthest along the view normal picks the visible grid edges, the axis
//! directions from that corner and the edges that carry the tickmarks.
use martensite_tkernel::ncollection::{Mat4, Vec3};
use martensite_tkservice::graphic3d::transform_utils::un_project;

/// Grid edges between two box corners, named `<from>_<to>` with O for the minimum and X, Y, Z
/// for the maximum on that axis (native enum values).
pub(super) mod edge {
    pub const XOO_XYO: u16 = 1 << 1;
    pub const XOO_XOZ: u16 = 1 << 2;
    pub const OYO_OYZ: u16 = 1 << 3;
    pub const OYO_XYO: u16 = 1 << 4;
    pub const OOZ_XOZ: u16 = 1 << 5;
    pub const OOZ_OYZ: u16 = 1 << 6;
    pub const OYZ_XYZ: u16 = 1 << 7;
    pub const XOZ_XYZ: u16 = 1 << 8;
    pub const XYO_XYZ: u16 = 1 << 9;
}
use edge::*;

/// Native `GridAxes`: the grid origin corner, the axis directions from it and the tick origins.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GridAxes {
    pub origin: Vec3<f32>,
    pub axes: [Vec3<f32>; 3],
    pub ticks: [Vec3<f32>; 3],
}

/// Per corner (bit 2 = max x, bit 1 = max y, bit 0 = max z): the corners of the three tick
/// origins and the visible grid edges, as the eight `getGridAxes` cases list them.
const CORNER_CASES: [([usize; 3], u16); 8] = [
    (
        [1, 4, 4],
        OOZ_XOZ | OYO_XYO | XOO_XYO | OOZ_OYZ | XOO_XOZ | OYO_OYZ,
    ),
    (
        [0, 0, 5],
        OOZ_XOZ | OYZ_XYZ | OOZ_OYZ | XOZ_XYZ | XOO_XOZ | OYO_OYZ,
    ),
    (
        [0, 6, 6],
        OYO_XYO | OYZ_XYZ | XOO_XYO | OOZ_OYZ | XYO_XYZ | OYO_OYZ,
    ),
    (
        [1, 2, 1],
        OOZ_XOZ | OYZ_XYZ | OYO_XYO | OOZ_OYZ | XOZ_XYZ | OYO_OYZ | XYO_XYZ,
    ),
    (
        [5, 0, 0],
        OOZ_XOZ | OYO_XYO | XOO_XYO | XOZ_XYZ | XOO_XOZ | XYO_XYZ,
    ),
    (
        [4, 4, 1],
        OOZ_XOZ | OYZ_XYZ | XOO_XYO | XOZ_XYZ | OOZ_OYZ | XOO_XOZ | XYO_XYZ,
    ),
    (
        [4, 2, 4],
        OYO_XYO | OYZ_XYZ | XOO_XYO | XOZ_XYZ | XOO_XOZ | XYO_XYZ | OYO_OYZ,
    ),
    (
        [6, 6, 5],
        OYO_XYO | OYZ_XYZ | OOZ_XOZ | XOO_XYO | XOZ_XYZ | OOZ_OYZ | XOO_XOZ | XYO_XYZ | OYO_OYZ,
    ),
];

/// The box corner of a corner index (bit 2 = max x, bit 1 = max y, bit 0 = max z).
/// Example: `box_corner(5, min, max) == Vec3::new(max.x, min.y, max.z)`.
pub(super) fn box_corner(index: usize, min: Vec3<f32>, max: Vec3<f32>) -> Vec3<f32> {
    let pick = |bit: usize, low: f32, high: f32| if index & bit != 0 { high } else { low };
    Vec3::new(
        pick(4, min.x, max.x),
        pick(2, min.y, max.y),
        pick(1, min.z, max.z),
    )
}

/// `getGridAxes`: the grid of the corner with the greatest positive distance (corner 0 when
/// none is positive); returns the axes and the visible edge mask.
/// Example: `let (axes, edges) = grid_axes(&distances, min, max);`.
pub(super) fn grid_axes(corners: &[f32; 8], min: Vec3<f32>, max: Vec3<f32>) -> (GridAxes, u16) {
    let mut best = 0;
    let mut best_value = corners[0].max(0.0);
    for (index, &value) in corners.iter().enumerate() {
        if value > best_value {
            best_value = value;
            best = index;
        }
    }
    let (ticks, edges) = CORNER_CASES[best];
    let sign = |bit: usize| if best & bit != 0 { -1.0 } else { 1.0 };
    let axes = GridAxes {
        origin: box_corner(best, min, max),
        axes: [
            Vec3::new(sign(4), 0.0, 0.0),
            Vec3::new(0.0, sign(2), 0.0),
            Vec3::new(0.0, 0.0, sign(1)),
        ],
        ticks: ticks.map(|corner| box_corner(corner, min, max)),
    };
    (axes, edges)
}

/// `getDistanceToCorner`: the signed distance of a corner along the normal, in native order.
/// Example: `distance_to_corner(normal, center, corner)`.
pub(super) fn distance_to_corner(normal: Vec3<f32>, center: Vec3<f32>, corner: Vec3<f32>) -> f32 {
    normal.x * (corner.x - center.x)
        + normal.y * (corner.y - center.y)
        + normal.z * (corner.z - center.z)
}

/// `getNormal`: the view-plane normal (unnormalized) from three unprojected viewport corners,
/// and the world size of one pixel along the viewport width.
/// Example: `let (normal, dpix) = view_normal(&world_view, &projection, [0, 0, 200, 200]);`.
pub(super) fn view_normal(
    world_view: &Mat4<f32>,
    projection: &Mat4<f32>,
    viewport: [i32; 4],
) -> (Vec3<f32>, f32) {
    let unproject = |x: i32, y: i32| {
        // A singular matrix leaves the native output point at its zero initial value.
        let point = un_project([x as f32, y as f32, 0.0], world_view, projection, viewport)
            .unwrap_or([0.0; 3]);
        Vec3::new(point[0], point[1], point[2])
    };
    let [x, y, width, height] = viewport;
    let first = unproject(x, y);
    let second = unproject(x + width, y);
    let third = unproject(x, y + height);
    let up = third - first;
    let across = second - first;
    (Vec3::cross(&up, &across), across.modulus() / width as f32)
}

#[cfg(test)]
#[path = "grid_axes_tests.rs"]
mod tests;
