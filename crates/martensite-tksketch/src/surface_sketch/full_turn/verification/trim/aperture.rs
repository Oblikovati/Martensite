// SPDX-License-Identifier: GPL-2.0-only
//! Check the aperture's actual directed loop and every analytic wall trim.
use super::*;

pub(in crate::surface_sketch::full_turn::verification) fn verify_aperture_walls(
    section: &BandSection,
    bindings: &Bindings,
) -> Checked<()> {
    for (key, shape) in bindings
        .iter()
        .filter(|(key, _)| key.role == HistoryRole::SideFace)
    {
        verify_aperture_wall(section, *key, shape, SurfaceType::Plane)?;
    }
    Ok(())
}

pub(in crate::surface_sketch::full_turn::verification) fn verify_aperture_wall(
    section: &BandSection,
    key: HistoryKey,
    shape: &topods::Shape,
    carrier: SurfaceType,
) -> Checked<()> {
    let face = topods::face(shape).map_err(|e| native("aperture wall", e))?;
    let surface = BRepAdaptorSurface::with_face(&face, true)
        .map_err(|e| native("aperture wall surface", e))?;
    require_surface(&surface, carrier)?;
    let mut edges = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("aperture wall edges", e))?;
    while edges.more() {
        let edge = topods::edge(edges.current()).map_err(|e| native("aperture wall edge", e))?;
        let pcurve = BRepTool::curve_on_face(&edge, &face).map_err(|e| native("wall pcurve", e))?;
        verify_wall_curve(&edge, &pcurve, key, carrier)?;
        verify_representation(section, &surface, &edge, &pcurve)?;
        edges
            .next()
            .map_err(|e| native("next aperture wall edge", e))?;
    }
    Ok(())
}

fn verify_wall_curve(
    edge: &topods::Edge,
    pcurve: &EdgePCurve,
    key: HistoryKey,
    carrier: SurfaceType,
) -> Checked<()> {
    let curve = BRepAdaptorCurve::with_edge(edge).map_err(|e| native("aperture wall curve", e))?;
    let kind = curve
        .get_type()
        .map_err(|e| native("aperture wall edge type", e))?;
    if carrier == SurfaceType::Cone {
        return verify_conical_wall_curve(edge, pcurve, kind);
    }
    let supported = match kind {
        CurveType::Circle => pcurve.pcurve.downcast_ref::<Geom2dCircle>().is_some(),
        CurveType::Line => pcurve.pcurve.downcast_ref::<Geom2dLine>().is_some(),
        _ => false,
    };
    if supported {
        return Ok(());
    }
    Err(incomplete(
        "paired analytic line or circle on aperture wall",
        format!("{key:?}: {kind:?}"),
    ))
}

fn verify_conical_wall_curve(
    edge: &topods::Edge,
    pcurve: &EdgePCurve,
    kind: CurveType,
) -> Checked<()> {
    if !pcurve.is_stored || !matches!(kind, CurveType::Circle | CurveType::Line) {
        return Err(incomplete(
            "stored conical wall pcurve of a circle or line",
            format!("{kind:?}"),
        ));
    }
    increasing(edge, pcurve, usize::from(kind == CurveType::Line))?;
    Ok(())
}

pub(super) fn verify_wire(
    section: &BandSection,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    stock_lower: &topods::Edge,
    hole: &[topods::Edge; 4],
) -> Checked<()> {
    let outer =
        BRepTool::curve_on_face(stock_lower, face).map_err(|e| native("stock rim winding", e))?;
    let mut directions = [false; 4];
    for (index, edge) in hole.iter().enumerate() {
        directions[index] = verify_aperture_edge(section, face, surface, edge, index)?;
    }
    let winding = increasing(stock_lower, &outer, 0)?;
    if directions[0] == winding || directions[0] == directions[2] || directions[1] == directions[3]
    {
        return Err(incomplete(
            "closed aperture wire opposite to stock rim",
            format!("{directions:?} with stock travel {winding}"),
        ));
    }
    Ok(())
}

fn verify_aperture_edge(
    section: &BandSection,
    face: &topods::Face,
    surface: &BRepAdaptorSurface,
    edge: &topods::Edge,
    index: usize,
) -> Checked<bool> {
    let pcurve =
        BRepTool::curve_on_face(edge, face).map_err(|e| native("aperture depth pcurve", e))?;
    if !pcurve.is_stored {
        return Err(incomplete(
            "stored aperture pcurve",
            format!("authored edge {index}"),
        ));
    }
    let axis = index % 2;
    let direction = increasing(edge, &pcurve, axis)?;
    verify_lift(section, surface, &pcurve, index)?;
    verify_representation(section, surface, edge, &pcurve)?;
    Ok(direction)
}

pub(in crate::surface_sketch::full_turn::verification) fn increasing(
    edge: &topods::Edge,
    pcurve: &EdgePCurve,
    axis: usize,
) -> Checked<bool> {
    let line = pcurve.pcurve.downcast_ref::<Geom2dLine>().ok_or_else(|| {
        incomplete(
            "analytic iso-parametric aperture line",
            format!("axis {axis}"),
        )
    })?;
    let direction = [line.direction().x(), line.direction().y()];
    if direction[axis].abs() != 1. || direction[1 - axis] != 0. {
        return Err(incomplete(
            "signed isoparametric aperture direction",
            format!("axis {axis}: {direction:?}"),
        ));
    }
    match edge.orientation() {
        Orientation::Forward => Ok(direction[axis] > 0.),
        Orientation::Reversed => Ok(direction[axis] < 0.),
        actual => Err(incomplete(
            "directed aperture edge use",
            format!("{actual:?}"),
        )),
    }
}

fn verify_lift(
    section: &BandSection,
    surface: &BRepAdaptorSurface,
    pcurve: &EdgePCurve,
    index: usize,
) -> Checked<()> {
    let [umin, umax] = [
        surface
            .first_u_parameter()
            .map_err(|e| native("stock lift start", e))?,
        surface
            .last_u_parameter()
            .map_err(|e| native("stock lift end", e))?,
    ];
    let [vmin, vmax] = [
        surface
            .first_v_parameter()
            .map_err(|e| native("stock meridian start", e))?,
        surface
            .last_v_parameter()
            .map_err(|e| native("stock meridian end", e))?,
    ];
    for parameter in [pcurve.first, pcurve.last] {
        let uv = trim_uv(pcurve, parameter)?;
        let clear = umin + angular(section) < uv.x()
            && uv.x() < umax - angular(section)
            && vmin + tolerance(section) < uv.y()
            && uv.y() < vmax - tolerance(section);
        if !clear {
            return Err(incomplete(
                "aperture strictly inside one stock lift",
                format!("edge {index}: [{}, {}]", uv.x(), uv.y()),
            ));
        }
    }
    Ok(())
}
