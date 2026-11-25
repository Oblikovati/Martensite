//! A detached directed tool rebinds independently and refuses changed native ownership.
use super::*;
use crate::surface_sketch::wrapped_profile::{BindingKey, Source};
use crate::surface_sketch::{
    HistoryRole, WrappedConeArithmeticGrantPolicy, WrappedConeProofOptions, WrappedRegionSelection,
    attachment::ConeFaceAttachment,
};
use martensite_tkbo::profile_extrusion::{
    DisplayTolerance, LoopUse, PlanarProofVersion, PlanarProofWorkLimit, ProfileCurve,
    ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex, ProfileVertexId, Resolution,
    SketchProfile, ToleranceCap,
};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis2, Point3, StandardDirection3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const REVISION: SupportRevision = SupportRevision(3);
const REGION: SurfaceRegionId = SurfaceRegionId(7);
const SELECTED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarAndCapCyclesV9 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
        authored_max_components: 64,
        authored_max_subintervals: 1024,
    };

#[test]
fn attachment_tool_rebinds_a_fresh_exact_native_tool_and_rejects_changed_datum() {
    let attachment = stock();
    let chart = attachment.authoring_chart(REVISION, 2.5).unwrap();
    let request = request(&chart);
    let before = request.clone();
    let tool = chart
        .materialize_attachment_tool(REVISION, &request, SELECTED, &mut || false)
        .unwrap();
    let checked = chart
        .extrude_within_trim_band(REVISION, &request, SELECTED)
        .unwrap();
    let native = checked.extrusion().export_native().unwrap();
    assert!(!native.bodies.is_partner(&tool.native().bodies));
    let rebound = tool.rebind(&native).unwrap();
    assert_eq!(tool.regions(), &[REGION]);
    assert_eq!(tool.history(), checked.extrusion().canonical().history());
    assert_eq!(
        tool.spatial_budget(),
        &checked.extrusion().placement().spatial_budget
    );
    assert_eq!(rebound.certified_body().native_census().members, 1);
    assert_eq!(tool.work().authored.loops, 1);
    assert_eq!(tool.work().caps.len(), 1);
    let (_, _, spans) = tool.fitted(REGION).unwrap();
    let key = BindingKey {
        source: Source::Span(spans[0].span),
        role: HistoryRole::FirstDepthEdge,
    };
    assert!(
        rebound
            .cap_residual_upper(key)
            .is_some_and(|upper| upper.is_finite() && upper >= 0.)
    );
    assert!(
        rebound
            .cap_residual_upper(BindingKey {
                role: HistoryRole::SideFace,
                ..key
            })
            .is_none()
    );
    assert_eq!(rebound.cap_residuals().count(), spans.len() * 2);
    let work = tool.work();
    assert!(work.arithmetic_operations >= work.actual_operations);
    assert_eq!(
        work.planar.consumed_pair_bisections,
        work.authored.pair.consumed_pair_bisections
            + work.caps[&REGION].pair.consumed_pair_bisections,
    );
    assert_eq!(
        u64::from(work.integration_cells),
        work.authored.integration_cells + work.caps[&REGION].integration_cells,
    );

    let changed = changed_datum(native);
    assert!(tool.rebind(&changed).is_err());
    let independent = checked.extrusion().export_native().unwrap();
    assert!(tool.rebind(&swapped_labels(independent)).is_err());
    assert!(tool.rebind(tool.native()).is_ok());
    assert_eq!(request, before);
}

#[test]
fn attachment_tool_reports_volume_policy_and_cooperative_cancellation() {
    let attachment = stock();
    let chart = attachment.authoring_chart(REVISION, 2.5).unwrap();
    let mut request = request(&chart);
    let cancelled = chart.materialize_attachment_tool(REVISION, &request, SELECTED, &mut || true);
    assert!(
        matches!(cancelled, Err(ConeFootprintFailure::Extrusion(cause))
        if cause.stage() == WrappedStage::Admission && cause.class() == WrappedFailureClass::Cancelled)
    );
    request.options.volume = WrappedVolumePolicy::Refuse(0.);
    let refused = chart.materialize_attachment_tool(REVISION, &request, SELECTED, &mut || false);
    assert!(
        matches!(refused, Err(ConeFootprintFailure::Extrusion(cause))
        if cause.stage() == WrappedStage::Admission && cause.class() == WrappedFailureClass::Unsupported)
    );
}

