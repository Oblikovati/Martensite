//! Explicit stored-proxy and required-check choices remain distinct during replay.
use super::*;

/// Select a complete stored-proxy behavior independently of the unchanged request.
/// Example: `let selection = WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1;`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrappedConeExecutionSelection {
    /// Existing rounded proxy and native SI search, with the selected domain policy.
    RoundedProxyNativeSearchV1 {
        /// Existing domain proof, retaining its original construction and replay behavior.
        domain: WrappedConeDomainPolicy,
    },
    /// Directed physical rows, PositiveXCommonChartV2, HalfSplitV1 and required certified SI.
    /// Requires explicitly requested FilteredCompositeBernsteinV5 or GeometricCompositeV6.
    /// Missing or incomplete certification refuses; native search is not a fallback.
    DirectedCommonChartRequireCertifiedV1,
    /// The same directed certified pipeline with PositiveXFamilyChartV3 source capacity.
    /// Numerical/work limits remain requested separately; no limit grows after refusal.
    DirectedFamilyChartRequireCertifiedV2 {
        /// Finite aggregate admission; the request retains the actual requested allowance.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Directed certified pipeline with a center chosen from the complete authored family.
    DirectedRecenteredChartRequireCertifiedV3 {
        /// Finite aggregate admission; the request retains the actual requested allowance.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Independently centered cone regions with certified periodic and stored-solid separation.
    DirectedRegionAtlasRequireCertifiedV4 {
        /// Finite aggregate admission; the request retains the actual requested allowance.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Directed construction with the canonical intersection certificate rebound after placement.
    DirectedTransportedPlacementV5 {
        /// Any of the directed chart domains, V2 through V5.
        domain: WrappedConeDomainPolicy,
        /// Finite aggregate admission; the request retains the actual requested allowance.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Source-bound local charts, decoded cap reference and native member validation.
    /// Requires explicitly requested FilteredCompositeBernsteinV5 or GeometricCompositeV6.
    ConnectedChartAtlasV6 {
        /// Finite aggregate arithmetic admission.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Connected local charts with certified native side and periodic cap validation.
    DirectedConnectedChartAtlasV7 {
        /// Finite aggregate arithmetic admission.
        arithmetic: WrappedConeArithmeticGrantPolicy,
    },
    /// Transported placement with GeometricCompositeV6 for the authored planar selection.
    /// The authored program shares the operation integration allowance with the decoded UV proof.
    DirectedTransportedAuthoredPlanarV8 {
        /// Any of the directed chart domains, V2 through V5.
        domain: WrappedConeDomainPolicy,
        /// Finite aggregate arithmetic admission.
        arithmetic: WrappedConeArithmeticGrantPolicy,
        /// Maximum components in each complete authored integral program.
        authored_max_components: u16,
        /// Maximum authored integration cells, subject to the original operation allowance.
        authored_max_subintervals: u64,
    },
    /// Transported family-chart placement with directed selected-cap proof on retained UV.
    /// Requests use GeometricCompositeV6 limits for the cap proof and retain authored V6.
    DirectedTransportedAuthoredPlanarAndCapCyclesV9 {
        /// Finite aggregate arithmetic admission.
        arithmetic: WrappedConeArithmeticGrantPolicy,
        /// Maximum components in a complete authored integral program.
        authored_max_components: u16,
        /// Original authored integration cap, subject to the operation allowance.
        authored_max_subintervals: u64,
    },
}
impl WrappedConeExecutionSelection {
    pub(in crate::surface_sketch::wrapped_profile) fn is_connected_atlas(self) -> bool {
        matches!(
            self,
            Self::ConnectedChartAtlasV6 { .. } | Self::DirectedConnectedChartAtlasV7 { .. }
        )
    }
    pub(in crate::surface_sketch::wrapped_profile) fn requires_transported_placement(self) -> bool {
        matches!(
            self,
            Self::DirectedTransportedPlacementV5 { .. }
                | Self::DirectedConnectedChartAtlasV7 { .. }
                | Self::DirectedTransportedAuthoredPlanarV8 { .. }
                | Self::DirectedTransportedAuthoredPlanarAndCapCyclesV9 { .. }
        )
    }

    pub(in crate::surface_sketch::wrapped_profile) fn is_directed(self) -> bool {
        matches!(
            self,
            Self::DirectedCommonChartRequireCertifiedV1
                | Self::DirectedFamilyChartRequireCertifiedV2 { .. }
                | Self::DirectedRecenteredChartRequireCertifiedV3 { .. }
                | Self::DirectedRegionAtlasRequireCertifiedV4 { .. }
                | Self::DirectedTransportedPlacementV5 { .. }
                | Self::DirectedConnectedChartAtlasV7 { .. }
                | Self::DirectedTransportedAuthoredPlanarV8 { .. }
                | Self::DirectedTransportedAuthoredPlanarAndCapCyclesV9 { .. }
        )
    }
    pub(in crate::surface_sketch::wrapped_profile) fn uses_directed_cap_cycles(self) -> bool {
        matches!(
            self,
            Self::DirectedTransportedAuthoredPlanarAndCapCyclesV9 { .. }
        )
    }
    pub(in crate::surface_sketch::wrapped_profile) fn authored_planar_program(
        self,
    ) -> Option<martensite_tkbo::profile_extrusion::PlanarProofVersion> {
        let (authored_max_components, authored_max_subintervals) = match self {
            Self::DirectedTransportedAuthoredPlanarV8 {
                authored_max_components,
                authored_max_subintervals,
                ..
            }
            | Self::DirectedTransportedAuthoredPlanarAndCapCyclesV9 {
                authored_max_components,
                authored_max_subintervals,
                ..
            } => (authored_max_components, authored_max_subintervals),
            _ => return None,
        };
        Some(
            martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
                max_components: authored_max_components,
                max_total_subintervals: authored_max_subintervals,
            },
        )
    }
}

impl WrappedConeNormalResult {
    /// Actual directed-mode coverage borrowed from the retained native-bound certificate.
    /// Rounded modes return `None`; counts describe completed checks, not a standalone proof.
    /// Example: `let coverage = result.actual_self_intersection_coverage();`.
    pub fn actual_self_intersection_coverage(
        &self,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::ConicalActualPatchCoverage>
    {
        self.family
            .budget
            .actual_certificate
            .as_ref()
            .map(|issued| issued.certificate().coverage())
    }
    /// Executed proxy/check selection; replay retains it alongside the original request.
    /// Example: `let selection = result.execution_selection();`.
    pub fn execution_selection(&self) -> WrappedConeExecutionSelection {
        self.family.family.selection.executed_selection
    }
}

/// Execute the selected complete cone behavior without changing request defaults.
/// Request FilteredCompositeBernsteinV5 or GeometricCompositeV6 for directed or connected-atlas selections.
/// The directed selected-cap selection requires GeometricCompositeV6 limits.
/// Example: `request.proof.planar_proof = PlanarProofVersion::FilteredCompositeBernsteinV5 { max_components: 64, max_total_subintervals: 100_000 }; normal_extrude_cone_profile_with_selection(&request, selection)?;`.
pub fn normal_extrude_cone_profile_with_selection(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
) -> Checked<WrappedConeNormalResult> {
    normal_extrude_cone_profile_with_selection_and_cancel(request, selection, || false)
}

/// Cancel the selected complete operation without publishing partial geometry.
/// Example: `normal_extrude_cone_profile_with_selection_and_cancel(&request, selection, || stop())`.
pub fn normal_extrude_cone_profile_with_selection_and_cancel(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedConeNormalResult> {
    operation::cone::execute_with_selection(request, selection, &mut cancelled)
}

#[cfg(test)]
mod tests;
