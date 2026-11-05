use super::argument_lists;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use martensite_tkbrep::topods::Shape;

#[test]
fn actual_source_factories_make_121_and_100_distinct_copies() {
    for (shapes, expected) in [
        (argument_lists::box_grid().unwrap(), 121),
        (argument_lists::cylinder_grid().unwrap(), 100),
    ] {
        assert_eq!(shapes.len(), expected);
        for (index, shape) in shapes.iter().enumerate() {
            assert!(shapes[..index].iter().all(|prior| !shape.is_same(prior)));
        }
    }
}

fn two_overlapping_boxes() -> (Vec<Shape>, Vec<Shape>) {
    let first = crate::primitive_shapes::box_sized([2., 2., 2.]).unwrap();
    let second = crate::primitive_shapes::translated(
        &crate::primitive_shapes::box_sized([2., 2., 2.]).unwrap(),
        [1., 0., 0.],
    )
    .unwrap();
    (vec![first], vec![second])
}

#[test]
fn actual_common_and_reverse_cut_match_closed_form_box_volumes() {
    let (objects, tools) = two_overlapping_boxes();
    for operation in [
        argument_lists::ArgumentListOperation::Common,
        argument_lists::ArgumentListOperation::Cut21,
    ] {
        let shape = argument_lists::boolean(operation, &objects, &tools).unwrap();
        assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
        let props = check_mass(&shape, &MassCheck::near(MassKind::Volume, 4.)).unwrap();
        assert!((props.mass() - 4.).abs() < 1e-12);
    }
}

#[test]
fn actual_general_fuse_matches_closed_form_union_volume() {
    let (objects, tools) = two_overlapping_boxes();
    let shape = argument_lists::general_fuse(&objects, &tools).unwrap();
    assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
    let props = check_mass(&shape, &MassCheck::near(MassKind::Volume, 12.)).unwrap();
    assert!((props.mass() - 12.).abs() < 1e-12);
}
