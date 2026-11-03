//! Decode only immutable coherent inputs; no geometry getter is invoked here.
mod cells;
mod edges;
use super::{budget::*, placements::*};
use cells::Cells;
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
    let root = input.root();
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
    let wires = cells.children(root.owner, ShapeEnum::Face)?;
    if wires.len() != 1 {
        return Err(invalid(
            root.owner,
            "one boundary wire",
            format!("wires={}", wires.len()),
        ));
    }
    let mut replay = PlacementReplay::new();
    let root_location = replay.location(&root.location, budget)?;
    let (surface, face_location, tolerance) = face(&cells)?;
    let surface_location = root_location
        .multiplied(&replay.location(face_location, budget)?)
        .map_err(native)?;
    let (basis, direction) = support(&cells, surface)?;
    let wire = &wires[0];
    let wire_location = root_location
        .multiplied(&replay.location(&wire.location, budget)?)
        .map_err(native)?;
    let children = cells.children(wire.owner, ShapeEnum::Wire)?;
    if children.len() != 4 {
        return Err(invalid(
            wire.owner,
            "four boundary occurrences",
            format!("edges={}", children.len()),
        ));
    }
    let mut edges = Vec::new();
    for child in children {
        edges.push(edges::decode(
            &cells,
            child,
            wire.orientation,
            &wire_location,
            surface,
            &surface_location,
            &mut replay,
            budget,
        )?);
    }
    Ok(Strip {
        basis: basis.clone(),
        direction,
        placement: embedding(&surface_location)?,
        tolerance,
        edges,
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
    let Cell::Face {
        surface,
        location,
        tolerance,
        natural_restriction,
    } = cells.one(owner, |c| matches!(c, Cell::Face { .. }), "face support")?
    else {
        return Err(invalid(owner, "face slot", "wrong type"));
    };
    if *natural_restriction {
        return Err(unsupported(
            owner,
            "natural restriction",
            "enabled",
            "explicit finite trim wire",
        ));
    }
    Ok((*surface, location, *tolerance))
}
fn support(cells: &Cells<'_>, owner: usize) -> CheckResult<(&StoredSplineInput, [f64; 3])> {
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
