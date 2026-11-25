//! The lateral face originates in Circle → Extrude and survives a radial bore (#600).
use super::*;
use crate::surface_sketch::attachment::{
    CylinderAuthoringChart, DetachedCylinderSupport, OutOfSupportPolicy,
};
use martensite_cmath::{atan2, cos, sin};
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::State};
use martensite_tkmath::gp::{Axis2, Point3, StandardDirection3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_class3d::BRepClass3dSolidClassifier,
};
use std::f64::consts::TAU;

const RADIUS: f64 = 10.;
const BORE_Z: f64 = 1.25;
const BORE_RADIUS: f64 = 0.5;
const SEAM_TEST_Z: f64 = 2.75;
const DEPTH: f64 = 0.25;
const PREVIOUS: SupportRevision = SupportRevision(6);

struct CarriedCylinder {
    attachment: CylinderFaceAttachment,
    native: DetachedCylinderSupport,
    direct: bool,
}

fn previous_support(
    parent: &CylinderFaceAttachment,
) -> (CylinderFaceAttachment, DetachedCylinderSupport) {
    let mut record = parent.record().clone();
    record.revision = PREVIOUS;
    let previous = CylinderFaceAttachment::restore(record).unwrap();
    let native = previous.export_native(PREVIOUS).unwrap();
    (previous, native)
}

fn carry(
    previous: &CylinderFaceAttachment,
    original: &DetachedCylinderSupport,
    body: &Shape,
    history: &History,
    direct: bool,
) -> CarriedCylinder {
    let attachment = previous
        .reattach(original, body, REVISION, history)
        .unwrap();
    assert_eq!(attachment.record().id, previous.record().id);
    let native = attachment.export_native(REVISION).unwrap();
    let carried = CarriedCylinder {
        attachment,
        native,
        direct,
    };
    assert_eq!(
        carried.adaptor().cylinder().unwrap().direct().unwrap(),
        direct
    );
    carried
}

fn drill(parent: &CylinderFaceAttachment, direct: bool) -> CarriedCylinder {
    let (previous, original) = previous_support(parent);
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., BORE_Z),
        StandardDirection3::Y,
        StandardDirection3::X,
    );
    let tool = BRepPrimAPIMakeCylinder::with_axes(axes, BORE_RADIUS, 12.)
        .shape()
        .unwrap()
        .clone();
    let mut cut = BRepAlgoAPICut::new();
    cut.set_non_destructive(true);
    cut.set_arguments(&List::from_iter([original.body.clone()]));
    cut.set_tools(&List::from_iter([tool]));
    cut.build().unwrap();
    assert!(cut.is_done());
    carry(
        &previous,
        &original,
        cut.shape(),
        cut.history().unwrap(),
        direct,
    )
}

impl CarriedCylinder {
    fn adaptor(&self) -> BRepAdaptorSurface {
        BRepAdaptorSurface::with_face(&self.native.face, false).unwrap()
    }

    fn uv(&self, [x, y, z]: [f64; 3]) -> [f64; 2] {
        let cylinder = self.adaptor().cylinder().unwrap();
        let frame = cylinder.position();
        let [ox, oy, oz] = *frame.location().coordinates();
        let displacement = [x - ox, y - oy, z - oz];
        let dot = |axis: [f64; 3]| {
            axis.into_iter()
                .zip(displacement)
                .map(|(a, b)| a * b)
                .sum::<f64>()
        };
        [
            atan2(
                dot(*frame.y_direction().coordinates()),
                dot(*frame.x_direction().coordinates()),
            )
            .rem_euclid(TAU),
            dot(*frame.direction().coordinates()),
        ]
    }

    fn heights(&self, [low, high]: [f64; 2]) -> [f64; 2] {
        let frame = self
            .attachment
            .extrusion_support(REVISION, 0., NormalSense::Outward)
            .unwrap();
        assert_eq!(frame.axis, [0., 0., 1.]);
        [low - frame.origin[2], high - frame.origin[2]]
    }
}

fn hit(chart: &CylinderAuthoringChart, uv: [f64; 2]) -> (State, [f64; 2]) {
    let result = chart
        .develop_parameters(REVISION, uv, OutOfSupportPolicy::Report)
        .unwrap();
    (
        result.trim_state,
        result.coordinates.map(|value| value.value),
    )
}

