//! Checks every source-declared Boolean alert runtime identity and merge policy.
use martensite_tkbo::bop_algo::*;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::message::{Alert, AlertHandle};
use martensite_tkmath::gp::Point3;
use std::sync::Arc;

fn alert_vertex() -> Shape {
    let builder = BRepBuilder::default();
    let mut vertex = Vertex::default();
    builder
        .make_vertex_at(&mut vertex, Point3::new(1.0, 2.0, 3.0), 1e-7)
        .unwrap();
    vertex.into()
}

fn assert_alert_identity(alert: &dyn Alert, name: &str, parent: &str, merge: bool) {
    assert_eq!(alert.get_message_key().as_bytes(), name.as_bytes());
    assert_eq!(alert.dynamic_type().name(), name);
    assert_eq!(alert.dynamic_type().parent().unwrap().name(), parent);
    assert_eq!(alert.supports_merge(), merge);
}

macro_rules! simple_alert_case {
    ($test:ident, $alert:ident, $name:literal) => {
        #[test]
        fn $test() {
            let alert = $alert;
            assert_alert_identity(&alert, $name, "Message_Alert", true);
            let target: AlertHandle = Arc::new($alert);
            assert!(alert.merge(&target));
        }
    };
}

macro_rules! shape_alert_case {
    ($test:ident, $alert:ident, $name:literal) => {
        #[test]
        fn $test() {
            let shape = alert_vertex();
            let alert = $alert::new(&shape);
            assert_alert_identity(&alert, $name, "TopoDS_AlertWithShape", false);
            assert!(alert.get_shape().is_same(&shape));
            let target: AlertHandle = Arc::new($alert::new(&shape));
            assert!(!alert.merge(&target));
            alert.set_shape(&Shape::default());
            assert!(alert.get_shape().is_null());
        }
    };
}

