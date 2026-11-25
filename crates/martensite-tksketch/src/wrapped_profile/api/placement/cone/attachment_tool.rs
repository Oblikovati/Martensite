//! Place one attached working graph and independently bind an immutable canonical authority.
use super::*;
use crate::surface_sketch::{
    encoding,
    wrapped_profile::{
        api::{error as wrapped_error, export_bindings},
        frozen_placement,
        operation::cone::attachment_tool::{
            AttachmentAuthority, AttachmentToolWork, CanonicalTool,
        },
    },
};
use martensite_tkbo::body_check::CertifiedConicalBody;
use martensite_tkmath::{
    predicates::roots::metric_bounds::{ConicalSectorWedge, ConicalTrimCarrier},
    toploc::Location,
};
use std::sync::Arc;

pub(in crate::surface_sketch::wrapped_profile::api) struct AttachmentTool {
    native: DetachedWrappedBodies,
    source: AttachmentToolSource,
}

struct AttachmentToolSource {
    canonical: Vec<u8>,
    keys: Vec<BindingKey>,
    history: history_projection::HistoryPublication,
    regions: Vec<SurfaceRegionId>,
    budget: CertifiedRotatedFrameSpatialBudget,
    transform: Transform3,
    authority: AttachmentAuthority,
}

pub(in crate::surface_sketch::wrapped_profile::api) struct ReboundAttachmentTool<'a> {
    source: &'a AttachmentToolSource,
}

