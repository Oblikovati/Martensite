//! Stored representation admission never substitutes sampled geometry.
use super::{copy::finite, types::FaceCaptureIncomplete as Incomplete};
use martensite_tkg2d::geom2d::StoredPlanarSplineInput;
use martensite_tkg3d::geom::StoredSplineInput;

pub(super) fn validate_spline(input: &StoredSplineInput) -> Result<(), Incomplete> {
    validate_layout(input)?;
    let mults = input.multiplicities();
    if mults.len() < 2 || mults.first() != Some(&3) || mults.last() != Some(&3) {
        return Err(unsupported("unclamped quadratic spline"));
    }
    if !mults
        .iter()
        .skip(1)
        .take(mults.len() - 2)
        .all(|m| (1..=2).contains(m))
    {
        return Err(unsupported("unsupported interior knot multiplicity"));
    }
    finite_spline(input)
}
fn validate_layout(input: &StoredSplineInput) -> Result<(), Incomplete> {
    if input.evaluation_override()
        || input.periodic()
        || input.degree() != 2
        || input.lower_bounds() != [1; 5]
    {
        return Err(unsupported("unsupported stored spline layout or evaluator"));
    }
    Ok(())
}

fn finite_spline(input: &StoredSplineInput) -> Result<(), Incomplete> {
    for pole in input.poles() {
        for value in pole {
            finite(*value, "spline pole")?;
        }
    }
    for value in input
        .weights()
        .iter()
        .chain(input.knots())
        .chain(input.flat_knots())
        .chain(input.domain().iter())
    {
        finite(*value, "spline weight or knot")?;
    }
    Ok(())
}

pub(super) fn validate_planar_spline(input: &StoredPlanarSplineInput) -> Result<(), Incomplete> {
    if input.evaluation_override() {
        return Err(unsupported_planar(
            "active planar spline evaluation override",
        ));
    }
    for pole in input.poles() {
        for value in pole {
            finite(*value, "planar spline pole")?;
        }
    }
    for value in input
        .weights()
        .iter()
        .chain(input.knots())
        .chain(input.flat_knots())
        .chain(input.domain().iter())
    {
        finite(*value, "planar spline weight or knot")?;
    }
    Ok(())
}

fn unsupported_planar(actual: &str) -> Incomplete {
    // The descriptor can reference an independently mutable evaluator graph.
    Incomplete::Unsupported {
        owner: 0,
        actual: actual.into(),
        expected: "stored planar B-spline without an external evaluator override",
    }
}

pub(super) fn plane_input(
    axis: martensite_tkmath::gp::Axis3,
) -> Result<super::types::CapturedFaceCell, Incomplete> {
    let origin = axis.location();
    let x = axis.x_direction();
    let y = axis.y_direction();
    let normal = axis.direction();
    for value in [
        origin.x(),
        origin.y(),
        origin.z(),
        x.x(),
        x.y(),
        x.z(),
        y.x(),
        y.y(),
        y.z(),
        normal.x(),
        normal.y(),
        normal.z(),
    ] {
        finite(value, "plane frame")?;
    }
    Ok(super::types::CapturedFaceCell::Plane(axis))
}
fn unsupported(actual: &str) -> Incomplete {
    Incomplete::Unsupported {
        owner: 0,
        actual: actual.into(),
        expected: "stored clamped degree-two nonperiodic spline without evaluator override",
    }
}

pub(super) fn validate_transform(
    transform: &martensite_tkmath::gp::Transform3,
) -> Result<(), Incomplete> {
    for row in 1..=3 {
        for column in 1..=4 {
            let value = transform
                .value(row, column)
                .map_err(|_| unsupported("invalid transform matrix"))?;
            finite(value, "location transform")?;
        }
    }
    Ok(())
}
pub(super) fn line_input(
    axis: martensite_tkmath::gp::Axis1,
) -> Result<super::types::CapturedFaceCell, Incomplete> {
    let p = axis.location();
    let d = axis.direction();
    for value in [p.x(), p.y(), p.z(), d.x(), d.y(), d.z()] {
        finite(value, "spatial line")?;
    }
    Ok(super::types::CapturedFaceCell::Line(axis))
}
pub(super) fn planar_line_input(
    axis: martensite_tkmath::gp::Axis2d,
) -> Result<super::types::CapturedFaceCell, Incomplete> {
    let p = axis.location();
    let d = axis.direction();
    for value in [p.x(), p.y(), d.x(), d.y()] {
        finite(value, "planar line")?;
    }
    Ok(super::types::CapturedFaceCell::PlanarLine(axis))
}
