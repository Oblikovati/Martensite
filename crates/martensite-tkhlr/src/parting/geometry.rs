// SPDX-License-Identifier: GPL-2.0-only
//! Certified constant planar signs and checked detached native edge carriers.
use super::{
    PartingCarrier, PartingCurve, PartingEntity, PartingFaceUse, PartingFailureKind, PartingOrigin,
    cylinder,
    failure::{Checked, native},
    topology::SourceFace,
};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topods::{Edge, Face},
};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    geom_abs::{CurveType, SurfaceType},
    gp::{Direction3, Plane3, Point3, Transform3},
    predicates::{OrientationSign, compare_scalar_triple_product_3d, orient2d},
};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FaceSign {
    Negative,
    Zero,
    Positive,
    Varying,
}

pub(super) struct FaceGeometry {
    pub surface: BRepAdaptorSurface,
    pub kind: SurfaceType,
}

impl FaceGeometry {
    pub fn read(face: &SourceFace) -> Checked<Self> {
        let entity = PartingEntity::Face(face.id);
        let forward = Face::unchecked(face.face.oriented(Orientation::Forward));
        let surface = BRepAdaptorSurface::with_face(&forward, false).map_err(native(entity))?;
        check_placement(surface.transformed_surface().trsf(), entity)?;
        let kind = surface.get_type().map_err(native(entity))?;
        Ok(Self { surface, kind })
    }

    pub fn evaluate(
        &self,
        face: &Face,
        trace: PartingFaceUse,
        pull: Direction3,
    ) -> Checked<(FaceSign, Vec<PartingCurve>)> {
        let entity = PartingEntity::Face(trace.id);
        match self.kind {
            SurfaceType::Plane => {
                let plane = self.surface.plane().map_err(native(entity))?;
                let frame = plane.position();
                if parallel(frame.x_direction(), frame.y_direction(), entity)? {
                    return Err(PartingFailureKind::PlaneFrame(trace.id));
                }
                let sign = plane_sign(plane, pull, trace.orientation, entity)?;
                Ok((sign, Vec::new()))
            }
            SurfaceType::Cylinder => cylinder::evaluate(face, trace, pull),
            surface => Err(PartingFailureKind::UnsupportedFace {
                face: trace.id,
                surface,
            }),
        }
    }
}

pub(super) fn plane_sign(
    plane: Plane3,
    pull: Direction3,
    orientation: Orientation,
    entity: PartingEntity,
) -> Checked<FaceSign> {
    // The surface normal is ∂u × ∂v. Using its stored parameter directions
    // also preserves indirect frames and avoids rounding the cross product.
    let frame = plane.position();
    let order = compare_scalar_triple_product_3d(
        direction(pull),
        direction(frame.x_direction()),
        direction(frame.y_direction()),
    )
    .map_err(|cause| PartingFailureKind::Coordinate { entity, cause })?;
    let order = if orientation == Orientation::Reversed {
        order.reverse()
    } else {
        order
    };
    Ok(match order {
        Ordering::Less => FaceSign::Negative,
        Ordering::Equal => FaceSign::Zero,
        Ordering::Greater => FaceSign::Positive,
    })
}

pub(super) fn edge_curve(
    edge: &Edge,
    id: super::PartingEdgeId,
    origin: PartingOrigin,
) -> Checked<PartingCurve> {
    let entity = PartingEntity::Edge(id);
    let stored = BRepTool::curve_with_location(edge).map_err(native(entity))?;
    check_placement(stored.location.transformation(), entity)?;
    let adaptor = BRepAdaptorCurve::with_edge(edge).map_err(native(entity))?;
    let range = [
        adaptor.first_parameter().map_err(native(entity))?,
        adaptor.last_parameter().map_err(native(entity))?,
    ];
    if stored.curve.is_null() || BRepTool::degenerated(edge).map_err(native(entity))? {
        return Err(PartingFailureKind::CurveDomain { entity, range });
    }
    let carrier = match adaptor.get_type().map_err(native(entity))? {
        CurveType::Line => PartingCarrier::Line(adaptor.line().map_err(native(entity))?),
        CurveType::Circle => PartingCarrier::Circle(adaptor.circle().map_err(native(entity))?),
        curve => return Err(PartingFailureKind::UnsupportedEdge { edge: id, curve }),
    };
    checked_curve(carrier, range, origin, entity)
}

pub(super) fn checked_curve(
    carrier: PartingCarrier,
    range: [f64; 2],
    origin: PartingOrigin,
    entity: PartingEntity,
) -> Checked<PartingCurve> {
    let curve = PartingCurve {
        carrier,
        range,
        origin,
    };
    if !range.into_iter().all(f64::is_finite) || range[0] >= range[1] || !regular(carrier) {
        return Err(PartingFailureKind::CurveDomain { entity, range });
    }
    let ends = curve.endpoints();
    if ends.into_iter().all(finite_point)
        && (!matches!(carrier, PartingCarrier::Line(_)) || ends[0] != ends[1])
    {
        return Ok(curve);
    }
    Err(PartingFailureKind::CurveDomain { entity, range })
}

fn regular(carrier: PartingCarrier) -> bool {
    match carrier {
        PartingCarrier::Line(line) => {
            finite_point(line.location())
                && direction(line.direction()) != [0.; 3]
                && direction(line.direction()).into_iter().all(f64::is_finite)
        }
        PartingCarrier::Circle(circle) => {
            let frame = circle.position();
            circle.radius().is_finite()
                && circle.radius() > 0.
                && finite_point(circle.location())
                && direction(frame.x_direction())
                    .into_iter()
                    .all(f64::is_finite)
                && direction(frame.y_direction())
                    .into_iter()
                    .all(f64::is_finite)
        }
    }
}

pub(super) fn check_placement(transform: Transform3, entity: PartingEntity) -> Checked<()> {
    let scale = transform.scale_factor();
    let mirrored = transform.is_negative();
    if scale == 1.
        && !mirrored
        && (1..=3)
            .all(|row| (1..=4).all(|column| transform.value(row, column).is_ok_and(f64::is_finite)))
    {
        return Ok(());
    }
    Err(PartingFailureKind::Placement {
        entity,
        scale,
        mirrored,
    })
}

pub(super) fn parallel(a: Direction3, b: Direction3, entity: PartingEntity) -> Checked<bool> {
    let a = direction(a);
    let b = direction(b);
    for [u, v] in [[0, 1], [0, 2], [1, 2]] {
        let sign = orient2d([0.; 2], [a[u], a[v]], [b[u], b[v]])
            .map_err(|cause| PartingFailureKind::Coordinate { entity, cause })?;
        if sign != OrientationSign::Zero {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn direction(direction: Direction3) -> [f64; 3] {
    [direction.x(), direction.y(), direction.z()]
}

pub(super) fn point(point: Point3) -> [f64; 3] {
    [point.x(), point.y(), point.z()]
}

fn finite_point(value: Point3) -> bool {
    point(value).into_iter().all(f64::is_finite)
}