impl AttachmentTool {
    pub(in crate::surface_sketch::wrapped_profile::api) fn native(&self) -> &DetachedWrappedBodies {
        &self.native
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn history(
        &self,
    ) -> &[WrappedHistoryRecord] {
        &self.source.history.records
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn regions(&self) -> &[SurfaceRegionId] {
        &self.source.regions
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn spatial_budget(
        &self,
    ) -> &CertifiedRotatedFrameSpatialBudget {
        &self.source.budget
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn work(&self) -> &AttachmentToolWork {
        self.source.authority.work()
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn fitted(
        &self,
        region: SurfaceRegionId,
    ) -> Option<(
        ConicalSectorWedge,
        [ConicalTrimCarrier; 2],
        &[operation_cone::SpanProof],
    )> {
        self.source.authority.fitted(region)
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn rebind<'a>(
        &'a self,
        native: &'a DetachedWrappedBodies,
    ) -> Checked<ReboundAttachmentTool<'a>> {
        self.source.rebind(native)
    }
}

impl ReboundAttachmentTool<'_> {
    pub(in crate::surface_sketch::wrapped_profile::api) fn certified_body(
        &self,
    ) -> &Arc<CertifiedConicalBody> {
        self.source.authority.body()
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn cap_residual_upper(
        &self,
        key: BindingKey,
    ) -> Option<f64> {
        self.source.authority.cap_residual_upper(key)
    }

    // Physical caller-relative names can reverse normal-depth roles.
    pub(in crate::surface_sketch::wrapped_profile::api) fn cap_residuals(
        &self,
    ) -> impl Iterator<Item = (WrappedHistoryKey, f64)> + '_ {
        self.source
            .keys
            .iter()
            .zip(&self.source.history.bindings)
            .filter_map(|(&key, &name)| Some((name, self.cap_residual_upper(key)?)))
    }
}

impl AttachmentToolSource {
    fn rebind<'a>(
        &'a self,
        native: &'a DetachedWrappedBodies,
    ) -> Checked<ReboundAttachmentTool<'a>> {
        let location = native.bodies.location();
        check_datum(location, self.transform)?;
        let bindings = self.typed_bindings(native)?;
        let stripped = frozen_placement::strip_bindings::<Failure>(location, &bindings)
            .map_err(freeze::project)?;
        let root = native
            .bodies
            .located(&Location::default(), false)
            .map_err(|cause| {
                fail(PlacementFailure::Native {
                    step: "strip attached root placement",
                    cause,
                })
            })?;
        let canonical = encoding::encode(&root, &stripped).map_err(|cause| {
            freeze::project(frozen_placement::ArchivePlacementFailure::Encoding(cause))
        })?;
        frozen_placement::compare_archive::<Failure>(&canonical, &self.canonical)
            .map_err(freeze::project)?;
        self.authority.rebind(&stripped)?;
        Ok(ReboundAttachmentTool { source: self })
    }

    fn typed_bindings(&self, native: &DetachedWrappedBodies) -> Checked<Vec<(BindingKey, Shape)>> {
        if self.keys.len() != self.history.bindings.len()
            || self.keys.len() != native.entities.len()
        {
            return Err(freeze::project(
                frozen_placement::ArchivePlacementFailure::FrozenGraph(
                    frozen_placement::GraphReadout {
                        actual_length: native.entities.len(),
                        expected_length: self.keys.len(),
                        offset: 0,
                        actual: None,
                        expected: None,
                    },
                ),
            ));
        }
        self.keys
            .iter()
            .zip(&self.history.bindings)
            .map(|(&key, name)| {
                let shape = native
                    .entities
                    .get(name)
                    .ok_or_else(|| fail(PlacementFailure::SemanticDatum(key)))?;
                Ok((key, shape.clone()))
            })
            .collect()
    }
}

pub(in crate::surface_sketch::wrapped_profile::api) fn on_chart<T, E>(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
    cancelled: &mut CancelPoll<'_>,
    inspect: impl FnOnce(&[DevelopedRegionRequest], &mut CancelPoll<'_>) -> Result<T, E>,
) -> Result<(AttachmentTool, T), E>
where
    E: From<WrappedNormalFailure>,
{
    input::poll(cancelled, WrappedStage::Admission)?;
    if !selection.uses_directed_cap_cycles()
        || !matches!(request.options.volume, WrappedVolumePolicy::Report)
    {
        return Err(operation_cone::attachment_tool::unsupported().into());
    }
    let (transform, frame) = frame(request.support)?;
    let mut run = canonical_run(request);
    let mut inputs = operation_cone::input::capture_with_selection(&run, selection, cancelled)?;
    let inspected = inspect(&inputs.originals, cancelled)?;
    let plan = allocate(request, transform, &frame, &inputs.originals)?;
    run.options.spatial_error = plan.budget.canonical_allowance();
    for source in &mut inputs.originals {
        source.tolerance = run.options.spatial_error;
    }
    let canonical =
        operation_cone::attachment_tool::materialize(&run, selection, inputs, cancelled)?;
    let tool = place(canonical, plan)?;
    Ok((tool, inspected))
}

fn place(canonical: CanonicalTool, plan: ConePlan) -> Checked<AttachmentTool> {
    let (bodies, bindings) = move_graph(canonical.graph, plan.transform)?;
    let mut entities = BTreeMap::new();
    export_bindings::append(
        &canonical.keys,
        &canonical.history.bindings,
        bindings,
        &mut entities,
    )
    .map_err(export_bindings_failure)?;
    let source = AttachmentToolSource {
        canonical: canonical.bytes,
        keys: canonical.keys,
        history: canonical.history,
        regions: canonical.regions,
        budget: plan.budget,
        transform: plan.transform,
        authority: canonical.authority,
    };
    Ok(AttachmentTool {
        native: DetachedWrappedBodies { bodies, entities },
        source,
    })
}

fn move_graph(
    (root, bindings): frozen_placement::DecodedGraph,
    transform: Transform3,
) -> Checked<frozen_placement::DecodedGraph> {
    if freeze::identity(transform) {
        return Ok((root, bindings));
    }
    let location = Location::new(transform);
    let root = root.moved(&location, true).map_err(|cause| {
        fail(PlacementFailure::Native {
            step: "attached root placement",
            cause,
        })
    })?;
    check_datum(root.location(), transform)?;
    let bindings = frozen_placement::move_bindings(bindings, &location).map_err(freeze::project)?;
    for (key, shape) in &bindings {
        if !root.location().is_equal(shape.location()) {
            return Err(fail(PlacementFailure::SemanticDatum(*key)));
        }
    }
    Ok((root, bindings))
}

fn check_datum(location: &Location, transform: Transform3) -> Checked<()> {
    if location.is_identity() && freeze::identity(transform) {
        return Ok(());
    }
    frozen_placement::check_location::<Failure>(location, transform).map_err(freeze::project)
}

fn export_bindings_failure(cause: export_bindings::ExportBindingFailure) -> WrappedNormalFailure {
    wrapped_error::failure(
        WrappedStage::History,
        WrappedFailureClass::Incomplete,
        None,
        wrapped_error::Cause::ExportBindings(cause),
    )
}
