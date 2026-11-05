//! The `removefeatures` grid's `begin`: the `CheckIsFeatureRemoved` procedure of its scripts.
use super::commands::distinct_parts;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topods::{Compound, Shape},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `CheckIsFeatureRemoved feature {kinds}`: one `Error` line for each sub-shape of the feature,
/// of those kinds, that the history of the removal does not mark as deleted.
/// Example: `check_is_feature_removed(&history, &feature, &[ShapeEnum::Face])`.
pub fn check_is_feature_removed(history: &History, feature: &Shape, kinds: &[ShapeEnum]) {
    for _ in 0..parts_not_removed(history, feature, kinds) {
        println!("Error: Feature is not removed");
    }
}

/// How many sub-shapes of those kinds the history does not mark as deleted.
/// Example: `parts_not_removed(&history, &feature, &[ShapeEnum::Edge]) == 0`.
pub fn parts_not_removed(history: &History, feature: &Shape, kinds: &[ShapeEnum]) -> usize {
    let wrapper = compound_of(feature);
    let parts = kinds
        .iter()
        .flat_map(|kind| distinct_parts(&wrapper, *kind));
    parts
        .filter(|part| !history.is_removed(part).unwrap())
        .count()
}

/// `compound feature wrapper`: the explored shape is never the feature itself.
fn compound_of(feature: &Shape) -> Shape {
    let builder = BRepBuilder::default();
    let mut wrapper = Compound::default();
    builder.make_compound(&mut wrapper);
    builder.add(&mut wrapper, feature).unwrap();
    wrapper.into_shape()
}

#[test]
fn the_count_follows_the_history_part_by_part() {
    use martensite_tkmath::gp::Point3;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    let mut maker = BRepPrimAPIMakeBox::at_point(Point3::new(0., 0., 0.), 1., 1., 1.).unwrap();
    let cube = maker.shape().unwrap().clone();
    let faces = distinct_parts(&cube, ShapeEnum::Face);
    let history = History::new();
    assert_eq!(parts_not_removed(&history, &cube, &[ShapeEnum::Face]), 6);
    history.remove(&faces[0]).unwrap();
    assert_eq!(parts_not_removed(&history, &cube, &[ShapeEnum::Face]), 5);
    let kinds = [ShapeEnum::Face, ShapeEnum::Solid];
    assert_eq!(parts_not_removed(&history, &cube, &kinds), 6);
}
