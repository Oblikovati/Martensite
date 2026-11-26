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
