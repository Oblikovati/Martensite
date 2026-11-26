//! The edge curve queries of `BRep_Tool`: the 3D curve, the 3D polygon and the p-curves.
use super::{BRepTool, EdgeCurve3D, EdgeCurveOnSurface, EdgePCurve};
use crate::brep::access::edge_tshape;
use crate::brep::builder::update_curves::as_gcurve;
use crate::brep::{BRepCurve3D, BRepCurveRepresentation, BRepGCurve};
use crate::topods::{EdgeView, FaceView};
use martensite_tkernel::standard::{Failure, Handle, project_concrete, project_interface};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::poly::Polygon3D;
use martensite_tkmath::toploc::Location;

/// The representations of an edge, in list order.
pub(super) fn representations(
    edge: &EdgeView<'_>,
    member: &str,
) -> Result<Vec<Handle<dyn BRepCurveRepresentation>>, Failure> {
    Ok(edge_tshape(edge, member)?.curves())
}

/// The first representation accepted by `matches`, as a parametric curve.
pub(super) fn first_gcurve(
    edge: &EdgeView<'_>,
    member: &str,
    matches: impl Fn(&dyn BRepCurveRepresentation) -> bool,
) -> Result<Option<Handle<dyn BRepGCurve>>, Failure> {
    let found = edge_tshape(edge, member)?.with_curves(|curves| {
        curves
            .iter()
            .find(|representation| representation.get().is_some_and(&matches))
            .map(as_gcurve)
    });
    Ok(found)
}

impl BRepTool {
    /// `Curve(E, L, First, Last)`: the first 3D curve representation, with the edge
    /// location composed with its location; `BRepTool::curve_with_location(&edge)?`.
    pub fn curve_with_location(edge: &EdgeView<'_>) -> Result<EdgeCurve3D, Failure> {
        let tedge = edge_tshape(edge, "BRep_Tool::Curve")?;
        tedge.with_curves(|curves| -> Result<EdgeCurve3D, Failure> {
            for representation in curves {
                let Some(r) = representation.get() else {
                    continue;
                };
                if !r.is_curve3d() {
                    continue;
                }
                let (curve, rep_loc, (first, last)) =
                    if let Some(c3d) = project_concrete::<BRepCurve3D>(r) {
                        (c3d.curve3d()?, c3d.location(), c3d.range())
                    } else if let Some(gcurve) = project_interface::<dyn BRepGCurve>(r) {
                        (gcurve.curve3d()?, gcurve.location(), gcurve.range())
                    } else {
                        return Ok(EdgeCurve3D::default());
                    };
                let location = if edge.location().is_identity() {
                    rep_loc
                } else if rep_loc.is_identity() {
                    edge.location().clone()
                } else {
                    (edge.location() * &rep_loc)?
                };
                return Ok(EdgeCurve3D {
                    curve,
                    location,
                    first,
                    last,
                });
            }
            Ok(EdgeCurve3D::default())
        })
    }

    /// `Curve(E, First, Last)`: the 3D curve, transformed into a copy when its location
    /// is not the identity; `let (curve, first, last) = BRepTool::curve(&edge)?;`.
    pub fn curve(edge: &EdgeView<'_>) -> Result<(Handle<dyn GeomCurve>, f64, f64), Failure> {
        let found = Self::curve_with_location(edge)?;
        let Some(curve) = found.curve.get() else {
            return Ok((found.curve.clone(), found.first, found.last));
        };
        if found.location.is_identity() {
            return Ok((found.curve.clone(), found.first, found.last));
        }
        let copy = curve.transformed(found.location.transformation())?;
        Ok((copy.cast::<dyn GeomCurve>(), found.first, found.last))
    }

    /// `IsGeometric(E)`: the edge has a non-null 3D curve or a curve on surface (the
    /// search stops at the first of the two kinds that answers yes);
    /// `BRepTool::is_geometric_edge(&edge)?`.
    pub fn is_geometric_edge(edge: &EdgeView<'_>) -> Result<bool, Failure> {
        let found = edge_tshape(edge, "BRep_Tool::IsGeometric")?.with_curves(|curves| {
            for representation in curves {
                let Some(r) = representation.get() else {
                    continue;
                };
                if r.is_curve3d() {
                    let curve3d = representation.downcast::<BRepCurve3D>();
                    if curve3d
                        .get()
                        .is_some_and(|c| c.curve3d().is_ok_and(|c| c.is_some()))
                    {
                        return true;
                    }
                } else if r.is_curve_on_surface() {
                    return true;
                }
            }
            false
        });
        Ok(found)
    }

    /// `Polygon3D(E, L)`: the first 3D polygon and the composed location, or a null
    /// polygon at the identity; `BRepTool::polygon3d(&edge)?`.
    pub fn polygon3d(edge: &EdgeView<'_>) -> Result<(Handle<Polygon3D>, Location), Failure> {
        let found = edge_tshape(edge, "BRep_Tool::Polygon3D")?.with_curves(|curves| {
            curves
                .iter()
                .find(|r| r.get().is_some_and(|r| r.is_polygon3d()))
                .cloned()
        });
        if let Some(representation) = found
            && let Some(r) = representation.get()
        {
            return Ok((r.polygon3d()?, (edge.location() * &r.location())?));
        }
        Ok((Handle::default(), Location::default()))
    }

