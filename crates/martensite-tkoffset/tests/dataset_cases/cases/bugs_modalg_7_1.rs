//! Script case tables.
#[allow(unused_imports)]
use crate::conditions::{Cond, Value};
#[allow(unused_imports)]
use crate::extra_steps::*;
#[allow(unused_imports)]
use crate::groups::{End, WireEnd};
#[allow(unused_imports)]
use crate::steps::*;
#[allow(unused_imports)]
use crate::support::{properties::Prop, session::ViewStep};

offset_case!(
    bugs_modalg_7_bug22750,
    "tests/bugs/modalg_7/bug22750",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("22750"),
        Step::Puts("========"),
        Step::Curve(Curve::Ellipse("e1", &[0.0, 0.0, 30.0, 20.0])),
        Step::Curve(Curve::Ellipse("e2", &[0.0, 0.0, 20.0, 10.0])),
        Step::Curve(Curve::Ellipse("e3", &[0.0, 0.0, 16.0, 8.0])),
        Step::Extra(Extra::To3d("ee1", "e1", None)),
        Step::Extra(Extra::To3d("ee2", "e2", None)),
        Step::Extra(Extra::CurveTranslate(&["ee2"], [0.0, 0.0, 100.0])),
        Step::Extra(Extra::To3d("ee3", "e3", None)),
        Step::Extra(Extra::CurveTranslate(&["ee3"], [0.0, 0.0, 200.0])),
        Step::Extra(Extra::CurveRotate(
            &["ee2"],
            [0.0, 0.0, 100.0, 0.0, 1.0, 0.0, 10.0]
        )),
        Step::Extra(Extra::CurveRotate(
            &["ee3"],
            [0.0, 0.0, 200.0, 0.0, 1.0, 0.0, 20.0]
        )),
        Step::Extra(Extra::Appro("c1", 16, "ee1")),
        Step::Extra(Extra::Appro("c2", 16, "ee2")),
        Step::Extra(Extra::Appro("c3", 16, "ee3")),
        Step::Extra(Extra::AppSurf("s1", &["c1", "c2", "c3"])),
        Step::More(More::Face("f1", "s1", None)),
        Step::Extra(Extra::AppSurf("s2", &["ee1", "ee2", "ee3"])),
        Step::More(More::Face("f2", "s2", None)),
        Step::Curve(Curve::Circle(
            "c1",
            &[-100.0, 0.0, 0.0, 1.0, 0.0, 0.0, 10.0]
        )),
        Step::Curve(Curve::Circle("c2", &[-50.0, 0.0, 0.0, 1.0, 0.0, 0.0, 30.0])),
        Step::Curve(Curve::Circle("c3", &[50.0, 0.0, 0.0, 1.0, 0.0, 0.0, 29.0])),
        Step::Curve(Curve::Circle("c4", &[100.0, 0.0, 0.0, 1.0, 0.0, 0.0, 20.0])),
        Step::Curve(Curve::Circle("c5", &[150.0, 0.0, 0.0, 1.0, 0.0, 0.0, 10.0])),
        Step::Extra(Extra::AppSurf("s3", &["c1", "c2", "c3", "c4", "c5"])),
        Step::More(More::Face("f3", "s3", None)),
        Step::Profile(Profile::Edge("ed1", "ee1", None)),
        Step::Profile(Profile::Edge("ed3", "ee3", None)),
        Step::Profile(Profile::Wire("w1", &["ed1"])),
        Step::Profile(Profile::Wire("w3", &["ed3"])),
        Step::Profile(Profile::Plane("f5", "w1", false)),
        Step::Profile(Profile::Plane("f6", "w3", false)),
        Step::More(More::Sewing("r", 1.0e-6, &["f2", "f5", "f6"])),
        Step::More(More::EmptyShape("so1", "So")),
        Step::Topo(Topo::Add("r", "so1")),
        Step::Check(Check::Shape("so1")),
        Step::Topo(Topo::NumberedExplode("f3", "e")),
        Step::Profile(Profile::Wire("w1", &["f3_1"])),
        Step::Profile(Profile::Wire("w3", &["f3_3"])),
        Step::Profile(Profile::Plane("f7", "w1", true)),
        Step::Profile(Profile::Plane("f8", "w3", true)),
        Step::Extra(Extra::MakerVolume("so2", &["f3", "f7", "f8"], false)),
        Step::Check(Check::Shape("so2")),
        Step::Place(Place::Translate(&["so1"], [0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("so1", "so2")),
        Step::Bool(Bool::TwoStep("rrr", BopKind::Fuse)),
        Step::Check(Check::Shape("rrr")),
        Step::Check(Check::Counts(
            "rrr",
            &[("FACE", 5), ("WIRE", 5), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "rrr", 57448.9, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "rrr", 664969.0, 0.01, None))
        ]),
        Step::Topo(Topo::GeometryCopy("so1", "so3")),
        Step::Place(Place::Rotate(
            &["so3"],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 180.0]
        )),
        Step::Bool(Bool::Prepare("rrr", "so3")),
        Step::Bool(Bool::TwoStep("result", BopKind::Fuse)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("WIRE", 8), ("FACE", 7), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 73752.9, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 784375.0, 0.01, None))
        ]),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug23187,
    "tests/bugs/modalg_7/bug23187",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("23187"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v0", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v1", [1.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [2.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [3.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [4.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v01", [0.0, 0.0, 5.0])),
        Step::Profile(Profile::Vertex("v11", [1.0, 0.0, 5.0])),
        Step::Profile(Profile::Vertex("v21", [2.0, 0.0, 5.0])),
        Step::Profile(Profile::Vertex("v31", [3.0, 0.0, 5.0])),
        Step::Profile(Profile::Vertex("v41", [4.0, 0.0, 5.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v0", "v1")),
        Step::Profile(Profile::EdgeOfVertices("ee2", "v1", "v11")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v11", "v01")),
        Step::Profile(Profile::EdgeOfVertices("e4", "v01", "v0")),
        Step::Profile(Profile::Wire("w1", &["e1", "ee2", "e3", "e4"])),
        Step::Profile(Profile::Plane("f1", "w1", false)),
        Step::Topo(Topo::Orient("ee2", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("eee2", "v2", "v21")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v21", "v11")),
        Step::Profile(Profile::Wire("w1", &["e1", "eee2", "e3", "ee2"])),
        Step::Profile(Profile::Plane("f2", "w1", false)),
        Step::Topo(Topo::Orient("eee2", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e1", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("eeee2", "v3", "v31")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v31", "v21")),
        Step::Profile(Profile::Wire("w1", &["e1", "eeee2", "e3", "eee2"])),
        Step::Profile(Profile::Plane("f3", "w1", false)),
        Step::Topo(Topo::Orient("eeee2", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e1", "v3", "v4")),
        Step::Profile(Profile::EdgeOfVertices("eeeee2", "v4", "v41")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v41", "v31")),
        Step::Profile(Profile::Wire("w1", &["e1", "eeeee2", "e3", "eeee2"])),
        Step::Profile(Profile::Plane("f4", "w1", false)),
        Step::More(More::EmptyShape("sp", "Sh")),
        Step::Topo(Topo::Add("f1", "sp")),
        Step::Topo(Topo::Add("f2", "sp")),
        Step::Topo(Topo::Add("f3", "sp")),
        Step::Topo(Topo::Add("f4", "sp")),
        Step::Sweep(Sweep::Revol(
            "r",
            "sp",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::More(More::Surface(Surface::Plane("p", &[]))),
        Step::More(More::Face("fp", "p", Some([-6.0, 6.0, -6.0, 6.0]))),
        Step::Place(Place::Rotate(&["fp"], [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 20.0])),
        Step::Place(Place::Translate(&["fp"], [0.0, 0.0, 3.0])),
        Step::More(More::EmptyShape("s", "Sh")),
        Step::Topo(Topo::Add("fp", "s")),
        Step::More(More::EmptyShape("so", "So")),
        Step::Topo(Topo::Add("s", "so")),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::More(More::Compound(&["r_1", "r_2", "r_3", "r_4"], "cr")),
        Step::Bool(Bool::Cut("result", "cr", "so")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 231.511, 0.01, None)),
        Step::CheckView("result"),
        Step::More(More::Compound(&["r_2", "r_1", "r_4", "r_3"], "cr")),
        Step::Bool(Bool::Cut("result2", "cr", "so")),
        Step::Check(Check::Shape("result2")),
        Step::Check(Check::Props(Prop::Area, "result2", 231.511, 0.01, None)),
        Step::CheckView("result2"),
    ]
);

offset_case!(
    bugs_modalg_7_bug23838,
    "tests/bugs/modalg_7/bug23838",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts(
            "0023838: Modeling Data - Standard_Out Of Range in Geom_OffsetSurface::LocalDN (called by BRepFillPipeShell)"
        ),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug23838_profile.brep", "prof")),
        Step::Topo(Topo::Restore("brep/bug23838_SpineAndSupport.brep", "comp")),
        Step::Topo(Topo::ExplodeChildren("comp")),
        Step::Topo(Topo::Rename("comp_1", "supp")),
        Step::Topo(Topo::Rename("comp_2", "spine")),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Support("supp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "prof",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'R',
            solid: false
        })),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug24145_1,
    "tests/bugs/modalg_7/bug24145_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("24145"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug24145_face.brep", "f")),
        Step::Topo(Topo::Restore("brep/bug24145_wire1.brep", "w")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["w"],
            surface: "f",
            unlimited: false,
            max_distance: None,
            tolerance: 1.0e-4,
            continuity: 2,
            max_degree: 14,
            max_segments: 16
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("VERTEX", 1), ("EDGE", 1), ("COMPOUND", 1), ("SHAPE", 3)]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug24145_2,
    "tests/bugs/modalg_7/bug24145_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("24145"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug24145_face.brep", "f")),
        Step::Topo(Topo::Restore("brep/bug24145_wire2.brep", "w")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["w"],
            surface: "f",
            unlimited: false,
            max_distance: None,
            tolerance: 1.0e-4,
            continuity: 2,
            max_degree: 14,
            max_segments: 16
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("VERTEX", 1), ("EDGE", 1), ("COMPOUND", 1), ("SHAPE", 3)]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25395_2,
    "tests/bugs/modalg_7/bug25395_2",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25395 ALL: Standard_ConstructionError"),
        Step::Puts("TODO 25395 ALL:TEST INCOMPLETE"),
        Step::Puts("========"),
        Step::Puts("25395"),
        Step::Puts("========"),
        Step::Curve(Curve::Ellipse(
            "Ellipse-curve",
            &[0.0, 0.0, 0.0, 100.87, 22.0]
        )),
        Step::Profile(Profile::Edge(
            "Ellipse-edge",
            "Ellipse-curve",
            Some([0.0, 6.28318530718])
        )),
        Step::Profile(Profile::Wire("Ellipse-wire", &["Ellipse-edge"])),
        Step::Profile(Profile::Plane("Ellipse", "Ellipse-wire", false)),
        Step::Place(Place::Translate(&["Ellipse"], [0.0, -27.0, 184.5])),
        Step::Topo(Topo::Explode("Ellipse", "E")),
        Step::Profile(Profile::Wire("Sweep-0-spine", &["Ellipse_1"])),
        Step::Shell(PipeShell::Start("Sweep-0-spine")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Curve(Curve::Circle("Circle-curve", &[0.0, 0.0, 0.0, 18.0])),
        Step::Profile(Profile::Edge(
            "Circle-edge",
            "Circle-curve",
            Some([0.0, 0.0])
        )),
        Step::Profile(Profile::Wire("Circle", &["Circle-edge"])),
        Step::Place(Place::Rotate(
            &["Circle"],
            [0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 90.0]
        )),
        Step::Place(Place::Translate(&["Circle"], [0.0, -70.0, 202.5])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Circle",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep",
            mode: 'C',
            solid: true
        })),
        Step::Prim(Prim::Box("Box001", &[300.0, 90.0, 300.0])),
        Step::Place(Place::Translate(&["Box001"], [-150.0, -27.0, 0.0])),
        Step::Bool(Bool::Cut("Cut031", "Sweep", "Box001")),
        Step::Prim(Prim::Box("Box002", &[300.0, 100.0, 30.0])),
        Step::Place(Place::Translate(&["Box002"], [-150.0, -100.0, 202.5])),
        Step::Bool(Bool::Cut("Cut032", "Cut031", "Box002")),
        Step::Topo(Topo::Explode("Cut032", "F")),
        Step::Offset(Offset::Parameter(1e-07, false, 'a', false)),
        Step::Offset(Offset::Load(
            "Cut032",
            1.0,
            &["Cut032_3", "Cut032_2", "Cut032_4"]
        )),
        Step::Offset(Offset::Perform("Thickness")),
        Step::CheckView("Thickness"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25458,
    "tests/bugs/modalg_7/bug25458",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25458 ALL: Error : 1-fold tori is bad"),
        Step::Puts("============"),
        Step::Puts("25458"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore(
            "brep/bug25458_sweep-torus1-draw-Wire.brep",
            "Wire"
        )),
        Step::Topo(Topo::Explode("Wire", "E")),
        Step::Profile(Profile::Wire("Sweep-0-spine", &["Wire_1"])),
        Step::Shell(PipeShell::Start("Sweep-0-spine")),
        Step::Shell(PipeShell::Frenet),
        Step::Curve(Curve::Circle("Circle-curve", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge(
            "Circle-edge",
            "Circle-curve",
            Some([0.0, 0.0])
        )),
        Step::Profile(Profile::Wire("Circle", &["Circle-edge"])),
        Step::Place(Place::Rotate(
            &["Circle"],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0]
        )),
        Step::Place(Place::Translate(
            &["Circle"],
            [40.1121575545, 0.0, 36.5930306315]
        )),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Circle",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep",
            mode: 'C',
            solid: true
        })),
        Step::Let("info", Value::Bopcheck("Sweep")),
        Step::If(
            Cond::Lacks("info", "This shape seems to be OK"),
            &[Step::Puts("Error : 1-fold tori is bad")],
            &[]
        ),
        Step::CheckView("Sweep"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25626,
    "tests/bugs/modalg_7/bug25626",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25626 ALL: Error :  is WRONG because number of EDGE entities"),
        Step::Puts("TODO 25626 ALL: Error :  is WRONG because number of WIRE entities"),
        Step::Puts("============"),
        Step::Puts("25626"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore(
            "brep/bug25626_loft-el1-draw-Loft-00-section.brep",
            "Loft-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug25626_loft-el1-draw-Loft-01-section.brep",
            "Loft-01-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug25626_loft-el1-draw-Loft-02-section.brep",
            "Loft-02-section"
        )),
        Step::Loft(Loft {
            result: "Loft",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft-00-section", "Loft-01-section", "Loft-02-section"]
        }),
        Step::Check(Check::Shape("Loft")),
        Step::Check(Check::Counts("Loft", &[("WIRE", 1), ("EDGE", 3)])),
        Step::CheckView("Loft"),
        Step::CheckView("Loft"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25856_1,
    "tests/bugs/modalg_7/bug25856_1",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25856 ALL: Faulty shapes in variables faulty_1 to"),
        Step::Puts("============"),
        Step::Puts("25856"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug25856_threeWires.brep", "t")),
        Step::Topo(Topo::ExplodeChildren("t")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: false,
            mutable_input: true,
            sections: &["t_1", "t_2"]
        }),
        Step::Check(Check::Shape("r")),
        Step::Extra(Extra::FixShape("rr", "r")),
        Step::Check(Check::Shape("rr")),
        Step::CheckView("r"),
        Step::CheckView("rr"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25856_2,
    "tests/bugs/modalg_7/bug25856_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25856"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug25856_threeWires.brep", "t")),
        Step::Topo(Topo::ExplodeChildren("t")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: false,
            mutable_input: true,
            sections: &["t_1", "t_2", "t_3"]
        }),
        Step::Check(Check::Shape("r")),
        Step::Extra(Extra::FixShape("rr", "r")),
        Step::Check(Check::Shape("rr")),
        Step::CheckView("r"),
        Step::CheckView("rr"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25930,
    "tests/bugs/modalg_7/bug25930",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25930"),
        Step::Puts("============"),
        Step::Prim(Prim::Sphere("Sphere", &[5.0, -90.0, 40.0, 360.0])),
        Step::Topo(Topo::Explode("Sphere", "F")),
        Step::Offset(Offset::Parameter(1e-07, false, 'a', false)),
        Step::Offset(Offset::Load("Sphere", -1.0, &["Sphere_2"])),
        Step::Offset(Offset::Perform("Thickness")),
        Step::Check(Check::Shape("Thickness")),
        Step::CheckView("Thickness"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25939,
    "tests/bugs/modalg_7/bug25939",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("0025939: S I G S E G V in MakeThickSolid"),
        Step::Puts("============"),
        Step::Prim(Prim::Sphere("Sphere", &[5.0])),
        Step::Prim(Prim::Cylinder("Cylinder", &[83.0, 100.0])),
        Step::Place(Place::Translate(&["Cylinder"], [-83.0, 0.0, -50.0])),
        Step::Bool(Bool::Cut("Cut", "Sphere", "Cylinder")),
        Step::Topo(Topo::Explode("Cut", "F")),
        Step::Offset(Offset::Parameter(1e-07, false, 'a', false)),
        Step::Offset(Offset::Load("Cut", -2.0, &["Cut_4"])),
        Step::Catch(&Step::Offset(Offset::Perform("Thickness"))),
        Step::CheckView("Thickness"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25983_1,
    "tests/bugs/modalg_7/bug25983_1",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25983 ALL: Error :  is WRONG because number of"),
        Step::Puts("TODO 25983 ALL: Error : The area of result shape is"),
        Step::Puts("TODO 25983 ALL: Error : The volume of result shape is"),
        Step::Puts("========"),
        Step::Puts("25983"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug25983_deform-fusion1-tcl-BSpline.brep",
            "BSpline"
        )),
        Step::Profile(Profile::Wire("Knurling-0-spine", &["BSpline"])),
        Step::Shell(PipeShell::Start("Knurling-0-spine")),
        Step::Shell(PipeShell::Frenet),
        Step::Profile(Profile::Polyline(
            "DWire",
            &[
                0.0,
                0.0,
                0.0,
                1.0,
                -0.9999999999999998,
                0.0,
                1.0000000000000002,
                0.9999999999999998,
                0.0,
                0.0,
                0.0,
                0.0
            ]
        )),
        Step::Place(Place::Rotate(
            &["DWire"],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 134.99999999999693]
        )),
        Step::Place(Place::Translate(&["DWire"], [9.0, 0.0, 0.0])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "DWire",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Knurling",
            mode: 'C',
            solid: true
        })),
        Step::Topo(Topo::SharedCopy("Knurling", "Clone")),
        Step::More(More::Mirror(&["Clone"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Bool(Bool::Fuse("result", "Knurling", "Clone")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("WIRE", 24), ("FACE", 24), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 262.476, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 54.0383, 0.01, None))
        ]),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug25983_2,
    "tests/bugs/modalg_7/bug25983_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("25983"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug25983_deform-fusion1-tcl-BSpline.brep",
            "BSpline"
        )),
        Step::Profile(Profile::Wire("Knurling-0-spine", &["BSpline"])),
        Step::Shell(PipeShell::Start("Knurling-0-spine")),
        Step::Shell(PipeShell::Frenet),
        Step::Profile(Profile::Polyline(
            "DWire",
            &[
                0.0,
                0.0,
                0.0,
                1.0,
                -0.9999999999999998,
                0.0,
                1.0000000000000002,
                0.9999999999999998,
                0.0,
                0.0,
                0.0,
                0.0
            ]
        )),
        Step::Place(Place::Rotate(
            &["DWire"],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 134.99999999999693]
        )),
        Step::Place(Place::Translate(&["DWire"], [9.0, 0.0, 0.0])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "DWire",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Knurling",
            mode: 'C',
            solid: true
        })),
        Step::Topo(Topo::SharedCopy("Knurling", "Clone")),
        Step::More(More::Mirror(&["Clone"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Bool(Bool::Fuzzy(0.001)),
        Step::Bool(Bool::Fuse("result", "Knurling", "Clone")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("WIRE", 24), ("FACE", 24), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 262.476, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 54.0383, 0.01, None))
        ]),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug26077_2,
    "tests/bugs/modalg_7/bug26077_2",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts(
            "TODO 26077 ALL: Error: Boolean operation of the given type is not allowed on the given inputs"
        ),
        Step::Puts("TODO 26077 ALL: ERROR: 26077 is reproduced."),
        Step::Puts("TODO 26077 ALL: ERROR: Problematic shape in 26077:"),
        Step::Puts("========"),
        Step::Puts("26077"),
        Step::Puts("========"),
        Step::Curve(Curve::Ellipse(
            "Ellipse002-untrimmed",
            &[0.0, 0.0, 0.0, 8.2, 6.2]
        )),
        Step::Profile(Profile::Edge(
            "Ellipse002",
            "Ellipse002-untrimmed",
            Some([0.0, 6.283185307179586])
        )),
        Step::Place(Place::Rotate(
            &["Ellipse002"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0]
        )),
        Step::Place(Place::Translate(&["Ellipse002"], [0.0, 0.0, 15.0])),
        Step::Sweep(Sweep::Prism(
            "Extrude_Ellipse",
            "Ellipse002",
            [0.0, 0.0, 6.0]
        )),
        Step::Place(Place::Translate(&["Extrude_Ellipse"], [0.0, 0.0, -4.0])),
        Step::Topo(Topo::Restore(
            "brep/bug26077_ellipsoid1-tcl-Loft-00-section.brep",
            "Loft-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug26077_ellipsoid1-tcl-Loft-01-section.brep",
            "Loft-01-section"
        )),
        Step::Loft(Loft {
            result: "Loft",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft-00-section", "Loft-01-section"]
        }),
        Step::Prim(Prim::Sphere("Ellipsoid-sphere", &[12.5])),
        Step::Extra(Extra::Deform(
            "Ellipsoid",
            "Ellipsoid-sphere",
            [1.0, 1.04, 1.2]
        )),
        Step::Run(crate::one_off::loft_ellipsoid_fuses),
    ]
);

