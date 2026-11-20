use super::*;
use crate::surface_sketch::{HistoryRole, NormalSense};
use martensite_tkbo::profile_extrusion::{
    DisplayTolerance, LoopUse, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId, ProfileLoop,
    ProfilePlane, ProfileVertex, ProfileVertexId, Resolution, SketchProfile, ToleranceCap,
};
use martensite_tkbrep::{
    brep::BRepTool, brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const REVISION: SupportRevision = SupportRevision(7);

#[test]
fn partial_band_tool_reuses_chart_extrusion_and_checks_both_cap_pcurves() {
    let attachment = attachment(Some(1.));
    let mut proposal = request(&attachment, [1., 2.], [1., 2.]);
    proposal.support.chart_origin = 0.25;
    let original = proposal.clone();
    let saved_support = attachment.record().brep.clone();
    let tool = attachment
        .extrude_within_trim_band(REVISION, &proposal)
        .unwrap();
    check_partial_band_tool(&tool, &proposal);
    assert_eq!(proposal, original);
    assert_eq!(attachment.record().brep, saved_support);
}

fn check_partial_band_tool(tool: &CylinderTrimmedTool, proposal: &WrappedNormalRequest) {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CylinderTrimmedTool>();
    let standalone = normal_extrude_profile_on_chart(proposal).unwrap();
    assert_eq!(tool.footprint().angular, Some([0., 1.]));
    assert_eq!(tool.footprint().revision, REVISION);
    assert_eq!(tool.extrusion().encoded_brep(), standalone.encoded_brep());
    assert_eq!(tool.extrusion().history(), standalone.history());
    let report = &tool.extrusion().reports()[&SurfaceRegionId(3)];
    assert_eq!(report.coverage.cap_pcurves, 2 * tool.fitted_spans());
    assert!(tool.fitted_spans() >= proposal.profile.curves.len());
    assert!(report.validation.is_valid());
    let native = tool.extrusion().export_native().unwrap();
    assert_eq!(
        checked_partial_cap(
            &native,
            HistoryRole::FirstDepthFace,
            HistoryRole::FirstDepthEdge
        ),
        tool.fitted_spans()
    );
    assert_eq!(
        checked_partial_cap(
            &native,
            HistoryRole::LastDepthFace,
            HistoryRole::LastDepthEdge
        ),
        tool.fitted_spans()
    );
}

fn checked_partial_cap(
    native: &DetachedWrappedBodies,
    cap: HistoryRole,
    rim: HistoryRole,
) -> usize {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(3)),
        role: cap,
    };
    let face = topods::face(&native.entities[&key]).unwrap();
    let rim_edges = native.entities.iter().filter(|(key, _)| key.role == rim);
    let mut checked = 0;
    for (_, edge) in rim_edges {
        check_partial_pcurve(&face, &topods::edge(edge).unwrap());
        checked += 1;
    }
    checked
}

fn check_partial_pcurve(face: &Face, edge: &Edge) {
    let stored = BRepTool::curve_on_face(edge, face).unwrap();
    assert!(stored.is_stored);
    assert_eq!([stored.first, stored.last], [0., 1.]);
    let pcurve: Handle<Geom2dBezierCurve> = stored.pcurve.cast();
    for point in pcurve.get().unwrap().poles().iter() {
        let native_u = 0.25 + point.x();
        assert!((0.3..0.5).contains(&native_u));
        assert!((1. ..=2.).contains(&point.y()));
    }
}

#[test]
fn trim_checked_outward_tool_crosses_a_full_bands_paired_seam() {
    let attachment = attachment(None);
    let mut proposal = request(&attachment, [-0.25, 0.25], [1., 2.]);
    proposal.support.normal = NormalSense::Outward;
    let tool = attachment
        .extrude_within_trim_band(REVISION, &proposal)
        .unwrap();
    assert_eq!(tool.footprint().angular, None);
    let report = &tool.extrusion().reports()[&SurfaceRegionId(3)];
    assert_eq!(report.coverage.cap_pcurves, 2 * tool.fitted_spans());
    assert!(report.validation.is_valid());
}

#[test]
fn trimmed_tool_refuses_real_edge_contact_before_extrusion() {
    let attachment = attachment(Some(1.));
    let mut proposal = request(&attachment, [-0.25, 0.25], [1., 2.]);
    // This value would fail fitting. The requested footprint already crosses the trim.
    proposal.options.maximum_fit_level = 255;
    let result = attachment.extrude_within_trim_band(REVISION, &proposal);
    assert!(matches!(
        result,
        Err(CylinderFootprintFailure::ControlOutside {
            axis: 0,
            region: SurfaceRegionId(3),
            ..
        })
    ));
}

#[test]
fn trim_checked_extrusion_cancels_before_loading_the_attachment() {
    let attachment = attachment(None);
    let proposal = request(&attachment, [-0.25, 0.25], [1., 2.]);
    let result =
        attachment.extrude_within_trim_band_with_cancel(SupportRevision(8), &proposal, || true);
    assert!(
        matches!(result, Err(CylinderFootprintFailure::Extrusion(cause))
        if cause.class() == WrappedFailureClass::Cancelled)
    );
}

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
fn round_support_bore_refuses_an_overlapping_footprint_even_on_another_turn() {
    let attachment = drilled_attachment();
    let turn = std::f64::consts::TAU * 10.;
    for start in [0., turn] {
        let proposal = request(&attachment, [14.5 + start, 17. + start], [1.1, 2.9]);
        let result = attachment.certify_footprint(REVISION, &proposal);
        assert!(
            matches!(
                result,
                Err(CylinderFootprintFailure::InnerTrimOverlap {
                    region: SurfaceRegionId(3),
                    wire: 1,
                    fitted: false,
                })
            ),
            "{result:?}"
        );
    }
}

#[test]
fn selected_cylinder_can_publish_a_pocket_away_from_its_support_bore() {
    let attachment = drilled_attachment();
    let proposal = request(&attachment, [1., 2.], [1., 2.]);
    let original = attachment.record().brep.clone();
    let feature = attachment
        .apply_wrapped_profile(REVISION, CylinderFeatureKind::Pocket, &proposal)
        .unwrap();
    assert_eq!(feature.tool().excluded_trim_wires(), 1);
    assert!(feature.validation().is_valid());
    assert_eq!(attachment.record().brep, original);
}

fn drilled_attachment() -> CylinderFaceAttachment {
    let support = BRepPrimAPIMakeCylinder::new(10., 4.)
        .shape()
        .unwrap()
        .clone();
    let cut = bore_support(&support);
    let face = selected_cylinder(&cut);
    let wires = Explorer::new(&face, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    assert!(wires.count() > 1);
    freeze(&cut)
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

pub(super) fn attachment(angle: Option<f64>) -> CylinderFaceAttachment {
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
pub(super) fn request(
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
