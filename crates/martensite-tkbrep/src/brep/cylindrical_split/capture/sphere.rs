// SPDX-License-Identifier: GPL-2.0-only
//! Share the literal full-period rectangle while requiring a circular meridian.
use super::{
    AnalyticCell, AnalyticFaceInput, Captured, Checked, Curve, Error, ShapeEnum, Wall, placement,
    required, reversed,
};
use martensite_tkmath::{
    gp::Sphere3, predicates::roots::metric_bounds::spherical_band::SphericalBandTraceInput,
};

pub(in crate::brep) fn decode(
    input: &AnalyticFaceInput,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<SphericalBandTraceInput> {
    let root = input.common.root();
    reversed(root).map_err(Error::Source)?;
    let values = Captured::index(&input.common);
    let surface = values.surface(root.owner).map_err(Error::Source)?;
    let [wire] = values
        .children(root.owner, ShapeEnum::Face)
        .map_err(Error::Source)?
    else {
        return Err(required(
            root.owner,
            "one closed spherical band wire without holes",
        ));
    };
    let trimmed = Wall {
        values,
        carriers: &input.carriers,
        surface,
    };
    let sphere = sphere(&trimmed)?;
    let placement = placement(root, cancelled)?;
    let ([start, end], [low, high]) = trimmed.wire(wire, true, cancelled)?.outer()?;
    let Curve::CurvedSeam(meridian) = start.curve else {
        return Err(required(
            start.owner,
            "two circular coedges on the same spherical meridian",
        ));
    };
    if !matches!(end.curve, Curve::CurvedSeam(_)) || start.offset != end.offset {
        return Err(required(
            end.owner,
            "the same exact meridian phase on both coedges",
        ));
    }
    let [Curve::Rim(first), Curve::Rim(last)] = [low.curve, high.curve] else {
        return Err(required(
            low.owner,
            "two nondegenerate complete latitude circles",
        ));
    };
    let [[_, lo], [_, hi]] = end.points;
    let [a, b] = start.vertices;
    Ok(SphericalBandTraceInput {
        sphere,
        placement,
        latitude: [lo, hi],
        rims: [first, last],
        meridian,
        meridian_range: start.range,
        meridian_offset: start.offset,
        anchors: [
            trimmed.values.vertex(a).map_err(Error::Source)?,
            trimmed.values.vertex(b).map_err(Error::Source)?,
        ],
    })
}

fn sphere(wall: &Wall<'_>) -> Checked<Sphere3> {
    match wall.analytic(wall.surface) {
        Some(AnalyticCell::Sphere(frame, radius)) if *radius > 0. => Sphere3::new(*frame, *radius)
            .map_err(|_| required(wall.surface, "positive finite stored sphere")),
        _ => Err(required(wall.surface, "an original native sphere support")),
    }
}
