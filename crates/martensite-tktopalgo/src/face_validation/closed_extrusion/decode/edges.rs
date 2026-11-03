//! Branch selection uses explicit stored representation and oriented occurrence identity.
use super::*;
use martensite_tkbrep::brep::face_input_capture::CapturedOccurrence;
use martensite_tkg3d::top_abs;

pub(super) fn decode<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    occurrence: &CapturedOccurrence,
    wire_orientation: Orientation,
    wire_location: &Location,
    surface: usize,
    surface_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<EdgeUse> {
    let owner = occurrence.owner;
    let location = wire_location
        .multiplied(&replay.location(&occurrence.location, budget)?)
        .map_err(native)?;
    let orientation = top_abs::compose(wire_orientation, occurrence.orientation);
    let (representations, tolerance) = edge_fields(cells, owner)?;
    let (spatial, parametric) = representations_pair(
        cells,
        representations,
        surface,
        &location,
        surface_location,
        replay,
        budget,
    )?;
    let (pcurve, branch, seam) = pcurve(
        cells,
        parametric,
        orientation,
        surface,
        &location,
        surface_location,
        replay,
        budget,
    )?;
    let range = cells.range(spatial)?;
    if range != cells.range(parametric)?
        || range.iter().any(|v| !v.is_finite())
        || range[0] >= range[1]
    {
        return Err(invalid(
            owner,
            "same finite increasing range",
            format!(
                "spatial={range:?}, parametric={:?}",
                cells.range(parametric)?
            ),
        ));
    }
    let curve = space(cells, spatial)?;
    let placed = location
        .multiplied(&replay.location(cells.location(spatial)?, budget)?)
        .map_err(native)?;
    let vertices = vertices(cells, owner, &location, replay, budget)?;
    Ok(EdgeUse {
        owner,
        location,
        orientation,
        pcurve,
        range,
        curve,
        placement: embedding(&placed)?,
        tolerance,
        branch,
        seam,
        vertices,
    })
}
fn edge_fields<'a>(cells: &Cells<'a>, owner: usize) -> CheckResult<(&'a [usize], f64)> {
    let Cell::Edge {
        tolerance,
        flags,
        representations,
    } = cells.one(owner, |c| matches!(c, Cell::Edge { .. }), "edge fields")?
    else {
        return Err(invalid(owner, "edge fields", "wrong type"));
    };
    if flags & 7 != 3 {
        return Err(unsupported(
            owner,
            "edge flags",
            format!("{flags}"),
            "same-parameter, same-range and non-degenerated",
        ));
    }
    Ok((representations, *tolerance))
}
fn representations_pair<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    owners: &[usize],
    surface: usize,
    edge_location: &Location,
    surface_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<(usize, usize)> {
    let mut spatial = None;
    let mut parametric = None;
    for owner in owners {
        let cell = cells.one(
            *owner,
            |c| matches!(c, Cell::Spatial { .. } | Cell::Parametric { .. }),
            "spatial or stored surface representation",
        )?;
        match cell {
            Cell::Spatial { .. } => {
                if spatial.replace(*owner).is_some() {
                    return Err(unsupported(
                        *owner,
                        "spatial curve",
                        "multiple",
                        "one spatial representation",
                    ));
                }
            }
            Cell::Parametric {
                surface: actual, ..
            } if *actual == surface => {
                let placed = edge_location
                    .multiplied(&replay.location(cells.location(*owner)?, budget)?)
                    .map_err(native)?;
                if placed == *surface_location && parametric.replace(*owner).is_some() {
                    return Err(unsupported(
                        *owner,
                        "current-face map",
                        "multiple",
                        "one located current-face representation",
                    ));
                }
            }
            Cell::Parametric { .. } => {}
            _ => return Err(invalid(*owner, "representation slot", "wrong type")),
        }
    }
    match (spatial, parametric) {
        (Some(a), Some(b)) if a != b => Ok((a, b)),
        _ => Err(unsupported(
            0,
            "face representations",
            format!("{owners:?}"),
            "one spatial curve and one exact located face map",
        )),
    }
}

