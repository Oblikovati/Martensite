//! The regression case `bugs_modalg_6::bug32876`.
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomRectangularTrimmedSurface, GeomSurface},
    top_abs::State,
};
use martensite_tkmath::gp::{Axis3, Point2};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeFace, brep_class::BRepClassFaceClassifier,
};

fn trimmed_origin_plane() -> martensite_tkbrep::topods::Face {
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let trimmed =
        GeomRectangularTrimmedSurface::new(&plane, (0.0, 4.0), (0.0, 2.0), (true, true)).unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(trimmed).cast();
    BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn bug32876_all_four_exterior_corner_points_are_on_with_tolerance() {
    crate::isolated_topology_case::run_isolated_case(
        "face_corner_tolerance::bug32876_all_four_exterior_corner_points_are_on_with_tolerance",
        || {
            bug32876_all_four_exterior_corner_points_are_on_with_tolerance_body();
            Ok(())
        },
    );
}

fn bug32876_all_four_exterior_corner_points_are_on_with_tolerance_body() {
    let face = trimmed_origin_plane();
    for (x, y) in [(4.02, -0.02), (4.06, -0.02), (3.8, -0.08), (4.09, 0.0)] {
        let classifier =
            BRepClassFaceClassifier::with_face_point2d(&face, &Point2::new(x, y), 0.1, false, 0.1)
                .unwrap();
        assert_eq!(classifier.state(), State::On, "point ({x}, {y})");
    }
}
