//! One unchanged geometric query is shared by sequential and ranged SD dispatch.
use crate::bop_tools::are_faces_same_domain;
use crate::int_tools::Context;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::standard::Failure;

/// Preserve actual first/second face and fuzzy evaluation; `same_domain_pair(a, b, context, fuzzy)?`.
pub(super) fn same_domain_pair(
    first: &Shape,
    second: &Shape,
    context: &Context,
    fuzzy: f64,
) -> Result<bool, Failure> {
    let first_face = Face::unchecked(first.clone());
    are_faces_same_domain(
        &first_face,
        &Face::unchecked(second.clone()),
        context,
        fuzzy,
    )
}
