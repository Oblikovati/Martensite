//! Decode only immutable coherent inputs; no geometry getter is invoked here.
mod cells;
mod edges;
use super::{budget::*, placements::*};
use cells::Cells;
use martensite_tkbrep::brep::face_input_capture::CapturedOccurrence;
use martensite_tkbrep::brep::face_input_capture::{CapturedFaceCell as Cell, FaceInputSnapshot};
use martensite_tkg3d::{
    geom::StoredSplineInput,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Axis1, Axis2d},
    predicates::roots::metric_bounds::MetricEmbedding,
    toploc::Location,
};
type FaceFields<'a> = (
    usize,
    &'a martensite_tkbrep::brep::face_input_capture::CapturedLocation,
    f64,
    bool,
);
struct WireEdgeContext<'a> {
    wire: &'a CapturedOccurrence,
    location: &'a Location,
    surface: usize,
    surface_location: &'a Location,
}
struct StripDecodeInput<'a, 'cells, 'budget, F: FnMut() -> bool> {
    cells: &'a Cells<'cells>,
    root: &'a CapturedOccurrence,
    wire: &'a CapturedOccurrence,
    budget: &'a mut Budget<'budget, F>,
}
struct PositionedStripInput<'a, 'cells, 'budget, F: FnMut() -> bool> {
    cells: &'a Cells<'cells>,
    root_location: Location,
    wire: &'a CapturedOccurrence,
    replay: &'a mut PlacementReplay,
    budget: &'a mut Budget<'budget, F>,
}
pub(super) struct Strip {
    pub basis: StoredSplineInput,
    pub direction: [f64; 3],
    pub placement: MetricEmbedding<3>,
    pub tolerance: f64,
    pub edges: Vec<EdgeUse>,
}
pub(super) enum SpaceCurve {
    Line(Axis1),
    Spline(StoredSplineInput),
}
pub(super) struct VertexUse {
    pub owner: usize,
    pub location: Location,
    pub placement: MetricEmbedding<3>,
    pub point: [f64; 3],
    pub tolerance: f64,
    pub side: usize,
}
pub(super) struct EdgeUse {
    pub owner: usize,
    pub location: Location,
    pub orientation: Orientation,
    pub pcurve: Axis2d,
    pub cached_endpoints: [[f64; 2]; 2],
    pub range: [f64; 2],
    pub curve: SpaceCurve,
    pub placement: MetricEmbedding<3>,
    pub tolerance: f64,
    pub branch: u8,
    pub seam: bool,
    pub vertices: Vec<VertexUse>,
}
pub(super) fn decode<F: FnMut() -> bool>(
    input: &FaceInputSnapshot,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Strip> {
    budget.poll()?;
    let cells = Cells { input };
    let (root, wire) = root_wire(&cells)?;
    decode_strip(StripDecodeInput {
        cells: &cells,
        root,
        wire,
        budget,
    })
}
fn decode_strip<F: FnMut() -> bool>(input: StripDecodeInput<'_, '_, '_, F>) -> CheckResult<Strip> {
    let StripDecodeInput {
        cells,
        root,
        wire,
        budget,
    } = input;
    let mut replay = PlacementReplay::new();
    let root_location = replay.location(&root.location, budget)?;
    decode_positioned_strip(PositionedStripInput {
        cells,
        root_location,
        wire,
        replay: &mut replay,
        budget,
    })
}
fn decode_positioned_strip<F: FnMut() -> bool>(
    input: PositionedStripInput<'_, '_, '_, F>,
) -> CheckResult<Strip> {
    let (surface, surface_location, basis, direction, tolerance) = support_frame(
        input.cells,
        &input.root_location,
        input.replay,
        input.budget,
    )?;
    let edges = positioned_edges(
        input.cells,
        &input.root_location,
        input.wire,
        surface,
        &surface_location,
        input.replay,
        input.budget,
    )?;
    strip(basis, direction, surface_location, tolerance, edges)
}
fn strip(
    basis: &StoredSplineInput,
    direction: [f64; 3],
    surface_location: Location,
    tolerance: f64,
    edges: Vec<EdgeUse>,
) -> CheckResult<Strip> {
    Ok(Strip {
        basis: basis.clone(),
        direction,
        placement: embedding(&surface_location)?,
        tolerance,
        edges,
    })
}
fn root_wire<'a>(
    cells: &Cells<'a>,
) -> CheckResult<(&'a CapturedOccurrence, &'a CapturedOccurrence)> {
    let root = face_root(cells)?;
    let wire = boundary_wire(cells, root.owner)?;
    Ok((root, wire))
}
fn face_root<'a>(cells: &Cells<'a>) -> CheckResult<&'a CapturedOccurrence> {
    let root = cells.input.root();
    if !matches!(
        root.orientation,
        Orientation::Forward | Orientation::Reversed
    ) {
        return Err(invalid(
            root.owner,
            "face orientation",
            format!("{:?}", root.orientation),
        ));
    }
    Ok(root)
}
fn boundary_wire<'a>(cells: &Cells<'a>, owner: usize) -> CheckResult<&'a CapturedOccurrence> {
    let wires = cells.children(owner, ShapeEnum::Face)?;
    if wires.len() != 1 {
        return Err(invalid(
            owner,
            "one boundary wire",
            format!("wires={}", wires.len()),
        ));
    }
    Ok(&wires[0])
}
fn support_frame<'a, F: FnMut() -> bool>(
    cells: &Cells<'a>,
    root_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<(usize, Location, &'a StoredSplineInput, [f64; 3], f64)> {
    let (surface, face_location, tolerance) = face(cells)?;
    let surface_location = root_location
        .multiplied(&replay.location(face_location, budget)?)
        .map_err(native)?;
    let (basis, direction) = support(cells, surface)?;
    Ok((surface, surface_location, basis, direction, tolerance))
}
fn positioned_edges<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    root_location: &Location,
    wire: &CapturedOccurrence,
    surface: usize,
    surface_location: &Location,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Vec<EdgeUse>> {
    let location = wire_location(root_location, wire, replay, budget)?;
    let context = WireEdgeContext {
        wire,
        location: &location,
        surface,
        surface_location,
    };
    wire_edges(cells, &context, replay, budget)
}

