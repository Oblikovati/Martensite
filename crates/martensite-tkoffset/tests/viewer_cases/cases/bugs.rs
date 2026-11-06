//! The `bugs` cases that build their model from an input and show it shaded.
use crate::commands::{check_shape, explode, restore, triangulation_counts};
use crate::grading::{BUGS, PIPE, run_alone};
use crate::scene::{SHADED, ViewParameters, ViewScene};
use crate::support::checkprops::{Expected, checkprops};
use crate::support::nbshapes::{Counting, NbCount, checknbshapes};
use crate::support::properties::Prop;
use crate::support::rules::Verdict;
use crate::sweeps::{Sweep, normal_projection, pipe, smooth_curve, thick_shell};
use crate::variables::Variables;
use martensite_tkbo::bop_algo::BOPAlgoCheckerSI;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::geom::GeomCircle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape as _, BRepBuilderAPIMakeWire,
};

const BLACK: [f64; 3] = [0., 0., 0.];

/// `checkview -screenshot -3d`: the image of the view, with something drawn.
fn screenshot(scene: &ViewScene, case: &str) {
    let drawn = scene.dump(BLACK);
    assert!(drawn > 0, "{case}: the view shows nothing");
}

/// `checktrinfo name -tri value -nod value`: each count is a line, an `Error` when it differs.
fn check_triangulation(shape: &Shape, triangles: usize, nodes: usize) {
    let (got_triangles, got_nodes) = triangulation_counts(shape);
    for (what, value, expected) in [
        ("Number of triangles", got_triangles, triangles),
        ("Number of nodes", got_nodes, nodes),
    ] {
        if value == expected {
            println!("Check of {what} OK: value = {value}, expected = {expected}");
        } else {
            println!("Error: {what} is wrong: value = {value}, expected = {expected}");
        }
    }
}

/// `wire name edge`, the edge made from a curve.
fn wire_of_curve(
    curve: &martensite_tkernel::standard::Handle<dyn martensite_tkg3d::geom::GeomCurve>,
) -> Shape {
    let edge = BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap()
        .edge()
        .unwrap();
    let mut maker = BRepBuilderAPIMakeWire::new();
    maker.add_edge(&edge).unwrap();
    maker.shape().unwrap().clone()
}

#[test]
fn bug30442() {
    run_alone("cases::bugs::bug30442", &BUGS, Verdict::Ok, || {
        println!("0030442: Mesh - broken triangulation on pipe shape");
        let mut variables = Variables::default();
        let spine = smooth_curve("bug30442_points.txt", 1.0);
        let frame = Axis2::new(
            Point3::new(0., 0., 0.),
            Direction3::new(1., 1., 1.).unwrap(),
        )
        .unwrap();
        let circle =
            martensite_tkernel::standard::Handle::new(GeomCircle::new(frame, 20.).unwrap());
        variables.set("aSpineWire", wire_of_curve(&spine));
        variables.set("aProfileWire", wire_of_curve(&circle.cast()));
        pipe(&mut variables, "result", "aSpineWire", "aProfileWire");
        let result = variables.get("result");
        let mut scene = ViewScene::open("bug30442");
        scene.display("result", &result);
        scene.set_default_display_mode(SHADED);
        scene.fit();
        check_triangulation(&result, 25814, 12955);
        screenshot(&scene, "bug30442");
        scene.close();
        println!("TEST COMPLETED");
    });
}

#[test]
fn bug22978() {
    run_alone("cases::bugs::bug22978", &BUGS, Verdict::Ok, || {
        println!("Shading for a pipe shape is wrong");
        let mut variables = Variables::default();
        restore(&mut variables, "bug22978_1.brep", "c1");
        restore(&mut variables, "bug22978_2.brep", "c2");
        pipe(&mut variables, "result", "c1", "c2");
        let mut scene = ViewScene::open("bug22978");
        scene.set_default_display_mode(SHADED);
        scene.display("result", &variables.get("result"));
        scene.fit();
        checkprops(&variables, "result", Prop::Area, Expected::Mass(493480.));
        check_shape(&variables, "result");
        screenshot(&scene, "bug22978");
        scene.close();
        println!("TEST COMPLETED");
    });
}

