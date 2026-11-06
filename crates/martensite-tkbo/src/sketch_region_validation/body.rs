// SPDX-License-Identifier: GPL-2.0-only
//! Declared body invariants precede mapped representation checks.
use super::outcome::{BodyKind, ValidationCause, ValidationStage};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topods::{Shape, face};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;

pub(super) fn body_cause(
    shape: &Shape,
    kind: BodyKind,
    stage: ValidationStage,
) -> Result<Option<ValidationCause>, Failure> {
    let actual = shape.shape_type()?;
    let Some(expected) = expected_shape_kind(kind) else {
        return Ok(None);
    };
    if actual != expected {
        return Ok(Some(ValidationCause::BodyKindMismatch {
            expected: kind,
            actual,
        }));
    }
    if kind == BodyKind::PlanarRegion && stage == ValidationStage::Geometry {
        return plane_cause(shape);
    }
    Ok(None)
}

fn plane_cause(shape: &Shape) -> Result<Option<ValidationCause>, Failure> {
    let surface = BRepAdaptorSurface::with_face(&face(shape)?, false)?;
    if surface.get_type()? == SurfaceType::Plane {
        return Ok(None);
    }
    Ok(Some(ValidationCause::NonPlanarRegion))
}

fn expected_shape_kind(kind: BodyKind) -> Option<ShapeEnum> {
    match kind {
        BodyKind::PlanarRegion => Some(ShapeEnum::Face),
        BodyKind::Solid => Some(ShapeEnum::Solid),
        BodyKind::Any => None,
    }
}
