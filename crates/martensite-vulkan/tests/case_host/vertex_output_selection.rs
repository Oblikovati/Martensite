//! Optional harness binding observes exact named actors and retained UIDs in the same operation.
use super::*;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{ais::AisInteractiveObject, prs_mgr::PrsMgrPresentation};
use martensite_vulkan::{VertexOutputFailure, VertexOutputRequest};
type RetainedPrimitiveBinding = (Handle<martensite_vulkan::Graphic3dDrawGroup>, usize);
type DiagnosticDriverPolicies = (
    RenderPipelinePolicies,
    RenderCoordinateConvention,
    martensite_vulkan::RenderLineClippingBehavior,
    martensite_vulkan::RenderLightDotBehavior,
);
use std::{io, num::NonZeroU64};
#[path = "vertex_output_group_selection.rs"]
mod group_selection;
#[path = "vertex_output_request.rs"]
mod request;
use request::{bounded_request, request_header};
const REQUEST: &str = "MARTENSITE_CASE_VERTEX_OUTPUT_REQUEST";
#[derive(Clone)]
pub(super) struct CaseVertexOutput {
    actor: String,
    mode: i32,
    request: VertexOutputRequest,
    configuration: martensite_vulkan::RenderBehaviorConfiguration,
    group_selection: group_selection::GroupSelection,
}
impl CaseVertexOutput {
    pub(super) fn environment() -> Result<Option<Self>, Box<dyn Error>> {
        let Some(path) = std::env::var_os(REQUEST) else {
            return Ok(None);
        };
        let mut observation = Self::parse(&std::fs::read_to_string(path)?)?;
        observation.configuration = capture_configuration()?;
        Ok(Some(observation))
    }
    fn parse(text: &str) -> Result<Self, Box<dyn Error>> {
        let (actor, mode, format, indices, group_selection) = request_header(text)?;
        let request = bounded_request(indices, format)?;
        Ok(Self {
            actor: actor.to_owned(),
            mode,
            request,
            configuration: Default::default(),
            group_selection,
        })
    }
    pub(super) fn driver(
        &self,
        widths: RenderLineWidthPolicy,
    ) -> Result<Handle<Graphic3dRenderDriver>, Box<dyn Error>> {
        let (policies, coordinates, clipping, _) = diagnostic_driver_policies()?;
        Ok(Graphic3dRenderDriver::with_vertex_output_behavior(
            crate::device_options::validation_options(),
            policies,
            coordinates,
            widths,
            clipping,
            self.configuration,
            self.request.clone(),
        )?)
    }
    pub(super) fn bind(&self, session: &ViewerSession) -> Result<(), Box<dyn Error>> {
        let Some(actor) = session.vertex_output_actor_binding(&self.actor)? else {
            return Ok(());
        };
        let presentation = self.existing_presentation(session, &actor)?;
        let actor_address = actor.object_address() as usize;
        let sources = self.presentation_sources(&actor, &presentation)?;
        self.bind_sources(
            actor_address,
            presentation.object_address() as usize,
            sources,
        )
    }
    fn existing_presentation(
        &self,
        session: &ViewerSession,
        actor: &Handle<dyn AisInteractiveObject>,
    ) -> Result<Handle<PrsMgrPresentation>, Box<dyn Error>> {
        let presentation = session.context().main_prs_mgr().presentation(
            &actor.cast(),
            self.mode,
            false,
            &Handle::default(),
        )?;
        if presentation.is_null() {
            return Err(self.missing().into());
        }
        Ok(presentation)
    }
    fn presentation_sources(
        &self,
        actor: &Handle<dyn AisInteractiveObject>,
        presentation: &Handle<PrsMgrPresentation>,
    ) -> Result<Vec<RetainedPrimitiveBinding>, Box<dyn Error>> {
        let mut sources = Vec::new();
        for group in self.group_selection.groups(actor, presentation)? {
            let source = retained_source_group(&group)?;
            let uids = match self.request.format() {
                martensite_vulkan::VertexOutputFormat::TrianglesClipNormalId32 => {
                    source.vertex_output_triangle_primitive_uids()
                }
                martensite_vulkan::VertexOutputFormat::PointsClipId20 => {
                    source.vertex_output_point_primitive_uids()
                }
                martensite_vulkan::VertexOutputFormat::TextClipUvId28 => {
                    source.vertex_output_text_sources()
                }
            };
            for uid in uids {
                self.emit_candidate(
                    actor.object_address() as usize,
                    presentation.object_address() as usize,
                    &source,
                    uid,
                );
                sources.push((source.clone(), uid));
            }
        }
        Ok(sources)
    }
    fn bind_sources(
        &self,
        actor: usize,
        presentation: usize,
        sources: Vec<RetainedPrimitiveBinding>,
    ) -> Result<(), Box<dyn Error>> {
        let (group, uid) = self.unique_source(&sources)?;
        if self.request.format() == martensite_vulkan::VertexOutputFormat::TextClipUvId28 {
            group.bind_vertex_output_text(*uid, NonZeroU64::MIN)?;
        } else {
            group.bind_vertex_output_primitive(*uid, NonZeroU64::MIN)?;
        }
        self.emit_binding(actor, presentation, group, *uid);
        Ok(())
    }
    fn unique_source<'a>(
        &self,
        sources: &'a [RetainedPrimitiveBinding],
    ) -> Result<&'a RetainedPrimitiveBinding, Box<dyn Error>> {
        if sources.is_empty() {
            return Err(self.missing().into());
        }
        if sources.len() != 1 {
            return Err(VertexOutputFailure::AmbiguousSource {
                source: self.actor.clone(),
                matches: sources.len(),
            }
            .into());
        }
        Ok(&sources[0])
    }
    fn emit_candidate(
        &self,
        actor: usize,
        presentation: usize,
        source: &Handle<martensite_vulkan::Graphic3dDrawGroup>,
        uid: usize,
    ) {
        eprintln!(
            "{{\"kind\":\"vertex-output-source-candidate\",\"actor_name\":{:?},\"actor\":{},\"presentation\":{},\"mode\":{},\"group\":{},\"structure\":{},\"primitive_uid\":{uid}}}",
            self.actor,
            actor,
            presentation,
            self.mode,
            source.object_address() as usize,
            source.structure().object_address() as usize
        );
    }
    fn emit_binding(
        &self,
        actor: usize,
        presentation: usize,
        group: &Handle<martensite_vulkan::Graphic3dDrawGroup>,
        uid: usize,
    ) {
        eprintln!(
            "{{\"kind\":\"vertex-output-actor-binding\",\"source_token\":1,\"actor_name\":{:?},\"actor\":{actor},\"presentation\":{presentation},\"mode\":{},\"group\":{},\"structure\":{},\"primitive_uid\":{uid}}}",
            self.actor,
            self.mode,
            group.object_address() as usize,
            group.structure().object_address() as usize
        );
    }
    fn missing(&self) -> VertexOutputFailure {
        VertexOutputFailure::MissingSource {
            source: format!(
                "actor={:?}, mode={}; expected one actual current-operation ordinary triangle primitive",
                self.actor, self.mode
            ),
        }
    }
    pub(super) fn finish(&self) -> Result<(), VertexOutputFailure> {
        self.request.finish().map_err(|_| self.missing())
    }
}
fn capture_environment_choices() -> Result<light_dot_selection::BehaviorChoices, Box<dyn Error>> {
    let depth = std::env::var_os(light_dot_selection::depth_bias_selection::SELECTION);
    light_dot_selection::depth_bias_selection::require_capture_default(depth.as_deref())?;
    let [points, dots, axes, lines, interpolation, triangles, text] = [
        light_dot_selection::point_raster_selection::SELECTION,
        "MARTENSITE_CASE_LIGHT_DOT_BEHAVIOR",
        light_dot_selection::thin_axis_selection::SELECTION,
        light_dot_selection::thin_line_selection::SELECTION,
        light_dot_selection::phong_interpolation_selection::SELECTION,
        light_dot_selection::triangle_coverage_selection::SELECTION,
        light_dot_selection::text_interpolation_selection::SELECTION,
    ]
    .map(std::env::var_os);
    Ok(light_dot_selection::selected_choices(
        points.as_deref(),
        dots.as_deref(),
        axes.as_deref(),
        lines.as_deref(),
        interpolation.as_deref(),
        triangles.as_deref(),
        text.as_deref(),
    )?)
}