fn check_window(drilled: &CarriedCylinder, chart: &CylinderAuthoringChart) {
    let [u, v] = drilled.uv([0., RADIUS, BORE_Z]);
    let [mirror_u, mirror_v] = drilled.uv([0., RADIUS, 4. - BORE_Z]);
    assert_eq!(u, mirror_u);
    assert_eq!(hit(chart, [u, v]).0, State::Out);
    assert_eq!(hit(chart, [u, mirror_v]).0, State::In);
    let error = window_footprint(drilled, u, BORE_Z).unwrap_err();
    assert!(
        matches!(
            error,
            CylinderFootprintFailure::InnerTrimOverlap {
                wire: 1,
                fitted: false,
                ..
            }
        ),
        "{error:?}"
    );
    window_footprint(drilled, u, 4. - BORE_Z).unwrap();
}

fn window_footprint(
    stock: &CarriedCylinder,
    u: f64,
    height: f64,
) -> Result<CylinderFootprintReport, CylinderFootprintFailure> {
    let arc = RADIUS * u;
    let sketch = request(
        &stock.attachment,
        [arc - 0.1, arc + 0.1],
        stock.heights([height - 0.1, height + 0.1]),
    );
    stock.attachment.certify_footprint(REVISION, &sketch)
}

fn check_seam(drilled: &CarriedCylinder, chart: &CylinderAuthoringChart) {
    let [_, v] = drilled.uv([RADIUS, 0., SEAM_TEST_Z]);
    let [height, _] = drilled.heights([SEAM_TEST_Z, SEAM_TEST_Z]);
    assert_eq!(v, if drilled.direct { height } else { -height });
    let left = TAU - 0.025;
    let right = 0.025;
    let (left_state, left_coordinates) = hit(chart, [left, v]);
    let (right_state, right_coordinates) = hit(chart, [right, v]);
    assert_eq!((left_state, right_state), (State::In, State::In));
    assert_eq!(left_coordinates, [RADIUS * left, v]);
    assert_eq!(right_coordinates, [RADIUS * right, v]);
    let left_chart = drilled.attachment.authoring_chart(REVISION, TAU).unwrap();
    assert_eq!(
        hit(&left_chart, [left, v]),
        (State::In, [RADIUS * (left - TAU), v])
    );
}

fn state(body: &Shape, point: Point3) -> State {
    BRepClass3dSolidClassifier::with_point(body, &point, resolution().sew)
        .unwrap()
        .state()
}

fn verify_publication(
    drilled: &CarriedCylinder,
    kind: CylinderFeatureKind,
    normal: NormalSense,
    proposal: &WrappedNormalRequest,
) {
    let result = drilled
        .attachment
        .apply_wrapped_profile(REVISION, kind, proposal)
        .unwrap();
    assert!(result.validation().is_valid());
    assert_eq!(result.tool().excluded_trim_wires(), 1);
    assert_eq!(result.tool().footprint().angular, None);
    let native = result.export_native().unwrap();
    check_history(&result, &native);
    check_material(drilled, &native.body, normal);
    check_replay(drilled, kind, proposal, &result);
}

fn check_material(stock: &CarriedCylinder, result: &Shape, normal: NormalSense) {
    let [start, end] = match normal {
        NormalSense::Outward => [State::Out, State::In],
        NormalSense::Inward => [State::In, State::Out],
    };
    let [_, native_v] = stock.uv([RADIUS, 0., SEAM_TEST_Z]);
    let surface = stock.adaptor().value(0.01, native_v).unwrap();
    let radius = RADIUS
        + if normal == NormalSense::Outward {
            DEPTH / 2.
        } else {
            -DEPTH / 2.
        };
    let scale = radius / RADIUS;
    let point = Point3::new(scale * surface.x(), scale * surface.y(), SEAM_TEST_Z);
    assert_eq!(state(&stock.native.body, point), start);
    assert_eq!(state(result, point), end);
    assert_eq!(
        state(result, Point3::new(0., RADIUS - 0.2, BORE_Z)),
        State::Out
    );
}

