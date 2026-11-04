//! Branch selection uses explicit stored representation and oriented occurrence identity.
use super::super::types::ClosedFaceCompletion;
use super::*;
use martensite_tkbrep::brep::face_input_capture::CapturedOccurrence;
use martensite_tkg3d::top_abs;

pub(super) struct EdgeDecodeInput<'ctx, 'input, 'budget, F: FnMut() -> bool> {
    pub(super) cells: &'ctx Cells<'input>,
    pub(super) occurrence: &'ctx CapturedOccurrence,
    pub(super) wire_orientation: Orientation,
    pub(super) wire_location: &'ctx Location,
    pub(super) surface: usize,
    pub(super) surface_location: &'ctx Location,
    pub(super) replay: &'ctx mut PlacementReplay,
    pub(super) budget: &'ctx mut Budget<'budget, F>,
}
struct DecodeContext<'ctx, 'input, 'budget, F: FnMut() -> bool> {
    cells: &'ctx Cells<'input>,
    surface: usize,
    edge_location: Location,
    surface_location: &'ctx Location,
    replay: &'ctx mut PlacementReplay,
    budget: &'ctx mut Budget<'budget, F>,
}
struct PcurveUse {
    axis: Axis2d,
    branch: u8,
    seam: bool,
    cached_endpoints: [[f64; 2]; 2],
}
struct SpatialUse {
    range: [f64; 2],
    curve: SpaceCurve,
    placement: MetricEmbedding<3>,
    vertices: Vec<VertexUse>,
}
struct EdgeUseBuild {
    owner: usize,
    location: Location,
    orientation: Orientation,
    tolerance: f64,
    pcurve: PcurveUse,
    geometry: SpatialUse,
}

pub(super) fn decode<F: FnMut() -> bool>(
    mut input: EdgeDecodeInput<'_, '_, '_, F>,
) -> CheckResult<EdgeUse> {
    let owner = input.occurrence.owner;
    let location = edge_location(&mut input)?;
    let orientation = top_abs::compose(input.wire_orientation, input.occurrence.orientation);
    let mut context = decode_context(input, location);
    let (tolerance, pcurve, geometry) = edge_parts(&mut context, owner, orientation)?;
    Ok(finish_edge(
        context,
        owner,
        orientation,
        tolerance,
        pcurve,
        geometry,
    ))
}
fn edge_location<F: FnMut() -> bool>(
    input: &mut EdgeDecodeInput<'_, '_, '_, F>,
) -> CheckResult<Location> {
    occurrence_location(
        input.occurrence,
        input.wire_location,
        input.replay,
        input.budget,
    )
}
fn finish_edge<F: FnMut() -> bool>(
    context: DecodeContext<'_, '_, '_, F>,
    owner: usize,
    orientation: Orientation,
    tolerance: f64,
    pcurve: PcurveUse,
    geometry: SpatialUse,
) -> EdgeUse {
    edge_use(EdgeUseBuild {
        owner,
        location: context.edge_location,
        orientation,
        tolerance,
        pcurve,
        geometry,
    })
}
fn decode_context<'ctx, 'input, 'budget, F: FnMut() -> bool>(
    input: EdgeDecodeInput<'ctx, 'input, 'budget, F>,
    edge_location: Location,
) -> DecodeContext<'ctx, 'input, 'budget, F> {
    DecodeContext {
        cells: input.cells,
        surface: input.surface,
        edge_location,
        surface_location: input.surface_location,
        replay: input.replay,
        budget: input.budget,
    }
}

