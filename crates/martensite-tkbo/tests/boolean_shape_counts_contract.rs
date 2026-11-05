//! Native-qualified nested containers separate typed exploration from default storage counts.
#[allow(dead_code)] // This dedicated host checks getters without registering controls in native hosts.
#[path = "boolean_shape_counts.rs"]
mod boolean_shape_counts;
#[allow(dead_code)] // Existing default full-count entry points remain unchanged and covered elsewhere.
#[path = "support/default_shape_counts.rs"]
mod default_shape_counts;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};

fn nested_empty_compounds() -> Shape {
    let builder = BRepBuilder::default();
    let mut inner = Compound::default();
    let mut outer = Compound::default();
    builder.make_compound(&mut inner);
    builder.make_compound(&mut outer);
    builder.add(&mut outer, &inner).unwrap();
    outer.into_shape()
}

#[test]
fn typed_exploration_stops_at_the_matching_outer_compound() {
    let actual = boolean_shape_counts::counts(&nested_empty_compounds());
    // Independent native compound(inner)/compound(inner,outer), nbshapes outer -t gives1/1.
    assert_eq!(
        actual,
        [0, 0, 0, 0, 0, 0, 0, 1, 1],
        "native typed Compound1/Shape1"
    );
}

#[test]
fn default_storage_keeps_both_nested_compounds() {
    let shape = nested_empty_compounds();
    let actual = default_shape_counts::counts(&shape);
    assert_eq!(
        actual,
        [2, 0, 0, 0, 0, 0, 0, 0, 2],
        "native default Compound2/Shape2"
    );
    default_shape_counts::assert_vertex_first_counts(&shape, &[0, 0, 0, 0, 0, 0, 0, 2, 2]);
}