fn check_replay(
    stock: &CarriedCylinder,
    kind: CylinderFeatureKind,
    proposal: &WrappedNormalRequest,
    result: &CylinderFeatureResult,
) {
    let restored = CylinderFaceAttachment::restore(stock.attachment.record().clone()).unwrap();
    let replay = restored
        .apply_wrapped_profile(REVISION, kind, proposal)
        .unwrap();
    assert_eq!(replay.history(), result.history());
    assert_eq!(replay.validation(), result.validation());
    assert!(replay.encoded_brep() == result.encoded_brep());
}

#[test]
fn indirect_shaped_inner_trim_matches_direct_control_for_boss_and_pocket() {
    for (stock, direct) in [
        (attachment(None), true),
        (indirect_extruded_circle(), false),
    ] {
        let drilled = drill(&stock, direct);
        let record = drilled.attachment.record().brep.clone();
        let chart = drilled.attachment.authoring_chart(REVISION, 0.).unwrap();
        check_window(&drilled, &chart);
        check_seam(&drilled, &chart);
        for (kind, normal) in [
            (CylinderFeatureKind::Boss, NormalSense::Outward),
            (CylinderFeatureKind::Pocket, NormalSense::Inward),
        ] {
            let mut proposal = request(
                &drilled.attachment,
                [-0.25, 0.25],
                drilled.heights([SEAM_TEST_Z - 0.15, SEAM_TEST_Z + 0.15]),
            );
            proposal.support.normal = normal;
            proposal.depths = [0., DEPTH];
            verify_publication(&drilled, kind, normal, &proposal);
        }
        assert!(record == drilled.attachment.record().brep);
    }
}

fn slope_prism() -> Shape {
    use martensite_tkmath::gp::Vector3;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
    use martensite_tktopalgo::brep_builder_api::{
        BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon,
    };
    let mut wire = BRepBuilderAPIMakePolygon::new();
    for [x, z] in [[1., -1.], [1., 1.62], [12., 2.94], [12., -1.]] {
        wire.add_point(Point3::new(x, 3., z)).unwrap();
    }
    wire.close().unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&wire.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap();
    BRepPrimAPIMakePrism::new(&face, Vector3::new(0., 3., 0.), false, true)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn clipped_lateral(parent: &CylinderFaceAttachment, direct: bool) -> CarriedCylinder {
    use martensite_tkbo::brep_algo_api::BRepAlgoAPICommon;
    let (previous, original) = previous_support(parent);
    let mut common = BRepAlgoAPICommon::new();
    common.set_non_destructive(true);
    common.set_arguments(&List::from_iter([original.body.clone()]));
    common.set_tools(&List::from_iter([slope_prism()]));
    common.build().unwrap();
    assert!(common.is_done());
    carry(
        &previous,
        &original,
        common.shape(),
        common.history().unwrap(),
        direct,
    )
}

#[test]
fn indirect_shaped_outer_trim_from_an_oblique_native_cut() {
    let angle = 0.61;
    let [x, y] = [RADIUS * cos(angle), RADIUS * sin(angle)];
    let ceiling = 1.5 + 0.12 * x;
    assert!((3.0..6.0).contains(&y));
    for (stock, direct) in [
        (attachment(None), true),
        (indirect_extruded_circle(), false),
    ] {
        let clipped = clipped_lateral(&stock, direct);
        let [u, inside_v] = clipped.uv([x, y, ceiling - 0.2]);
        let [_, outside_v] = clipped.uv([x, y, ceiling + 0.08]);
        let chart = clipped.attachment.authoring_chart(REVISION, 0.).unwrap();
        assert_eq!(hit(&chart, [u, inside_v]).0, State::In);
        assert_eq!(hit(&chart, [u, outside_v]).0, State::Out);
        let arc = RADIUS * u;
        let inside = request(
            &clipped.attachment,
            [arc - 0.02, arc + 0.02],
            clipped.heights([ceiling - 0.3, ceiling - 0.2]),
        );
        let checked = clipped.attachment.certify_footprint(REVISION, &inside);
        assert!(
            matches!(
                checked,
                Err(CylinderFootprintFailure::UnsupportedTrim {
                    expected: "exact representable outer pcurve endpoints",
                })
            ),
            "direct={direct}: {checked:?}"
        );
    }
}