offset_case!(
    bugs_modalg_7_bug26295,
    "tests/bugs/modalg_7/bug26295",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 26295 ALL: An exception was caught"),
        Step::Puts("TODO 26295 ALL: TEST INCOMPLETE"),
        Step::Puts("========"),
        Step::Puts("0026295: Sweep fails with BRepFillContactOnBorder"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug26295_section.brep", "section")),
        Step::Topo(Topo::Restore("brep/bug26295_profile1.brep", "profile1")),
        Step::Topo(Topo::Restore("brep/bug26295_profile3.brep", "profile3")),
        Step::Topo(Topo::ExplodeChildren("section")),
        Step::Topo(Topo::ExplodeChildren("profile1")),
        Step::Topo(Topo::ExplodeChildren("profile3")),
        Step::Shell(PipeShell::Start("profile1_1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "section_1",
            vertex: None,
            translate: true,
            rotate: true,
            law: &[]
        })),
        Step::Shell(PipeShell::Guide("profile3_1", false, 2)),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "sw",
            mode: 'M',
            solid: true
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug26568,
    "tests/bugs/modalg_7/bug26568",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================================="),
        Step::Puts("26568 - Modeling Algorithms - Crash when creating pipe"),
        Step::Puts("============================================================="),
        Step::Prim(Prim::Box("b", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b", "f")),
        Step::Curve(Curve::Circle("c", &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e", "c", None)),
        Step::Profile(Profile::Wire("w", &["e"])),
        Step::Sweep(Sweep::Pipe("result", "w", "b_1", 0, false)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("SOLID", 1),
                ("SHELL", 1),
                ("FACE", 3),
                ("WIRE", 3),
                ("EDGE", 7),
                ("VERTEX", 4)
            ]
        )),
        Step::Let("tolres", Value::CheckedMaxTolerance("result")),
        Step::If(
            Cond::Above("tolres", 2e-07),
            &[Step::Puts("Error: bad tolerance of result")],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_7_bug27090,
    "tests/bugs/modalg_7/bug27090",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("27090"),
        Step::Puts("========"),
        Step::View(ViewStep::SmallView),
        Step::More(More::Surface(Surface::Plane(
            "pln_f1",
            &[
                0.0,
                -875.0,
                -1.4432899320127035e-14,
                0.0,
                1.0,
                1.1102230246251565e-16
            ]
        ))),
        Step::More(More::Face(
            "f1",
            "pln_f1",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Plane(
            "pln_f2",
            &[
                1084.71007164,
                -980.0,
                -2128.67968684,
                1.110223024625157e-16,
                -1.0,
                1.1102230246251563e-16
            ]
        ))),
        Step::More(More::Face(
            "f2",
            "pln_f2",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Plane(
            "pln_f3",
            &[
                -308.4315574055877,
                -800.0,
                -2407.1041518310276,
                0.8090169943749477,
                9.244463733058731e-33,
                -0.5877852522924728
            ]
        ))),
        Step::More(More::Face(
            "f3",
            "pln_f3",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Plane(
            "pln_f4",
            &[
                -381.4805392043022,
                -800.0,
                -2409.117597419774,
                -0.1563999881178786,
                -1.5422229525403152e-32,
                -0.9876938005863596
            ]
        ))),
        Step::More(More::Face(
            "f4",
            "pln_f4",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Cylinder(
            "cyl_f5",
            &[
                0.0,
                0.0,
                -1.0103029524088925e-13,
                0.0,
                1.0,
                1.1102230246251565e-16
            ],
            2440.0
        ))),
        Step::More(More::Face(
            "f5",
            "cyl_f5",
            Some([0.0, 6.283185307179586, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Plane(
            "pln_f6",
            &[
                -309.45754782652125,
                -800.0,
                -2369.8806246344966,
                0.9876938005863596,
                -4.440892098500626e-16,
                -0.1563999881178788
            ]
        ))),
        Step::More(More::Face(
            "f6",
            "pln_f6",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Plane(
            "pln_f7",
            &[
                -330.1160239224329,
                -800.0,
                -2299.7847255135234,
                -0.5877852522924727,
                -1.6653345369377348e-16,
                -0.8090169943749477
            ]
        ))),
        Step::More(More::Face(
            "f7",
            "pln_f7",
            Some([-1000000.0, 1000000.0, -1000000.0, 1000000.0])
        )),
        Step::More(More::Surface(Surface::Cylinder(
            "cyl_f8",
            &[
                0.0,
                0.0,
                -2.020605904817785e-13,
                0.0,
                -1.0,
                1.1102230246251565e-16
            ],
            2340.0
        ))),
        Step::More(More::Face(
            "f8",
            "cyl_f8",
            Some([0.0, 6.283185307179586, -1000000.0, 1000000.0])
        )),
        Step::Extra(Extra::MakerVolume(
            "v",
            &["f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8"],
            true
        )),
        Step::View(ViewStep::Donly(&["v"])),
        Step::Topo(Topo::ExplodeChildren("v")),
        Step::Run(crate::one_off::sixth_solid_shells),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_7_bug27784,
    "tests/bugs/modalg_7/bug27784",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("27784"),
        Step::Puts("========"),
        Step::Prim(Prim::Cylinder("cyl", &[50.0, 30.0])),
        Step::Topo(Topo::Explode("cyl", "f")),
        Step::Draft(Draft {
            result: "draft",
            shape: "cyl",
            direction: [0.0, 0.0, -1.0],
            faces: &[("cyl_1", 20.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Topo(Topo::Explode("draft", "f")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "d_thick",
            shape: "draft",
            value: -5.0,
            tolerance: None,
            complete: true,
            faces: &["draft_2"]
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug28108,
    "tests/bugs/modalg_7/bug28108",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 28108 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("========"),
        Step::Puts("28108"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug28108_sweep.brep", "sweep")),
        Step::Topo(Topo::Explode("sweep", "w")),
        Step::Shell(PipeShell::Start("sweep_2")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "sweep_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Shape("result")),
    ]
);

offset_case!(
    bugs_modalg_7_bug28656,
    "tests/bugs/modalg_7/bug28656",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("28656"),
        Step::Puts("========"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 15.0])),
        Step::Topo(Topo::SetFlags("s", &[("locked", true)])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Catch(&Step::Offset(Offset::Perform("result"))),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug28684_1,
    "tests/bugs/modalg_7/bug28684_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts(
            "25298: Modeling Algorithms - BRepOffsetAPIMakeOffset produces wire with another orientation in compare with source wire"
        ),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c", &[0.0, 0.0, 0.0, 100.0])),
        Step::Profile(Profile::Edge("e", "c", None)),
        Step::Profile(Profile::Wire("a", &["e"])),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["a"])),
        Step::View(ViewStep::Fit),
        Step::Puts("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL"),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: 10.0,
                approx: false,
                join: 'a',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Puts("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL"),
                Step::Check(Check::Props(Prop::Length, "result", 691.15, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::View(ViewStep::Donly(&["result"])),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_7_bug28696_1,
    "tests/bugs/modalg_7/bug28696_1",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 28696 ALL: ERROR: Micro edge is produced."),
        Step::Puts("========"),
        Step::Puts("28696"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC16667-wire.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: 20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Run(crate::one_off::micro_edges),
    ]
);

offset_case!(
    bugs_modalg_7_bug28696_2,
    "tests/bugs/modalg_7/bug28696_2",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 28696 ALL: ERROR: Micro edge is produced."),
        Step::Puts("========"),
        Step::Puts("28696"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC16667-wire.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: -20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Run(crate::one_off::micro_edges),
    ]
);

offset_case!(
    bugs_modalg_7_bug28739,
    "tests/bugs/modalg_7/bug28739",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 28739 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("========"),
        Step::Puts("28739"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug28739_draft.brep", "draft")),
        Step::Topo(Topo::Explode("draft", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "draft",
            direction: [0.0, 1.0, 0.0],
            faces: &[("draft_7", 87.0, [11.0, -5.0, 0.0, 1.0, 0.0, 0.0])]
        }),
        Step::Check(Check::Shape("result")),
    ]
);

offset_case!(
    bugs_modalg_7_bug28949_1,
    "tests/bugs/modalg_7/bug28949_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("28949"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "sp")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "pr")),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("sp")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sp_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sp_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sp_3"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 80000.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 463014.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r3", 449490.0, 0.01, None)),
        Step::Topo(Topo::Explode("sp", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sp_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sp_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sp_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "sp_4"
        ))),
        Step::Check(Check::Props(Prop::Length, "r1", 800.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2", 800.0, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r3", 30911.3, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r4", 800.0, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug28949_6,
    "tests/bugs/modalg_7/bug28949_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("28949"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 10.0, 10.0, 0.0, 10.0, 10.0,
                0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "pr",
            &[
                0.0, -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0, -2.0, -2.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("sp")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sp_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sp_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sp_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "sp_4"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r5",
            "sweep_hist",
            "sp_5"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r6",
            "sweep_hist",
            "sp_6"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 136.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 136.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r3", 136.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r4", 136.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r5", 136.0, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r6", 136.0, 0.01, None)),
        Step::Topo(Topo::Explode("sp", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sp_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sp_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sp_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "sp_4"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r5",
            "sweep_hist",
            "sp_5"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r6",
            "sweep_hist",
            "sp_6"
        ))),
        Step::Check(Check::Counts("r1", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r1", 18.8496, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r2", 18.8496, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r3", 18.8496, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r4", 18.8496, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r5", 18.8496, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("EDGE", 11), ("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r6", 18.8496, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug28982,
    "tests/bugs/modalg_7/bug28982",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("28982"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug28982_ellipse.brep", "e")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "result",
            base: "e",
            count: 1,
            step: 1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Check(Check::Shape("result_1")),
        Step::Let("answer_8_bopcheck", Value::Bopcheck("result_1")),
        Step::If(
            Cond::Lacks("answer_8_bopcheck", "This shape seems to be OK"),
            &[Step::Puts("Error: The resulting wire is self-interfered")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Length, "result_1", 305.299, 0.01, None)),
        Step::CheckView("result_1"),
    ]
);

offset_case!(
    bugs_modalg_7_bug29182_1,
    "tests/bugs/modalg_7/bug29182_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("29182"),
        Step::Puts("========"),
        Step::More(More::Surface(Surface::Cylinder(
            "c",
            &[0.0, 10.0, 0.0, 1.0, 0.0, 0.0],
            10.0
        ))),
        Step::More(More::Face(
            "fc",
            "c",
            Some([1.0471975511965976, 2.0943951023931953, -10.0, 10.0])
        )),
        Step::More(More::EmptyShape("a", "C")),
        Step::Topo(Topo::Add("fc", "a")),
        Step::View(ViewStep::Donly(&["a"])),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 15.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 20.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 25.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 30.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 35.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 40.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 45.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::Topo(Topo::SharedCopy("fc", "f1")),
        Step::Place(Place::Rotate(&["f1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Topo(Topo::Add("f1", "a")),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::Run(crate::one_off::repeated_fill),
    ]
);

offset_case!(
    bugs_modalg_7_bug29334,
    "tests/bugs/modalg_7/bug29334",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("# ======================================================================"),
        Step::Puts("# 0029334: Simple offsets complicate geometry"),
        Step::Puts("# ======================================================================"),
        Step::Puts(
            "Make box, fillet one of its edges, and extract two planar faces connected by fillet"
        ),
        Step::Prim(Prim::Box("b", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b", "e")),
        Step::Fillet(Fillet::Blend("r", "b", &[(1.0, "b_5")])),
        Step::Topo(Topo::Explode("r", "f")),
        Step::More(More::Compound(&["r_1", "r_3", "r_6"], "shape")),
        Step::Puts("Offset these faces using simple offset algo"),
        Step::Offset(Offset::Simple("result", "shape", 1.0, 1.0e-7)),
        Step::Puts("Verify that all faces of the result are either planes or cylinders"),
        Step::Run(crate::one_off::plane_or_cylinder),
    ]
);
