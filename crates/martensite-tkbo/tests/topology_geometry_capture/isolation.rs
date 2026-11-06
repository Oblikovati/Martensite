use super::support::{drawable, viewer};
use martensite_raster_core::drawable::RasterDrawable;
use martensite_tkbrep::topods::Shape;
use std::rc::Rc;
fn edge(x: f64) -> Shape {
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
    BRepBuilderAPIMakeEdge::from_points(Point3::new(x, 0., 0.), Point3::new(x + 10., 0., 0.))
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}
pub fn check_pick_isolation() {
    let first_shape = edge(0.);
    let second_shape = edge(20.);
    let (first, second) = (
        drawable(first_shape.clone()),
        drawable(second_shape.clone()),
    );
    let mut viewer = viewer("+X+Y");
    let handles: [Rc<dyn RasterDrawable>; 2] = [first.clone(), second.clone()];
    display_owned_pair(&mut viewer, &handles);
    let selected = viewer.pick(1, 5, 0, 2, 0).unwrap().1.unwrap();
    assert!(Rc::ptr_eq(&selected, &handles[0]));
    assert!(first.last_pick().shape.is_equal(&first_shape));
    assert!(second.last_pick().shape.is_null());
    viewer.pick(1, 25, 0, 2, 0).unwrap();
    assert!(first.last_pick().shape.is_equal(&first_shape));
    assert!(second.last_pick().shape.is_equal(&second_shape));
    viewer.delete_view(1);
}

fn display_owned_pair(
    viewer: &mut martensite_raster_core::RasterViewer,
    handles: &[Rc<dyn RasterDrawable>],
) {
    for handle in handles {
        viewer.display(handle).unwrap();
    }
}