simple_alert_case!(alert_user_break, AlertUserBreak, "BOPAlgoAlertUserBreak");
simple_alert_case!(
    alert_b_o_p_not_allowed,
    AlertBOPNotAllowed,
    "BOPAlgoAlertBOPNotAllowed"
);
simple_alert_case!(alert_b_o_p_not_set, AlertBOPNotSet, "BOPAlgoAlertBOPNotSet");
simple_alert_case!(
    alert_builder_failed,
    AlertBuilderFailed,
    "BOPAlgoAlertBuilderFailed"
);
simple_alert_case!(
    alert_intersection_failed,
    AlertIntersectionFailed,
    "BOPAlgoAlertIntersectionFailed"
);
simple_alert_case!(
    alert_multiple_arguments,
    AlertMultipleArguments,
    "BOPAlgoAlertMultipleArguments"
);
simple_alert_case!(alert_no_filler, AlertNoFiller, "BOPAlgoAlertNoFiller");
simple_alert_case!(
    alert_null_input_shapes,
    AlertNullInputShapes,
    "BOPAlgoAlertNullInputShapes"
);
simple_alert_case!(
    alert_post_treat_f_f,
    AlertPostTreatFF,
    "BOPAlgoAlertPostTreatFF"
);
simple_alert_case!(
    alert_solid_builder_failed,
    AlertSolidBuilderFailed,
    "BOPAlgoAlertSolidBuilderFailed"
);
simple_alert_case!(
    alert_too_few_arguments,
    AlertTooFewArguments,
    "BOPAlgoAlertTooFewArguments"
);
shape_alert_case!(
    alert_bad_positioning,
    AlertBadPositioning,
    "BOPAlgoAlertBadPositioning"
);
shape_alert_case!(alert_empty_shape, AlertEmptyShape, "BOPAlgoAlertEmptyShape");
shape_alert_case!(
    alert_not_splittable_edge,
    AlertNotSplittableEdge,
    "BOPAlgoAlertNotSplittableEdge"
);
shape_alert_case!(
    alert_removal_of_i_b_for_edges_failed,
    AlertRemovalOfIBForEdgesFailed,
    "BOPAlgoAlertRemovalOfIBForEdgesFailed"
);
shape_alert_case!(
    alert_removal_of_i_b_for_faces_failed,
    AlertRemovalOfIBForFacesFailed,
    "BOPAlgoAlertRemovalOfIBForFacesFailed"
);
shape_alert_case!(
    alert_removal_of_i_b_for_m_dim_shapes,
    AlertRemovalOfIBForMDimShapes,
    "BOPAlgoAlertRemovalOfIBForMDimShapes"
);
shape_alert_case!(
    alert_removal_of_i_b_for_solids_failed,
    AlertRemovalOfIBForSolidsFailed,
    "BOPAlgoAlertRemovalOfIBForSolidsFailed"
);
shape_alert_case!(
    alert_self_interfering_shape,
    AlertSelfInterferingShape,
    "BOPAlgoAlertSelfInterferingShape"
);
shape_alert_case!(
    alert_shell_splitter_failed,
    AlertShellSplitterFailed,
    "BOPAlgoAlertShellSplitterFailed"
);
shape_alert_case!(
    alert_too_small_edge,
    AlertTooSmallEdge,
    "BOPAlgoAlertTooSmallEdge"
);
shape_alert_case!(
    alert_intersection_of_pair_of_shapes_failed,
    AlertIntersectionOfPairOfShapesFailed,
    "BOPAlgoAlertIntersectionOfPairOfShapesFailed"
);
shape_alert_case!(
    alert_building_p_curve_failed,
    AlertBuildingPCurveFailed,
    "BOPAlgoAlertBuildingPCurveFailed"
);
shape_alert_case!(
    alert_acquired_self_intersection,
    AlertAcquiredSelfIntersection,
    "BOPAlgoAlertAcquiredSelfIntersection"
);
shape_alert_case!(
    alert_unsupported_type,
    AlertUnsupportedType,
    "BOPAlgoAlertUnsupportedType"
);
simple_alert_case!(
    alert_no_faces_to_remove,
    AlertNoFacesToRemove,
    "BOPAlgoAlertNoFacesToRemove"
);
shape_alert_case!(
    alert_unable_to_remove_the_feature,
    AlertUnableToRemoveTheFeature,
    "BOPAlgoAlertUnableToRemoveTheFeature"
);
simple_alert_case!(
    alert_remove_features_failed,
    AlertRemoveFeaturesFailed,
    "BOPAlgoAlertRemoveFeaturesFailed"
);
shape_alert_case!(
    alert_solid_builder_unused_faces,
    AlertSolidBuilderUnusedFaces,
    "BOPAlgoAlertSolidBuilderUnusedFaces"
);
shape_alert_case!(
    alert_face_builder_unused_edges,
    AlertFaceBuilderUnusedEdges,
    "BOPAlgoAlertFaceBuilderUnusedEdges"
);
shape_alert_case!(
    alert_unable_to_orient_the_shape,
    AlertUnableToOrientTheShape,
    "BOPAlgoAlertUnableToOrientTheShape"
);
shape_alert_case!(
    alert_unknown_shape,
    AlertUnknownShape,
    "BOPAlgoAlertUnknownShape"
);
simple_alert_case!(
    alert_no_periodicity_required,
    AlertNoPeriodicityRequired,
    "BOPAlgoAlertNoPeriodicityRequired"
);
shape_alert_case!(
    alert_unable_to_trim,
    AlertUnableToTrim,
    "BOPAlgoAlertUnableToTrim"
);
shape_alert_case!(
    alert_unable_to_make_identical,
    AlertUnableToMakeIdentical,
    "BOPAlgoAlertUnableToMakeIdentical"
);
shape_alert_case!(
    alert_unable_to_repeat,
    AlertUnableToRepeat,
    "BOPAlgoAlertUnableToRepeat"
);
simple_alert_case!(
    alert_multi_dimensional_arguments,
    AlertMultiDimensionalArguments,
    "BOPAlgoAlertMultiDimensionalArguments"
);
shape_alert_case!(
    alert_unable_to_make_periodic,
    AlertUnableToMakePeriodic,
    "BOPAlgoAlertUnableToMakePeriodic"
);
shape_alert_case!(
    alert_unable_to_glue,
    AlertUnableToGlue,
    "BOPAlgoAlertUnableToGlue"
);
shape_alert_case!(
    alert_shape_is_not_periodic,
    AlertShapeIsNotPeriodic,
    "BOPAlgoAlertShapeIsNotPeriodic"
);
shape_alert_case!(
    alert_unable_to_make_closed_edge_on_face,
    AlertUnableToMakeClosedEdgeOnFace,
    "BOPAlgoAlertUnableToMakeClosedEdgeOnFace"
);
