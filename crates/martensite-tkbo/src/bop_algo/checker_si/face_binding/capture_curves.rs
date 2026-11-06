// SPDX-License-Identifier: GPL-2.0-only
//! Numeric copies, not live handles or delegated evaluator assumptions.
use super::types::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomLine};
use martensite_tkmath::gp::Transform3;

pub(super) fn placement(transform: Transform3) -> Result<AffinePlacement, BindingFailure> {
    let captured = crate::sketch_region_geometry_capture::captured_placement(transform)
        .map_err(|cause| unsupported("affine placement", format!("{cause:?}")))?;
    Ok(AffinePlacement {
        linear: captured.linear,
        origin: captured.origin,
        scale: captured.scale,
    })
}
pub(super) fn space(source: &Handle<dyn GeomCurve>) -> Result<OwnedSpaceCurve, BindingFailure> {
    let curve = source
        .get()
        .ok_or_else(|| unsupported("3D curve", "null"))?
        .copy()?
        .cast::<dyn GeomCurve>();
    if let Some(line) = curve.cast::<GeomLine>().get() {
        let position = line.position();
        let origin = position.location();
        let direction = position.direction();
        return Ok(OwnedSpaceCurve::Line(OwnedLine {
            origin: [origin.x(), origin.y(), origin.z()],
            direction: [direction.x(), direction.y(), direction.z()],
        }));
    }
    let handle = curve.cast::<GeomBSplineCurve>();
    let spline = handle
        .get()
        .ok_or_else(|| unsupported("3D curve", format!("{:?}", curve.dynamic_type())))?;
    Ok(OwnedSpaceCurve::Spline(spline_values(spline)?))
}
pub(super) fn spline_values(curve: &GeomBSplineCurve) -> Result<OwnedSpline, BindingFailure> {
    if curve.has_eval_representation() {
        return Err(unsupported("spline evaluator", "delegated override"));
    }
    let poles = (1..=curve.nb_poles())
        .map(|i| curve.pole(i).map(|p| [p.x(), p.y(), p.z()]))
        .collect::<Result<Vec<_>, _>>()?;
    let weights = (1..=curve.nb_poles())
        .map(|i| curve.weight(i))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(OwnedSpline {
        degree: curve.degree(),
        poles,
        weights,
        knots: curve.knots().as_slice().to_vec(),
        multiplicities: curve.multiplicities().as_slice().to_vec(),
        periodic: curve.is_periodic(),
        domain: [curve.first_parameter(), curve.last_parameter()],
    })
}
pub(super) fn plane(source: &Handle<dyn Geom2dCurve>) -> Result<OwnedLine<2>, BindingFailure> {
    let curve = source
        .get()
        .ok_or_else(|| unsupported("stored pcurve", "null"))?
        .copy()?
        .cast::<dyn Geom2dCurve>();
    let handle = curve.cast::<Geom2dLine>();
    let line = handle.get().ok_or_else(|| {
        unsupported(
            "pcurve map",
            format!("{:?}; expected built-in line", curve.dynamic_type()),
        )
    })?;
    let origin = line.location();
    let direction = line.direction();
    Ok(OwnedLine {
        origin: [origin.x(), origin.y()],
        direction: [direction.x(), direction.y()],
    })
}
pub(super) fn unsupported(role: &'static str, actual: impl Into<String>) -> BindingFailure {
    BindingFailure::Unsupported {
        role,
        actual: actual.into(),
    }
}