    /// `CurveOnSurface(E, F, First, Last, theIsStored)`: the p-curve on the face's
    /// surface, for the edge reversed when the face is REVERSED;
    /// `BRepTool::curve_on_face(&edge, &face)?`.
    pub fn curve_on_face(edge: &EdgeView<'_>, face: &FaceView<'_>) -> Result<EdgePCurve, Failure> {
        let (surface, location) = Self::surface_with_location(face)?;
        let mut local = edge.to_edge();
        if face.orientation() == Orientation::Reversed {
            local.reverse();
        }
        Self::curve_on_surface(&local, &surface, &location)
    }

    /// `CurveOnSurface(E, S, L, First, Last, theIsStored)`: the stored p-curve on
    /// (`surface`, `location`), the second one of a seam for a REVERSED edge; when none
    /// is stored, the planar projection of `CurveOnPlane` (`is_stored` false);
    /// `BRepTool::curve_on_surface(&edge, &surface, &location)?`.
    pub fn curve_on_surface(
        edge: &EdgeView<'_>,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<EdgePCurve, Failure> {
        let tedge = edge_tshape(edge, "BRep_Tool::CurveOnSurface")?;
        let found: Option<Handle<dyn BRepCurveRepresentation>> = tedge.with_curves(
            |curves| -> Result<Option<Handle<dyn BRepCurveRepresentation>>, Failure> {
                if curves.is_empty() {
                    return Ok(None);
                }
                let relative = location.predivided(edge.location())?;
                Ok(curves
                    .iter()
                    .find(|r| {
                        r.get()
                            .is_some_and(|r| r.is_curve_on_surface_at(surface, &relative))
                    })
                    .cloned())
            },
        )?;
        if let Some(representation) = found {
            return stored_pcurve(&representation, edge.orientation() == Orientation::Reversed);
        }
        let (pcurve, first, last) = Self::curve_on_plane(edge, surface, location)?;
        Ok(EdgePCurve {
            pcurve,
            first,
            last,
            is_stored: false,
        })
    }

    /// `CurveOnSurface(E, C, S, L, First, Last)`: the first curve on surface of the
    /// edge; `BRepTool::first_curve_on_surface(&edge)?`.
    pub fn first_curve_on_surface(edge: &EdgeView<'_>) -> Result<EdgeCurveOnSurface, Failure> {
        let found = first_gcurve(edge, "BRep_Tool::CurveOnSurface", |r| {
            r.is_curve_on_surface()
        })?;
        let Some(gcurve) = found.as_ref().and_then(|g| g.get()) else {
            return Ok(EdgeCurveOnSurface::default());
        };
        curve_on_surface_of(edge, gcurve, gcurve.pcurve()?)
    }

    /// `CurveOnSurface(E, C, S, L, First, Last, Index)`: the p-curve of rank `index`,
    /// counting both p-curves of a seam. An index below 1 returns the null result
    /// here; natively the outputs are left unchanged;
    /// `BRepTool::curve_on_surface_by_index(&edge, 1)?`.
    pub fn curve_on_surface_by_index(
        edge: &EdgeView<'_>,
        index: i32,
    ) -> Result<EdgeCurveOnSurface, Failure> {
        if index < 1 {
            return Ok(EdgeCurveOnSurface::default());
        }
        let mut rank = 0;
        for representation in representations(edge, "BRep_Tool::CurveOnSurface")? {
            let Some(r) = representation.get().filter(|r| r.is_curve_on_surface()) else {
                continue;
            };
            let gcurve = as_gcurve(&representation);
            let Some(gcurve) = gcurve.get() else { continue };
            rank += 1;
            if rank == index {
                return curve_on_surface_of(edge, gcurve, r.pcurve()?);
            }
            if r.is_curve_on_closed_surface() {
                rank += 1;
                if rank == index {
                    return curve_on_surface_of(edge, gcurve, r.pcurve2()?);
                }
            }
        }
        Ok(EdgeCurveOnSurface::default())
    }
}

/// The p-curve and range of a found curve on surface: the second p-curve of a seam
/// for a REVERSED edge.
fn stored_pcurve(
    representation: &Handle<dyn BRepCurveRepresentation>,
    reversed: bool,
) -> Result<EdgePCurve, Failure> {
    let gcurve = as_gcurve(representation);
    let Some(gcurve) = gcurve.get() else {
        return Ok(EdgePCurve::default());
    };
    let (first, last) = gcurve.range();
    let pcurve = if gcurve.is_curve_on_closed_surface() && reversed {
        gcurve.pcurve2()?
    } else {
        gcurve.pcurve()?
    };
    Ok(EdgePCurve {
        pcurve,
        first,
        last,
        is_stored: true,
    })
}

/// The outputs of the `CurveOnSurface(E, C, S, L, ...)` overloads for a found curve.
fn curve_on_surface_of(
    edge: &EdgeView<'_>,
    gcurve: &dyn BRepGCurve,
    pcurve: Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
) -> Result<EdgeCurveOnSurface, Failure> {
    let (first, last) = gcurve.range();
    Ok(EdgeCurveOnSurface {
        pcurve,
        surface: gcurve.surface()?,
        location: (edge.location() * &gcurve.location())?,
        first,
        last,
    })
}
