//! Check whether the cylinder can encounter the vacuous-relation refusal in #517.
use super::*;
use crate::surface_sketch::{HistoryRole, HistorySource};
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex,
    ProfileVertexId, SketchProfile,
};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::geom_abs::SurfaceType;
const REGION: SurfaceRegionId = SurfaceRegionId(517);

fn request() -> WrappedNormalRequest {
    let mut input = operation_tests::request();
    input.profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vertices(),
        curves: curves(),
    };
    input.selections = vec![selection()];
    input.depths = [-0.5, 0.5];
    input
}
fn vertices() -> Vec<ProfileVertex> {
    [[0., 0.], [1., 0.], [0., 1.]]
        .into_iter()
        .enumerate()
        .map(|(index, point)| ProfileVertex {
            id: ProfileVertexId(index as u64 + 1),
            point,
        })
        .collect()
}
fn curves() -> Vec<ProfileCurve> {
    (1..=3)
        .map(|id| ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(id % 3 + 1),
        })
        .collect()
}
fn selection() -> WrappedRegionSelection {
    let uses = (1..=3)
        .map(|id| LoopUse {
            curve: ProfileCurveId(id),
            reversed: false,
        })
        .collect();
    WrappedRegionSelection {
        region: REGION,
        outer: ProfileLoop { uses },
        holes: Vec::new(),
    }
}

#[test]
fn a_three_edge_triangle_publishes_with_complete_zero_pair_clearance() {
    let input = request();
    let result = normal_extrude_profile(&input).unwrap_or_else(|cause| panic!("{cause}"));
    let prepared = &result.family.family.selection.prepared;
    let clearance = &prepared.boundary_clearance.certificate;
    assert_eq!(clearance.pair_count(), 0);
    assert_eq!(clearance.excluded_pairs().len(), 3);
    assert_eq!(
        clearance.clearance_lower_bound(),
        4. * input.options.spatial_error
    );
    assert!(result.budget().cap_homotopy_clearance <= clearance.clearance_lower_bound());
    assert_eq!(result.reports()[&REGION].coverage.bindings, 21);
    assert_volume(&result);
    assert_caps_and_history(&result);
}
fn assert_volume(result: &WrappedNormalResult) {
    let report = &result.reports()[&REGION];
    assert!(report.validation.is_valid());
    // A=1/2, R=10, depths=[-1/2,1/2]: A*(10.5²-9.5²)/(2R)=1/2.
    let expected = 0.5;
    assert!(report.measures.authored_volume.lower <= expected);
    assert!(report.measures.authored_volume.upper >= expected);
    assert!((report.measures.native_volume.value - expected).abs() <= 1e-4);
    assert!(report.measures.retained_ideal_discrepancy.upper <= 1e-4);
}
fn assert_caps_and_history(result: &WrappedNormalResult) {
    let native = result.export_native().unwrap();
    assert_eq!(result.history().len(), 21);
    assert_eq!(native.entities.len(), 21);
    for (role, radius) in [
        (HistoryRole::FirstDepthFace, 9.5),
        (HistoryRole::LastDepthFace, 10.5),
    ] {
        let key = WrappedHistoryKey {
            source: WrappedEntitySource::Region(REGION),
            role,
        };
        assert_cap_geometry(&native, key, radius);
        assert_cap_history(result, key);
    }
}
fn assert_cap_geometry(native: &DetachedWrappedBodies, key: WrappedHistoryKey, radius: f64) {
    let face = topods::face(&native.entities[&key]).unwrap();
    let surface = BRepAdaptorSurface::with_face(&face, true).unwrap();
    assert_eq!(surface.get_type().unwrap(), SurfaceType::Cylinder);
    assert_eq!(surface.cylinder().unwrap().radius(), radius);
}
fn assert_cap_history(result: &WrappedNormalResult, key: WrappedHistoryKey) {
    let record = result
        .history()
        .iter()
        .find(|record| record.result == key)
        .unwrap();
    assert_eq!(record.owner, REGION);
    assert!(record.parents.contains(&HistorySource::Region(REGION)));
}
