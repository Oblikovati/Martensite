use super::*;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkg3d::{geom::GeomSphericalSurface, top_abs::ShapeEnum};

fn analytic_sphere(shape: &Shape) -> ([f64; 3], f64) {
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    assert!(faces.more());
    let face = topods::face(faces.current()).unwrap();
    let support = BRepTool::surface(&face)
        .unwrap()
        .cast::<GeomSphericalSurface>();
    assert!(
        !support.is_null(),
        "sphere face; expected exact spherical support"
    );
    let sphere = support.sphere().unwrap();
    let center = sphere.location();
    faces.next().unwrap();
    assert!(!faces.more(), "sphere topology; expected one face");
    ([center.x(), center.y(), center.z()], sphere.radius())
}

#[test]
fn six_real_sphere_supports_keep_native_centers_radii_and_placements() {
    let session = child_cpu_session();
    let scene = recompute::displayed_sphere_children(session.context()).unwrap();
    assert_eq!(scene.children.len(), 5);
    for (index, object) in scene_objects(&scene).iter().enumerate() {
        let shape = object.shape();
        assert!(shape.location().is_identity());
        assert_eq!(analytic_sphere(&shape), ([index as f64; 3], 1.));
        assert!(
            !scene_objects(&scene)[..index]
                .iter()
                .any(|old| old.shape().is_same(&shape))
        );
    }
}

fn assert_origin_box_vertices(shape: &Shape) {
    let mut vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    let mut corners = Vec::new();
    while vertices.more() {
        let vertex = topods::vertex(vertices.current()).unwrap();
        let point = BRepTool::pnt(&vertex).unwrap();
        let corner = [point.x(), point.y(), point.z()];
        assert!(corner.iter().all(|value| *value == 0. || *value == 20.));
        if !corners.contains(&corner) {
            corners.push(corner);
        }
        vertices.next().unwrap();
    }
    assert_eq!(corners.len(), 8);
}

#[test]
fn discarded_located_copy_does_not_move_consumed_sphere_or_box() {
    let sphere = parent_display::origin_sphere_after_discard().unwrap();
    assert!(sphere.location().is_identity());
    assert_eq!(analytic_sphere(&sphere), ([0.; 3], 10.));
    let located = sphere
        .located(&parent_display::discarded_sphere_location(), false)
        .unwrap();
    let shift = located.location().transformation().translation_part();
    assert_eq!([shift.x(), shift.y(), shift.z()], [20., 20., 0.]);
    assert_eq!(analytic_sphere(&located), ([20., 20., 0.], 10.));
    assert!(sphere.location().is_identity());
    assert_origin_box_vertices(&crate::primitive_shapes::box_sized([20.; 3]).unwrap());
}
