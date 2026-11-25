//! These controls check diagnostic projection, not geometric proof generation.
use super::*;
use martensite_tkmath::predicates::roots::{DyadicBound, metric_bounds::BoundedArithmeticFailure};
fn inventory() -> prepare::directed::Inventory {
    let sides = [(7, 9, 11), (8, 13, 12), (99, 99, 99)].map(|(region, curve, id)| {
        (
            (
                SurfaceRegionId(region),
                CurveSpan {
                    curve: SurfaceCurveId(curve),
                    index: 0,
                    level: 0,
                },
            ),
            ConicalActualSideId(id),
        )
    });
    prepare::directed::Inventory {
        sides: BTreeMap::from(sides),
        caps: BTreeMap::from([
            ((SurfaceRegionId(21), 0), ConicalActualCapId(31)),
            ((SurfaceRegionId(22), 1), ConicalActualCapId(32)),
        ]),
    }
}
fn error(cause: ConicalActualPatchFailure) -> ConicalActualPatchError {
    ConicalActualPatchError {
        stage: ConicalActualPatchStage::Hierarchy,
        side: None,
        cap: None,
        operations: 17,
        options: Default::default(),
        cause: Box::new(cause),
    }
}
fn side_pair(cause: ConicalActualPatchFailure) -> ConicalActualPatchFailure {
    ConicalActualPatchFailure::SidePair {
        sides: [ConicalActualSideId(11), ConicalActualSideId(12)],
        cause: Box::new(cause),
    }
}
fn project(cause: ConicalActualPatchFailure) -> WrappedNormalFailure {
    let failure = prepare::directed::actual_failure(error(cause), &inventory());
    let source = find_source::<ConicalActualPatchError>(&failure).unwrap();
    assert_eq!(source.operations, 17);
    failure.public()
}
#[test]
fn named_side_pair_preserves_each_owner_and_nested_failure_class() {
    for (cause, expected) in arithmetic_causes() {
        let failure = project(side_pair(ConicalActualPatchFailure::Arithmetic(cause)));
        assert_eq!(failure.class(), expected);
        assert_eq!(failure.entities(), side_owners());
    }
}
fn arithmetic_causes() -> [(BoundedArithmeticFailure, WrappedFailureClass); 3] {
    [
        (
            BoundedArithmeticFailure::WorkLimit {
                completed: 4,
                limit: 4,
            },
            WrappedFailureClass::Incomplete,
        ),
        (
            BoundedArithmeticFailure::NonFiniteScalar { value: f64::NAN },
            WrappedFailureClass::InvalidInput,
        ),
        (
            BoundedArithmeticFailure::Cancelled,
            WrappedFailureClass::Cancelled,
        ),
    ]
}
fn side_owners() -> [HistorySource; 4] {
    [
        HistorySource::Region(SurfaceRegionId(7)),
        HistorySource::Region(SurfaceRegionId(8)),
        HistorySource::Curve(SurfaceCurveId(9)),
        HistorySource::Curve(SurfaceCurveId(13)),
    ]
}
#[test]
fn nested_cap_pair_retains_only_its_named_regions_and_inner_side_owners() {
    let cause = ConicalActualPatchFailure::CapPair {
        caps: [ConicalActualCapId(31), ConicalActualCapId(32)],
        cause: Box::new(side_pair(ConicalActualPatchFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled,
        ))),
    };
    let failure = project(cause);
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert_eq!(
        failure.entities(),
        [
            HistorySource::Region(SurfaceRegionId(7)),
            HistorySource::Region(SurfaceRegionId(8)),
            HistorySource::Region(SurfaceRegionId(21)),
            HistorySource::Region(SurfaceRegionId(22)),
            HistorySource::Curve(SurfaceCurveId(9)),
            HistorySource::Curve(SurfaceCurveId(13))
        ]
    );
}
#[test]
fn unresolved_named_side_pair_is_incomplete_with_original_hulls_retained() {
    let failure = project(unresolved_pair());
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert_eq!(failure.entities(), side_owners());
    assert!(matches!(
        find_source::<ConicalActualPatchError>(&failure)
            .unwrap()
            .cause
            .as_ref(),
        ConicalActualPatchFailure::UnresolvedSidePair {
            expected: "strict whole-hull separation",
            ..
        }
    ));
}

fn unresolved_pair() -> ConicalActualPatchFailure {
    let zero = DyadicBound::from_f64(0.).unwrap();
    let bounds: [[DyadicBound; 3]; 2] =
        std::array::from_fn(|_| std::array::from_fn(|_| zero.clone()));
    ConicalActualPatchFailure::UnresolvedSidePair {
        sides: [ConicalActualSideId(11), ConicalActualSideId(12)],
        lower: Box::new(bounds.clone()),
        upper: Box::new(bounds),
        expected: "strict whole-hull separation",
    }
}
