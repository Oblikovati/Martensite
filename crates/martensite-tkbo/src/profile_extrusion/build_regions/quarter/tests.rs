// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::profile_extrusion::{
    DisplayTolerance, ExtrudeOptions, ProfilePlane, ProfileVertex, ToleranceCap, extrude,
};
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::{geom::GeomCylindricalSurface, top_abs::ShapeEnum};

fn fixture() -> SketchProfile {
    let uv = [[5., -25.], [5., 0.], [0., 5.], [-25., 5.], [-25., -25.]];
    let vertices = uv
        .into_iter()
        .enumerate()
        .map(|(index, point)| ProfileVertex {
            id: ProfileVertexId(index as u64 + 1),
            point,
        })
        .collect();
    let curves = (0..5u64)
        .map(|index| {
            let (id, start, end) = (
                ProfileCurveId(10 + index),
                ProfileVertexId(1 + index),
                ProfileVertexId(1 + (index + 1) % 5),
            );
            if index == 1 {
                ProfileCurve::Arc {
                    id,
                    start,
                    end,
                    center: [0.; 2],
                    sense: ArcSense::CounterClockwise,
                }
            } else {
                ProfileCurve::Line { id, start, end }
            }
        })
        .collect();
    SketchProfile {
        plane: ProfilePlane {
            origin: [55., 5., 0.],
            x_direction: [0., -1., 0.],
            normal: [0., 0., 1.],
        },
        vertices,
        curves,
    }
}

#[test]
fn scoped_quarter_reaches_native_prism_with_retained_curved_trims() {
    let region = build_planar_cardinal_quarter_regions_with_work_and_cancel(
        &fixture(),
        resolution(),
        PlanarProofWorkLimit::CurrentDefault,
        || false,
    )
    .unwrap();
    let [region] = region.regions() else {
        panic!("one admitted rounded region");
    };
    assert!(
        region
            .area()
            .encloses(875. + 25. * std::f64::consts::FRAC_PI_4)
    );
    for height in [3., -3.] {
        let result = extrude(
            region,
            [0., 0., height],
            &ExtrudeOptions {
                resolution: resolution(),
            },
        )
        .unwrap();
        assert!(result.validation().is_valid());
        assert_eq!(
            (
                result.vertices().len(),
                result.edges().len(),
                result.faces().len()
            ),
            (10, 15, 7)
        );
        let mut circular_faces = 0;
        let mut curved_caps = 0;
        for face in result.faces() {
            let (surface, _) = BRepTool::surface_with_location(face).unwrap();
            let cylinder = surface.downcast_ref::<GeomCylindricalSurface>();
            if cylinder.is_some() {
                circular_faces += 1;
            }
            let mut edges = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
            while edges.more() {
                let edge = topods::edge(edges.current()).unwrap();
                let pcurve = BRepTool::curve_on_face(&edge, face).unwrap();
                if cylinder.is_some() {
                    assert!(pcurve.is_stored);
                    assert!(pcurve.pcurve.downcast_ref::<Geom2dLine>().is_some());
                } else if pcurve.pcurve.downcast_ref::<Geom2dCircle>().is_some() {
                    assert!(pcurve.is_stored);
                    curved_caps += 1;
                }
                edges.next().unwrap();
            }
        }
        assert_eq!((circular_faces, curved_caps), (1, 2));
    }
}

fn resolution() -> Resolution {
    let display = DisplayTolerance {
        chordal: 1e-3,
        angular: 0.1,
    };
    Resolution::new(1e-6, 1e-7, 1e-9, display, ToleranceCap::Refuse(1e-6))
}

#[test]
fn scoped_quarter_requires_exact_intrinsic_endpoint_and_preserves_cancellation() {
    let mut profile = fixture();
    profile.vertices[2].point[0] = 1e-15;
    let work = PlanarProofWorkLimit::CurrentDefault;
    let refused = build_planar_cardinal_quarter_regions_with_work_and_cancel(
        &profile,
        resolution(),
        work,
        || false,
    )
    .err()
    .unwrap();
    assert!(matches!(
        refused.decline().cause,
        DeclineCause::InvalidCurve {
            curve: ProfileCurveId(11),
            ..
        }
    ));
    let cancelled = build_planar_cardinal_quarter_regions_with_work_and_cancel(
        &fixture(),
        resolution(),
        work,
        || true,
    )
    .err()
    .unwrap();
    assert!(cancelled.is_cancelled());
    assert_eq!(cancelled.requested_work(), work);
}