fn capture_configuration() -> Result<martensite_vulkan::RenderBehaviorConfiguration, Box<dyn Error>>
{
    let choices = capture_environment_choices()?;
    require_hardware_triangle_policy(choices.5)?;
    admit_supported_configuration(choices.0, choices.2, choices.3, choices.4)?;
    eprintln!(
        "{}",
        light_dot_selection::phong_interpolation_selection::metadata(choices.4)
    );
    eprintln!(
        "{}",
        light_dot_selection::thin_line_selection::metadata(choices.3)
    );
    let projection = std::env::var_os(light_dot_selection::projection_selection::SELECTION);
    let depth = std::env::var_os(light_dot_selection::window_depth_selection::SELECTION);
    let configuration = light_dot_selection::projection_selection::capture_configuration(
        light_dot_selection::configuration(choices),
        projection.as_deref(),
    )?;
    Ok(
        light_dot_selection::window_depth_selection::capture_configuration(
            configuration,
            depth.as_deref(),
        )?,
    )
}

fn require_hardware_triangle_policy(
    behavior: martensite_vulkan::RenderTriangleCoverageBehavior,
) -> io::Result<()> {
    if behavior == martensite_vulkan::RenderTriangleCoverageBehavior::HardwareV1 {
        return Ok(());
    }
    Err(unsupported_behavior("triangle coverage", behavior))
}