fn edge_parts<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    orientation: Orientation,
) -> CheckResult<(f64, PcurveUse, SpatialUse)> {
    let (representations, tolerance) = edge_fields(context.cells, owner)?;
    let (spatial, parametric) = representations_pair(context, representations)?;
    let pcurve = pcurve(context, parametric, orientation)?;
    let geometry = spatial_use(context, owner, spatial, parametric)?;
    Ok((tolerance, pcurve, geometry))
}
fn occurrence_location<F: FnMut() -> bool>(
    occurrence: &CapturedOccurrence,
    wire_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Location> {
    let location = wire_location
        .multiplied(&replay.location(&occurrence.location, budget)?)
        .map_err(native)?;
    Ok(location)
}
fn edge_use(parts: EdgeUseBuild) -> EdgeUse {
    EdgeUse {
        owner: parts.owner,
        location: parts.location,
        orientation: parts.orientation,
        pcurve: parts.pcurve.axis,
        cached_endpoints: parts.pcurve.cached_endpoints,
        range: parts.geometry.range,
        curve: parts.geometry.curve,
        placement: parts.geometry.placement,
        tolerance: parts.tolerance,
        branch: parts.pcurve.branch,
        seam: parts.pcurve.seam,
        vertices: parts.geometry.vertices,
    }
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
    context: &mut DecodeContext<'_, '_, '_, F>,
    owners: &[usize],
) -> CheckResult<(usize, usize)> {
    let mut spatial = None;
    let mut parametric = None;
    for owner in owners {
        representation_slot(context, *owner, &mut spatial, &mut parametric)?;
    }
    representation_pair(owners, spatial, parametric)
}
fn representation_slot<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    spatial: &mut Option<usize>,
    parametric: &mut Option<usize>,
) -> CheckResult<()> {
    let cell = context.cells.one(
        owner,
        |c| matches!(c, Cell::Spatial { .. } | Cell::Parametric { .. }),
        "spatial or stored surface representation",
    )?;
    match cell {
        Cell::Spatial { .. } => record_spatial(owner, spatial),
        Cell::Parametric {
            surface: actual, ..
        } => record_parametric(context, owner, *actual, parametric),
        _ => Err(invalid(owner, "representation slot", "wrong type")),
    }
}
fn record_spatial(owner: usize, spatial: &mut Option<usize>) -> CheckResult<()> {
    if spatial.replace(owner).is_some() {
        return Err(unsupported(
            owner,
            "spatial curve",
            "multiple",
            "one spatial representation",
        ));
    }
    Ok(())
}
fn record_parametric<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    actual_surface: usize,
    parametric: &mut Option<usize>,
) -> CheckResult<()> {
    if actual_surface != context.surface {
        return Ok(());
    }
    let placed = face_representation_location(context, owner)?;
    if placed == *context.surface_location && parametric.replace(owner).is_some() {
        return Err(unsupported(
            owner,
            "current-face map",
            "multiple",
            "one located current-face representation",
        ));
    }
    Ok(())
}
fn face_representation_location<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
) -> CheckResult<Location> {
    context
        .edge_location
        .multiplied(
            &context
                .replay
                .location(context.cells.location(owner)?, context.budget)?,
        )
        .map_err(native)
}

fn representation_pair(
    owners: &[usize],
    spatial: Option<usize>,
    parametric: Option<usize>,
) -> CheckResult<(usize, usize)> {
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
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    orientation: Orientation,
) -> CheckResult<PcurveUse> {
    let (curve, actual, endpoints) = pcurve_fields(context.cells, owner)?;
    validate_pcurve_location(context, owner, actual)?;
    let seam = seam_fields(context.cells, owner);
    let (selected, branch) = select_pcurve(orientation, curve, seam);
    let axis = pcurve_line(context.cells, selected)?;
    let cached_endpoints = selected_endpoints(branch, endpoints, seam);
    Ok(PcurveUse {
        axis,
        branch,
        seam: seam.is_some(),
        cached_endpoints,
    })
}
fn pcurve_fields(cells: &Cells<'_>, owner: usize) -> CheckResult<(usize, usize, [[f64; 2]; 2])> {
    let Cell::Parametric {
        curve,
        surface,
        endpoints,
    } = cells.one(
        owner,
        |c| matches!(c, Cell::Parametric { .. }),
        "pcurve binding",
    )?
    else {
        return Err(invalid(owner, "pcurve binding", "wrong type"));
    };
    Ok((*curve, *surface, *endpoints))
}
fn validate_pcurve_location<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    actual_surface: usize,
) -> CheckResult<()> {
    let located = face_representation_location(context, owner)?;
    if actual_surface != context.surface || located != *context.surface_location {
        return Err(unsupported(
            owner,
            "pcurve support binding",
            format!("support={actual_surface}"),
            "same actual located face support",
        ));
    }
    Ok(())
}

