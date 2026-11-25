//! The existing directed row primitive consumes the wide strip's local charts.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::cone::prepare::directed;

#[test]
fn wide_strip_finalizes_directed_rows_across_distinct_chart_centers() {
    let request = strip_request();
    let run = ConeRun::from(&request);
    let inputs = input::capture(&request, &mut || false).unwrap();
    let original = &inputs.originals[0];
    let mut work = Work::new(request.proof.arithmetic);
    let mut prepared = prepare_local(&run, original, None, &mut work);
    let centers = prepared
        .fits
        .values()
        .map(|fit| fit.fit.view().chart_center.to_bits())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(centers.len() > 1);
    let before = work.consumed;
    let refinement = ConicalPrimitiveRefinement::UnsplitV1;
    directed::finalize_rows(
        &inputs.originals,
        std::slice::from_mut(&mut prepared),
        refinement,
        &mut work,
        &mut || false,
    )
    .unwrap();
    assert!(work.consumed > before);
    check_transitions(original, &prepared, refinement);
    check_decoded(original, &prepared, request.support, &mut work);
}

fn check_transitions(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    refinement: ConicalPrimitiveRefinement,
) {
    let family = prepared.directed_family.as_ref().unwrap();
    assert_eq!(family.gamma.primitive_refinement(), refinement);
    assert!(family.gamma.gamma().into_iter().all(|bias| bias > 0.));
    for piece in prepared.source.loops.iter().flatten() {
        let fit = &prepared.fits[&piece.span];
        let transition = fit.transition.as_ref().unwrap();
        let domain = directed::chart(prepared, piece.span, fit).unwrap();
        assert!(domain.source_restriction().is_some());
        let query = storage::prepared_query(original, prepared, piece, &fit.poles).unwrap();
        assert!(
            transition
                .boundary()
                .matches_chart_with_refinement(&query, domain, refinement)
        );
    }
}

fn check_decoded(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    support: ConeSupport,
    work: &mut Work,
) {
    let body = decode_member(prepared, support);
    let stored = storage::capture(original, prepared, body, work, &mut || false).unwrap();
    storage::bind_cap_joins(original, prepared, &stored).unwrap();
    for span in &stored.spans {
        let transition = prepared.fits[&span.span].transition.as_ref().unwrap();
        assert!(std::sync::Arc::ptr_eq(
            &span.boundary,
            &transition.boundary_shared()
        ));
    }
}
