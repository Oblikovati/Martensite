//! A native bore leaves an inner trim wire on the selected conical face.
use super::*;
use crate::surface_sketch::WrappedFeatureKind;
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkernel::ncollection::List;
use martensite_tkmath::gp::StandardDirection3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier;

const SEAM: f64 = 0.7;

#[test]
fn conical_support_hole_refuses_a_filled_region_that_surrounds_it() {
    let attachment = drilled_cone();
    let chart = attachment.authoring_chart(REVISION, SEAM).unwrap();
    let mut request = proposal(&chart);
    let surround = [[1.35, 2.8], [1.35, 4.3], [1.8, 4.3], [1.8, 2.8]];
    request.profile = profile_at(&chart, surround, OutOfSupportPolicy::Refuse);
    request.proof.arithmetic.max_operations = 0;
    let result = chart.extrude_within_trim_band(REVISION, &request, directed());
    assert!(
        matches!(
            result,
            Err(ConeFootprintFailure::InnerTrimOverlap {
                region: SurfaceRegionId(19),
                wire: 1,
                depth_row: None,
            })
        ),
        "{}",
        result.err().unwrap()
    );
}

#[test]
fn conical_pocket_away_from_an_inner_support_trim_preserves_the_bore() {
    let attachment = drilled_cone();
    let chart = attachment.authoring_chart(REVISION, SEAM).unwrap();
    let mut request = proposal(&chart);
    request.support.normal = NormalSense::Inward;
    request.depths = [0., 0.2];
    request.options.spatial_error = 1e-5;
    let original = attachment.record().brep.clone();
    let feature = attachment
        .apply_wrapped_profile(
            REVISION,
            SEAM,
            WrappedFeatureKind::Pocket,
            &request,
            directed(),
        )
        .unwrap();
    assert_eq!(feature.tool().excluded_trim_wires(), 1);
    assert_eq!(attachment.record().brep, original);
    assert!(feature.validation().is_valid());
    verify_bore(&feature.export_native().unwrap().body);
}

fn verify_bore(body: &Shape) {
    let mut classifier = BRepClass3dSolidClassifier::with_shape(body).unwrap();
    classifier.perform(&Point3::new(0., 2., 3.5), 1e-7).unwrap();
    assert_eq!(classifier.state(), State::Out);
    classifier.perform(&Point3::new(0., 2., 4.), 1e-7).unwrap();
    assert_eq!(classifier.state(), State::In);
}

fn drilled_cone() -> ConeFaceAttachment {
    let body = BRepPrimAPIMakeCone::new(2., 4., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cut = bore(&body);
    let face = select_kind(&cut, SurfaceType::Cone);
    assert_eq!(
        Explorer::new(&face, ShapeEnum::Wire, ShapeEnum::Shape)
            .unwrap()
            .count(),
        2
    );
    let resolution = super::super::super::super::tests::resolution();
    ConeFaceAttachment::capture(ID, REVISION, &cut, &face, resolution).unwrap()
}

fn bore(body: &Shape) -> Shape {
    let axis = Axis2::with_standard_frame(
        Point3::new(0., 0., 3.5),
        StandardDirection3::Y,
        StandardDirection3::X,
    );
    let drill = BRepPrimAPIMakeCylinder::with_axes(axis, 0.25, 6.)
        .shape()
        .unwrap()
        .clone();
    let mut cut = BRepAlgoAPICut::new();
    cut.set_non_destructive(true);
    cut.set_arguments(&List::from_iter([body.clone()]));
    cut.set_tools(&List::from_iter([drill]));
    cut.build().unwrap();
    assert!(cut.is_done());
    cut.shape().clone()
}
