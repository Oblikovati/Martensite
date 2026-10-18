//! The pole and weight storage shared by the T-Bezier and AHT-Bezier curves of
//! Geom2dEval and GeomEval: the owning array copy of `myPoles(thePoles)` and the
//! weight checks of the rational constructors, which the four classes run in the
//! same order.
use crate::geom2d::errors;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;

/// An owning copy that keeps the source bounds, as the Array1 copy
/// constructor; `let stored: Array1<'static, f64> = owned(&weights);`.
pub fn owned<T: Clone>(source: &Array1<'_, T>) -> Array1<'static, T> {
    let mut copy: Array1<'static, T> = Array1::default();
    copy.assign(source);
    copy
}

/// a construction-error failure unless there is one weight per pole and every weight
/// is > 0, checked from `Lower()` upwards; `check_weights("GeomEvalTBezierCurve", &w, 3)?`.
pub fn check_weights(class: &str, weights: &Array1<'_, f64>, count: usize) -> Result<(), Failure> {
    if weights.len() != count {
        return Err(curve_construction(
            class,
            if class == "GeomEval_AHTBezierCurve" {
                "weights array size must match poles array size"
            } else {
                "weights size must equal poles size"
            },
            &format!("{} weights; expected {count} (one per pole)", weights.len()),
        ));
    }
    let lower: i32 = weights.lower();
    for (offset, &weight) in weights.as_slice().iter().enumerate() {
        if weight <= 0.0 {
            let index: i64 = i64::from(lower) + offset as i64;
            return Err(curve_construction(
                class,
                "all weights must be > 0",
                &format!("weight({index}) = {weight}; expected every weight > 0"),
            ));
        }
    }
    Ok(())
}

/// Preserve the distinct 3D constructor contract without changing 2D diagnostics;
/// `curve_construction(CLASS, "alpha must be > 0", "alpha = 0; expected > 0")`.
pub(crate) fn curve_construction(class: &str, native: &str, detail: &str) -> Failure {
    if matches!(class, "GeomEval_TBezierCurve" | "GeomEval_AHTBezierCurve") {
        return Failure::with_message("Standard_ConstructionError", &format!("{class}: {native}"));
    }
    errors::construction(&format!("{class}: {detail}"))
}

#[cfg(test)]
mod constructor_contracts {
    use super::curve_construction;
    #[test]
    fn planar_constructor_diagnostic_is_unchanged() {
        let failure = curve_construction(
            "Geom2dEval_TBezierCurve",
            "native reason",
            "3 weights; expected 5 (one per pole)",
        );
        assert_eq!(failure.exception_type(), "Standard_ConstructionError");
        assert_eq!(
            failure.message(),
            "Geom2dEval_TBezierCurve: 3 weights; expected 5 (one per pole)"
        );
    }
    #[test]
    fn spatial_constructor_uses_literal_reason() {
        let failure = curve_construction(
            "GeomEval_TBezierCurve",
            "alpha must be > 0",
            "alpha = 0; expected > 0",
        );
        assert_eq!(failure.exception_type(), "Standard_ConstructionError");
        assert_eq!(
            failure.message(),
            "GeomEval_TBezierCurve: alpha must be > 0"
        );
    }
}
