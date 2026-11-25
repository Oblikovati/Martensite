use super::*;
use martensite_tkbrep::topods;
use martensite_tkbrep::{
    bintools::{BinCurve2dSet, BinCurveSet, BinOStream, BinSurfaceSet},
    brep::{
        BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
        BRepPointRepresentation, BRepTEdge, BRepTFace, BRepTVertex,
    },
};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkg3d::geom::{
    GeomBezierSurface, GeomCircle, GeomCurve, GeomLine, GeomPlane, GeomSurface,
};
use martensite_tkmath::gp::Point2;
use martensite_tkmath::{gp::Transform3, toploc::Location};

pub(super) type LocatedBits = (LocationBits, Vec<u8>);
pub(super) type Checked<T> = Result<T, Failure>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct LocationBits {
    composite: [u64; 14],
    datums: Vec<(i32, [u64; 14])>,
}

pub(super) fn location_bits(location: &Location) -> Checked<LocationBits> {
    let composite = transform_bits(&location.transformation());
    let mut datums = Vec::new();
    let mut chain = location;
    while !chain.is_identity() {
        let datum = chain.first_datum()?;
        datums.push((chain.first_power()?, transform_bits(datum.transformation())));
        chain = chain.next_location();
    }
    Ok(LocationBits { composite, datums })
}

fn transform_bits(transform: &Transform3) -> [u64; 14] {
    let mut bits = [0; 14];
    bits[0] = transform.scale_factor().to_bits();
    bits[1..4].copy_from_slice(&transform.translation_part().coordinates().map(f64::to_bits));
    for (i, row) in transform.matrix().coefficients().iter().enumerate() {
        bits[4 + 3 * i..7 + 3 * i].copy_from_slice(&row.map(f64::to_bits));
    }
    bits[13] = transform.form() as u64;
    bits
}

