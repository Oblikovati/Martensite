// SPDX-License-Identifier: GPL-2.0-only
//! The exact finite patch boundaries implied by the cardinal authored route.
use super::{Point, Span, point_add, scaled};
use crate::spatial_sweep::{
    SweepFailure as Failure,
    admission::{Admission, Axis, direction},
    geometry::transported,
    lineage::Face,
};

pub(super) enum PlanarModel {
    Band(Vec<Point>),
    Corner {
        center: Point,
        before: Point,
        after: Point,
    },
}

pub(super) fn planar(admission: &Admission, face: &Face) -> Result<PlanarModel, Failure> {
    let profile = &admission.profile.points;
    let section = [face.profile, (face.profile + 1) % profile.len()].map(|i| profile[i].1);
    if face.corner {
        corner(admission, section, face.station)
    } else {
        band(admission, section, face.station).map(PlanarModel::Band)
    }
}

fn band(admission: &Admission, section: [[f64; 3]; 2], band: usize) -> Result<Vec<Point>, Failure> {
    let section = section.map(|p| transported(admission, p, band));
    let middle = scaled(point_add(section[0], section[1]), Span::at(0.5));
    let previous = band.checked_sub(1).map(|i| flip(admission.axes[i]));
    let next = admission.axes.get(band + 1).copied();
    let low = Port::new(admission, previous, band, admission.axes[band]);
    let high = Port::new(admission, next, band + 1, flip(admission.axes[band]));
    let mut points = vec![low.clip(section[0]), high.clip(section[0])];
    if high.crosses_clip(section)? {
        points.push(high.clip(middle));
    }
    points.extend([high.clip(section[1]), low.clip(section[1])]);
    if low.crosses_clip(section)? {
        points.push(low.clip(middle));
    }
    Ok(points)
}

struct Port {
    center: Point,
    inward: Option<Axis>,
    trim: Axis,
}

impl Port {
    fn new(admission: &Admission, inward: Option<Axis>, station: usize, trim: Axis) -> Self {
        Self {
            center: station_xyz(admission, station),
            inward,
            trim,
        }
    }

    fn clip(&self, relative: Point) -> Point {
        let mut placed = point_add(self.center, relative);
        if let Some(axis) = self.inward {
            let distance = radial(relative, axis);
            let depth = Span {
                lo: distance.lo.max(0.),
                hi: distance.hi.max(0.),
            };
            placed[self.trim.index] = placed[self.trim.index] + Span::at(self.trim.sign) * depth;
        }
        placed
    }

    fn crosses_clip(&self, ends: [Point; 2]) -> Result<bool, Failure> {
        let Some(axis) = self.inward else {
            return Ok(false);
        };
        let [a, b] = ends.map(|point| strict_side(radial(point, axis))).map(|n| {
            n.ok_or(Failure::Geometry(
                "a certified strict radial side of the bend clip",
            ))
        });
        Ok(a? != b?)
    }
}

fn corner(
    admission: &Admission,
    section: [[f64; 3]; 2],
    station: usize,
) -> Result<PlanarModel, Failure> {
    let failure =
        Failure::Geometry("an authored outer profile endpoint on the planar quarter-sector");
    let incoming = admission.axes[station - 1];
    let outgoing = admission.axes[station];
    let turn = incoming.cross(outgoing);
    let direction = direction(section[0], section[1]).ok_or(failure)?;
    let before_frame = admission.frames[station - 1];
    if before_frame[direction.index].index == turn.index {
        return Err(Failure::Geometry(
            "a planar corner strip traveling in its quarter-turn plane",
        ));
    }
    let outer = section
        .into_iter()
        .find(|&p| radial(transported(admission, p, station - 1), outgoing).hi < 0.)
        .ok_or(Failure::Geometry(
            "one proven outer endpoint of the planar corner strip",
        ))?;
    let mut before = transported(admission, outer, station - 1);
    let mut after = transported(admission, outer, station);
    let mut center = station_xyz(admission, station);
    center[turn.index] = center[turn.index] + before[turn.index];
    before[turn.index] = Span::ZERO;
    after[turn.index] = Span::ZERO;
    Ok(PlanarModel::Corner {
        center,
        before,
        after,
    })
}

fn station_xyz(admission: &Admission, station: usize) -> Point {
    std::array::from_fn(|i| {
        Span::at(admission.stations[station].xyz[i]) - Span::at(admission.stations[0].xyz[i])
    })
}

fn radial(point: Point, axis: Axis) -> Span {
    Span::at(axis.sign) * point[axis.index]
}

fn strict_side(span: Span) -> Option<bool> {
    (span.hi < 0. || span.lo > 0.).then_some(span.lo > 0.)
}

fn flip(axis: Axis) -> Axis {
    Axis {
        index: axis.index,
        sign: -axis.sign,
    }
}
