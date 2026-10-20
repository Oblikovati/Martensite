//! Original disjoint coplanar wires and signed area from bugs/modalg_6/bug26607.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Builder, Face, ShapeIterator, Wire},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{geom::GeomSurface, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape},
    brep_gprop::BRepGProp,
};

fn fifth_box_face(origin_x: f64) -> Face {
    let mut maker =
        BRepPrimAPIMakeBox::at_point(Point3::new(origin_x, 0., 0.), 10., 10., 10.).unwrap();
    let shape = maker.shape().unwrap();
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    topods::face(&faces.nth(4).expect("box: expected fifth face")).unwrap()
}

fn oriented_face_wire(face: &Face) -> Wire {
    let child = ShapeIterator::new(face, true, true)
        .value()
        .unwrap()
        .clone();
    topods::wire(&child).unwrap()
}

fn copied_face_surface(face: &Face) -> Handle<dyn GeomSurface> {
    let (surface, location) = BRepTool::surface_with_location(face).unwrap();
    // mksurface copies even an identity-located surface; sharing it changes pcurve lookup.
    surface
        .get()
        .unwrap()
        .transformed(location.transformation())
        .unwrap()
        .cast()
}

fn face_with_original_disjoint_wires() -> Face {
    let first = fifth_box_face(0.);
    let second = fifth_box_face(20.);
    let surface = copied_face_surface(&first);
    let mut face = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())
        .unwrap()
        .face()
        .unwrap();
    let builder = Builder;
    builder.add(&mut face, &oriented_face_wire(&first)).unwrap();
    builder
        .add(&mut face, &oriented_face_wire(&second))
        .unwrap();
    face
}

#[test]
fn disjoint_coplanar_box_wires_keep_original_negative_area() {
    crate::isolated_primitive_case::run_primitive_case(
        "disjoint_face_area::disjoint_coplanar_box_wires_keep_original_negative_area",
        disjoint_coplanar_box_wires_keep_original_negative_area_body,
    );
}

fn disjoint_coplanar_box_wires_keep_original_negative_area_body() {
    let face = face_with_original_disjoint_wires();
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(&face, &mut properties, 0.5 * 0.01 + 0.001, false).unwrap();
    let measured = properties.mass();
    assert!(
        (measured - (-200.)).abs() <= 1e-6 + 0.01 * 200.,
        "area {measured}: expected -200 with original absolute1e-6/relative0.01"
    );
}
