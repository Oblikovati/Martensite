// SPDX-License-Identifier: GPL-2.0-only
//! Read literal carrier coefficients and effective placements without evaluating pcurves.
use super::{Checked, PlanarSectionCause as Cause};
use crate::sketch_region_geometry_capture::{CapturedGeometryCause, captured_placement};
use martensite_tkbrep::{
    brep::{BRepCurveRepresentation, BRepTEdge, BRepTVertex, BRepTool},
    topods::{Edge, Face, Vertex},
};
use martensite_tkernel::standard::{Handle, Transient};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface, GeomTrimmedCurve};
use martensite_tkmath::{
    gp::{Line2, Line3, Plane3, Transform3},
    predicates::roots::metric_bounds::{
        SectionChartSource, SectionLineSource, SectionPlacement, SectionUvLine, SectionVertexSource,
    },
};

pub(super) struct RawChart {
    pub value: SectionChartSource,
    pub owner: Handle<dyn GeomSurface>,
}

pub(super) struct RawLine {
    pub value: SectionLineSource,
    pub owner: Handle<dyn GeomCurve>,
}

pub(super) struct RawPcurve {
    pub value: SectionUvLine,
    pub owner: Handle<dyn Geom2dCurve>,
}

pub(super) fn face_chart(face: &Face) -> Checked<RawChart> {
    let (owner, location) = BRepTool::surface_with_location(face)?;
    let plane = owner
        .downcast_ref::<GeomPlane>()
        .ok_or_else(|| representation(&owner, "Geom_Plane"))?;
    let value = SectionChartSource {
        plane: plane.pln(),
        placement: placement(location.transformation())?,
    };
    Ok(RawChart { value, owner })
}

pub(super) fn unplaced_chart(plane: Plane3) -> SectionChartSource {
    SectionChartSource {
        plane,
        placement: [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]],
    }
}

pub(super) fn same_chart(left: SectionChartSource, right: SectionChartSource) -> bool {
    left.placement == right.placement && frame(left.plane) == frame(right.plane)
}

fn frame(plane: Plane3) -> [[f64; 3]; 4] {
    let frame = plane.position();
    [
        *frame.location().coordinates(),
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    ]
}

pub(super) fn vertex_source(vertex: &Vertex) -> Checked<SectionVertexSource> {
    let owner = vertex.tshape().downcast::<BRepTVertex>();
    let geometry = owner
        .get()
        .ok_or_else(|| representation(vertex.tshape(), "BRep_TVertex"))?;
    Ok(SectionVertexSource {
        point: geometry.pnt(),
        placement: placement(vertex.location().transformation())?,
    })
}

pub(super) fn space_line(edge: &Edge) -> Checked<RawLine> {
    if BRepTool::degenerated(edge)? {
        return Err(Cause::Representation {
            actual: Some("degenerate edge".into()),
            expected: "positive-length linear edge",
        });
    }
    require_single_spatial(edge)?;
    let stored = BRepTool::curve_with_location(edge)?;
    let range = [stored.first, stored.last];
    let value = SectionLineSource {
        line: spatial_carrier(&stored.curve, range)?,
        placement: placement(stored.location.transformation())?,
        range,
    };
    Ok(RawLine {
        value,
        owner: stored.curve,
    })
}

pub(super) fn face_pcurve(edge: &Edge, face: &Face) -> Checked<Option<RawPcurve>> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let relative = location.predivided(edge.location())?;
    let representations = edge_representations(edge)?;
    let mut matches = representations
        .iter()
        .filter_map(Handle::get)
        .filter(|stored| stored.is_curve_on_surface_at(&surface, &relative));
    let Some(representation) = matches.next() else {
        return Ok(None);
    };
    if representation.is_curve_on_closed_surface() || matches.next().is_some() {
        return Err(Cause::Representation {
            actual: Some("multiple or seam pcurves".into()),
            expected: "one regular affine pcurve",
        });
    }
    let stored = BRepTool::curve_on_face(edge, face)?;
    if !stored.is_stored {
        return Err(Cause::Representation {
            actual: None,
            expected: "registered stored affine pcurve",
        });
    }
    let range = [stored.first, stored.last];
    let value = SectionUvLine {
        line: planar_carrier(&stored.pcurve, range)?,
        range,
    };
    Ok(Some(RawPcurve {
        value,
        owner: stored.pcurve,
    }))
}

fn require_single_spatial(edge: &Edge) -> Checked<()> {
    let count = edge_representations(edge)?
        .iter()
        .filter_map(Handle::get)
        .filter(|stored| stored.is_curve3d())
        .count();
    if count != 1 {
        return Err(Cause::Representation {
            actual: Some(format!("{count} spatial representations")),
            expected: "one stored 3D line",
        });
    }
    Ok(())
}

fn edge_representations(edge: &Edge) -> Checked<Vec<Handle<dyn BRepCurveRepresentation>>> {
    let owner = edge.tshape().downcast::<BRepTEdge>();
    let storage = owner
        .get()
        .ok_or_else(|| representation(edge.tshape(), "BRep_TEdge"))?;
    Ok(storage.curves())
}

fn spatial_carrier(owner: &Handle<dyn GeomCurve>, range: [f64; 2]) -> Checked<Line3> {
    if let Some(line) = owner.downcast_ref::<GeomLine>() {
        return Ok(line.lin());
    }
    if let Some(trim) = owner.downcast_ref::<GeomTrimmedCurve>() {
        let basis = trim.basis_curve();
        if let Some(line) = basis.downcast_ref::<GeomLine>()
            && contains([trim.first_parameter(), trim.last_parameter()], range)
        {
            return Ok(line.lin());
        }
    }
    Err(representation(owner, "Geom_Line covering the full range"))
}

fn planar_carrier(owner: &Handle<dyn Geom2dCurve>, range: [f64; 2]) -> Checked<Line2> {
    if let Some(line) = owner.downcast_ref::<Geom2dLine>() {
        return Ok(line.lin2d());
    }
    if let Some(trim) = owner.downcast_ref::<Geom2dTrimmedCurve>() {
        let basis = trim.basis_curve();
        if let Some(line) = basis.downcast_ref::<Geom2dLine>()
            && contains([trim.first_parameter(), trim.last_parameter()], range)
        {
            return Ok(line.lin2d());
        }
    }
    Err(representation(owner, "Geom2d_Line covering the full range"))
}

fn contains(outer: [f64; 2], inner: [f64; 2]) -> bool {
    outer.into_iter().all(f64::is_finite) && outer[0] <= inner[0] && inner[1] <= outer[1]
}

fn placement(transform: Transform3) -> Checked<SectionPlacement> {
    if transform.scale_factor() != 1. {
        return Err(Cause::PlacementScale(transform.scale_factor()));
    }
    let embedding = captured_placement(transform).map_err(|cause| match cause {
        CapturedGeometryCause::Native(cause) => Cause::from(cause),
        _ => Cause::PrivateCopy,
    })?;
    Ok(std::array::from_fn(|i| {
        let [a, b, c] = embedding.linear[i];
        [a, b, c, embedding.origin[i]]
    }))
}

fn representation<T: Transient + ?Sized>(owner: &Handle<T>, expected: &'static str) -> Cause {
    Cause::Representation {
        actual: owner.get().map(|value| value.dynamic_type().name().into()),
        expected,
    }
}
