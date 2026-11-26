// SPDX-License-Identifier: GPL-2.0-only
//! Actual corresponding cap curves and Green area of the retained translated trim.
use super::super::{
    Failure, Stage,
    images::Images,
    native::{self, Frame, Subject},
    poll,
    source::{Source, Uv},
};
use crate::spatial_prism::volume::interval::Span;
use martensite_tkbo::profile_extrusion::LoopUse;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tksketch3d::CurveId;
use std::f64::consts::FRAC_PI_2;

pub(super) fn verify(
    source: &Source,
    images: &Images<'_>,
    outer: &[LoopUse],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let start = cap_area(source, images, outer, 0, cancelled)?;
    let end = cap_area(source, images, outer, 1, cancelled)?;
    Subject(None).require(
        start == end,
        "the complete cap area transported by the normal sweep",
    )?;
    Ok(start)
}

fn cap_area(
    source: &Source,
    images: &Images<'_>,
    outer: &[LoopUse],
    index: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let cap = Cap::capture(images.caps[index], source, index)?;
    let subject = Subject(None);
    let boundary = native::boundary_edges(cap.face, subject, outer.len(), cancelled)?;
    let start = cap_order(&boundary, images, outer, index, subject)?;
    let mut integral = Span::ZERO;
    for (offset, &used) in outer.iter().enumerate() {
        poll(cancelled, Stage::Sweep)?;
        let edge = &boundary[(offset + start) % boundary.len()];
        let forward = cap.member(edge, source, images, used)?;
        integral = integral + if used.reversed { -forward } else { forward };
    }
    Ok(integral)
}

fn cap_order(
    actual: &[Edge],
    images: &Images<'_>,
    outer: &[LoopUse],
    cap: usize,
    subject: Subject,
) -> Result<usize, Failure> {
    let first = outer
        .first()
        .ok_or_else(|| subject.refused("a nonempty native cap outer loop"))?;
    let first = images.curves[&first.curve].caps[cap];
    actual
        .iter()
        .position(|edge| edge.is_same(first))
        .ok_or_else(|| subject.refused("the named source cycle on each native cap"))
}

struct Cap<'a> {
    face: &'a Face,
    frame: Frame,
    index: usize,
}

impl<'a> Cap<'a> {
    fn capture(face: &'a Face, source: &Source, index: usize) -> Result<Self, Failure> {
        let subject = Subject(None);
        let frame = Frame {
            origin: source.chart.origins[index],
            axes: source.chart.axes,
        };
        let outward_normal = (source.chart.height > 0.) == (index == 1);
        let orientation = if outward_normal {
            Orientation::Forward
        } else {
            Orientation::Reversed
        };
        subject.require(
            native::plane(face, subject)? == frame && face.orientation() == orientation,
            "the exact translated cap plane, parameter frame and outward orientation",
        )?;
        Ok(Self { face, frame, index })
    }

    fn member(
        &self,
        edge: &Edge,
        source: &Source,
        images: &Images<'_>,
        used: LoopUse,
    ) -> Result<Span, Failure> {
        let curve = source.curves[&used.curve];
        let subject = Subject(Some(curve.used.curve));
        let orientation = if used.reversed {
            Orientation::Reversed
        } else {
            Orientation::Forward
        };
        subject.require(
            edge.is_same(images.curves[&used.curve].caps[self.index])
                && edge.orientation() == orientation,
            "the complete directed named outer boundary of each native cap",
        )?;
        let vertices = curve
            .endpoints
            .map(|id| images.points[&id].caps[self.index]);
        native::edge_vertices(edge, vertices, subject)?;
        let positions = curve.endpoints.map(|id| source.points[&id]);
        if matches!(curve.used.curve, CurveId::Arc(_)) {
            return self.arc(edge, source.arc.authored().radius(), subject);
        }
        self.line(
            edge,
            positions.map(|point| point.xyz[self.index]),
            positions.map(|point| point.uv),
            subject,
        )
    }

    fn line(
        &self,
        edge: &Edge,
        xyz: [[f64; 3]; 2],
        uv: [Uv; 2],
        subject: Subject,
    ) -> Result<Span, Failure> {
        let range = native::line_xyz(edge, xyz, subject)?;
        let stored = native::pcurve(edge, self.face, range, true, subject)?;
        let actual = native::line_uv(&stored, subject)?;
        subject.require(
            actual == uv,
            "the same affine cap pcurve between the exact source vertices",
        )?;
        let [[a, b], [c, d]] = actual.map(|point| point.map(Span::at));
        Ok(a * d - b * c)
    }

    fn arc(&self, edge: &Edge, radius: f64, subject: Subject) -> Result<Span, Failure> {
        let range = [0., FRAC_PI_2];
        native::circle_xyz(edge, self.frame, radius, range, subject)?;
        let stored = native::pcurve(edge, self.face, range, true, subject)?;
        native::circle_uv(&stored, radius, subject)?;
        // Green integral of the origin-centered actual circle, plus an enclosure
        // for including or omitting the tiny bridge from its binary64 terminal
        // to the authored vertex. For T < pi/2, 0 <= cos(T) <= pi/2 - T.
        let trim_and_bridge = Span {
            lo: range[1],
            hi: range[1].next_up(),
        };
        Ok(Span::at(radius) * Span::at(radius) * trim_and_bridge)
    }
}
