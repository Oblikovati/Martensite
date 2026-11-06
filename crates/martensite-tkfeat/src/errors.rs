//! The reference exceptions of martensite-tkfeat as `Failure` values, each naming the member and the
//! expected state.
use martensite_tkernel::standard::Failure;

/// `not_done` of a member called before a successful computation;
/// `not_done("CSIntersector::nb_points")`.
pub(crate) fn not_done(member: &str) -> Failure {
    Failure::with_message(
        "StdFail_NotDone",
        &format!("{member}: the algorithm is not done; expected a successful Perform first"),
    )
}

/// `out_of_range` of a 1-based index outside `[1, length]`;
/// `out_of_range("CSIntersector::point", 3, 2)`.
pub(crate) fn out_of_range(member: &str, index: i32, length: i32) -> Failure {
    Failure::out_of_range(&format!(
        "{member}: index {index}; expected an index in [1, {length}]"
    ))
}

/// `construction_error` of a member given unusable input;
/// `construction_error("CSIntersector::perform", "null shape")`.
pub(crate) fn construction_error(member: &str, detail: &str) -> Failure {
    Failure::with_message("Standard_ConstructionError", &format!("{member}: {detail}"))
}

/// `null_object` of a member dereferencing a null handle or shape;
/// `null_object("LocOpe::sample_edges", "edge without 3D curve")`.
pub(crate) fn null_object(member: &str, detail: &str) -> Failure {
    Failure::with_message("Standard_NullObject", &format!("{member}: {detail}"))
}

/// `no_such_object` of a lookup that found nothing;
/// `no_such_object("Gluer::descendant_faces", "face not bound")`.
pub(crate) fn no_such_object(member: &str, detail: &str) -> Failure {
    Failure::with_message("Standard_NoSuchObject", &format!("{member}: {detail}"))
}

/// `domain_error` of a member called in a state that does not allow it;
/// `domain_error("Generator::perform", "no generated shape")`.
pub(crate) fn domain_error(member: &str, detail: &str) -> Failure {
    Failure::with_message("Standard_DomainError", &format!("{member}: {detail}"))
}
