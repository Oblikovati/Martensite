//! Pure topology controls compare the actual untyped child bindings, never their coordinates.
use super::shapes;
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkg3d::top_abs::ShapeEnum;
#[test]
fn cones_bind_actual_shell_children_then_face_children() {
    let registry = shapes::cone_explode_bindings().unwrap();
    for name in ["c1", "c2", "c3", "c4"] {
        let shell = &registry[&format!("{name}_1")];
        assert_eq!(shell.shape_type().unwrap(), ShapeEnum::Shell);
        let children: Vec<_> = ShapeIterator::new(shell, true, true).collect();
        for (index, child) in children.iter().enumerate() {
            let bound = &registry[&format!("{name}_1_{}", index + 1)];
            assert!(bound.is_same(child));
            assert_eq!(bound.orientation(), child.orientation());
            assert_eq!(bound.shape_type().unwrap(), ShapeEnum::Face);
        }
    }
}