fn swapped_labels(mut native: DetachedWrappedBodies) -> DetachedWrappedBodies {
    let first_cap_edges = native
        .entities
        .keys()
        .filter(|key| key.role == HistoryRole::FirstDepthEdge);
    let [a, b] = first_cap_edges
        .take(2)
        .copied()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let a_shape = native.entities.remove(&a).unwrap();
    let b_shape = native.entities.remove(&b).unwrap();
    native.entities.insert(a, b_shape);
    native.entities.insert(b, a_shape);
    native
}

fn changed_datum(mut native: DetachedWrappedBodies) -> DetachedWrappedBodies {
    let mut shift = Transform3::default();
    shift.set_translation_part(Vector3::new(0., 1., 0.));
    native.bodies = native.bodies.moved(&Location::new(shift), true).unwrap();
    native
}

fn request(chart: &ConeAuthoringChart) -> WrappedConeNormalRequest {
    let pieces = sources();
    let mut proof = WrappedConeProofOptions::default();
    proof.integration.behavior_version = 2;
    proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: u64::from(proof.integration.max_total_subintervals),
    };
    WrappedConeNormalRequest {
        support: chart
            .extrusion_support(REVISION, NormalSense::Outward)
            .unwrap(),
        profile: SketchProfile {
            plane: ProfilePlane::world_xy(),
            vertices: pieces.into_iter().map(SourcePiece::vertex).collect(),
            curves: pieces.into_iter().map(SourcePiece::curve).collect(),
        },
        selections: vec![WrappedRegionSelection {
            region: REGION,
            outer: ProfileLoop {
                uses: pieces.into_iter().map(SourcePiece::curve_use).collect(),
            },
            holes: vec![],
        }],
        depths: [0., 0.2],
        options: WrappedNormalOptions {
            spatial_error: 1e-4,
            maximum_fit_level: 8,
            planar_work: PlanarProofWorkLimit::PairBisections(8192),
            volume: WrappedVolumePolicy::Report,
        },
        proof,
    }
}

#[derive(Clone, Copy)]
struct SourcePiece {
    id: u64,
    next: u64,
    point: [f64; 2],
}

fn sources() -> [SourcePiece; 4] {
    let pieces = [[94., -1.], [96., -1.], [96., 1.], [94., 1.]];
    std::array::from_fn(|index| SourcePiece {
        id: index as u64 + 1,
        next: (index as u64 + 1) % 4 + 1,
        point: pieces[index],
    })
}

impl SourcePiece {
    fn vertex(self) -> ProfileVertex {
        ProfileVertex {
            id: ProfileVertexId(self.id),
            point: self.point,
        }
    }
    fn curve(self) -> ProfileCurve {
        ProfileCurve::Line {
            id: ProfileCurveId(self.id),
            start: ProfileVertexId(self.id),
            end: ProfileVertexId(self.next),
        }
    }
    fn curve_use(self) -> LoopUse {
        LoopUse {
            curve: ProfileCurveId(self.id),
            reversed: false,
        }
    }
}

fn stock() -> ConeFaceAttachment {
    let (sine, cosine) = martensite_cmath::sincos(0.125);
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., 85. * cosine),
        StandardDirection3::Z,
        StandardDirection3::X,
    );
    let mut maker =
        BRepPrimAPIMakeCone::with_axes(axes, 85. * sine, 115. * sine, 30. * cosine).unwrap();
    let body = maker.shape().unwrap();
    let resolution = Resolution::new(
        1e-5,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    );
    ConeFaceAttachment::capture(
        FaceAttachmentId(44),
        REVISION,
        body,
        &lateral_face(body),
        resolution,
    )
    .unwrap()
}

fn lateral_face(body: &topods::Shape) -> topods::Face {
    let mut faces = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        let face = topods::face(faces.current()).unwrap();
        if BRepAdaptorSurface::with_face(&face, false)
            .unwrap()
            .get_type()
            .unwrap()
            == SurfaceType::Cone
        {
            return face;
        }
        faces.next().unwrap();
    }
    panic!("expected an analytic lateral cone face");
}
