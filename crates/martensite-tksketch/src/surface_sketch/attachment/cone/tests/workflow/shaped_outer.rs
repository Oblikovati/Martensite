//! Reattach to a curved conical cap produced by the placed operation itself.
use super::*;
use crate::surface_sketch::{WrappedFeatureKind, WrappedHistoryKey};
use martensite_tkbrep::brep_tools::uv_bounds;

#[test]
fn conical_pocket_can_be_cut_on_a_shaped_selected_cap() {
    let original = attach([2., 4.]);
    let original_chart = original.authoring_chart(REVISION, 0.7).unwrap();
    let mut request = proposal(&original_chart);
    request.options.spatial_error = 1e-5;
    curve_top(&mut request);
    let (attachment, native_uv) = shaped_cap(&request);
    let chart = attachment.authoring_chart(REVISION, 0.).unwrap();
    let points = central_profile(native_uv);
    request.profile = profile_at(&chart, points, OutOfSupportPolicy::Refuse);
    request.support = chart
        .extrusion_support(REVISION, NormalSense::Inward)
        .unwrap();
    request.depths = [0., 0.03];
    let feature = attachment
        .apply_wrapped_profile(
            REVISION,
            0.,
            WrappedFeatureKind::Pocket,
            &request,
            directed(),
        )
        .unwrap_or_else(|cause| panic!("curved conical cap refused: {cause:?}"));
    assert!(feature.tool().shaped_outer_edges() >= 3);
    assert!(feature.validation().is_valid());
    assert_eq!(
        feature.export_native().unwrap().body.shape_type().unwrap(),
        ShapeEnum::Solid
    );
}

fn shaped_cap(request: &WrappedConeNormalRequest) -> (ConeFaceAttachment, (f64, f64, f64, f64)) {
    let parent = normal_extrude_cone_profile_on_chart(request, directed()).unwrap();
    let native = parent.export_native().unwrap();
    let cap = WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(19)),
        role: HistoryRole::LastDepthFace,
    };
    let face = topods::face(&native.entities[&cap]).unwrap();
    let uv = uv_bounds(&face).unwrap();
    let resolution = super::super::super::super::tests::resolution();
    let attachment =
        ConeFaceAttachment::capture(ID, REVISION, &native.bodies, &face, resolution).unwrap();
    (attachment, uv)
}

fn central_profile((u0, u1, v0, v1): (f64, f64, f64, f64)) -> [[f64; 2]; 4] {
    let u = u0.midpoint(u1);
    let v = v0.midpoint(v1);
    let du = (u1 - u0) / 8.;
    let dv = (v1 - v0) / 8.;
    [
        [u - du, v - dv],
        [u - du, v + dv],
        [u + du, v + dv],
        [u + du, v - dv],
    ]
}

fn curve_top(request: &mut WrappedConeNormalRequest) {
    let first = request.profile.vertices[1].point;
    let last = request.profile.vertices[2].point;
    let inner = |t| {
        [
            first[0] + (last[0] - first[0]) * t + 0.05,
            first[1] + (last[1] - first[1]) * t,
        ]
    };
    request.profile.curves[1] = ProfileCurve::Spline {
        id: ProfileCurveId(2),
        start: ProfileVertexId(2),
        end: ProfileVertexId(3),
        degree: 3,
        poles: vec![first, inner(1. / 3.), inner(2. / 3.), last],
        weights: None,
        knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
    };
}
