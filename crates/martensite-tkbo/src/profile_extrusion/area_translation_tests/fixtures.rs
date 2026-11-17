use super::super::*;
pub(super) fn rectangle(origin: f64, size: [f64; 2], reverse: bool) -> SketchProfile {
    let mut profile = empty_profile();
    add_rectangle(
        &mut profile,
        1,
        [origin; 2],
        [origin + size[0], origin + size[1]],
        reverse,
    );
    profile
}
pub(super) fn ring(origin: f64, outer: bool, hole: bool) -> SketchProfile {
    let mut profile = rectangle(origin, [1.; 2], outer);
    add_rectangle(
        &mut profile,
        21,
        [origin + 0.25; 2],
        [origin + 0.5; 2],
        hole,
    );
    profile
}
pub(super) fn empty_profile() -> SketchProfile {
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: Vec::new(),
        curves: Vec::new(),
    }
}
fn add_rectangle(
    profile: &mut SketchProfile,
    base: u64,
    min: [f64; 2],
    max: [f64; 2],
    reverse: bool,
) {
    append_corners(
        profile,
        base,
        [min, [max[0], min[1]], max, [min[0], max[1]]],
    );
    profile
        .curves
        .extend((0..4).map(|i| side(base, i, reverse)));
}
fn append_corners(profile: &mut SketchProfile, base: u64, points: [[f64; 2]; 4]) {
    profile.vertices.extend(
        points
            .into_iter()
            .enumerate()
            .map(|(i, point)| ProfileVertex {
                id: ProfileVertexId(base + i as u64),
                point,
            }),
    );
}
fn side(base: u64, index: u64, reverse: bool) -> ProfileCurve {
    let (start, end) = (
        ProfileVertexId(base + index),
        ProfileVertexId(base + (index + 1) % 4),
    );
    ProfileCurve::Line {
        id: ProfileCurveId(base + 4 + index),
        start: if reverse { end } else { start },
        end: if reverse { start } else { end },
    }
}
pub(super) fn half_conic(origin: f64, elliptic: bool, reverse: bool) -> SketchProfile {
    let mut profile = empty_profile();
    profile.vertices = conic_vertices(origin, if elliptic { 2. } else { 1. });
    let ends = if reverse {
        [ProfileVertexId(1), ProfileVertexId(2)]
    } else {
        [ProfileVertexId(2), ProfileVertexId(1)]
    };
    profile
        .curves
        .push(conic_arc(origin, elliptic, reverse, ends));
    profile.curves.push(ProfileCurve::Line {
        id: ProfileCurveId(6),
        start: ends[1],
        end: ends[0],
    });
    profile
}
fn conic_vertices(origin: f64, radius: f64) -> Vec<ProfileVertex> {
    vec![
        ProfileVertex {
            id: ProfileVertexId(1),
            point: [origin - radius, origin],
        },
        ProfileVertex {
            id: ProfileVertexId(2),
            point: [origin + radius, origin],
        },
    ]
}
fn conic_arc(
    origin: f64,
    elliptic: bool,
    reverse: bool,
    ends: [ProfileVertexId; 2],
) -> ProfileCurve {
    let sense = if reverse {
        ArcSense::Clockwise
    } else {
        ArcSense::CounterClockwise
    };
    if elliptic {
        ellipse_arc(origin, ends, sense)
    } else {
        circle_arc(origin, ends, sense)
    }
}
fn ellipse_arc(origin: f64, ends: [ProfileVertexId; 2], sense: ArcSense) -> ProfileCurve {
    ProfileCurve::EllipticArc {
        id: ProfileCurveId(5),
        start: ends[0],
        end: ends[1],
        center: [origin; 2],
        major_axis: [2., 0.],
        minor_radius: 1.,
        sense,
    }
}
fn circle_arc(origin: f64, ends: [ProfileVertexId; 2], sense: ArcSense) -> ProfileCurve {
    ProfileCurve::Arc {
        id: ProfileCurveId(5),
        start: ends[0],
        end: ends[1],
        center: [origin; 2],
        sense,
    }
}
pub(super) fn whole_conic(origin: f64, elliptic: bool) -> SketchProfile {
    let mut profile = empty_profile();
    profile.curves.push(if elliptic {
        ProfileCurve::Ellipse {
            id: ProfileCurveId(5),
            center: [origin; 2],
            major_axis: [2., 0.],
            minor_radius: 1.,
        }
    } else {
        ProfileCurve::Circle {
            id: ProfileCurveId(5),
            center: [origin; 2],
            radius: 1.,
        }
    });
    profile
}