fn space(cells: &Cells<'_>, owner: usize) -> CheckResult<SpaceCurve> {
    let Cell::Spatial { curve } = cells.one(
        owner,
        |c| matches!(c, Cell::Spatial { .. }),
        "spatial binding",
    )?
    else {
        return Err(invalid(owner, "spatial binding", "wrong type"));
    };
    let cell = cells.one(
        *curve,
        |c| matches!(c, Cell::Line(_) | Cell::Spline(_)),
        "line or stored spline",
    )?;
    match cell {
        Cell::Line(axis) => Ok(SpaceCurve::Line(*axis)),
        Cell::Spline(input) => Ok(SpaceCurve::Spline(input.clone())),
        _ => Err(invalid(*curve, "spatial curve", "wrong type")),
    }
}
fn pcurve<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    owner: usize,
    orientation: Orientation,
    surface: usize,
    edge_location: &Location,
    surface_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<(Axis2d, u8, bool)> {
    let Cell::Parametric {
        curve,
        surface: actual,
        ..
    } = cells.one(
        owner,
        |c| matches!(c, Cell::Parametric { .. }),
        "pcurve binding",
    )?
    else {
        return Err(invalid(owner, "pcurve binding", "wrong type"));
    };
    let located = edge_location
        .multiplied(&replay.location(cells.location(owner)?, budget)?)
        .map_err(native)?;
    if *actual != surface || located != *surface_location {
        return Err(unsupported(
            owner,
            "pcurve support binding",
            format!("support={actual}"),
            "same actual located face support",
        ));
    }
    let seam = cells.input.cells().iter().find_map(|(id, c)| {
        if *id == owner {
            match c {
                Cell::Seam { curve, .. } => Some(*curve),
                _ => None,
            }
        } else {
            None
        }
    });
    let (selected, branch) = if orientation == Orientation::Reversed && seam.is_some() {
        (seam.unwrap_or(*curve), 2)
    } else {
        (*curve, 1)
    };
    let Cell::PlanarLine(axis) = cells.one(
        selected,
        |c| matches!(c, Cell::PlanarLine(_)),
        "line pcurve",
    )?
    else {
        return Err(invalid(selected, "pcurve geometry", "wrong type"));
    };
    Ok((*axis, branch, seam.is_some()))
}
fn vertices<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    owner: usize,
    edge_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Vec<VertexUse>> {
    let children = cells.children(owner, ShapeEnum::Edge)?;
    if children.len() != 2 {
        return Err(invalid(
            owner,
            "two intrinsic endpoints",
            format!("count={}", children.len()),
        ));
    }
    children
        .iter()
        .map(|child| vertex(cells, child, edge_location, replay, budget))
        .collect()
}
fn vertex<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    child: &CapturedOccurrence,
    edge_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<VertexUse> {
    if !cells.children(child.owner, ShapeEnum::Vertex)?.is_empty() {
        return Err(invalid(child.owner, "vertex children", "nonempty"));
    }
    let Cell::Vertex { point, tolerance } = cells.one(
        child.owner,
        |c| matches!(c, Cell::Vertex { .. }),
        "vertex fields",
    )?
    else {
        return Err(invalid(child.owner, "vertex fields", "wrong type"));
    };
    let side = match child.orientation {
        Orientation::Forward => 0,
        Orientation::Reversed => 1,
        other => {
            return Err(invalid(
                child.owner,
                "intrinsic endpoint orientation",
                format!("{other:?}"),
            ));
        }
    };
    let location = edge_location
        .multiplied(&replay.location(&child.location, budget)?)
        .map_err(native)?;
    Ok(VertexUse {
        owner: child.owner,
        placement: embedding(&location)?,
        location,
        point: *point,
        tolerance: *tolerance,
        side,
    })
}
