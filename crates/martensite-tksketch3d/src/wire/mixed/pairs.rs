// SPDX-License-Identifier: GPL-2.0-only
//! Strict pair proofs on complete line and quarter-circle trims.
use super::{
    CardinalWireArc, LineDirection, MixedWireGeometry as Geometry, MixedWireSegment, WireFailure,
    WireSegment, curves,
};
use crate::wire::{ordered::Adjacency, recipe::check_segment_pair};
use martensite_tkmath::{gp::Line3, predicates::roots::metric_bounds::exact_placed_line_ends};

const IDENTITY: [[f64; 4]; 3] = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];

pub(super) fn require_exact_line(line: &WireSegment) -> Result<(), WireFailure> {
    let length = line.authored.length_estimate;
    if martensite_tkernel::precision::is_infinite(length)
        || !exact_line(line.authored.carrier, [0., length], line)
    {
        return Err(WireFailure::MixedLineTrace(line.source.line));
    }
    Ok(())
}

pub(super) fn exact_line(carrier: Line3, range: [f64; 2], line: &WireSegment) -> bool {
    let expected = line.authored.points.map(|point| *point.coordinates());
    exact_placed_line_ends(carrier, IDENTITY, range)
        .ok()
        .flatten()
        == Some(expected)
}

pub(super) fn check(pair: [&MixedWireSegment; 2], adjacency: Adjacency) -> Result<(), WireFailure> {
    let [first, second] = pair;
    let adjacent = adjacency != Adjacency::Separate;
    if let [Geometry::Line(a), Geometry::Line(b)] = [first.geometry(), second.geometry()] {
        return check_segment_pair([a, b], adjacent);
    }
    let proved = match adjacency {
        Adjacency::Separate => separated(pair),
        Adjacency::Consecutive => tangent(pair),
        Adjacency::Wrap => tangent([second, first]),
    };
    if !proved {
        return Err(WireFailure::UnprovedCurvePair {
            curves: curves(pair),
            adjacent,
        });
    }
    Ok(())
}

fn tangent([before, after]: [&MixedWireSegment; 2]) -> bool {
    match (before.geometry(), after.geometry()) {
        (Geometry::Line(line), Geometry::Arc(arc)) => {
            line_direction(line) == arc.tangents(after.source.direction)[0]
        }
        (Geometry::Arc(arc), Geometry::Line(line)) => {
            arc.tangents(before.source.direction)[1] == line_direction(line)
        }
        _ => false,
    }
}

fn line_direction(line: &WireSegment) -> martensite_tkmath::gp::Direction3 {
    let direction = line.authored.carrier.direction();
    match line.source.direction {
        LineDirection::Forward => direction,
        LineDirection::Reversed => direction.reversed(),
    }
}

fn separated(pair: [&MixedWireSegment; 2]) -> bool {
    let [a, b] = pair.map(box_of);
    a.into_iter().zip(b).any(|([a, b], [c, d])| b < c || d < a)
}

fn box_of(segment: &MixedWireSegment) -> [[f64; 2]; 3] {
    match segment.geometry {
        Geometry::Line(line) => {
            let [a, b] = line.authored.points;
            std::array::from_fn(|i| {
                let [a, b] = [a.coordinates()[i], b.coordinates()[i]];
                [a.min(b), a.max(b)]
            })
        }
        Geometry::Arc(arc) => quarter_box(arc),
    }
}

fn quarter_box(arc: CardinalWireArc) -> [[f64; 2]; 3] {
    // Cardinal coordinates attain both ideal quarter extrema among its ends and
    // center. Inflate that box by the analytical trim transport for native use.
    let [a, b, c] = arc.authored().points();
    let error = arc.terminal_error_upper();
    std::array::from_fn(|i| {
        let [a, b, c] = [a.coordinates()[i], b.coordinates()[i], c.coordinates()[i]];
        [
            (a.min(b).min(c) - error).next_down(),
            (a.max(b).max(c) + error).next_up(),
        ]
    })
}
