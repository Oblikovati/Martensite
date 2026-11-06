//! The model of the `cad` sample script, shown as the script shows it. Each boolean operation
//! reports what its builder warned about, as the commands of the script do.
use crate::checks::explode;
use crate::scene::ViewScene;
use martensite_tkbo::brep_algo_api::{BRepAlgoAPICommon, BRepAlgoAPICut};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::{Gravity, Msg};
use martensite_tkfillet::brep_fillet_api::BRepFilletAPIMakeFillet;
use martensite_tkfillet::chfi3d::FilletShape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{self, Axis1, Axis2, Point3, Transform3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis as _, BRepPrimAPIMakeRevol,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape as _, BRepBuilderAPITransform,
};

fn solid_box(origin: [f64; 3], size: [f64; 3]) -> Shape {
    let [x, y, z] = origin;
    let [dx, dy, dz] = size;
    let first = Point3::new(x, y, z);
    let second = Point3::new(x + dx, y + dy, z + dz);
    BRepPrimAPIMakeBox::from_points(first, second)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

/// What `blend` writes before it works.
fn print_tolerances() {
    println!("tolerance ang : 0.01");
    println!("tolerance 3d  : 0.0001");
    println!("tolerance 2d  : 1.0000000000000001e-05");
    println!("fleche        : 0.001");
    println!("tolblend 0.01 0.0001 1.0000000000000001e-05 0.001");
}

/// `blend result shape radius edge`: the tolerances of the command, one edge.
fn blend(shape: &Shape, radius: f64, edge: &Shape) -> Shape {
    print_tolerances();
    let mut maker = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational).unwrap();
    maker.set_params(0.01, 0.0001, 0.00001, 0.0001, 0.00001, 0.001);
    maker.set_continuity(martensite_tkmath::geom_abs::Shape::C1, 0.01);
    maker
        .add_with_radius(radius, &topods::edge(edge).unwrap())
        .unwrap();
    maker.build(&Default::default()).unwrap();
    assert!(maker.is_done(), "blend of radius {radius}");
    maker.shape().unwrap().clone()
}

fn print_warnings(report: &martensite_tkernel::message::Report) {
    let mut seen = Vec::new();
    for alert in report.get_alerts(Gravity::Warning) {
        let kind = alert.dynamic_type().name();
        if !seen.contains(&kind) {
            seen.push(kind);
            let mut message = Msg::from_key(&alert.get_message_key());
            println!("Warning: {}", message.get().unwrap().to_utf8());
        }
    }
}

fn cut(first: &Shape, second: &Shape) -> Shape {
    let operation = BRepAlgoAPICut::from_shapes(first, second).unwrap();
    assert!(operation.is_done(), "bcut");
    print_warnings(operation.get_report());
    operation.shape().clone()
}

fn common(first: &Shape, second: &Shape) -> Shape {
    let operation = BRepAlgoAPICommon::from_shapes(first, second).unwrap();
    assert!(operation.is_done(), "bcommon");
    print_warnings(operation.get_report());
    operation.shape().clone()
}

/// `pcylinder name plane radius height`, on the plane of `plane name point normal`.
fn cylinder_on_plane(origin: [f64; 3], normal: [f64; 3], radius: f64, height: f64) -> Shape {
    let [x, y, z] = origin;
    let [nx, ny, nz] = normal;
    let axis = gp::Direction3::new(nx, ny, nz).unwrap();
    let frame = Axis2::new(Point3::new(x, y, z), axis).unwrap();
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(frame, radius, height);
    maker.solid().unwrap().into_shape()
}

fn base_body() -> Shape {
    let b0 = solid_box([0., 0., 0.], [12., 7., 6.]);
    let edges = explode(&b0, ShapeEnum::Edge);
    let b01 = blend(&b0, 1.0, &edges[7]);
    let b02 = blend(&b01, 1.0, &edges[5]);
    let b1 = solid_box([3., -1., 0.], [2., 9., 1.5]);
    let b2 = cut(&b02, &b1);
    let edges = explode(&b2, ShapeEnum::Edge);
    let b3 = blend(&b2, 1.0, &edges[26]);
    blend(&b3, 1.0, &edges[29])
}

fn with_central_cut(b4: &Shape) -> Shape {
    let h1 = solid_box([3., -1., 2.5], [2., 9., 0.5]);
    let h2 = solid_box([3., -1., 3.], [2., 9., 2.]);
    let c = cylinder_on_plane([4., -1., 3.], [0., 1., 0.], 1.0, 9.0);
    let h3 = common(&h2, &c);
    let b5 = cut(b4, &h1);
    cut(&b5, &h3)
}

fn with_side_cut(b6: &Shape) -> Shape {
    let b7 = solid_box([9., -1., 1.], [2., 9., 4.]);
    let edges = explode(&b7, ShapeEnum::Edge);
    let b71 = blend(&b7, 1.0, &edges[5]);
    let b72 = blend(&b71, 1.0, &edges[7]);
    cut(b6, &b72)
}

fn with_inner_cut(b8: &Shape) -> Shape {
    let bc1 = solid_box([-1., 0., 2.], [9., 1., 4.]);
    let bc2 = solid_box([2., -1., 2.], [4., 3., 3.]);
    let bc3 = cut(&bc1, &bc2);
    let edges = explode(&bc3, ShapeEnum::Edge);
    let bc4 = blend(&bc3, 1.0, &edges[19]);
    let bc5 = blend(&bc4, 1.0, &edges[22]);
    let bc6 = blend(&bc5, 1.99, &edges[20]);
    let bc7 = blend(&bc6, 1.99, &edges[21]);
    let faces = explode(&bc7, ShapeEnum::Face);
    let axis = Axis1::new(Point3::new(-1., 0., 6.), gp::dx());
    let mut revolved =
        BRepPrimAPIMakeRevol::new(&faces[0], axis, 100.0_f64.to_radians(), false).unwrap();
    let r = revolved.shape().unwrap().clone();
    cut(b8, &r)
}

/// The script: the model rotated by 90 degrees and shown shaded with its boundaries.
pub fn cad_model(scene_case: &'static str) -> (Shape, ViewScene) {
    let b4 = base_body();
    let b6 = with_central_cut(&b4);
    let b8 = with_side_cut(&b6);
    let b9 = with_inner_cut(&b8);
    let c2 = cylinder_on_plane([-1., 0., 6.], [1., 0., 0.], 6.0, 15.0);
    let res = common(&b9, &c2);
    let axis = Axis1::new(Point3::new(0., 0., 0.), gp::dz());
    let turn = Transform3::rotation(axis, 90.0_f64.to_radians()).unwrap();
    let res = BRepBuilderAPITransform::with_shape(&res, turn, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut scene = ViewScene::open(scene_case);
    scene.display_shape("res", &res);
    scene.draw_boundaries("res");
    scene.fit();
    (res, scene)
}