fn wire_location<F: FnMut() -> bool>(
    root_location: &Location,
    wire: &CapturedOccurrence,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Location> {
    root_location
        .multiplied(&replay.location(&wire.location, budget)?)
        .map_err(native)
}
fn wire_edges<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    context: &WireEdgeContext<'_>,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Vec<EdgeUse>> {
    let children = boundary_edges(cells, context.wire.owner)?;
    children
        .iter()
        .map(|child| decode_edge(cells, context, child, replay, budget))
        .collect()
}

fn boundary_edges<'a>(cells: &Cells<'a>, owner: usize) -> CheckResult<&'a [CapturedOccurrence]> {
    let children = cells.children(owner, ShapeEnum::Wire)?;
    if children.len() != 4 {
        return Err(invalid(
            owner,
            "four boundary occurrences",
            format!("edges={}", children.len()),
        ));
    }
    Ok(children)
}
fn decode_edge<F: FnMut() -> bool>(
    cells: &Cells<'_>,
    context: &WireEdgeContext<'_>,
    child: &CapturedOccurrence,
    replay: &mut PlacementReplay,
    budget: &mut Budget<'_, F>,
) -> CheckResult<EdgeUse> {
    edges::decode(edges::EdgeDecodeInput {
        cells,
        occurrence: child,
        wire_orientation: context.wire.orientation,
        wire_location: context.location,
        surface: context.surface,
        surface_location: context.surface_location,
        replay,
        budget,
    })
}

fn face<'a>(
    cells: &Cells<'a>,
) -> CheckResult<(
    usize,
    &'a martensite_tkbrep::brep::face_input_capture::CapturedLocation,
    f64,
)> {
    let owner = cells.input.root().owner;
    let (surface, location, tolerance, natural_restriction) = face_fields(cells, owner)?;
    if natural_restriction {
        return Err(unsupported(
            owner,
            "natural restriction",
            "enabled",
            "explicit finite trim wire",
        ));
    }
    Ok((surface, location, tolerance))
}
fn face_fields<'a>(cells: &Cells<'a>, owner: usize) -> CheckResult<FaceFields<'a>> {
    let Cell::Face {
        surface,
        location,
        tolerance,
        natural_restriction,
    } = cells.one(owner, |c| matches!(c, Cell::Face { .. }), "face support")?
    else {
        return Err(invalid(owner, "face slot", "wrong type"));
    };
    Ok((*surface, location, *tolerance, *natural_restriction))
}

fn support<'a>(cells: &Cells<'a>, owner: usize) -> CheckResult<(&'a StoredSplineInput, [f64; 3])> {
    let Cell::Extrusion { basis, direction } = cells.one(
        owner,
        |c| matches!(c, Cell::Extrusion { .. }),
        "stored extrusion",
    )?
    else {
        return Err(invalid(owner, "support slot", "wrong type"));
    };
    Ok((cells.spline(*basis)?, *direction))
}
