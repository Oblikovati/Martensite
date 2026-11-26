// SPDX-License-Identifier: GPL-2.0-only
//! Exact centered rectangle and three ordered orthogonal straight steps.
use super::{SweepFailure as Failure, SweepOptions, SweepStage as Stage, poll};
use crate::spatial_prism::{
    SpatialPrismFailure,
    plane::{AuthoredPlane, certify_authored_plane},
};
use martensite_tkbo::body_check::ResolutionRejected;
use martensite_tkmath::{
    gp::Point3,
    predicates::{compare_affine_point_error_3d, point_distance_upper_bound},
};
use martensite_tksketch3d::{
    PointId,
    wire::{WireClosure, WireRecipe, WireSegment},
};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub(super) struct Axis {
    pub index: usize,
    pub sign: f64,
}

#[derive(Clone, Copy)]
pub(super) struct Station {
    pub point: PointId,
    pub xyz: [f64; 3],
}

pub(super) struct Admission {
    pub profile: AuthoredPlane,
    pub stations: [Station; 4],
    pub axes: [Axis; 3],
    pub frames: [[Axis; 3]; 3],
}

pub(super) fn admit(
    route: &WireRecipe,
    profile: &WireRecipe,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Admission, Failure> {
    poll(cancelled, Stage::Admission)?;
    admit_options(options)?;
    if route.closure() != WireClosure::Open || profile.closure() != WireClosure::Closed {
        return Err(Failure::Closure {
            path: route.closure(),
            section: profile.closure(),
        });
    }
    let (axes, stations) = path(route)?;
    let profile = certify_authored_plane(profile, cancelled).map_err(|cause| match cause {
        SpatialPrismFailure::Cancelled(_) => Failure::Cancelled(Stage::Admission),
        source => Failure::ProfilePlane(source),
    })?;
    require_centered_rectangle(&profile, stations[0], axes[0])?;
    require_clearance(&profile, stations, axes)?;
    let initial = std::array::from_fn(|index| Axis { index, sign: 1. });
    let middle = initial.map(|axis| axis.turn(axes[0], axes[1]));
    let last = middle.map(|axis| axis.turn(axes[1], axes[2]));
    Ok(Admission {
        profile,
        stations,
        axes,
        frames: [initial, middle, last],
    })
}

fn admit_options(options: SweepOptions) -> Result<(), Failure> {
    options
        .resolution
        .admit()
        .map_err(|(field, value)| Failure::Resolution(ResolutionRejected { field, value }))?;
    for (field, value) in [
        ("boundary_tolerance", options.boundary_tolerance),
        ("source_trace_limit", options.source_trace_limit),
        ("approximation_limit", options.approximation_limit),
    ] {
        if !value.is_finite() || value <= 0. {
            return Err(Failure::Resolution(ResolutionRejected { field, value }));
        }
    }
    Ok(())
}

fn path(route: &WireRecipe) -> Result<([Axis; 3], [Station; 4]), Failure> {
    let [a, b, c] = route.segments() else {
        return Err(Failure::PathScope);
    };
    let [a, b, c] = [a, b, c].map(directed_ends);
    let axes = [
        direction(a[0].xyz, a[1].xyz).ok_or(Failure::PathScope)?,
        direction(b[0].xyz, b[1].xyz).ok_or(Failure::PathScope)?,
        direction(c[0].xyz, c[1].xyz).ok_or(Failure::PathScope)?,
    ];
    if axes[0].index == axes[1].index
        || axes[1].index == axes[2].index
        || axes[2].index == axes[0].index
    {
        return Err(Failure::PathScope);
    }
    Ok((axes, [a[0], a[1], b[1], c[1]]))
}

pub(super) fn directed_ends(segment: &WireSegment) -> [Station; 2] {
    segment.endpoints().map(|point| {
        let endpoint = usize::from(point != segment.authored.line.endpoints[0]);
        Station {
            point,
            xyz: *segment.authored.points[endpoint].coordinates(),
        }
    })
}

pub(super) fn direction(start: [f64; 3], end: [f64; 3]) -> Option<Axis> {
    let index = (0..3).find(|&index| start[index] != end[index])?;
    if (0..3).any(|i| i != index && start[i] != end[i]) {
        return None;
    }
    let sense = if end[index] > start[index] { 1. } else { -1. };
    Some(Axis { index, sign: sense })
}

fn require_centered_rectangle(
    profile: &AuthoredPlane,
    station: Station,
    normal: Axis,
) -> Result<(), Failure> {
    let [a, b, c, d] = profile.points.as_slice() else {
        return Err(Failure::ProfileScope);
    };
    let Some(width) = direction(a.1, b.1) else {
        return Err(Failure::ProfileScope);
    };
    let Some(height) = direction(b.1, c.1) else {
        return Err(Failure::ProfileScope);
    };
    if width.index == height.index || width.index == normal.index || height.index == normal.index {
        return Err(Failure::ProfileScope);
    }
    if [a, b, c, d]
        .iter()
        .any(|point| point.1[normal.index] != station.xyz[normal.index])
        || d.1[width.index] != a.1[width.index]
        || d.1[height.index] != c.1[height.index]
    {
        return Err(Failure::ProfileScope);
    }
    let neg_identity = [[-1., 0., 0.], [0., -1., 0.], [0., 0., -1.]];
    let sign = compare_affine_point_error_3d(a.1, c.1, [station.xyz; 2], neg_identity, 0.)
        .map_err(Failure::Predicate)?;
    if sign != Ordering::Equal {
        return Err(Failure::ProfileScope);
    }
    Ok(())
}

fn require_clearance(
    profile: &AuthoredPlane,
    stations: [Station; 4],
    axes: [Axis; 3],
) -> Result<(), Failure> {
    let origin = point(stations[0].xyz);
    let radius = profile
        .points
        .iter()
        .map(|(_, xyz)| point_distance_upper_bound(origin, point(*xyz)))
        .fold(0., f64::max);
    let required = (2. * radius).next_up();
    for (index, axis) in axes.into_iter().enumerate() {
        let length = (stations[index + 1].xyz[axis.index] - stations[index].xyz[axis.index])
            .abs()
            .next_down();
        if !required.is_finite() || !length.is_finite() || length <= required {
            return Err(Failure::Clearance {
                leg: index,
                lower: length,
                required,
            });
        }
    }
    Ok(())
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

impl Axis {
    pub(super) fn cross(self, other: Self) -> Self {
        let sign = if (self.index + 1) % 3 == other.index {
            1.
        } else {
            -1.
        };
        Self {
            index: 3 - self.index - other.index,
            sign: sign * self.sign * other.sign,
        }
    }

    fn turn(self, from: Self, to: Self) -> Self {
        let rotation = from.cross(to);
        if self.index == rotation.index {
            self
        } else {
            rotation.cross(self)
        }
    }
}
