// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const IDENTITY: [[f64; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];

pub(super) fn admit_family(query: CarrierSeparationQuery<'_>) -> Result<(), SeparationCause> {
    let (sides, caps) = (query.sides.len(), query.caps.len());
    if sides == 0 || caps == 0 || sides > MAX_PATCHES.saturating_sub(caps) || caps > MAX_PATCHES {
        return Err(SeparationCause::PatchCount { sides, caps });
    }
    admit_stock(query.stock)?;
    admit_rims(query.rims)?;
    ordered(query.sides.iter().map(|side| side.id))?;
    ordered(query.caps.iter().map(|cap| cap.id))
}

fn ordered(ids: impl Iterator<Item = u64>) -> Result<(), SeparationCause> {
    let mut previous = None;
    for next in ids {
        if let Some(previous) = previous.filter(|previous| *previous >= next) {
            return Err(SeparationCause::SourceOrder { previous, next });
        }
        previous = Some(next);
    }
    Ok(())
}

fn coordinate(value: f64) -> Result<(), SeparationCause> {
    if value.abs() <= MAX_ABS_SCALAR {
        Ok(())
    } else {
        Err(SeparationCause::Coordinate { value })
    }
}

fn admit_stock(stock: DecodedConeFrame) -> Result<(), SeparationCause> {
    coordinates(stock)?;
    if stock.location.matrix != IDENTITY || stock.location.translation != [0.; 3] {
        return Err(SeparationCause::UnsupportedFrame);
    }
    if !(0. < stock.semi_angle && stock.semi_angle < std::f64::consts::FRAC_PI_2) {
        return Err(SeparationCause::InvalidAngle {
            angle: stock.semi_angle,
        });
    }
    Ok(())
}

fn coordinates(f: DecodedConeFrame) -> Result<(), SeparationCause> {
    for v in frame_scalars(f) {
        coordinate(v)?;
    }
    if f.location.scale != 1. || f.origin[..2] != [0.; 2] || f.axes != IDENTITY {
        return Err(SeparationCause::UnsupportedFrame);
    }
    if f.radius <= 0. {
        return Err(SeparationCause::PositiveRadius { lower: f.radius });
    }
    Ok(())
}

fn frame_scalars(f: DecodedConeFrame) -> impl Iterator<Item = f64> {
    [f.semi_angle, f.radius]
        .into_iter()
        .chain(f.origin)
        .chain(f.axes.into_iter().flatten())
        .chain(location_scalars(f.location))
}

fn admit_rims(rims: [DecodedRimPlane; 2]) -> Result<(), SeparationCause> {
    for rim in rims {
        coordinate(rim.elevation)?;
        if rim.normal[..2] != [0., 0.] || rim.normal[2].abs() != 1. {
            return Err(SeparationCause::UnsupportedFrame);
        }
    }
    let elevations = rims.map(|rim| rim.elevation);
    if elevations[0] >= elevations[1] {
        return Err(SeparationCause::RimInterval { elevations });
    }
    Ok(())
}

fn location_scalars(location: DecodedCoaxialLocation) -> impl Iterator<Item = f64> {
    [location.scale]
        .into_iter()
        .chain(location.translation)
        .chain(location.matrix.into_iter().flatten())
}

fn coaxial(location: DecodedCoaxialLocation) -> Result<(), SeparationCause> {
    for coordinate_value in location_scalars(location) {
        coordinate(coordinate_value)?;
    }
    let [[a, b, z0], [c, d, z1], z] = location.matrix;
    if location.scale != 1.
        || location.translation[..2] != [0.; 2]
        || [z0, z1] != [0.; 2]
        || z != [0., 0., 1.]
        || a != d
        || b != -c
        || (a == 0. && b == 0.)
    {
        return Err(SeparationCause::UnsupportedFrame);
    }
    Ok(())
}

pub(super) fn admit_side(side: DecodedRuledSide<'_>) -> Result<(), SeparationCause> {
    coaxial(side.location)?;
    if side.domain != [[0., 1.]; 2] {
        return Err(SeparationCause::UnsupportedDomain {
            domain: side.domain,
        });
    }
    let rows = side.rows.map(<[[f64; 4]]>::len);
    if rows[0] != rows[1] || !(2..=MAX_POLES).contains(&rows[0]) {
        return Err(SeparationCause::PoleCount { rows });
    }
    for &[x, y, z, w] in side.rows.into_iter().flatten() {
        for v in [x, y, z] {
            coordinate(v)?;
        }
        if !(MIN_WEIGHT..=MAX_ABS_SCALAR).contains(&w) {
            return Err(SeparationCause::Weight { value: w });
        }
    }
    Ok(())
}

pub(super) fn admit_cap(
    cap: DecodedCapTrim,
    stock: DecodedConeFrame,
) -> Result<(), SeparationCause> {
    coaxial(cap.frame.location)?;
    coordinates(cap.frame)?;
    if cap.frame.semi_angle.to_bits() != stock.semi_angle.to_bits() {
        return Err(SeparationCause::UnsupportedFrame);
    }
    let bounds = cap.axial_parameters;
    for v in bounds {
        coordinate(v)?;
    }
    if bounds[0] >= bounds[1] {
        return Err(SeparationCause::AxialParameters { bounds });
    }
    Ok(())
}

pub(super) fn source_bits(query: CarrierSeparationQuery<'_>) -> Vec<u64> {
    let mut out: Vec<_> = frame_scalars(query.stock).map(f64::to_bits).collect();
    out.extend([
        query.sense as u64,
        u64::from(query.options.max_scalar_steps),
        u64::from(query.options.max_dyadic_operations),
        query.sides.len() as u64,
        query.caps.len() as u64,
    ]);
    for rim in query.rims {
        out.extend([rim.elevation, rim.normal[0], rim.normal[1], rim.normal[2]].map(f64::to_bits));
    }
    for side in query.sides {
        append_side(*side, &mut out);
    }
    for cap in query.caps {
        out.push(cap.id);
        out.extend(frame_scalars(cap.frame).map(f64::to_bits));
        out.extend(cap.axial_parameters.map(f64::to_bits));
    }
    out
}

fn append_side(side: DecodedRuledSide<'_>, out: &mut Vec<u64>) {
    out.extend([
        side.id,
        side.rows[0].len() as u64,
        side.rows[1].len() as u64,
    ]);
    out.extend(location_scalars(side.location).map(f64::to_bits));
    out.extend(side.domain.into_iter().flatten().map(f64::to_bits));
    out.extend(
        side.rows
            .into_iter()
            .flatten()
            .flatten()
            .map(|x| x.to_bits()),
    );
}
