// SPDX-License-Identifier: GPL-2.0-only
//! Exact product-domain geometry of every retained side face.
use super::super::{
    Failure, Stage,
    images::{CurveImages, Images},
    native::{self, Frame, Subject},
    poll,
    source::{Source, SourceCurve, Uv, Xyz},
};
use martensite_tkbrep::{
    topexp,
    topods::{Edge, Face},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tksketch3d::{CurveId, PointId};
use std::{collections::BTreeMap, f64::consts::FRAC_PI_2};

pub(super) fn verify(
    source: &Source,
    images: &Images<'_>,
    ranges: &BTreeMap<PointId, [f64; 2]>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    for (&id, &curve) in &source.curves {
        poll(cancelled, Stage::Sweep)?;
        let side = Side::new(curve, &images.curves[&id], images, ranges);
        side.verify(source, cancelled)?;
    }
    Ok(())
}

struct Side<'a> {
    face: &'a Face,
    caps: [&'a Edge; 2],
    laterals: [(&'a Edge, [f64; 2]); 2],
    curve: SourceCurve,
    subject: Subject,
}

impl<'a> Side<'a> {
    fn new(
        curve: SourceCurve,
        images: &CurveImages<'a>,
        all: &Images<'a>,
        ranges: &BTreeMap<PointId, [f64; 2]>,
    ) -> Self {
        let laterals = curve
            .endpoints
            .map(|point| (all.points[&point].lateral, ranges[&point]));
        Self {
            face: images.side,
            caps: images.caps,
            laterals,
            curve,
            subject: Subject(Some(curve.used.curve)),
        }
    }

    fn verify(&self, source: &Source, cancelled: &mut impl FnMut() -> bool) -> Result<(), Failure> {
        let actual = native::boundary_edges(self.face, self.subject, 4, cancelled)?;
        let edges = self.order_edges(&actual)?;
        require_cycle(edges, self.subject)?;
        if matches!(self.curve.used.curve, CurveId::Arc(_)) {
            self.cylinder(edges, source)
        } else {
            self.plane(edges, source)
        }
    }

    fn order_edges<'b>(&self, actual: &'b [Edge]) -> Result<[&'b Edge; 4], Failure> {
        let [start, end, a, b] = [
            self.caps[0],
            self.caps[1],
            self.laterals[0].0,
            self.laterals[1].0,
        ]
        .map(|named| actual.iter().find(|edge| edge.is_same(named)));
        match (start, end, a, b) {
            (Some(start), Some(end), Some(a), Some(b)) => Ok([start, end, a, b]),
            _ => Err(self
                .subject
                .refused("both named cap edges and the two named lateral edges")),
        }
    }

    fn cylinder(&self, [start, end, a, b]: [&Edge; 4], source: &Source) -> Result<(), Failure> {
        let chart = source.chart;
        let (frame, radius) = native::cylinder(self.face, self.subject)?;
        let [x, y, z] = chart.axes;
        let reversed = z.map(|value| -value * chart.height.signum());
        let expected = Frame {
            origin: chart.origins[0],
            axes: [x, y, reversed],
        };
        self.subject.require(
            radius == source.arc.authored().radius() && frame == expected,
            "the original circular carrier extruded along the actual signed normal",
        )?;
        let (angle, height) = (FRAC_PI_2, chart.height.abs());
        self.cylinder_trim(start, [0., angle], [[0., 0.], [angle, 0.]])?;
        self.cylinder_trim(end, [0., angle], [[0., -height], [angle, -height]])?;
        self.cylinder_trim(a, self.laterals[0].1, [[0., 0.], [0., -height]])?;
        self.cylinder_trim(b, self.laterals[1].1, [[angle, 0.], [angle, -height]])
    }

    fn cylinder_trim(
        &self,
        edge: &Edge,
        range: [f64; 2],
        expected: [Uv; 2],
    ) -> Result<(), Failure> {
        let stored = native::pcurve(edge, self.face, range, true, self.subject)?;
        let actual = native::line_uv(&stored, self.subject)?;
        self.subject.require(
            actual == expected,
            "the complete stored rectangular cylinder pcurve in native U/V",
        )
    }

    fn plane(&self, [start, end, a, b]: [&Edge; 4], source: &Source) -> Result<(), Failure> {
        let frame = native::plane(self.face, self.subject)?;
        let [first, last] = self.curve.endpoints.map(|id| source.points[&id].xyz);
        self.plane_trim(start, frame, [first[0], last[0]])?;
        self.plane_trim(end, frame, [first[1], last[1]])?;
        self.plane_trim(a, frame, first)?;
        self.plane_trim(b, frame, last)
    }

    fn plane_trim(&self, edge: &Edge, frame: Frame, xyz: [Xyz; 2]) -> Result<(), Failure> {
        let range = native::line_xyz(edge, xyz, self.subject)?;
        native::planar_trace(edge, self.face, frame, range, xyz, self.subject)
    }
}

fn require_cycle(actual: [&Edge; 4], subject: Subject) -> Result<(), Failure> {
    let named = |edge: &Edge| {
        matches!(
            edge.orientation(),
            Orientation::Forward | Orientation::Reversed
        )
    };
    let ends = actual.map(|edge| topexp::vertices(edge, true));
    // Start-cap and end-cap edges are opposite in the four-edge side cycle.
    let order = if actual[0].orientation() == Orientation::Forward {
        [0, 3, 1, 2]
    } else {
        [0, 2, 1, 3]
    };
    let joined = (0..4).all(|index| {
        ends[order[index]]
            .1
            .is_same(&ends[order[(index + 1) % 4]].0)
    });
    subject.require(
        actual.into_iter().all(named) && joined,
        "the single directed native product-boundary cycle",
    )
}
