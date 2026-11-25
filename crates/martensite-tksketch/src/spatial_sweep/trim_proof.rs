// SPDX-License-Identifier: GPL-2.0-only
//! Outward integration of the final solid's actual directed face trims.
mod model;
mod plane;
mod revolution;
mod trim;

use super::{
    SweepFailure as Failure, SweepStage as Stage,
    admission::Admission,
    lineage, native, poll,
    solid::{SolidSweepImage, SolidSweepOptions},
};
use crate::spatial_prism::volume::interval::{Point, Span};
use martensite_tkbo::profile_extrusion::{IntegrationMethod, Measured};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Face},
};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomRectangularTrimmedSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    FrameTransportInput, certify_frame_transport_at_angle,
};
use std::collections::BTreeMap;

pub(super) fn certify(
    images: &[SolidSweepImage],
    reference: [f64; 3],
    options: SolidSweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Measured, Failure> {
    let mut sum = Span::ZERO;
    let mut trig = BTreeMap::new();
    for (index, image) in images.iter().enumerate() {
        poll(cancelled, Stage::Integration)?;
        if native(image.shape.shape_type(), Stage::Integration)? != ShapeEnum::Face {
            continue;
        }
        let face = native(topods::face(&image.shape), Stage::Integration)?;
        let context = FaceProof {
            index,
            reference,
            limit: options.sweep.source_trace_limit,
        };
        sum = sum + flux(&face, context, &mut trig, cancelled)?;
    }
    report(sum * inverse(3.), options.volume_error_limit)
}

pub(super) fn verify_lateral(
    admission: &Admission,
    face: &lineage::Face,
    revolved: bool,
    index: usize,
    limit: f64,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    let proof = FaceProof {
        index,
        reference: admission.stations[0].xyz,
        limit,
    };
    let face_context = native(
        topods::face(&face.shape.oriented(Orientation::Forward)),
        Stage::Binding,
    )?;
    let (surface, location) = native(
        BRepTool::surface_with_location(&face_context),
        Stage::Binding,
    )?;
    if !location.is_identity() {
        return Err(proof.refuse("retained supports in the operation world frame"));
    }
    let boundary = trim::ordered(&face_context, proof, cancelled)?;
    if revolved {
        let held = surface
            .downcast_ref::<GeomRectangularTrimmedSurface>()
            .ok_or_else(|| proof.refuse("one retained rectangular quarter-revolution face"))?;
        return revolution::verify_model(held, &boundary, proof, trig, cancelled);
    }
    let plane = surface
        .downcast_ref::<GeomPlane>()
        .ok_or_else(|| proof.refuse("one retained planar face on its authored band or corner"))?;
    let ideal = model::planar(admission, face)?;
    plane::verify_model(
        plane.pln().position(),
        &boundary,
        ideal,
        proof,
        trig,
        cancelled,
    )
}

#[derive(Clone, Copy)]
struct FaceProof {
    index: usize,
    reference: [f64; 3],
    limit: f64,
}

impl FaceProof {
    fn refuse(self, expected: &'static str) -> Failure {
        Failure::FaceTrim {
            face: self.index,
            expected,
        }
    }
}

fn flux(
    face: &Face,
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let sign = match face.orientation() {
        Orientation::Forward => Span::ONE,
        Orientation::Reversed => Span::at(-1.),
        _ => return Err(proof.refuse("a forward or reversed oriented final face occurrence")),
    };
    let forward = native(
        topods::face(&face.oriented(Orientation::Forward)),
        Stage::Integration,
    )?;
    let (surface, location) = native(
        BRepTool::surface_with_location(&forward),
        Stage::Integration,
    )?;
    if !location.is_identity() {
        return Err(proof.refuse("native face placement in the bounded world frame"));
    }
    let trims = trim::ordered(&forward, proof, cancelled)?;
    let integral = if let Some(plane) = surface.downcast_ref::<GeomPlane>() {
        plane::flux(plane.pln().position(), &trims, proof, trig, cancelled)?
    } else if let Some(surface) = surface.downcast_ref::<GeomRectangularTrimmedSurface>() {
        revolution::flux(surface, &trims, proof, trig, cancelled)?
    } else {
        return Err(proof.refuse("literal planar or bounded rectangular quarter-revolution face"));
    };
    Ok(sign * integral)
}

fn report(bounded: Span, limit: f64) -> Result<Measured, Failure> {
    if !bounded.finite() || bounded.lo <= 0. {
        return Err(Failure::Volume(
            "a finite, strictly positive oriented physical volume",
        ));
    }
    let value = bounded.lo + 0.5 * (bounded.hi - bounded.lo);
    let bound = (value - bounded.lo).max(bounded.hi - value).next_up();
    if !bound.is_finite() || bound > limit {
        return Err(Failure::VolumeErrorBound { bound, limit });
    }
    Ok(Measured {
        value,
        error_bound: bound,
        method: IntegrationMethod::CertifiedEnclosure,
    })
}

pub(super) type Trig = BTreeMap<u64, [Span; 2]>;

fn coefficients(trig: &mut Trig, radians: f64) -> Result<[Span; 2], Failure> {
    let key = radians.to_bits();
    if let Some(&cached) = trig.get(&key) {
        return Ok(cached);
    }
    let cardinal = FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    };
    let proof = certify_frame_transport_at_angle(cardinal, radians)
        .map_err(|cause| Failure::Trigonometry(Box::new(cause)))?;
    let report = proof.rotation();
    let [cosine, sine] = [report.cosine_bounds, report.sine_bounds].map(|[lo, hi]| Span { lo, hi });
    trig.insert(key, [cosine, sine]);
    Ok([cosine, sine])
}

fn inverse(integer: f64) -> Span {
    let reciprocal = 1. / integer;
    Span {
        lo: reciprocal.next_down(),
        hi: reciprocal.next_up(),
    }
}

fn dot(a: Point, b: Point) -> Span {
    a.into_iter()
        .zip(b)
        .fold(Span::ZERO, |total, (x, y)| total + x * y)
}

fn scaled(point: Point, factor: Span) -> Point {
    point.map(|x| x * factor)
}

fn offset(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] - b[i])
}

fn point_add(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] + b[i])
}