#[test]
fn bug24185() {
    run_alone("cases::bugs::bug24185", &BUGS, Verdict::Ok, || {
        println!("Wrong projection of a wire to a face");
        let mut variables = Variables::default();
        restore(&mut variables, "bug24185_face.brep", "f");
        restore(&mut variables, "bug24185_wire.brep", "w");
        let names = ("result", "w", "f");
        normal_projection(&mut variables, names, 1.0e-4, Continuity::C1);
        check_shape(&variables, "result");
        checkprops(&variables, "result", Prop::Length, Expected::Mass(11.06));
        let mut scene = ViewScene::open("bug24185");
        scene.display("w", &variables.get("w"));
        scene.display("result", &variables.get("result"));
        scene.fit();
        screenshot(&scene, "bug24185");
        scene.close();
        println!("TEST COMPLETED");
    });
}

/// The `bopcheck` text of a shape, as the script reads it: empty of interferences is the OK line.
fn self_interference_free(shape: &Shape) -> bool {
    let mut arguments = List::default();
    arguments.append(shape.clone());
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&arguments);
    checker.set_level_of_check(9);
    checker.perform();
    let ds = checker.data_structure().unwrap();
    let new_shape = |pair: (i32, i32)| ds.is_new_shape(pair.0) || ds.is_new_shape(pair.1);
    !ds.interferences()
        .iter()
        .any(|pair| !new_shape(pair.indices()))
}

#[test]
fn bug27471() {
    let reference = Verdict::KnownProblem;
    run_alone("cases::bugs::bug27471", &BUGS, reference, || {
        println!("TODO 27471 ALL: ERROR: 27471 is reproduced.");
        println!("Wrong result of pipe");
        let mut variables = Variables::default();
        restore(&mut variables, "bug27471_s.brep", "s");
        restore(&mut variables, "bug27471_p.brep", "p");
        pipe(&mut variables, "result", "s", "p");
        let result = variables.get("result");
        if !self_interference_free(&result) {
            println!("ERROR: 27471 is reproduced. Result of pipe creation is WRONG.");
        }
        let mut scene = ViewScene::open("bug27471");
        scene.display("result", &result);
        scene.fit();
        screenshot(&scene, "bug27471");
        scene.close();
        println!("TEST COMPLETED");
    });
}

#[test]
fn bug30054() {
    let reference = Verdict::KnownProblem;
    run_alone("cases::bugs::bug30054", &BUGS, reference, || {
        println!("0030054: the offset fails to build joints in intersection mode");
        println!("TODO 33166 ALL: Faulty shapes");
        let mut variables = Variables::default();
        restore(&mut variables, "bug30054.brep", "a");
        thick_shell(&mut variables, "result", "a", 1.);
        check_shape(&variables, "result");
        let counts = [
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(14),
            NbCount::Faces(14),
        ];
        checknbshapes(&variables, "result", Counting::Typed, &counts);
        checkprops(&variables, "result", Prop::Area, Expected::Mass(1361.56));
        let mut scene = ViewScene::open("bug30054");
        scene.display("result", &variables.get("result"));
        scene.set_default_display_mode(SHADED);
        scene.set_view_parameters(&ViewParameters {
            scale: 33.8198,
            proj: [-0.185982, 0.955207, 0.230198],
            up: [0.223301, -0.187064, 0.956631],
            at: [0.279032, 2.94652, 11.8154],
        });
        screenshot(&scene, "bug30054");
        scene.close();
        println!("TEST COMPLETED");
    });
}

#[test]
fn bug31147() {
    run_alone("cases::bugs::bug31147", &PIPE, Verdict::Ok, || {
        println!("0031147: the pipe shell had a null access");
        let mut variables = Variables::default();
        restore(&mut variables, "bug31147.brep", "s");
        explode(&mut variables, "s", ShapeEnum::Edge);
        let wire = |edge: Shape| {
            let mut maker = BRepBuilderAPIMakeWire::new();
            maker
                .add_edge(&martensite_tkbrep::topods::edge(&edge).unwrap())
                .unwrap();
            maker.shape().unwrap().clone()
        };
        let mut sweep = Sweep::start(&wire(variables.get("s_1")));
        sweep.add(&wire(variables.get("s_2")));
        sweep.max_segments(1000);
        sweep.max_degree(14);
        variables.set("result", sweep.build_solid(true));
        if sweep.error_on_surface() > 1.0e-4 {
            println!("Error: bad tolerance of result");
        }
        let mut scene = ViewScene::open("bug31147");
        scene.display("result", &variables.get("result"));
        scene.set_default_display_mode(SHADED);
        scene.fit();
        check_shape(&variables, "result");
        screenshot(&scene, "bug31147");
        scene.close();
        println!("TEST COMPLETED");
    });
}
