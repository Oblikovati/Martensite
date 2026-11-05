//! Native BRepAlgoAsDes association state and offset-intersection dependencies.
use martensite_tkbool::brep_algo::BRepAlgoAsDes;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shape, Vertex},
};
use martensite_tkernel::{
    ncollection::List,
    standard::{Failure, RuntimeTyped},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;

#[test]
fn ancestry_mutation_and_failures_match_pinned_native_state() -> Result<(), Failure> {
    let [a, b, c, d, e] = vertices()?;
    let mut associations = BRepAlgoAsDes::default();
    add_repeated_descendants(&mut associations, &a, &b, &c);
    assert_initial_state(&mut associations, &a, &b, &c, &d)?;
    replace_and_remove(&mut associations, &a, &b, &d, &e)?;
    assert_add_many(&mut associations, &a, &b, &c, &d, &e);
    Ok(())
}

#[test]
fn replace_merges_into_existing_destination_without_losing_links() -> Result<(), Failure> {
    let [a, b, _, d, e] = vertices()?;
    let mut associations = BRepAlgoAsDes::new();
    associations.add(&a, &b);
    associations.add(&e, &d);
    associations.replace(&a, &e);
    assert_eq!(associations.descendant(&e).count(), 2);
    assert_eq!(associations.ascendant(&b).count(), 1);
    assert!(associations.ascendant(&b).any(|shape| shape.is_same(&e)));
    Ok(())
}

#[test]
fn shared_handle_mutators_match_the_exclusive_ones() -> Result<(), Failure> {
    let [a, b, c, d, e] = vertices()?;
    let shared = martensite_tkernel::standard::Handle::new(BRepAlgoAsDes::new());
    let associations = shared.get().expect("live AsDes");
    let mut many = List::default();
    many.append(b.clone());
    many.append(c.clone());
    associations.add_many_shared(&a, &many);
    let edited = associations.with_change_descendant(&a, |list| {
        list.iter_mut().for_each(|s| s.reverse());
        list.len()
    });
    assert_eq!(edited, 2);
    let unbound_count = associations.with_change_descendant(&d, |list| {
        list.append(d.clone());
        list.len()
    });
    assert_eq!(unbound_count, 1);
    assert!(!associations.has_descendant(&d));
    assert!(
        associations
            .descendant(&a)
            .all(|s| s.orientation() == Orientation::Reversed)
    );
    associations.replace_shared(&a, &e);
    assert!(!associations.has_descendant(&a));
    assert_eq!(associations.descendant(&e).count(), 2);
    associations.remove_shared(&b)?;
    assert_eq!(associations.descendant(&e).count(), 1);
    assert!(associations.remove_shared(&e).is_err());
    associations.clear_shared();
    assert!(!associations.has_descendant(&e) && !associations.has_ascendant(&c));
    Ok(())
}

fn add_repeated_descendants(associations: &mut BRepAlgoAsDes, a: &Shape, b: &Shape, c: &Shape) {
    associations.add(a, &b.oriented(Orientation::Reversed));
    associations.add(c, b);
    associations.add(a, b);
}

fn assert_common_descendants(associations: &BRepAlgoAsDes, a: &Shape, b: &Shape, c: &Shape) {
    let mut common = List::default();
    common.append(b.clone());
    assert!(associations.has_common_descendant(a, c, &mut common));
    assert_eq!(common.len(), 2);
    assert_eq!(common.iter().filter(|shape| shape.is_same(b)).count(), 2);
}

fn assert_initial_state(
    associations: &mut BRepAlgoAsDes,
    a: &Shape,
    b: &Shape,
    c: &Shape,
    d: &Shape,
) -> Result<(), Failure> {
    assert!(associations.has_descendant(a) && associations.has_ascendant(b));
    assert_eq!(associations.descendant(a).count(), 2);
    assert_eq!(associations.ascendant(b).count(), 3);
    associations.change_descendant(d).append(d.clone());
    assert!(!associations.has_descendant(d));
    assert_eq!(associations.change_descendant(d).len(), 1);
    assert_common_descendants(associations, a, b, c);
    let error = associations.remove(a).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ConstructionError");
    Ok(())
}

fn replace_and_remove(
    associations: &mut BRepAlgoAsDes,
    a: &Shape,
    b: &Shape,
    d: &Shape,
    e: &Shape,
) -> Result<(), Failure> {
    associations.change_descendant(a).append(d.clone());
    assert!(!associations.has_ascendant(d));
    associations.replace(a, e);
    assert_eq!(associations.descendant(e).count(), 3);
    assert_eq!(associations.ascendant(b).count(), 2);
    associations.remove(b)?;
    assert!(!associations.has_ascendant(b));
    assert_eq!(associations.descendant(e).count(), 2);
    Ok(())
}

fn assert_add_many(
    associations: &mut BRepAlgoAsDes,
    a: &Shape,
    b: &Shape,
    c: &Shape,
    d: &Shape,
    e: &Shape,
) {
    associations.clear();
    assert_eq!(BRepAlgoAsDes::static_type().name(), "BRepAlgoAsDes");
    assert!(!associations.has_descendant(e));
    // The shared empty list is independent of Clear().
    assert_eq!(associations.change_descendant(d).len(), 1);
    associations.add_many(a, &List::from_iter([b.clone(), c.clone()]));
    assert_eq!(associations.descendant(a).count(), 2);
    assert_eq!(
        associations.remove(d).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
}

fn vertices() -> Result<[Shape; 5], Failure> {
    let builder = BRepBuilder::default();
    let points = [0., 1., 2., 3., 4.];
    let mut result = Vec::new();
    for x in points {
        let mut vertex = Vertex::default();
        builder.make_vertex_at(&mut vertex, Point3::new(x, 0., 0.), 1e-7)?;
        result.push(vertex.into_shape());
    }
    result.try_into().map_err(|shapes: Vec<Shape>| {
        Failure::with_message(
            "Standard_ConstructionError",
            &format!("built {} vertices; expected 5", shapes.len()),
        )
    })
}

#[test]
fn common_descendants_preserve_both_lists_multiplicity() -> Result<(), Failure> {
    let [a, b, c, _, _] = vertices()?;
    let mut associations = BRepAlgoAsDes::new();
    associations.add(&a, &b.oriented(Orientation::Reversed));
    associations.add(&a, &b);
    associations.add(&c, &b);
    associations.add(&c, &b);
    let mut common = List::default();
    assert!(associations.has_common_descendant(&a, &c, &mut common));
    let orientations: Vec<_> = common.iter().map(Shape::orientation).collect();
    assert_eq!(
        orientations,
        [
            Orientation::Reversed,
            Orientation::Reversed,
            Orientation::Forward,
            Orientation::Forward
        ]
    );
    Ok(())
}

#[test]
fn common_descendants_clear_output_when_the_second_key_is_absent() -> Result<(), Failure> {
    let [a, b, c, _, _] = vertices()?;
    let mut associations = BRepAlgoAsDes::new();
    associations.add(&a, &b);
    assert!(!associations.has_descendant(&c));
    let mut common = List::from_iter([b]);
    assert!(!associations.has_common_descendant(&a, &c, &mut common));
    assert!(common.is_empty());
    Ok(())
}

#[test]
fn remove_refuses_sources_and_missing_leaves_with_the_defined_message() -> Result<(), Failure> {
    let [a, b, c, _, _] = vertices()?;
    let mut associations = BRepAlgoAsDes::new();
    associations.add(&a, &b);
    for refused in [&a, &c] {
        let failure = associations.remove(refused).expect_err("refused shape");
        assert_eq!(failure.exception_type(), "Standard_ConstructionError");
        assert_eq!(failure.message(), " BRepAlgoAsDes::Remove");
    }
    assert!(associations.has_descendant(&a));
    assert!(associations.has_ascendant(&b));
    Ok(())
}
