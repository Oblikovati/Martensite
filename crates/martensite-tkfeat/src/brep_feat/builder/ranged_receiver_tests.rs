//! Actual ranged feature receiver and removed-face identity controls.

use super::*;
use martensite_tkbo::bop_algo::AlertUserBreak;
use martensite_tkernel::standard::RuntimeTyped;
#[test]
fn ranged_feature_receiver_clears_its_report_before_actual_base_failure() {
    let mut builder = BRepFeatBuilder::new();
    let alert: martensite_tkernel::message::AlertHandle = std::sync::Arc::new(AlertUserBreak);
    builder.bop.options().add_error(&alert);
    let mut drafts = SameDomainShapes::with_hasher(martensite_tkbrep::toptools::ShapeMapHasher, 1);
    let range = martensite_tkernel::message::ProgressRange::default();
    assert!(
        BuilderHooks::fill_in_3d_parts_with_progress(
            &mut builder,
            &mut DataStructure::default(),
            &mut drafts,
            &range
        )
        .is_err()
    );
    assert!(
        !builder
            .bop
            .options()
            .has_error(AlertUserBreak::static_type())
    );
}
#[test]
fn removed_face_filter_preserves_kept_face_identity() {
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let face = || {
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 1., 0., 1.))
            .unwrap()
            .face()
            .unwrap()
            .into_shape()
    };
    let solid = face();
    let removed = face();
    let kept = face();
    let mut builder = BRepFeatBuilder::new();
    builder.removed.add(removed.clone());
    let mut parts = List::default();
    parts.append(removed);
    parts.append(kept.clone());
    builder.bop.in_parts_mut().bound(solid.clone(), parts);
    builder.filter_removed_in_parts();
    let parts = builder.bop.in_parts().seek(&solid).unwrap();
    assert_eq!(parts.len(), 1);
    assert!(parts.iter().next().unwrap().is_same(&kept));
}

#[test]
fn actual_ranged_feature_classification_filters_removed_interior_face() {
    let (mut builder, mut filler, solid) = removed_interior_face_case();
    let mut drafts = SameDomainShapes::with_hasher(martensite_tkbrep::toptools::ShapeMapHasher, 1);
    let completion = BuilderHooks::fill_in_3d_parts_with_progress(
        &mut builder,
        filler.data_structure_mut().unwrap(),
        &mut drafts,
        &martensite_tkernel::message::ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(
        completion,
        martensite_tkbo::bop_algo::ProgressCompletion::Completed
    );
    assert!(!builder.bop.options().has_errors());
    assert_eq!(drafts.len(), 1);
    assert_eq!(builder.bop.in_parts().len(), 1);
    assert!(builder.bop.in_parts().seek(&solid).unwrap().is_empty());
}

fn removed_interior_face_case() -> (BRepFeatBuilder, BOPAlgoPaveFiller, Shape) {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let solid = maker.shape().unwrap().clone();
    let plane = Plane3::new(Point3::new(1., 1., 2.), Direction3::Z).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-0.25, 0.25, -0.25, 0.25))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&solid);
    filler.add_argument(&face);
    filler.initialize().unwrap();
    let mut builder = BRepFeatBuilder::new();
    // The real filler-adoption path owns context initialization; no private base-state substitute.
    BOPAlgoBuilder::perform_with_filler(&mut builder.bop, &mut filler);
    assert!(builder.bop.options().has_errors());
    builder.removed.add(face);
    (builder, filler, solid)
}

#[test]
fn composed_ranged_feature_keeps_virtual_filter_before_solid_splitting() {
    let (mut builder, mut filler, solid) = removed_interior_face_case();
    let completion = martensite_tkbo::bop_algo::fill_images_solids_with_progress(
        &mut builder,
        filler.data_structure_mut().unwrap(),
        &martensite_tkernel::message::ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(
        completion,
        martensite_tkbo::bop_algo::ProgressCompletion::Completed
    );
    assert!(!builder.bop.options().has_errors());
    assert!(builder.bop.in_parts().seek(&solid).unwrap().is_empty());
    let images = builder.bop.images().seek(&solid).unwrap();
    assert_eq!(images.len(), 1);
    let image = images.iter().next().unwrap();
    assert_eq!(
        Explorer::new(image, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .count(),
        6
    );
    let origins = builder.bop.origins().seek(image).unwrap();
    assert_eq!(origins.len(), 1);
    assert!(origins.iter().next().unwrap().is_same(&solid));
}
