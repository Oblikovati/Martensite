//! Numeric copies use held guards and immutable value objects only.
use super::{
    cells::Lease,
    inventory::{Inventory, check_limit},
    types::*,
};
use crate::topods::Shape;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkmath::toploc::Location;
use std::collections::BTreeMap;

pub(super) struct CopyContext {
    owners: BTreeMap<usize, usize>,
    datums: BTreeMap<usize, usize>,
    used: usize,
    maximum: usize,
}
impl CopyContext {
    pub(super) fn new(inventory: &Inventory, maximum: usize) -> Self {
        Self {
            owners: inventory
                .owners
                .iter()
                .enumerate()
                .map(|(i, o)| (o.key(), i))
                .collect(),
            datums: BTreeMap::new(),
            used: 0,
            maximum,
        }
    }
    fn charge(&mut self, count: usize) -> Result<(), FaceCaptureIncomplete> {
        self.used = self
            .used
            .checked_add(count)
            .ok_or(FaceCaptureIncomplete::Limit {
                actual: usize::MAX,
                maximum: self.maximum,
                dimension: "entries",
            })?;
        check_limit(self.used, self.maximum, "entries")
    }
    fn reference<T: RuntimeTyped + ?Sized>(
        &self,
        handle: &Handle<T>,
    ) -> Result<usize, FaceCaptureIncomplete> {
        self.owners
            .get(&(handle.object_address() as usize))
            .copied()
            .ok_or(FaceCaptureIncomplete::Changed { owner: 0 })
    }
    pub(super) fn occurrence(
        &mut self,
        shape: &Shape,
    ) -> Result<CapturedOccurrence, FaceCaptureIncomplete> {
        Ok(CapturedOccurrence {
            owner: self.reference(shape.tshape())?,
            location: self.location(shape.location())?,
            orientation: shape.orientation(),
        })
    }
    fn location(&mut self, location: &Location) -> Result<CapturedLocation, FaceCaptureIncomplete> {
        let mut cursor = location;
        let mut steps = Vec::new();
        while !cursor.is_identity() {
            self.charge(1)?;
            let handle = cursor.first_datum().map_err(|_| changed_location())?;
            let datum = handle.get().ok_or_else(changed_location)?;
            let next = self.datums.len();
            let identity = *self
                .datums
                .entry(handle.object_address() as usize)
                .or_insert(next);
            let power = cursor.first_power().map_err(|_| changed_location())?;
            super::numeric::validate_transform(datum.transformation())?;
            steps.push((identity, power, *datum.transformation()));
            cursor = cursor.next_location();
        }
        completed_location(steps, location)
    }
    pub(super) fn cell(
        &mut self,
        lease: &Lease<'_>,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        self.charge(1)?;
        match lease {
            Lease::Topology(f) => self.topology(f.state, &f.shapes),
            Lease::Face(f) => self.face_fields(f),
            Lease::Edge(f) => self.edge(f.tolerance, f.flags, &f.curves),
            Lease::Vertex(f) => Ok(CapturedFaceCell::Vertex {
                point: finite3([f.point.x(), f.point.y(), f.point.z()], "vertex point")?,
                tolerance: finite(f.tolerance, "vertex tolerance")?,
            }),
            _ => self.representation(lease),
        }
    }
    fn face_fields(
        &mut self,
        fields: &crate::brep::tface::TFaceFields,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        Ok(CapturedFaceCell::Face {
            surface: self.reference(&fields.surface)?,
            location: self.location(&fields.location)?,
            tolerance: finite(fields.tolerance, "face tolerance")?,
            natural_restriction: fields.natural_restriction,
        })
    }
    fn topology(
        &mut self,
        state: u16,
        shapes: &[Shape],
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        self.charge(shapes.len())?;
        let children = shapes
            .iter()
            .map(|shape| self.occurrence(shape))
            .collect::<Result<_, _>>()?;
        Ok(CapturedFaceCell::Topology { state, children })
    }
    fn edge(
        &mut self,
        tolerance: f64,
        flags: i32,
        curves: &[Handle<dyn crate::brep::BRepCurveRepresentation>],
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        self.charge(curves.len())?;
        let representations = curves
            .iter()
            .map(|curve| self.reference(curve))
            .collect::<Result<_, _>>()?;
        Ok(CapturedFaceCell::Edge {
            tolerance: finite(tolerance, "edge tolerance")?,
            flags,
            representations,
        })
    }
    fn representation(
        &mut self,
        lease: &Lease<'_>,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        match lease {
            Lease::Spatial(c) => Ok(CapturedFaceCell::Spatial {
                curve: self.reference(&**c)?,
            }),
            Lease::PCurve(f) => self.pcurve_fields(f),
            Lease::Seam(f) => self.seam_fields(f),
            Lease::Range(r) => Ok(CapturedFaceCell::Range(finite2(
                [r.0, r.1],
                "representation range",
            )?)),
            Lease::Location(l) => Ok(CapturedFaceCell::Location(self.location(l)?)),
            _ => self.geometry(lease),
        }
    }
    fn pcurve_fields(
        &self,
        fields: &crate::brep::curve_on_surface::CurveOnSurfaceFields,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        Ok(CapturedFaceCell::Parametric {
            curve: self.reference(&fields.pcurve)?,
            surface: self.reference(&fields.surface)?,
            endpoints: [
                finite2([fields.uv1.x(), fields.uv1.y()], "pcurve start")?,
                finite2([fields.uv2.x(), fields.uv2.y()], "pcurve end")?,
            ],
        })
    }
    fn seam_fields(
        &self,
        fields: &crate::brep::curve_on_closed_surface::ClosedSurfaceFields,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        Ok(CapturedFaceCell::Seam {
            curve: self.reference(&fields.pcurve2)?,
            continuity: fields.continuity,
            endpoints: [
                finite2([fields.uv21.x(), fields.uv21.y()], "seam start")?,
                finite2([fields.uv22.x(), fields.uv22.y()], "seam end")?,
            ],
        })
    }
    fn geometry(&mut self, lease: &Lease<'_>) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        match lease {
            Lease::Line(l) => super::numeric::line_input(l.axis()),
            Lease::PlanarLine(l) => super::numeric::planar_line_input(l.axis()),
            Lease::PlanarSpline(s) => self.planar_spline(s),
            Lease::Spline(s) => self.spline(s),
            Lease::Plane(p) => super::numeric::plane_input(p.position()),
            Lease::Extrusion(s) => Ok(CapturedFaceCell::Extrusion {
                basis: self.reference(&s.basis())?,
                direction: finite3(s.direction(), "extrusion direction")?,
            }),
            _ => Err(FaceCaptureIncomplete::Changed { owner: 0 }),
        }
    }
    fn spline(
        &mut self,
        lease: &martensite_tkg3d::geom::SplineInputLease<'_>,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        for count in lease.stored_sizes() {
            self.charge(count)?;
        }
        let input = lease.stored_input();
        super::numeric::validate_spline(&input)?;
        Ok(CapturedFaceCell::Spline(input))
    }

    fn planar_spline(
        &mut self,
        lease: &martensite_tkg2d::geom2d::PlanarSplineInputLease<'_>,
    ) -> Result<CapturedFaceCell, FaceCaptureIncomplete> {
        for count in lease.stored_sizes() {
            self.charge(count)?;
        }
        let input = lease.stored_input();
        super::numeric::validate_planar_spline(&input)?;
        Ok(CapturedFaceCell::PlanarSpline(input))
    }
}
fn changed_location() -> FaceCaptureIncomplete {
    FaceCaptureIncomplete::Unsupported {
        owner: 0,
        actual: "invalid datum chain".into(),
        expected: "non-null immutable located datum",
    }
}
pub(super) fn finite(value: f64, field: &'static str) -> Result<f64, FaceCaptureIncomplete> {
    if !value.is_finite() {
        return Err(FaceCaptureIncomplete::NonFinite { owner: 0, field });
    }
    Ok(value)
}
fn finite2(values: [f64; 2], field: &'static str) -> Result<[f64; 2], FaceCaptureIncomplete> {
    for value in values {
        finite(value, field)?;
    }
    Ok(values)
}
fn finite3(values: [f64; 3], field: &'static str) -> Result<[f64; 3], FaceCaptureIncomplete> {
    for value in values {
        finite(value, field)?;
    }
    Ok(values)
}

fn completed_location(
    steps: Vec<(usize, i32, martensite_tkmath::gp::Transform3)>,
    location: &Location,
) -> Result<CapturedLocation, FaceCaptureIncomplete> {
    // The immutable cached map is the evaluator input; exact recomposition can differ (#213).
    let effective = location.transformation();
    super::numeric::validate_transform(&effective)?;
    Ok(CapturedLocation { steps, effective })
}
