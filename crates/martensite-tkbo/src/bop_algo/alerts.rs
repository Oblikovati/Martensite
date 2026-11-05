//! Typed Boolean failure and warning alerts.
use martensite_tkbrep::define_alert_with_shape;
use martensite_tkernel::define_simple_alert;

define_simple_alert!(AlertUserBreak, "BOPAlgoAlertUserBreak");
define_simple_alert!(AlertBOPNotAllowed, "BOPAlgoAlertBOPNotAllowed");
define_simple_alert!(AlertBOPNotSet, "BOPAlgoAlertBOPNotSet");
define_simple_alert!(AlertBuilderFailed, "BOPAlgoAlertBuilderFailed");
define_simple_alert!(AlertIntersectionFailed, "BOPAlgoAlertIntersectionFailed");
define_simple_alert!(AlertMultipleArguments, "BOPAlgoAlertMultipleArguments");
define_simple_alert!(AlertNoFiller, "BOPAlgoAlertNoFiller");
define_simple_alert!(AlertNullInputShapes, "BOPAlgoAlertNullInputShapes");
define_simple_alert!(AlertPostTreatFF, "BOPAlgoAlertPostTreatFF");
define_simple_alert!(AlertSolidBuilderFailed, "BOPAlgoAlertSolidBuilderFailed");
define_simple_alert!(AlertTooFewArguments, "BOPAlgoAlertTooFewArguments");
define_alert_with_shape!(AlertBadPositioning, "BOPAlgoAlertBadPositioning");
define_alert_with_shape!(AlertEmptyShape, "BOPAlgoAlertEmptyShape");
define_alert_with_shape!(AlertNotSplittableEdge, "BOPAlgoAlertNotSplittableEdge");
define_alert_with_shape!(
    AlertRemovalOfIBForEdgesFailed,
    "BOPAlgoAlertRemovalOfIBForEdgesFailed"
);
define_alert_with_shape!(
    AlertRemovalOfIBForFacesFailed,
    "BOPAlgoAlertRemovalOfIBForFacesFailed"
);
define_alert_with_shape!(
    AlertRemovalOfIBForMDimShapes,
    "BOPAlgoAlertRemovalOfIBForMDimShapes"
);
define_alert_with_shape!(
    AlertRemovalOfIBForSolidsFailed,
    "BOPAlgoAlertRemovalOfIBForSolidsFailed"
);
define_alert_with_shape!(
    AlertSelfInterferingShape,
    "BOPAlgoAlertSelfInterferingShape"
);
define_alert_with_shape!(AlertShellSplitterFailed, "BOPAlgoAlertShellSplitterFailed");
define_alert_with_shape!(AlertTooSmallEdge, "BOPAlgoAlertTooSmallEdge");
define_alert_with_shape!(
    AlertIntersectionOfPairOfShapesFailed,
    "BOPAlgoAlertIntersectionOfPairOfShapesFailed"
);
define_alert_with_shape!(
    AlertBuildingPCurveFailed,
    "BOPAlgoAlertBuildingPCurveFailed"
);
define_alert_with_shape!(
    AlertAcquiredSelfIntersection,
    "BOPAlgoAlertAcquiredSelfIntersection"
);
define_alert_with_shape!(AlertUnsupportedType, "BOPAlgoAlertUnsupportedType");
define_simple_alert!(AlertNoFacesToRemove, "BOPAlgoAlertNoFacesToRemove");
define_alert_with_shape!(
    AlertUnableToRemoveTheFeature,
    "BOPAlgoAlertUnableToRemoveTheFeature"
);
define_simple_alert!(
    AlertRemoveFeaturesFailed,
    "BOPAlgoAlertRemoveFeaturesFailed"
);
define_alert_with_shape!(
    AlertSolidBuilderUnusedFaces,
    "BOPAlgoAlertSolidBuilderUnusedFaces"
);
define_alert_with_shape!(
    AlertFaceBuilderUnusedEdges,
    "BOPAlgoAlertFaceBuilderUnusedEdges"
);
define_alert_with_shape!(
    AlertUnableToOrientTheShape,
    "BOPAlgoAlertUnableToOrientTheShape"
);
define_alert_with_shape!(AlertUnknownShape, "BOPAlgoAlertUnknownShape");
define_simple_alert!(
    AlertNoPeriodicityRequired,
    "BOPAlgoAlertNoPeriodicityRequired"
);
define_alert_with_shape!(AlertUnableToTrim, "BOPAlgoAlertUnableToTrim");
define_alert_with_shape!(
    AlertUnableToMakeIdentical,
    "BOPAlgoAlertUnableToMakeIdentical"
);
define_alert_with_shape!(AlertUnableToRepeat, "BOPAlgoAlertUnableToRepeat");
define_simple_alert!(
    AlertMultiDimensionalArguments,
    "BOPAlgoAlertMultiDimensionalArguments"
);
define_alert_with_shape!(
    AlertUnableToMakePeriodic,
    "BOPAlgoAlertUnableToMakePeriodic"
);
define_alert_with_shape!(AlertUnableToGlue, "BOPAlgoAlertUnableToGlue");
define_alert_with_shape!(AlertShapeIsNotPeriodic, "BOPAlgoAlertShapeIsNotPeriodic");
define_alert_with_shape!(
    AlertUnableToMakeClosedEdgeOnFace,
    "BOPAlgoAlertUnableToMakeClosedEdgeOnFace"
);

/// The `TopoDSAlertWithShape` part of a BOPAlgo alert carrying a shape
/// (`occ::down_cast<TopoDSAlertWithShape>`); `None` for the other alerts.
pub fn alert_with_shape(
    alert: &martensite_tkernel::message::AlertHandle,
) -> Option<&martensite_tkbrep::topods::AlertWithShape> {
    let any: &dyn std::any::Any = alert.as_ref();
    macro_rules! with_shape {
        ($($alert:ident),* $(,)?) => {
            $(if let Some(found) = any.downcast_ref::<$alert>() {
                return Some(&**found);
            })*
        };
    }
    with_shape!(
        AlertBadPositioning,
        AlertEmptyShape,
        AlertNotSplittableEdge,
        AlertRemovalOfIBForEdgesFailed,
        AlertRemovalOfIBForFacesFailed,
        AlertRemovalOfIBForMDimShapes,
        AlertRemovalOfIBForSolidsFailed,
        AlertSelfInterferingShape,
        AlertShellSplitterFailed,
        AlertTooSmallEdge,
        AlertIntersectionOfPairOfShapesFailed,
        AlertBuildingPCurveFailed,
        AlertAcquiredSelfIntersection,
        AlertUnsupportedType,
        AlertUnableToRemoveTheFeature,
        AlertSolidBuilderUnusedFaces,
        AlertFaceBuilderUnusedEdges,
        AlertUnableToOrientTheShape,
        AlertUnknownShape,
        AlertUnableToTrim,
        AlertUnableToMakeIdentical,
        AlertUnableToRepeat,
        AlertUnableToMakePeriodic,
        AlertUnableToGlue,
        AlertShapeIsNotPeriodic,
        AlertUnableToMakeClosedEdgeOnFace,
    );
    any.downcast_ref::<martensite_tkbrep::topods::AlertWithShape>()
}

/// Wraps one alert for a report (`new BOPAlgoAlert...`); `alert(AlertUserBreak)`.
pub(crate) fn alert<T: martensite_tkernel::message::Alert + 'static>(
    value: T,
) -> martensite_tkernel::message::AlertHandle {
    std::sync::Arc::new(value)
}
