//! Owned curved outer material and a clockwise hole exercise the actual public input path.
use super::*;
mod completed;
pub(in crate::surface_sketch::wrapped_profile) use completed::atlas::separated_request as atlas_request;
mod frozen;
mod inward;
mod narrow;
pub(in crate::surface_sketch::wrapped_profile) fn supported_common_chart_request()
-> WrappedConeNormalRequest {
    let mut request =
        narrow::fixture::request(NormalSense::Outward, narrow::fixture::Quadrant::PositiveX);
    request.proof.integration.behavior_version = 2;
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: u64::from(request.proof.integration.max_total_subintervals),
        };
    request
}

use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex,
    ProfileVertexId, SketchProfile,
};
pub(in crate::surface_sketch::wrapped_profile) fn request() -> WrappedConeNormalRequest {
    let (vertices, curves, outer, hole) = contours();
    WrappedConeNormalRequest {
        profile: SketchProfile {
            plane: ProfilePlane::world_xy(),
            vertices,
            curves,
        },
        selections: vec![WrappedRegionSelection {
            region: SurfaceRegionId(1),
            outer,
            holes: vec![hole],
        }],
        support: support(),
        depths: [-0.1, 0.2],
        proof: WrappedConeProofOptions::default(),
        options: options(),
    }
}
fn support() -> ConeSupport {
    ConeSupport {
        apex: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        semi_angle: 0.7,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}
fn options() -> WrappedNormalOptions {
    WrappedNormalOptions {
        spatial_error: 1e-4,
        maximum_fit_level: 8,
        planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        volume: WrappedVolumePolicy::Report,
    }
}

fn contours() -> (
    Vec<ProfileVertex>,
    Vec<ProfileCurve>,
    ProfileLoop,
    ProfileLoop,
) {
    let vertices = vertices();
    let mut curves = Vec::new();
    for offset in [0_u64, 4] {
        for i in 0..4 {
            curves.push(curve(
                offset + i + 1,
                offset + i + 1,
                offset + (i + 1) % 4 + 1,
            ));
        }
    }
    (vertices, curves, contour(1), contour(5))
}
fn vertices() -> Vec<ProfileVertex> {
    let points = points();
    points
        .into_iter()
        .enumerate()
        .map(|(i, point)| ProfileVertex {
            id: ProfileVertexId(i as u64 + 1),
            point,
        })
        .collect()
}

fn points() -> [[f64; 2]; 8] {
    [
        [4., 0.],
        [5., 0.],
        [5., 1.],
        [4., 1.],
        [4.25, 0.25],
        [4.25, 0.75],
        [4.75, 0.75],
        [4.75, 0.25],
    ]
}

fn curve(id: u64, first: u64, last: u64) -> ProfileCurve {
    let (start, end) = (ProfileVertexId(first), ProfileVertexId(last));
    if id != 3 {
        return ProfileCurve::Line {
            id: ProfileCurveId(id),
            start,
            end,
        };
    }
    ProfileCurve::Spline {
        id: ProfileCurveId(id),
        start,
        end,
        degree: 3,
        poles: vec![[5., 1.], [4.75, 1.1], [4.25, 1.1], [4., 1.]],
        weights: None,
        knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
    }
}
fn contour(first: u64) -> ProfileLoop {
    ProfileLoop {
        uses: (first..first + 4)
            .map(|id| LoopUse {
                curve: ProfileCurveId(id),
                reversed: false,
            })
            .collect(),
    }
}
#[test]
fn cone_capture_owns_curved_hole_and_keeps_caller_depths() {
    let mut request = request();
    request.support.normal = NormalSense::Inward;
    let inputs = input::capture(&request, &mut || false).unwrap();
    assert_eq!(inputs.originals[0].depths, [-0.2, 0.1]);
    assert_eq!(inputs.originals[0].loops.len(), 2);
    request.profile.curves.clear();
    assert_eq!(inputs.originals[0].loops[0][2].poles.len(), 4);
}
#[test]
fn cone_cancellation_precedes_input_without_polling_to_relabel_an_error() {
    let mut request = request();
    request.support.semi_angle = f64::NAN;
    let cancelled = execute(&request, &mut || true).err().unwrap();
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
    let mut polls = 0;
    let invalid = execute(&request, &mut || {
        polls += 1;
        polls > 1
    })
    .err()
    .unwrap();
    assert_eq!(invalid.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(polls, 1);
}
#[test]
fn cone_developed_angle_helper_lifts_the_negative_axis_continuously() {
    let a = native::developed_angle([-4., -0.5], ConicalSectorWedge::NegativeX);
    let b = native::developed_angle([-4., 0.5], ConicalSectorWedge::NegativeX);
    assert!(a > std::f64::consts::PI && b < std::f64::consts::PI);
    assert!((a - b).abs() < 0.3);
}

#[test]
fn cone_phase_a_refuses_unaccounted_placement_before_source_capture() {
    let mut request = request();
    request.support.apex = [2., 3., 4.];
    request.profile.curves.clear();
    let failure = execute(&request, &mut || false).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
    assert_eq!(failure.stage(), WrappedStage::Admission);
}
#[test]
fn cone_public_owners_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<WrappedConeNormalRequest>();
    assert_traits::<WrappedConeNormalResult>();
}

#[test]
fn cone_explicit_proof_policy_refuses_invalid_values_before_source_work() {
    let mut request = request();
    request.proof.arithmetic.max_operations = 1_000_001;
    request.profile.curves.clear();
    let failure = execute(&request, &mut || false).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(failure.stage(), WrappedStage::Admission);
}
#[test]
fn cone_exhausted_requested_work_does_not_mutate_input_or_enlarge_allowance() {
    let mut request = request();
    request.proof.arithmetic.max_operations = 0;
    let before = request.clone();
    let failure = execute(&request, &mut || false).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert_eq!(request, before);
}

#[test]
fn cone_authored_volume_policy_is_never_clamped_or_replaced() {
    for width in [0., f64::NAN] {
        let mut request = request();
        request.proof.authored_volume.enclosure_width = width;
        let failure = execute(&request, &mut || false).err().unwrap();
        assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
        assert_eq!(failure.stage(), WrappedStage::Admission);
    }
    let mut request = request();
    request.proof.authored_volume.subdivision_level = 13;
    let failure = execute(&request, &mut || false).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(failure.stage(), WrappedStage::Admission);
}

#[test]
fn phase_a_public_entry_refuses_a_valid_sector_outside_its_common_chart() {
    let mut request = request();
    for vertex in &mut request.profile.vertices {
        vertex.point = vertex.point.map(|value| -value);
    }
    for curve in &mut request.profile.curves {
        rotate_curve(curve);
    }
    let failure = crate::surface_sketch::normal_extrude_cone_profile(&request)
        .err()
        .unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
}

fn rotate_curve(curve: &mut ProfileCurve) {
    let ProfileCurve::Spline { poles, .. } = curve else {
        return;
    };
    for point in poles {
        *point = point.map(|value| -value);
    }
}

pub(in crate::surface_sketch::wrapped_profile) fn frozen_native()
-> (DevelopedRegionRequest, Shape, Vec<(BindingKey, Shape)>) {
    let frozen = frozen::capture(&request());
    (frozen.source, frozen.shape, frozen.bindings)
}
