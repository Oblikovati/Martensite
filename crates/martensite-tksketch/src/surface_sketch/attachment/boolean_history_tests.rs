//! A real non-destructive cut supplies history; attachment does not invent face matching.
use super::tests::capture;
use super::*;
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkernel::ncollection::List;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn upper_half_tool() -> Shape {
    let axes = Axis2::new(
        Point3::new(0., 0., 2.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    BRepPrimAPIMakeCylinder::with_axes(axes, 20., 4.)
        .shape()
        .unwrap()
        .clone()
}
fn cut(source: &Shape) -> BRepAlgoAPICut {
    let mut cut = BRepAlgoAPICut::new();
    cut.set_non_destructive(true);
    cut.set_arguments(&List::from_iter([source.clone()]));
    cut.set_tools(&List::from_iter([upper_half_tool()]));
    cut.build().unwrap();
    assert!(!cut.has_errors());
    cut
}
#[test]
fn actual_cut_history_rebinds_a_trimmed_cylindrical_face() {
    let attachment = capture();
    let source = attachment.export_native(SupportRevision(3)).unwrap();
    let cut = cut(&source.body);
    let history = cut.history().unwrap();
    let rebound = attachment
        .reattach(&source, cut.shape(), SupportRevision(4), history)
        .unwrap();
    assert_eq!(rebound.record().id, attachment.record().id);
    assert_eq!(rebound.record().revision, SupportRevision(4));
    assert!(rebound.validation().is_valid());
    let attempt = Attempt::new(
        AttachmentOperation::Export,
        FaceAttachmentId(7),
        SupportRevision(3),
    );
    let source_bytes = codec::encode(&attempt, &source.body, &source.face).unwrap();
    assert_eq!(source_bytes, attachment.record().brep);
}
