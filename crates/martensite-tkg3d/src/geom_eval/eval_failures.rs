//! The failure every GeomEval curve and surface raises from the members that would
//! change or reparameterize it (`Reverse`, `UReverse`, `ReversedParameter`,
//! `Transform`, `UIso`...).
use crate::geom::errors;
use martensite_tkernel::standard::Failure;

/// StandardNotImplemented with the retained member literal; arguments are ignored
/// before any geometry mutation; `unsupported("GeomEvalSineWaveCurve::Reverse", "")`.
pub(crate) fn unsupported(member: &str, _detail: &str) -> Failure {
    // The paraboloid's Transform throws this literal for every transformation.
    // Every audited GeomEval refusal has this same literal-only contract.
    Failure::with_message("Standard_NotImplemented", member)
}

/// retained order validation before reading surface parameters; `check_surface_order(CLASS, 0, 0)?`.
pub(crate) fn check_surface_order(class: &str, nu: i32, nv: i32) -> Result<(), Failure> {
    if nu + nv < 1 || nu < 0 || nv < 0 {
        return Err(surface_order_failure(class));
    }
    Ok(())
}

fn surface_order_failure(class: &str) -> Failure {
    if class == "GeomEval_HyperboloidSurface" {
        // The retained zero-argument exception constructor intentionally has no message.
        return errors::undefined_derivative("");
    }
    let message = format!("{class}::EvalDN: invalid derivative order");
    if matches!(
        class,
        "GeomEval_TBezierSurface" | "GeomEval_AHTBezierSurface"
    ) {
        return Failure::with_message("Standard_RangeError", &message);
    }
    errors::undefined_derivative(&message)
}

/// Preserve retained construction messages and configured stack capture; `construction("")`.
pub(crate) fn construction(message: &str) -> Failure {
    Failure::with_message("Standard_ConstructionError", message)
}
