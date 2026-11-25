// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{
    DisplayTolerance, ExtrudeOptions, HistoryRole, ProfileCurve, ProfileCurveId, ProfilePlane,
    ProfileVertex, ProfileVertexId, Resolution, ResultEntity, SketchProfile, ToleranceCap,
    build_planar_regions, extrude,
};
use martensite_tkg3d::geom::GeomGeometry;
use martensite_tkmath::gp::Vector3;

#[test]
fn a_moved_native_cap_plane_breaks_the_trim_trace_before_measuring() {
    let resolution = Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    );
    let regions = build_planar_regions(&rectangle(), resolution).unwrap();
    let result = extrude(
        &regions.regions()[0],
        [0., 0., 4.],
        &ExtrudeOptions { resolution },
    )
    .unwrap();
    let cap = result
        .history()
        .records
        .iter()
        .find(|entry| entry.role == HistoryRole::EndCap)
        .unwrap();
    let ResultEntity::Face(index) = cap.result else {
        panic!("end cap must be a face")
    };
    let cap = &result.faces()[index];
    assert!(contribution(cap, index, [0.; 3], 1e-8, &mut || false).is_ok());

    let (stored, _) = BRepTool::surface_with_location(cap).unwrap();
    let plane = stored.downcast_ref::<GeomPlane>().unwrap();
    plane.translate(Vector3::new(0., 0., 5e-7)).unwrap();
    let retained = contribution(cap, index, [0.; 3], 1e-8, &mut || false);
    assert!(
        matches!(
            retained,
            Err(Failure::SweepFaceTrace { face, bound, limit: 1e-8 })
                if face == index && (4.9e-7..=5.1e-7).contains(&bound)
        ),
        "{retained:?}"
    );
}

fn rectangle() -> SketchProfile {
    let vertices = [[0., 0.], [3., 0.], [3., 5.], [0., 5.]]
        .into_iter()
        .enumerate()
        .map(|(i, point)| ProfileVertex {
            id: ProfileVertexId(i as u64),
            point,
        })
        .collect();
    let curves = (0..4)
        .map(|i| ProfileCurve::Line {
            id: ProfileCurveId(i + 10),
            start: ProfileVertexId(i),
            end: ProfileVertexId((i + 1) % 4),
        })
        .collect();
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    }
}