fn capture<W>(writer: W) -> Checked<Vec<u8>>
where
    W: FnOnce(&mut BinOStream<'_>) -> Checked<()>,
{
    let mut stream = IoStream::from_bytes(Vec::new());
    writer(&mut BinOStream::new(&mut stream))?;
    let mut data =
        vec![0; usize::try_from(stream.tellp()).map_err(|_| Failure::new("binary geometry seek"))?];
    stream.seekg(0).read(&mut data);
    if stream.fail() {
        return Err(Failure::new("binary geometry round trip"));
    }
    Ok(data)
}

fn surface_bits(surface: &Handle<dyn GeomSurface>) -> Checked<Vec<u8>> {
    let side = surface.downcast::<GeomBezierSurface>();
    if !side.is_null() {
        if side.has_eval_representation() {
            return Err(Failure::new("active side evaluator"));
        }
    } else if surface.downcast::<GeomConicalSurface>().is_null()
        && surface.downcast::<GeomPlane>().is_null()
    {
        return Err(Failure::new("unsupported face surface"));
    }
    capture(|out| BinSurfaceSet::write_surface(surface, out))
}

fn curve_bits(curve: &Handle<dyn GeomCurve>) -> Checked<Vec<u8>> {
    let bezier = curve.downcast::<GeomBezierCurve>();
    if !bezier.is_null() {
        if bezier.has_eval_representation() {
            return Err(Failure::new("active edge evaluator"));
        }
    } else if curve.downcast::<GeomLine>().is_null() && curve.downcast::<GeomCircle>().is_null() {
        return Err(Failure::new("unsupported space curve"));
    }
    capture(|out| BinCurveSet::write_curve(curve, out))
}

fn uv_bits(curve: &Handle<dyn Geom2dCurve>) -> Checked<Vec<u8>> {
    let bezier = curve.downcast::<Geom2dBezierCurve>();
    let spline = curve.downcast::<Geom2dBSplineCurve>();
    if (!bezier.is_null() && bezier.has_eval_representation())
        || (!spline.is_null() && spline.has_eval_representation())
    {
        return Err(Failure::new("active pcurve evaluator"));
    }
    if bezier.is_null()
        && spline.is_null()
        && curve.downcast::<Geom2dLine>().is_null()
        && curve.downcast::<Geom2dCircle>().is_null()
    {
        return Err(Failure::new("unsupported UV curve"));
    }
    capture(|out| BinCurve2dSet::write_curve2d(curve, out))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FaceGeometry {
    pub orientation: Orientation,
    pub carrier: LocatedBits,
    pub tolerance: u64,
    pub natural_restriction: bool,
}

pub(super) fn face_geometry(face: &topods::Face) -> Checked<FaceGeometry> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let tface = face.tshape().downcast::<BRepTFace>();
    Ok(FaceGeometry {
        orientation: face.orientation(),
        carrier: (location_bits(&location)?, surface_bits(&surface)?),
        tolerance: tface.tolerance().to_bits(),
        natural_restriction: tface.natural_restriction(),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FaceTrace {
    range: [u64; 2],
    stored: bool,
    uv_points: [[u64; 2]; 2],
    uv_curve: Vec<u8>,
}

pub(super) fn face_trace(face: &topods::Face, edge: &topods::Edge) -> Checked<FaceTrace> {
    let found = BRepTool::curve_on_face(edge, face)?;
    let (first, last) = BRepTool::uv_points_on_face(edge, face)?;
    Ok(FaceTrace {
        range: [found.first, found.last].map(f64::to_bits),
        stored: found.is_stored,
        uv_points: [first, last].map(|p| [p.x().to_bits(), p.y().to_bits()]),
        uv_curve: uv_bits(&found.pcurve)?,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct EdgeGeometry {
    pub occurrence: LocationBits,
    pub flags: [bool; 3],
    pub tolerance: u64,
    pub representations: Vec<Representation>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Representation {
    Space {
        placed_curve: LocatedBits,
        range: [u64; 2],
    },
    Surface {
        surface: LocatedBits,
        range: [u64; 2],
        uv: Vec<u8>,
        endpoints: [[u64; 2]; 2],
        seam: Option<(Vec<u8>, u32, [[u64; 2]; 2])>,
    },
    Regularity {
        sides: [LocatedBits; 2],
        continuity: u32,
    },
}

pub(super) struct AddedSupport<'a> {
    pub carrier: &'a Handle<dyn GeomSurface>,
    pub relative: Location,
}

pub(super) fn edge_geometry(
    edge: &topods::Edge,
    allowed: Option<AddedSupport<'_>>,
) -> Checked<EdgeGeometry> {
    let raw = edge.tshape().downcast::<BRepTEdge>();
    let mut representations = Vec::new();
    let mut extras = 0;
    let curves = raw.curves();
    let any_after = curves.len();
    for (i, candidate) in curves.into_iter().enumerate() {
        let rep = candidate
            .get()
            .ok_or_else(|| Failure::new("null edge representation"))?;
        if let Some(match_) = &allowed
            && rep.is_curve_on_surface_at(match_.carrier, &match_.relative)
        {
            let spline = rep.pcurve()?.downcast::<Geom2dBSplineCurve>();
            if i + 1 != any_after || rep.is_curve_on_closed_surface() || spline.is_null() {
                return Err(Failure::new(
                    "added stock representation must be one open cubic",
                ));
            }
            extras += 1;
            continue;
        }
        representations.push(representation(&candidate, rep)?);
    }
    if extras != usize::from(allowed.is_some()) {
        return Err(Failure::new("expected exactly one additional stock pcurve"));
    }
    Ok(EdgeGeometry {
        occurrence: location_bits(edge.location())?,
        flags: [raw.same_parameter(), raw.same_range(), raw.degenerated()],
        tolerance: raw.tolerance().to_bits(),
        representations,
    })
}

fn representation(
    handle: &Handle<dyn BRepCurveRepresentation>,
    rep: &dyn BRepCurveRepresentation,
) -> Checked<Representation> {
    let place = location_bits(&rep.location())?;
    if rep.is_regularity() {
        return Ok(Representation::Regularity {
            sides: [
                (place, surface_bits(&rep.surface()?)?),
                (
                    location_bits(&rep.location2()?)?,
                    surface_bits(&rep.surface2()?)?,
                ),
            ],
            continuity: rep.continuity()? as u32,
        });
    }
    let concrete = handle.cast::<dyn BRepGCurve>();
    let geometric = concrete
        .get()
        .ok_or_else(|| Failure::new("unsupported edge representation"))?;
    let (a, b) = geometric.range();
    let range = [a, b].map(f64::to_bits);
    if rep.is_curve3d() {
        return Ok(Representation::Space {
            placed_curve: (place, curve_bits(&rep.curve3d()?)?),
            range,
        });
    }
    if rep.is_curve_on_surface() {
        let uv_pair =
            |pair: (Point2, Point2)| [pair.0, pair.1].map(|p| [p.x().to_bits(), p.y().to_bits()]);
        let (endpoints, seam) = if rep.is_curve_on_closed_surface() {
            let closed = handle.downcast::<BRepCurveOnClosedSurface>();
            let other = (
                uv_bits(&rep.pcurve2()?)?,
                rep.continuity()? as u32,
                uv_pair(closed.uv_points2()),
            );
            (uv_pair(closed.uv_points()), Some(other))
        } else {
            let open = handle.downcast::<BRepCurveOnSurface>();
            (uv_pair(open.uv_points()), None)
        };
        return Ok(Representation::Surface {
            surface: (place, surface_bits(&rep.surface()?)?),
            range,
            uv: uv_bits(&rep.pcurve()?)?,
            endpoints,
            seam,
        });
    }

    Err(Failure::new("unrecognized geometric edge representation"))
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VertexGeometry {
    pub occurrence: LocationBits,
    pub location: [u64; 3],
    pub tolerance: u64,
    pub point_representations: Vec<PointRepresentation>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PointRepresentation {
    Curve(LocatedBits, u64),
    SurfaceCurve(LocatedBits, Vec<u8>, u64),
    Surface(LocatedBits, [u64; 2]),
}

pub(super) fn vertex_geometry(vertex: &topods::Vertex) -> Checked<VertexGeometry> {
    let raw = vertex.tshape().downcast::<BRepTVertex>();
    let point_representations = raw
        .points()
        .iter()
        .map(|handle| {
            let point = handle
                .get()
                .ok_or_else(|| Failure::new("null vertex point representation"))?;
            point_representation(point)
        })
        .collect::<Checked<Vec<_>>>()?;
    Ok(VertexGeometry {
        occurrence: location_bits(vertex.location())?,
        location: raw.pnt().coordinates().map(f64::to_bits),
        tolerance: raw.tolerance().to_bits(),
        point_representations,
    })
}

fn point_representation(point: &dyn BRepPointRepresentation) -> Checked<PointRepresentation> {
    let location = location_bits(&point.location())?;
    let parameter = point.parameter().to_bits();
    if point.is_point_on_curve() {
        return Ok(PointRepresentation::Curve(
            (location, curve_bits(&point.curve()?)?),
            parameter,
        ));
    }
    let surface = (location, surface_bits(&point.surface()?)?);
    if point.is_point_on_curve_on_surface() {
        return Ok(PointRepresentation::SurfaceCurve(
            surface,
            uv_bits(&point.pcurve()?)?,
            parameter,
        ));
    }
    if point.is_point_on_surface() {
        return Ok(PointRepresentation::Surface(
            surface,
            [parameter, point.parameter2()?.to_bits()],
        ));
    }
    Err(Failure::new("unsupported point representation"))
}