fn seam_fields(cells: &Cells<'_>, owner: usize) -> Option<(usize, [[f64; 2]; 2])> {
    cells.input.cells().iter().find_map(|(id, cell)| {
        if *id == owner {
            match cell {
                Cell::Seam {
                    curve, endpoints, ..
                } => Some((*curve, *endpoints)),
                _ => None,
            }
        } else {
            None
        }
    })
}
fn select_pcurve(
    orientation: Orientation,
    curve: usize,
    seam: Option<(usize, [[f64; 2]; 2])>,
) -> (usize, u8) {
    if orientation == Orientation::Reversed && seam.is_some() {
        (seam.map(|value| value.0).unwrap_or(curve), 2)
    } else {
        (curve, 1)
    }
}
fn pcurve_line(cells: &Cells<'_>, selected: usize) -> CheckResult<Axis2d> {
    let Cell::PlanarLine(axis) = cells.one(
        selected,
        |c| matches!(c, Cell::PlanarLine(_)),
        "line pcurve",
    )?
    else {
        return Err(invalid(selected, "pcurve geometry", "wrong type"));
    };
    Ok(*axis)
}
fn selected_endpoints(
    branch: u8,
    endpoints: [[f64; 2]; 2],
    seam: Option<(usize, [[f64; 2]; 2])>,
) -> [[f64; 2]; 2] {
    if branch == 2 {
        seam.map(|value| value.1).unwrap_or(endpoints)
    } else {
        endpoints
    }
}

fn spatial_use<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
    spatial: usize,
    parametric: usize,
) -> CheckResult<SpatialUse> {
    let range = checked_range(context.cells, owner, spatial, parametric)?;
    let curve = space(context.cells, spatial)?;
    let placed = spatial_location(context, spatial)?;
    let vertices = vertices(context, owner)?;
    Ok(SpatialUse {
        range,
        curve,
        placement: embedding(&placed)?,
        vertices,
    })
}
fn spatial_location<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    spatial: usize,
) -> CheckResult<Location> {
    context
        .edge_location
        .multiplied(
            &context
                .replay
                .location(context.cells.location(spatial)?, context.budget)?,
        )
        .map_err(native)
}

fn checked_range(
    cells: &Cells<'_>,
    owner: usize,
    spatial: usize,
    parametric: usize,
) -> CheckResult<[f64; 2]> {
    let range = cells.range(spatial)?;
    if range != cells.range(parametric)?
        || range.iter().any(|value| !value.is_finite())
        || range[0] >= range[1]
    {
        return Err(invalid_range(cells, owner, range, parametric)?);
    }
    Ok(range)
}
fn invalid_range(
    cells: &Cells<'_>,
    owner: usize,
    range: [f64; 2],
    parametric: usize,
) -> CheckResult<ClosedFaceCompletion> {
    Ok(invalid(
        owner,
        "same finite increasing range",
        format!(
            "spatial={range:?}, parametric={:?}",
            cells.range(parametric)?
        ),
    ))
}

fn vertices<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    owner: usize,
) -> CheckResult<Vec<VertexUse>> {
    let children = context.cells.children(owner, ShapeEnum::Edge)?;
    if children.len() != 2 {
        return Err(invalid(
            owner,
            "two intrinsic endpoints",
            format!("count={}", children.len()),
        ));
    }
    children
        .iter()
        .map(|child| vertex(context, child))
        .collect()
}
fn vertex<F: FnMut() -> bool>(
    context: &mut DecodeContext<'_, '_, '_, F>,
    child: &CapturedOccurrence,
) -> CheckResult<VertexUse> {
    let (point, tolerance) = vertex_fields(context.cells, child.owner)?;
    let side = endpoint_side(child)?;
    let location = context
        .edge_location
        .multiplied(&context.replay.location(&child.location, context.budget)?)
        .map_err(native)?;
    Ok(VertexUse {
        owner: child.owner,
        placement: embedding(&location)?,
        location,
        point,
        tolerance,
        side,
    })
}
fn vertex_fields(cells: &Cells<'_>, owner: usize) -> CheckResult<([f64; 3], f64)> {
    if !cells.children(owner, ShapeEnum::Vertex)?.is_empty() {
        return Err(invalid(owner, "vertex children", "nonempty"));
    }
    let Cell::Vertex { point, tolerance } =
        cells.one(owner, |c| matches!(c, Cell::Vertex { .. }), "vertex fields")?
    else {
        return Err(invalid(owner, "vertex fields", "wrong type"));
    };
    Ok((*point, *tolerance))
}
fn endpoint_side(child: &CapturedOccurrence) -> CheckResult<usize> {
    match child.orientation {
        Orientation::Forward => Ok(0),
        Orientation::Reversed => Ok(1),
        other => Err(invalid(
            child.owner,
            "intrinsic endpoint orientation",
            format!("{other:?}"),
        )),
    }
}
