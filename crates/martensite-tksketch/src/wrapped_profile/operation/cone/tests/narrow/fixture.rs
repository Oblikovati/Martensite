//! Binary-exact polynomial glyph controls own every curve and selected material region.
use super::*;
#[derive(Clone, Copy, Debug)]
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) enum Quadrant {
    PositiveX,
    PositiveY,
    NegativeX,
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn request(
    sense: NormalSense,
    quadrant: Quadrant,
) -> WrappedConeNormalRequest {
    let mut request = super::super::request();
    request.profile = profile(quadrant);
    request.selections = selections();
    request.support.semi_angle = 0.125;
    request.support.normal = sense;
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::FilteredCompositeBernsteinV5 {
            max_components: 64,
            max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
        };
    request
}
fn profile(quadrant: Quadrant) -> SketchProfile {
    let mut vertices = Vec::new();
    let mut curves = Vec::new();
    for (first, scales) in [(1, [4., 18.]), (5, [3., 16.])] {
        append_oval(first, scales, 84., quadrant, &mut vertices, &mut curves);
    }
    append_dot(quadrant, &mut vertices, &mut curves);
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    }
}
fn append_oval(
    first: u64,
    scales: [f64; 2],
    center: f64,
    quadrant: Quadrant,
    vertices: &mut Vec<ProfileVertex>,
    curves: &mut Vec<ProfileCurve>,
) {
    for quarter in 0..4 {
        let poles = oval_controls(scales, center, quarter, quadrant);
        let id = first + u64::from(quarter);
        vertices.push(vertex(id, poles[0]));
        curves.push(cubic(id, first + u64::from((quarter + 1) % 4), poles));
    }
}
fn oval_controls(scales: [f64; 2], center: f64, quarter: u32, quadrant: Quadrant) -> [[f64; 2]; 4] {
    [[1., 0.], [1., 0.5], [0.5, 1.], [0., 1.]].map(|point| {
        let point = rotate(point, quarter);
        transform(
            [center + scales[0] * point[0], scales[1] * point[1]],
            quadrant,
        )
    })
}
fn rotate(mut point: [f64; 2], turns: u32) -> [f64; 2] {
    for _ in 0..turns {
        point = [-point[1], point[0]];
    }
    point
}
fn transform(point: [f64; 2], quadrant: Quadrant) -> [f64; 2] {
    let turns = match quadrant {
        Quadrant::PositiveX => 0,
        Quadrant::PositiveY => 1,
        Quadrant::NegativeX => 2,
    };
    // Authored shared endpoints use one zero encoding after all geometric rotations.
    rotate(point, turns).map(|value| if value == 0. { 0. } else { value })
}
fn vertex(id: u64, point: [f64; 2]) -> ProfileVertex {
    ProfileVertex {
        id: ProfileVertexId(id),
        point,
    }
}
fn cubic(id: u64, last: u64, poles: [[f64; 2]; 4]) -> ProfileCurve {
    ProfileCurve::Spline {
        id: ProfileCurveId(id),
        start: ProfileVertexId(id),
        end: ProfileVertexId(last),
        degree: 3,
        poles: poles.to_vec(),
        weights: None,
        knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
    }
}
fn append_dot(
    quadrant: Quadrant,
    vertices: &mut Vec<ProfileVertex>,
    curves: &mut Vec<ProfileCurve>,
) {
    for (index, point) in [[80., 22.], [82., 22.], [82., 24.], [80., 24.]]
        .into_iter()
        .enumerate()
    {
        let id = 9 + index as u64;
        vertices.push(vertex(id, transform(point, quadrant)));
        curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(9 + (index as u64 + 1) % 4),
        });
    }
}
fn selections() -> Vec<WrappedRegionSelection> {
    vec![
        WrappedRegionSelection {
            region: SurfaceRegionId(1),
            outer: contour(1, false),
            holes: vec![contour(5, true)],
        },
        WrappedRegionSelection {
            region: SurfaceRegionId(2),
            outer: contour(9, false),
            holes: vec![],
        },
    ]
}
fn contour(first: u64, reversed: bool) -> ProfileLoop {
    let mut uses: Vec<_> = (first..first + 4)
        .map(|id| LoopUse {
            curve: ProfileCurveId(id),
            reversed,
        })
        .collect();
    if reversed {
        uses.reverse();
    }
    ProfileLoop { uses }
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn two_ovals_and_dot()
-> WrappedConeNormalRequest {
    let mut request = request(NormalSense::Outward, Quadrant::PositiveX);
    for (first, scales) in [(17, [4., 18.]), (21, [3., 16.])] {
        append_oval(
            first,
            scales,
            108.,
            Quadrant::PositiveX,
            &mut request.profile.vertices,
            &mut request.profile.curves,
        );
    }
    request.selections.push(WrappedRegionSelection {
        region: SurfaceRegionId(3),
        outer: contour(17, false),
        holes: vec![contour(21, true)],
    });
    request
}
