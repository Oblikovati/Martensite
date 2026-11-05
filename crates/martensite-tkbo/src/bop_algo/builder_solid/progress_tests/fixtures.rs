//! The literal retained native/default recipes are not altered by progress settings.
use super::BuilderSolid;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};

pub(super) fn builder(recipe: &str, parallel: bool) -> BuilderSolid {
    let mut faces = Vec::new();
    append_box(&mut faces, Point3::default(), 10., false);
    match recipe {
        "two" => append_box(&mut faces, Point3::new(20., 0., 0.), 2., false),
        "cavity" => append_box(&mut faces, Point3::new(3., 3., 3.), 2., true),
        "internal" => append_internal(&mut faces),
        other => panic!("recipe={other:?}; expected two/cavity/internal"),
    }
    let mut builder = BuilderSolid::new();
    let faces: List<Shape> = faces.into_iter().collect();
    builder.set_shapes(&faces);
    builder.options_mut().set_run_parallel(parallel);
    builder
}

fn append_box(faces: &mut Vec<Shape>, origin: Point3, side: f64, reversed: bool) {
    let solid = BRepPrimAPIMakeBox::at_point(origin, side, side, side)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    for face in Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        faces.push(if reversed { face.reversed() } else { face });
    }
}

fn append_internal(faces: &mut Vec<Shape>) {
    let plane = Plane3::new(
        Point3::new(0., 0., 5.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    for start in [2., 12.] {
        let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (start, start + 2., 2., 4.))
            .unwrap()
            .face()
            .unwrap()
            .into_shape();
        faces.push(face.clone());
        faces.push(face.reversed());
    }
}
