//! Independent native-body and ownership acceptance controls for #416.
pub(crate) mod geometry;
mod isolation;

use geometry::*;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods;
use martensite_tkmath::gp::Point3;
use martensite_tksketch::surface_sketch::*;

static_assertions::assert_impl_all!(NormalExtrusionResult: Send, Sync);

fn resolution() -> Resolution {
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}

fn rectangle() -> CylinderRectangle {
    CylinderRectangle {
        support: CylinderSupport {
            origin: [0.; 3],
            axis: [0., 0., 1.],
            radial: [1., 0., 0.],
            radius: 10.,
            chart_origin: 0.2,
            normal: NormalSense::Outward,
        },
        region: SurfaceRegionId(71),
        s: [0., 7.],
        z: [0., 4.],
        corners: [
            SurfaceVertexId(19),
            SurfaceVertexId(3),
            SurfaceVertexId(81),
            SurfaceVertexId(7),
        ],
        boundaries: [
            SurfaceCurveId(62),
            SurfaceCurveId(4),
            SurfaceCurveId(91),
            SurfaceCurveId(8),
        ],
    }
}

fn construct(rectangle: &CylinderRectangle, depths: [f64; 2]) -> NormalExtrusionResult {
    normal_extrude(
        rectangle,
        depths,
        NormalExtrusionOptions {
            resolution: resolution(),
            volume_relative_error: 1e-9,
        },
    )
    .unwrap()
}

#[test]
fn ordinary_seam_and_period_shift_preserve_carriers_trims_and_history() {
    let mut region = rectangle();
    let original = construct(&region, [-1., 2.]);
    let keys = original.history().to_vec();
    for chart_origin in [0.2, -0.3, -0.3 + std::f64::consts::TAU] {
        region.support.chart_origin = chart_origin;
        let result = construct(&region, [-1., 2.]);
        assert_eq!(result.history(), keys);
        assert_body(&result, &region, [-1., 2.]);
    }
}

#[test]
fn rotated_translated_frame_keeps_physical_depth_and_positive_volume() {
    let mut region = rectangle();
    region.support.origin = [1000., -2000., 3000.];
    region.support.axis = [1., 0., 0.];
    region.support.radial = [0., 0., 1.];
    region.z = [-2., 2.];
    assert_body(&construct(&region, [-1., 2.]), &region, [-1., 2.]);
}

#[test]
fn reversed_normal_swaps_depth_roles_without_reversing_chart() {
    let mut region = rectangle();
    let outward = construct(&region, [-1., 2.]);
    region.support.normal = NormalSense::Inward;
    let inward = construct(&region, [-2., 1.]);
    assert_body(&inward, &region, [-2., 1.]);
    let first = HistoryKey {
        source: HistorySource::Vertex(region.corners[0]),
        role: HistoryRole::FirstDepthVertex,
    };
    let last = HistoryKey {
        role: HistoryRole::LastDepthVertex,
        ..first
    };
    let out_body = outward.export_brep().unwrap();
    let in_body = inward.export_brep().unwrap();
    assert_point(vertex_point(&out_body, first), vertex_point(&in_body, last));
    assert_point(vertex_point(&out_body, last), vertex_point(&in_body, first));
}

#[test]
fn coordinate_edits_preserve_authored_keys_and_prior_result() {
    let mut region = rectangle();
    let result = construct(&region, [-1., 2.]);
    let keys = result.history().to_vec();
    region.s = [1., 8.];
    region.z = [-3., 1.];
    region.support.radius = 20.;
    let edited = construct(&region, [-1., 2.]);
    assert_eq!(edited.history(), keys);
    assert_body(&result, &rectangle(), [-1., 2.]);
}

#[test]
fn mutating_one_export_does_not_change_other_exports_or_published_result() {
    let region = rectangle();
    let result = construct(&region, [-1., 2.]);
    let first = result.export_brep().unwrap();
    let second = result.export_brep().unwrap();
    let key = HistoryKey {
        source: HistorySource::Vertex(region.corners[0]),
        role: HistoryRole::FirstDepthVertex,
    };
    let expected = vertex_point(&second, key);
    let vertex = topods::vertex(entity(&first, key)).unwrap();
    BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(91., 92., 93.), 1e-7)
        .unwrap();
    assert_point(vertex_point(&first, key), Point3::new(91., 92., 93.));
    assert_point(vertex_point(&second, key), expected);
    assert_point(vertex_point(&result.export_brep().unwrap(), key), expected);
    assert_body(&result, &region, [-1., 2.]);
    assert_ne!(BRepTool::pnt(&vertex).unwrap(), expected);
}

#[test]
fn small_resolvable_cylinder_retains_relative_geometry_accuracy() {
    let mut region = rectangle();
    region.support.radius *= 1e-3;
    region.s = region.s.map(|value| value * 1e-3);
    region.z = region.z.map(|value| value * 1e-3);
    let result = construct(&region, [-0.001, 0.002]);
    assert_body(&result, &region, [-0.001, 0.002]);
    let volume = result.volume();
    // Scaling the baseline solid by 1/1000 scales its volume by 1/10^9.
    assert!(volume.encloses(8.82e-8), "{volume:?}");
    assert!(volume.error_bound <= volume.value * result.options().volume_relative_error);
}

#[test]
fn half_major_and_nearly_closed_sectors_keep_complete_native_geometry() {
    let mut region = rectangle();
    let reference = construct(&region, [-1., 2.]);
    for angle in [
        std::f64::consts::PI,
        1.5 * std::f64::consts::PI,
        std::f64::consts::TAU - 1e-4,
    ] {
        region.s[1] = region.s[0] + region.support.radius * angle;
        let result = construct(&region, [-1., 2.]);
        assert_eq!(result.history(), reference.history(), "angle={angle}");
        assert_body(&result, &region, [-1., 2.]);
    }
}

#[test]
fn oblique_translated_frame_preserves_carriers_trims_and_physical_depth() {
    let mut region = rectangle();
    region.support.origin = [17., -23., 31.];
    let axis_norm = 14_f64.sqrt();
    let radial_norm = 5_f64.sqrt();
    region.support.axis = [1. / axis_norm, 2. / axis_norm, 3. / axis_norm];
    region.support.radial = [2. / radial_norm, -1. / radial_norm, 0.];
    region.support.chart_origin = -0.3;
    let result = construct(&region, [-1., 2.]);
    assert_body(&result, &region, [-1., 2.]);
    assert_eq!(
        result.history(),
        construct(&rectangle(), [-1., 2.]).history()
    );
}
