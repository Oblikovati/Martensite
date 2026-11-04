//! Exact weak source predicates, kept separate from additional diagnostic controls.
use crate::session_run::check_point;

/// The source compares the literal no-hit command reply; `check_no_axis_hits(text)`.
pub(crate) fn check_no_axis_hits(text: &str) {
    if text != "There are no any intersections with this axis." {
        println!("Error: there should be no any intersections");
    }
}

/// A miss reports RealLast's exponent; `check_miss(&point, "box")`.
pub(crate) fn check_miss(point: [f64; 3], label: &str) {
    if !point
        .iter()
        .any(|value| format!("{value:e}").contains("e308"))
    {
        println!("Faulty : Selection of {label} with sensitivity 0");
    }
}

/// Check the original checkpoint coordinates without changing their tolerances; `check_points(&points, &expected, 0.001)`.
pub(crate) fn check_points(points: &[[f64; 3]], expected: &[[f64; 3]], tolerance: f64) {
    for (index, expected) in expected.iter().enumerate() {
        let Some(point) = points.get(index) else {
            println!("Error: missing original intersection point {}", index + 1);
            continue;
        };
        check_point(&format!("point{}", index + 1), *point, *expected, tolerance);
    }
}

/// Preserve the source's conjunction, including its absent-name false case; `weak_hollow_error(&types)`.
pub(crate) fn weak_hollow_error(types: &[String]) -> bool {
    let text = types.join("\n");
    let target = "Select3DSensitiveCylinder";
    !text.contains(target) && text.find(target) != text.rfind(target)
}

/// The original entity-name test over actual ranked sensitives; `has_cylinder(&types)`.
pub(crate) fn has_cylinder(types: &[String]) -> bool {
    let target = "Select3DSensitiveCylinder";
    types.iter().any(|name| name.contains(target))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_hollow_conjunction_is_not_replaced_by_positive_count() {
        assert!(!weak_hollow_error(&[]));
        assert!(!weak_hollow_error(&["Select3DSensitiveCylinder".into()]));
        assert!(!weak_hollow_error(&[
            "Select3DSensitiveCylinder".into(),
            "Select3DSensitiveCylinder".into()
        ]));
    }
}
