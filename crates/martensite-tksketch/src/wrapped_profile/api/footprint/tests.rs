use super::*;
use crate::surface_sketch::NormalSense;
use martensite_tkbo::profile_extrusion::{
    DisplayTolerance, LoopUse, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop,
    ProfilePlane, ProfileVertex, ProfileVertexId, Resolution, SketchProfile, ToleranceCap,
};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const REVISION: SupportRevision = SupportRevision(7);

#[test]
fn full_band_contains_a_selected_profile_crossing_its_paired_seam() {
    let attachment = attachment(None);
    let proposal = request(&attachment, [-0.25, 0.25], [1., 2.]);
    let report = attachment.certify_footprint(REVISION, &proposal).unwrap();
    assert_eq!(report.attachment, FaceAttachmentId(19));
    assert_eq!(report.revision, REVISION);
    assert_eq!(report.angular, None);
    assert_eq!(report.axial, [0., 4.]);
    assert_eq!(report.regions, 1);
}

#[test]
fn partial_band_has_actual_angular_boundary_not_a_periodic_seam() {
    let attachment = attachment(Some(1.));
    let inside = request(&attachment, [1., 2.], [1., 2.]);
    assert_eq!(
        attachment
            .certify_footprint(REVISION, &inside)
            .unwrap()
            .angular,
        Some([0., 1.])
    );
    let crossing = request(&attachment, [-0.25, 0.25], [1., 2.]);
    let error = attachment
        .certify_footprint(REVISION, &crossing)
        .unwrap_err();
    assert!(
        matches!(
            error,
            CylinderFootprintFailure::ControlOutside {
                axis: 0,
                region: SurfaceRegionId(3),
                curve: SurfaceCurveId(1),
                coordinate: -0.25,
                ..
            }
        ),
        "{error:?}"
    );
}

#[test]
fn same_radius_profiles_that_touch_the_real_axial_boundary_refuse() {
    let attachment = attachment(None);
    let crossing = request(&attachment, [1., 2.], [0., 1.]);
    let error = attachment
        .certify_footprint(REVISION, &crossing)
        .unwrap_err();
    assert!(
        matches!(
            error,
            CylinderFootprintFailure::ControlOutside {
                axis: 1,
                region: SurfaceRegionId(3),
                required_interval: [0., 4.],
                ..
            }
        ),
        "{error:?}"
    );
}

#[test]
fn radius_mismatch_and_stale_revisions_refuse_before_profile_check() {
    let attachment = attachment(None);
    let mut wrong = request(&attachment, [1., 2.], [1., 2.]);
    wrong.support.radius += 1.;
    assert!(matches!(
        attachment.certify_footprint(REVISION, &wrong),
        Err(CylinderFootprintFailure::SupportMismatch { .. })
    ));
    let error = attachment
        .certify_footprint(SupportRevision(8), &wrong)
        .unwrap_err();
    assert!(matches!(error, CylinderFootprintFailure::Attachment(cause)
        if cause.class() == crate::surface_sketch::attachment::AttachmentFailureClass::StaleRevision));
}

#[test]
fn round_bore_adds_a_trim_loop_that_the_band_does_not_admit() {
    let support = BRepPrimAPIMakeCylinder::new(10., 4.)
        .shape()
        .unwrap()
        .clone();
    let cut = bore_support(&support);
    let face = selected_cylinder(&cut);
    let wires = Explorer::new(&face, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    assert!(wires.count() > 1);
    let attachment = freeze(&cut);
    let proposal = request(&attachment, [15.5, 15.75], [1.75, 2.25]);
    assert!(matches!(
        attachment.certify_footprint(REVISION, &proposal),
        Err(CylinderFootprintFailure::UnsupportedTrim { .. })
    ));
}

fn bore_support(support: &Shape) -> Shape {
    use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
    use martensite_tkernel::ncollection::List;
    use martensite_tkmath::gp::{Axis2, Point3, StandardDirection3};
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., 2.),
        StandardDirection3::Y,
        StandardDirection3::X,
    );
    let drill = BRepPrimAPIMakeCylinder::with_axes(axes, 0.5, 12.)
        .shape()
        .unwrap()
        .clone();
    let mut cut = BRepAlgoAPICut::new();
    cut.set_non_destructive(true);
    cut.set_arguments(&List::from_iter([support.clone()]));
    cut.set_tools(&List::from_iter([drill]));
    cut.build().unwrap();
    assert!(cut.is_done());
    cut.shape().clone()
}

fn attachment(angle: Option<f64>) -> CylinderFaceAttachment {
    let mut maker = match angle {
        Some(angle) => BRepPrimAPIMakeCylinder::with_angle(10., 4., angle).unwrap(),
        None => BRepPrimAPIMakeCylinder::new(10., 4.),
    };
    freeze(maker.shape().unwrap())
}
fn freeze(body: &Shape) -> CylinderFaceAttachment {
    let face = selected_cylinder(body);
    CylinderFaceAttachment::capture(FaceAttachmentId(19), REVISION, body, &face, resolution())
        .unwrap()
}
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
fn selected_cylinder(body: &Shape) -> Face {
    Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::face(&shape).unwrap())
        .find(|face| {
            let surface = BRepAdaptorSurface::with_face(face, false).unwrap();
            surface.get_type().unwrap() == SurfaceType::Cylinder
                && surface.cylinder().unwrap().radius() == 10.
        })
        .unwrap()
}
fn request(
    attachment: &CylinderFaceAttachment,
    [x0, x1]: [f64; 2],
    [y0, y1]: [f64; 2],
) -> WrappedNormalRequest {
    WrappedNormalRequest {
        support: attachment
            .extrusion_support(REVISION, 0., NormalSense::Inward)
            .unwrap(),
        depths: [0., 0.25],
        profile: profile([[x0, y0], [x1, y0], [x1, y1], [x0, y1]]),
        selections: vec![selection()],
        options: WrappedNormalOptions {
            spatial_error: 1e-5,
            planar_work: PlanarProofWorkLimit::CurrentDefault,
            maximum_fit_level: 8,
            volume: WrappedVolumePolicy::Report,
        },
    }
}
fn profile(points: [[f64; 2]; 4]) -> SketchProfile {
    let mut vertices = Vec::new();
    let mut curves = Vec::new();
    for (ordinal, point) in points.into_iter().enumerate() {
        let id = (ordinal + 1) as u64;
        vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point,
        });
        curves.push(ProfileCurve::Line {
            id: ProfileCurveId(id),
            start: ProfileVertexId(id),
            end: ProfileVertexId(id % 4 + 1),
        });
    }
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        curves,
        vertices,
    }
}
fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: SurfaceRegionId(3),
        outer: ProfileLoop {
            uses: (1..=4)
                .map(|id| LoopUse {
                    curve: ProfileCurveId(id),
                    reversed: false,
                })
                .collect(),
        },
        holes: Vec::new(),
    }
}
