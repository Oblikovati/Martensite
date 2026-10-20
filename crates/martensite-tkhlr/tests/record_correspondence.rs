//! Record identities and static package operations use their existing Rust representations.
use martensite_tkhlr::hlr_algo::{BiPoint, BiPointFace, BiPointIndices, BiPointPoints};
use martensite_tkhlr::{hlr_algo, hlr_brep};

#[test]
fn segment_index_record_initializes_every_declared_field() {
    let indices = BiPointIndices::default();
    assert_eq!(
        [
            indices.shape_index,
            indices.face_conex1,
            indices.face1_pt1,
            indices.face1_pt2,
            indices.face_conex2,
            indices.face2_pt1,
            indices.face2_pt2,
            indices.min_seg,
            indices.max_seg,
            indices.seg_flags
        ],
        [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn segment_point_record_preserves_model_and_projection_coordinates() {
    let points =
        BiPointPoints::from_coordinates([1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]);
    assert_model_points(&points);
    assert_eq!((points.pnt_p1_2d().x(), points.pnt_p1_2d().y()), (7., 8.));
    assert_eq!((points.pnt_p2_2d().x(), points.pnt_p2_2d().y()), (10., 11.));
    let faces = adjacent_faces();
    let segment = BiPoint::with_faces_flags(points, 19, faces, 31);
    assert_eq!(*segment.points(), points);
    assert_eq!(segment.indices().shape_index, 19);
    assert_eq!(segment.indices().face_conex1, 13);
    assert_eq!(segment.indices().face_conex2, 16);
    assert_eq!(segment.indices().seg_flags, 15);
    assert!(!segment.hidden());
}

fn assert_model_points(points: &BiPointPoints) {
    assert_eq!(
        (points.pnt1.x(), points.pnt1.y(), points.pnt1.z()),
        (1., 2., 3.)
    );
    assert_eq!(
        (points.pnt2.x(), points.pnt2.y(), points.pnt2.z()),
        (4., 5., 6.)
    );
}

fn adjacent_faces() -> [BiPointFace; 2] {
    [
        BiPointFace {
            conex: 13,
            pt1: 14,
            pt2: 15,
        },
        BiPointFace {
            conex: 16,
            pt1: 17,
            pt2: 18,
        },
    ]
}

#[test]
fn forward_declared_edge_and_face_records_have_initialized_flags() {
    let edge: hlr_brep::EdgeData = hlr_brep::EdgeData::new();
    assert!(edge.selected());
    assert!(!edge.used() && !edge.vertical() && !edge.simple());
    assert_eq!(edge.hide_count(), 0);
    let face: hlr_brep::FaceData = hlr_brep::FaceData::new();
    assert!(face.selected());
    assert_eq!(face.size(), 0.0);
    assert!(face.wires().is_null());
}

#[test]
fn static_package_classes_are_exercised_through_modules() {
    let (mut min, mut max) = ([0.; 16], [0.; 16]);
    hlr_algo::init_min_max(16., &mut min, &mut max);
    assert_eq!(min, [16.; 16]);
    assert_eq!(max, [-16.; 16]);
    let (angle, deflection) = hlr_brep::poly_hlr_angle_and_deflection(0.);
    let minimum_angle = 5. * std::f64::consts::PI / 180.;
    assert_eq!(angle, minimum_angle);
    assert_eq!(deflection, minimum_angle * minimum_angle * 0.5);
}
