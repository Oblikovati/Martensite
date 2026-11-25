//! Owned analytic surface sketches and physical normal extrusion (#414).
mod admission;
pub mod attachment;
mod cone_admission;
mod cone_faces;
mod cone_math;
mod cone_plane;
mod cone_request;
mod cone_semantic;
mod construction;
mod cylinder_sweep;
mod encoding;
mod failure;
mod frame;
mod full_turn;
mod history;
mod pipeline;
mod request;
mod result;
mod semantic;
mod sweep;
mod tolerance;
mod trim_binding;
mod validation;
mod volume;

#[path = "../wrapped_profile/mod.rs"]
mod wrapped_profile;

pub use cone_request::{ConeSector, ConeSupport};
pub use failure::{SurfaceFailureCause, SurfaceSketchFailure, SurfaceStage};
pub use full_turn::{
    ConeBandHole, CrossedBandResult, CurvedBandResult, CylinderBandHole, FullTurnBand,
    FullTurnSupport, PeriodicConicMap, PeriodicCrossing, PeriodicCrossingId, PeriodicDepth,
    PeriodicImageKey, PeriodicSeamEnd, PeriodicSourceEnd, SheetTransition,
    normal_extrude_cone_band_at_authored_seam, normal_extrude_cone_band_holes_at_authored_seam,
    normal_extrude_cone_band_with_curved_hole, normal_extrude_cone_band_with_curved_holes,
    normal_extrude_cone_band_with_rational_hole, normal_extrude_cylinder_band_at_authored_seam,
    normal_extrude_cylinder_band_holes_at_authored_seam,
    normal_extrude_cylinder_band_with_curved_hole, normal_extrude_cylinder_band_with_curved_holes,
    normal_extrude_cylinder_band_with_hole, normal_extrude_cylinder_band_with_rational_hole,
    normal_extrude_full_turn_band,
};
pub use history::{HistoryKey, HistoryRole, HistorySource};
pub use request::*;
pub use result::{
    DetachedNormalBody, NormalExtrusionResult, SurfaceDiagnostic, VolumeErrorEstimates,
};
pub use wrapped_profile::ConicalStockFailure;
pub use wrapped_profile::{
    CertifiedConeAttachmentReport, CertifiedConeFeatureResult, CertifiedCylinderAttachmentReport,
    CertifiedCylinderFeatureResult, ConeFeatureResult, ConeFootprintFailure, ConeFootprintReport,
    ConeRegionFootprint, ConeTrimmedTool, CylinderBodyRef, CylinderFeatureCause,
    CylinderFeatureFailure, CylinderFeatureHistory, CylinderFeatureKind, CylinderFeatureParent,
    CylinderFeatureRelation, CylinderFeatureResult, CylinderFeatureSource, CylinderFeatureStage,
    CylinderFootprintFailure, CylinderFootprintReport, CylinderTrimmedTool,
    DetachedCylinderFeature, DetachedWrappedBodies, DetachedWrappedFeature, OuterTrimRefusal,
    RationalWrappedFailure, RationalWrappedResult, WrappedAdmittedArcMap,
    WrappedAnalyticArcEndpoint, WrappedAnalyticArcMap, WrappedBudgetReport, WrappedCertifiedBound,
    WrappedCertifiedRange, WrappedChartNormalResult, WrappedChartPlacementReport,
    WrappedConeArithmeticGrantPolicy, WrappedConeArithmeticGrantReceipt,
    WrappedConeAuthoredVolumeOptions, WrappedConeBudgetReport, WrappedConeDomainPolicy,
    WrappedConeExecutionSelection, WrappedConeMaterialReport, WrappedConeMemberReport,
    WrappedConeNormalRequest, WrappedConeNormalResult, WrappedConePlacementReport,
    WrappedConeProofOptions, WrappedConeVolumeBounds, WrappedConicParameterMap,
    WrappedConicParameterization, WrappedEntitySource, WrappedFailureClass, WrappedFeatureFailure,
    WrappedFeatureHistory, WrappedFeatureKind, WrappedHistoryKey, WrappedHistoryRecord,
    WrappedHistoryRelation, WrappedIncidenceCoverage, WrappedMaterialRelation, WrappedMaterialWork,
    WrappedMeasureReport, WrappedMemberReport, WrappedNativeKnot, WrappedNativeVolumeEstimate,
    WrappedNormalFailure, WrappedNormalOptions, WrappedNormalRequest, WrappedNormalResult,
    WrappedOperation, WrappedOutsideMaterial, WrappedPlacedConeMemberReport,
    WrappedPlacedConeResult, WrappedPlacementReport, WrappedRegionSelection, WrappedSourcePoint,
    WrappedSourceSpan, WrappedSpatialReport, WrappedSplineParameterMap, WrappedStage,
    WrappedVolumeBounds, WrappedVolumePolicy, WrappedWindingLocation, WrappedWorldSpatialReport,
    normal_extrude_cone_profile, normal_extrude_cone_profile_on_chart,
    normal_extrude_cone_profile_on_chart_with_cancel, normal_extrude_cone_profile_with_cancel,
    normal_extrude_cone_profile_with_domain_policy,
    normal_extrude_cone_profile_with_domain_policy_and_cancel,
    normal_extrude_cone_profile_with_selection,
    normal_extrude_cone_profile_with_selection_and_cancel, normal_extrude_profile,
    normal_extrude_profile_on_chart, normal_extrude_profile_on_chart_with_cancel,
    normal_extrude_profile_with_cancel, normal_extrude_rational_cone,
    normal_extrude_rational_cone_with_selection, normal_extrude_rational_cylinder,
};

/// Extrude one developed rectangle through an ordered physical normal-depth interval.
/// The returned body is complete, validated and independently owned.
/// Example: `let result = normal_extrude(&rectangle, [-1.0, 2.0], options)?;`.
pub fn normal_extrude(
    rectangle: &CylinderRectangle,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let admitted =
        admission::admit(rectangle, depths, options).map_err(|cause| SurfaceSketchFailure {
            region: rectangle.region,
            stage: SurfaceStage::Admission,
            cause,
        })?;
    let sweep = cylinder_sweep::prepare(&admitted);
    pipeline::build(&sweep, &|bindings| semantic::validate(&admitted, bindings))
}

#[cfg(test)]
mod semantic_tests;

/// Extrude an annular developed cone sector through signed physical normal depths.
/// Example: `let result = normal_extrude_cone(&sector, [-1.0, 2.0], options)?;`.
pub fn normal_extrude_cone(
    sector: &ConeSector,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let admitted =
        cone_admission::admit(sector, depths, options).map_err(|cause| SurfaceSketchFailure {
            region: sector.region,
            stage: SurfaceStage::Admission,
            cause,
        })?;
    pipeline::build(&admitted.sweep, &|bindings| {
        cone_semantic::validate(&admitted, bindings)
    })
}

#[cfg(test)]
mod cone_tests;