fn admit_supported_configuration(
    points: martensite_vulkan::RenderPointRasterBehavior,
    axes: martensite_vulkan::RenderThinAxisCoverage,
    lines: martensite_vulkan::RenderThinLineCoverage,
    interpolation: martensite_vulkan::RenderPhongInterpolationBehavior,
) -> Result<(), Box<dyn Error>> {
    if points != martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1 {
        return Err(unsupported_behavior("point raster", points).into());
    }
    if axes != martensite_vulkan::RenderThinAxisCoverage::HardwareV1 {
        return Err(unsupported_behavior("thin-axis coverage", axes).into());
    }
    if interpolation == martensite_vulkan::RenderPhongInterpolationBehavior::UnsnappedV3
        && lines == martensite_vulkan::RenderThinLineCoverage::DiamondExitFusedProjectionV4
    {
        return Ok(());
    }
    if lines != martensite_vulkan::RenderThinLineCoverage::HardwareV1 {
        return Err(unsupported_behavior("thin-line coverage", lines).into());
    }
    light_dot_selection::phong_interpolation_selection::admit_observation(interpolation)?;
    Ok(())
}
fn unsupported_behavior(label: &str, behavior: impl std::fmt::Debug) -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        format!("{REQUEST} does not support {label} behavior {behavior:?}"),
    )
}
fn require_hardware_point_policy(value: Option<&std::ffi::OsStr>) -> io::Result<()> {
    let behavior = super::light_dot_selection::selected_point_behavior(value)?;
    if behavior == martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1 {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "point_raster_behavior={behavior:?}; expected HardwareQuadV1 for actual unexpanded vertex-output capture"
        ),
    ))
}
fn diagnostic_driver_policies() -> Result<DiagnosticDriverPolicies, Box<dyn Error>> {
    require_hardware_point_policy(
        std::env::var_os("MARTENSITE_CASE_POINT_RASTER_BEHAVIOR").as_deref(),
    )?;
    let (policies, coordinates) = super::coordinate_policy_selection::from_environment()?;
    let clipping = super::line_clipping_selection::selected_behavior(
        std::env::var_os(super::line_clipping_selection::LINE_CLIPPING_SELECTION).as_deref(),
    )?;
    let dots = super::light_dot_selection::selected(
        std::env::var_os("MARTENSITE_CASE_LIGHT_DOT_BEHAVIOR").as_deref(),
    )?;
    Ok((policies, coordinates, clipping, dots))
}
fn retained_source_group(
    group: &Handle<dyn Graphic3dGroup>,
) -> Result<Handle<martensite_vulkan::Graphic3dDrawGroup>, Box<dyn Error>> {
    let source = group.cast::<martensite_vulkan::Graphic3dDrawGroup>();
    if !source.is_null() {
        return Ok(source);
    }
    Err(VertexOutputFailure::Unsupported {
        operation: "actual retained source group type",
        actual: format!(
            "group={:?}; expected Graphic3dDrawGroup",
            group.object_address()
        ),
    }
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tf_request_never_overrides_triangle_coverage() {
        use martensite_vulkan::{
            RenderTriangleCoverageBehavior as Coverage, RenderTriangleGridOrigin as Origin,
        };
        assert!(require_hardware_triangle_policy(Coverage::HardwareV1).is_ok());
        for origin in [Origin::TopDown, Origin::BottomUp] {
            let error =
                require_hardware_triangle_policy(Coverage::FixedGrid12V2 { origin }).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::Unsupported);
            assert!(error.to_string().contains("triangle coverage"));
        }
    }
    #[test]
    fn diagnostic_never_silently_overrides_point_replay_selection() {
        use std::ffi::OsStr;
        assert!(require_hardware_point_policy(None).is_ok());
        assert!(require_hardware_point_policy(Some(OsStr::new("hardware-quad-v1"))).is_ok());
        assert!(require_hardware_point_policy(Some(OsStr::new("fixed-grid-aliased-v2"))).is_err());
        assert!(require_hardware_point_policy(Some(OsStr::new("invalid"))).is_err());
    }
    #[test]
    fn point_request_is_explicit_with_case_direct_indices_and_no_normal() {
        let parsed = CaseVertexOutput::parse("p\n0\npoints-clip-id20\n1,0\n").unwrap();
        assert_eq!(
            parsed.request.format(),
            martensite_vulkan::VertexOutputFormat::PointsClipId20
        );
        assert!(parsed.request.counts(2, 2).is_ok());
        assert!(parsed.request.counts(1, 1).is_err());
        assert!(CaseVertexOutput::parse("p\n0\npoints-clip-id32\n1,0\n").is_err());
    }
    #[test]
    fn tf_request_accepts_hardware_v1_defaults() {
        assert!(
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1,
                axis_default(),
                line_default(),
                phong_default(),
            )
            .is_ok()
        );
    }
    #[test]
    fn tf_request_admits_only_reviewed_v3_and_fused_v4_pair() {
        use martensite_vulkan::{
            RenderPhongInterpolationBehavior as Phong, RenderThinLineCoverage as Lines,
        };
        let admit = |phong, lines| {
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1,
                axis_default(),
                lines,
                phong,
            )
        };
        assert!(admit(Phong::UnsnappedV3, Lines::DiamondExitFusedProjectionV4).is_ok());
        assert!(admit(Phong::UnsnappedV3, Lines::HardwareV1).is_err());
        assert!(admit(Phong::UnsnappedV2, Lines::DiamondExitFusedProjectionV4).is_err());
        assert!(admit(Phong::HardwareV1, Lines::DiamondExitFusedProjectionV4).is_err());
    }
    #[test]
    fn tf_request_refuses_nondefault_point_raster() {
        assert!(
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::FixedGridAliasedV2,
                axis_default(),
                line_default(),
                phong_default(),
            )
            .is_err()
        );
    }
    #[test]
    fn tf_request_refuses_nondefault_axis_coverage() {
        assert!(
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1,
                martensite_vulkan::RenderThinAxisCoverage::FixedGridV2 {
                    grid: martensite_vulkan::RenderLineGrid::Fractional12,
                    origin: martensite_vulkan::RenderLineOrigin::BottomUp,
                },
                line_default(),
                phong_default(),
            )
            .is_err()
        );
    }
    #[test]
    fn tf_request_refuses_nondefault_phong_interpolation() {
        assert!(
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1,
                axis_default(),
                line_default(),
                martensite_vulkan::RenderPhongInterpolationBehavior::UnsnappedV2,
            )
            .is_err()
        );
    }
    fn axis_default() -> martensite_vulkan::RenderThinAxisCoverage {
        martensite_vulkan::RenderThinAxisCoverage::HardwareV1
    }
    fn phong_default() -> martensite_vulkan::RenderPhongInterpolationBehavior {
        martensite_vulkan::RenderPhongInterpolationBehavior::HardwareV1
    }
    fn line_default() -> martensite_vulkan::RenderThinLineCoverage {
        martensite_vulkan::RenderThinLineCoverage::HardwareV1
    }
    #[test]
    fn tf_request_refuses_nondefault_line_coverage() {
        assert!(
            admit_supported_configuration(
                martensite_vulkan::RenderPointRasterBehavior::HardwareQuadV1,
                axis_default(),
                martensite_vulkan::RenderThinLineCoverage::DiamondExitV2,
                phong_default(),
            )
            .is_err()
        );
    }
    #[test]
    fn vertex_output_case_request_requires_explicit_actor_mode_uid_and_ebo() {
        let parsed = CaseVertexOutput::parse("body\n1\nauto\n0,1,2\n").unwrap();
        assert_eq!((parsed.actor.as_str(), parsed.mode), ("body", 1));
        assert!(parsed.finish().is_err());
        for invalid in [
            "",
            "body\n1\n0\n0,1,2",
            "body\n1\n173\n0,1",
            "body\n1\n173\n0,1,2\nextra",
        ] {
            assert!(CaseVertexOutput::parse(invalid).is_err());
        }
    }
}

#[cfg(test)]
#[path = "vertex_output_controls.rs"]
mod controls;
