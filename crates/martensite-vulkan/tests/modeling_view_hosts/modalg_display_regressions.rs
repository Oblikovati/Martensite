//! Modelling regressions of the bugs group whose result is checked, then shown in a 3D view;
//! each runs in an isolated worker under the bugs policy.
use super::*;

use cases::{
    ConeCase, CutPath, FIRST_AXIS, HalfSpaceObject, RevolvedTriangle, SECOND_AXIS, SewingCase,
    SewingInputs::*, SewingOptions, ShownMeshCase, SphereChange, ToolCut,
};
use modalg_display_cases as cases;
use shape_checks::{MassCheck, MassKind::*};
use shape_mesh::CountExpectation::{self, Not};
use std::error::Error;

fn run(identity: &str, body: impl FnOnce() -> Result<(), Box<dyn Error>>) {
    case_bugs_process_case::run_bugs_case(identity, body);
}

/// A regression whose script declares `TODO` lines: it must fail exactly as expected.
/// Run a recorded failure's body under the worker identity `recorded_case!` passes.
fn run_body(identity: &str, body: fn(&str)) {
    body(identity);
}

fn run_todo(
    identity: &str,
    todos: &[&'static str],
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
) {
    case_bugs_process_case::run_bugs_case_expecting(identity, todos, body);
}

const fn exactly(value: f64) -> CountExpectation {
    CountExpectation::Near {
        value,
        tolerance_absolute: 0.,
        tolerance_relative: 0.,
    }
}

/// Floating-edge sewing at tolerance 0.1, with `+e` (`floating`) or `-e`.
const fn edges(floating: bool) -> SewingOptions {
    SewingOptions {
        floating_edges: floating,
        ..SewingOptions::with_tolerance(0.1)
    }
}

/// Facing box faces sewn at tolerance 1.1, faces mode `faces`.
const fn facing(faces: bool) -> SewingOptions {
    SewingOptions {
        faces,
        ..SewingOptions::with_tolerance(1.1)
    }
}

const NON_MANIFOLD: SewingOptions = SewingOptions {
    non_manifold: true,
    ..SewingOptions::DEFAULT
};

const TINY: &str = "9.9999999999999995e-008";

#[test]
fn bug22770_1() {
    let case = SewingCase {
        inputs: FloatingEdges,
        options: edges(true),
        mass: MassCheck::near(Length, 18.99),
        counts: [3, 2, 0, 0, 0, 0, 0, 1, 6],
        max_tolerance: "0.0050001000000007819",
        free_edges: 0,
        image: "bug22770_1.png",
    };
    run("modalg_display_regressions::bug22770_1", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_2() {
    let case = SewingCase {
        inputs: FloatingEdges,
        options: edges(false),
        mass: MassCheck::near(Length, 18.99),
        counts: [4, 2, 0, 0, 0, 0, 0, 1, 7],
        max_tolerance: TINY,
        free_edges: 0,
        image: "bug22770_2.png",
    };
    run("modalg_display_regressions::bug22770_2", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_3() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(true),
        mass: MassCheck::near(Area, 12.),
        counts: [4, 4, 2, 2, 1, 0, 0, 0, 13],
        max_tolerance: "1.05",
        free_edges: 0,
        image: "bug22770_3.png",
    };
    run("modalg_display_regressions::bug22770_3", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_4() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(false),
        mass: MassCheck::near(Area, 12.),
        counts: [8, 8, 2, 2, 0, 0, 0, 1, 21],
        max_tolerance: TINY,
        free_edges: 8,
        image: "bug22770_4.png",
    };
    run("modalg_display_regressions::bug22770_4", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_5() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(true),
        mass: MassCheck::near(Area, 12.),
        counts: [4, 4, 2, 2, 1, 0, 0, 0, 13],
        max_tolerance: "1.05",
        free_edges: 0,
        image: "bug22770_5.png",
    };
    run("modalg_display_regressions::bug22770_5", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_6() {
    let case = SewingCase {
        inputs: AdjacentBoxFaces,
        options: SewingOptions::DEFAULT,
        mass: MassCheck::near(Area, 3.),
        counts: [10, 11, 3, 3, 1, 0, 0, 1, 29],
        max_tolerance: TINY,
        free_edges: 10,
        image: "bug22770_6.png",
    };
    run("modalg_display_regressions::bug22770_6", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_7() {
    let case = SewingCase {
        inputs: AdjacentBoxFaces,
        options: NON_MANIFOLD,
        mass: MassCheck::near(Area, 3.),
        counts: [8, 10, 3, 3, 1, 0, 0, 0, 25],
        max_tolerance: TINY,
        free_edges: 6,
        image: "bug22770_7.png",
    };
    run("modalg_display_regressions::bug22770_7", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_16() {
    let case = SewingCase {
        inputs: FloatingEdges,
        options: edges(true),
        mass: MassCheck::near(Length, 18.99),
        counts: [3, 2, 0, 0, 0, 0, 0, 1, 6],
        max_tolerance: "0.0050001000000007819",
        free_edges: 0,
        image: "bug22770_16.png",
    };
    run("modalg_display_regressions::bug22770_16", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_17() {
    let case = SewingCase {
        inputs: FloatingEdges,
        options: edges(false),
        mass: MassCheck::near(Length, 18.99),
        counts: [4, 2, 0, 0, 0, 0, 0, 1, 7],
        max_tolerance: TINY,
        free_edges: 0,
        image: "bug22770_17.png",
    };
    run("modalg_display_regressions::bug22770_17", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_18() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(true),
        mass: MassCheck::near(Area, 12.),
        counts: [4, 4, 2, 2, 1, 0, 0, 0, 13],
        max_tolerance: "1.05",
        free_edges: 0,
        image: "bug22770_18.png",
    };
    run("modalg_display_regressions::bug22770_18", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_19() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(false),
        mass: MassCheck::near(Area, 12.),
        counts: [8, 8, 2, 2, 0, 0, 0, 1, 21],
        max_tolerance: TINY,
        free_edges: 8,
        image: "bug22770_19.png",
    };
    run("modalg_display_regressions::bug22770_19", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_20() {
    let case = SewingCase {
        inputs: FacingBoxFaces,
        options: facing(true),
        mass: MassCheck::near(Area, 12.),
        counts: [4, 4, 2, 2, 1, 0, 0, 0, 13],
        max_tolerance: "1.05",
        free_edges: 0,
        image: "bug22770_20.png",
    };
    run("modalg_display_regressions::bug22770_20", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_21() {
    let case = SewingCase {
        inputs: AdjacentBoxFaces,
        options: SewingOptions::DEFAULT,
        mass: MassCheck::near(Area, 3.),
        counts: [10, 11, 3, 3, 1, 0, 0, 1, 29],
        max_tolerance: TINY,
        free_edges: 10,
        image: "bug22770_21.png",
    };
    run("modalg_display_regressions::bug22770_21", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug22770_22() {
    let case = SewingCase {
        inputs: AdjacentBoxFaces,
        options: NON_MANIFOLD,
        mass: MassCheck::near(Area, 3.),
        counts: [8, 10, 3, 3, 1, 0, 0, 0, 25],
        max_tolerance: TINY,
        free_edges: 6,
        image: "bug22770_22.png",
    };
    run("modalg_display_regressions::bug22770_22", || {
        cases::sewn_result_view(&case)
    });
}

#[test]
fn bug264_0() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 0)", "========"],
        input: "OCC264_00.rle",
        triangles: exactly(26.0),
        nodes: exactly(20.0),
        area: 1.3135,
        image: "bug264_0.png",
    };
    run("modalg_display_regressions::bug264_0", || {
        cases::cleared_wireframe_view(&case)
    });
}

#[test]
fn bug264_1() {
    run_body("modalg_display_regressions::bug264_1", bug264_1_body);
}

fn bug264_1_body(identity: &str) {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 1)", "========"],
        input: "OCC264_01.brep",
        triangles: exactly(26.0),
        nodes: exactly(28.0),
        area: 0.0,
        image: "bug264_1.png",
    };
    run_todo(
        identity,
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_2() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 2)", "========"],
        input: "OCC264_02.brep",
        triangles: exactly(10.0),
        nodes: exactly(12.0),
        area: 0.0,
        image: "bug264_2.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_2",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_3() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 3)", "========"],
        input: "OCC264_03.brep",
        triangles: exactly(13.0),
        nodes: exactly(15.0),
        area: 0.0,
        image: "bug264_3.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_3",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_4() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 4)", "========"],
        input: "OCC264_04.brep",
        triangles: exactly(13.0),
        nodes: exactly(15.0),
        area: 0.0,
        image: "bug264_4.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_4",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_5() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 5)", "========"],
        input: "OCC264_05.brep",
        triangles: exactly(85.0),
        nodes: exactly(87.0),
        area: 0.0,
        image: "bug264_5.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_5",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_6() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 6)", "========"],
        input: "OCC264_06.brep",
        triangles: exactly(20.0),
        nodes: exactly(22.0),
        area: 19.2399,
        image: "bug264_6.png",
    };
    run("modalg_display_regressions::bug264_6", || {
        cases::cleared_wireframe_view(&case)
    });
}

#[test]
fn bug264_7() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 7)", "========"],
        input: "OCC264_07.brep",
        triangles: exactly(103.0),
        nodes: exactly(105.0),
        area: 150.283,
        image: "bug264_7.png",
    };
    run("modalg_display_regressions::bug264_7", || {
        cases::cleared_wireframe_view(&case)
    });
}

#[test]
fn bug264_8() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 8)", "========"],
        input: "OCC264_08.brep",
        triangles: exactly(19.0),
        nodes: exactly(21.0),
        area: 0.0,
        image: "bug264_8.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_8",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_9() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 9)", "========"],
        input: "OCC264_09.brep",
        triangles: exactly(19.0),
        nodes: exactly(21.0),
        area: 0.0,
        image: "bug264_9.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_9",
        &[
            "Error: Number of triangles",
            "Error: Number of nodes",
            "Error : The area of result shape is",
        ],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_10() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 10)", "========"],
        input: "OCC264_10.brep",
        triangles: exactly(4.0),
        nodes: exactly(6.0),
        area: 0.0,
        image: "bug264_10.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_10",
        &["Error : The area of result shape is"],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug264_11() {
    let case = ShownMeshCase {
        heading: &["========", "OCC264", " (case 11)", "========"],
        input: "OCC264_11.brep",
        triangles: exactly(55.0),
        nodes: exactly(40.0),
        area: 0.0,
        image: "bug264_11.png",
    };
    run_todo(
        "modalg_display_regressions::bug264_11",
        &["Error : The area of result shape is"],
        || cases::cleared_wireframe_view(&case),
    );
}

#[test]
fn bug287() {
    run_body("modalg_display_regressions::bug287", bug287_body);
}

fn bug287_body(identity: &str) {
    let case = ShownMeshCase {
        heading: &[
            "========================",
            " OCC287 ",
            "========================",
            "",
        ],
        input: "shading_170.brep",
        triangles: Not(1315.),
        nodes: Not(784.),
        area: 0.,
        image: "bug287.png",
    };
    run_todo(identity, &["Error : The area of result shape is"], || {
        cases::incremental_mesh_view(&case)
    });
}

#[test]
fn bug292() {
    let case = ShownMeshCase {
        heading: &[
            "========================",
            " OCC292 ",
            "========================",
            "",
        ],
        input: "OCC292.brep",
        triangles: exactly(26.),
        nodes: exactly(20.),
        area: 1.3135,
        image: "bug292.png",
    };
    run("modalg_display_regressions::bug292", || {
        cases::shaded_object_view(&case)
    });
}

#[test]
fn bug109() {
    run_body("modalg_display_regressions::bug109", bug109_body);
}

fn bug109_body(identity: &str) {
    let case = ShownMeshCase {
        heading: &[
            "================",
            "BUC61057",
            "OCC109",
            "================",
            "",
        ],
        input: "OCC109.brep",
        triangles: exactly(294.),
        nodes: exactly(300.),
        area: 33.8757,
        image: "bug109.png",
    };
    let todos = ["Error: Number of triangles", "Error: Number of nodes"];
    run_todo(identity, &todos, || cases::shaded_screenshot(&case));
}

#[test]
fn bug22798_1() {
    run("modalg_display_regressions::bug22798_1", || {
        cases::changed_sphere_view(SphereChange::Nurbs, 126261., "bug22798_1.png")
    });
}

#[test]
fn bug22798_2() {
    run("modalg_display_regressions::bug22798_2", || {
        cases::changed_sphere_view(SphereChange::Scaled, 1.12112e+06, "bug22798_2.png")
    });
}

#[test]
fn bug23170() {
    run(
        "modalg_display_regressions::bug23170",
        cases::full_sphere_face_view,
    );
}

#[test]
fn bug22818_1() {
    let case = RevolvedTriangle {
        corner: [0., 0., -10.],
        plane_point: [0., 10., 0.],
        area: 673.385,
        image: "bug22818_1.png",
    };
    run("modalg_display_regressions::bug22818_1", || {
        cases::revolved_triangle_view(&case)
    });
}

#[test]
fn bug22818_2() {
    let case = RevolvedTriangle {
        corner: [0., 5., 5.],
        plane_point: [0., 5., 5.],
        area: 161.072,
        image: "bug22818_2.png",
    };
    run("modalg_display_regressions::bug22818_2", || {
        cases::revolved_triangle_view(&case)
    });
}

fn cone_case(
    bug: &'static str,
    height: f64,
    angle: f64,
    meshed: bool,
    area: f64,
    image: &'static str,
) -> ConeCase {
    ConeCase {
        bug,
        height,
        angle,
        meshed,
        area,
        image,
    }
}

#[test]
fn bug22818_3() {
    let case = cone_case("OCC22818", 10., 180., false, 479.224, "bug22818_3.png");
    run("modalg_display_regressions::bug22818_3", || {
        cases::partial_cone_view(&case)
    });
}

#[test]
fn bug22818_4() {
    let case = cone_case("OCC22818", 20., 180., true, 708.32, "bug22818_4.png");
    run("modalg_display_regressions::bug22818_4", || {
        cases::partial_cone_view(&case)
    });
}

#[test]
fn bug23282_1() {
    let case = cone_case("OCC23282", 10., 90., false, 289.612, "bug23282_1.png");
    run("modalg_display_regressions::bug23282_1", || {
        cases::partial_cone_view(&case)
    });
}

#[test]
fn bug23282_2() {
    let case = cone_case("OCC23282", 20., 90., true, 454.16, "bug23282_2.png");
    run("modalg_display_regressions::bug23282_2", || {
        cases::partial_cone_view(&case)
    });
}

#[test]
fn bug22588() {
    run(
        "modalg_display_regressions::bug22588",
        cases::revolved_wedge_view,
    );
}

#[test]
fn bug578_1() {
    run("modalg_display_regressions::bug578_1", || {
        cases::box_minus_wires_view(CutPath::Direct, "bug578_1.png")
    });
}

#[test]
fn bug578_2() {
    run("modalg_display_regressions::bug578_2", || {
        cases::box_minus_wires_view(CutPath::Prepared, "bug578_2.png")
    });
}

#[test]
fn bug492() {
    run(
        "modalg_display_regressions::bug492",
        cases::dataset_fuse_view,
    );
}

#[test]
fn bug2785_1() {
    run("modalg_display_regressions::bug2785_1", || {
        cases::patched_cylinder_view(CutPath::Prepared, "bug2785_1.png")
    });
}

#[test]
fn bug2785_2() {
    run("modalg_display_regressions::bug2785_2", || {
        cases::patched_cylinder_view(CutPath::Direct, "bug2785_2.png")
    });
}

#[test]
fn bug25937_1() {
    run("modalg_display_regressions::bug25937_1", || {
        cases::half_space_cut_view(HalfSpaceObject::Square, "bug25937_1.png")
    });
}

#[test]
fn bug25937_2() {
    run("modalg_display_regressions::bug25937_2", || {
        cases::half_space_cut_view(HalfSpaceObject::Face, "bug25937_2.png")
    });
}

#[test]
fn bug25937_3() {
    run("modalg_display_regressions::bug25937_3", || {
        cases::half_space_cut_view(HalfSpaceObject::Box, "bug25937_3.png")
    });
}

#[test]
fn bug27270() {
    run(
        "modalg_display_regressions::bug27270",
        cases::fused_wire_compounds_view,
    );
}

#[test]
fn bug26253() {
    run(
        "modalg_display_regressions::bug26253",
        cases::general_fuse_view,
    );
}

#[test]
fn bug25982() {
    run(
        "modalg_display_regressions::bug25982",
        cases::mirrored_face_fuse_view,
    );
}

#[test]
fn bug24890() {
    run(
        "modalg_display_regressions::bug24890",
        cases::locked_face_nurbs_view,
    );
}

#[test]
fn bug24220() {
    run(
        "modalg_display_regressions::bug24220",
        cases::faulty_argument_view,
    );
}

#[test]
fn bug24654() {
    run(
        "modalg_display_regressions::bug24654",
        cases::valid_argument_view,
    );
}

#[test]
fn bug29807_b1() {
    bug29807_b1_body("modalg_display_regressions::bug29807_b1");
}

fn bug29807_b1_body(identity: &str) {
    let case = ToolCut {
        rotations: &[],
        lift: None,
        remove_location: false,
        tighten_vertex: false,
        area: 4012.74,
        volume: 9465.07,
        image: "bug29807_b1.png",
    };
    run(identity, || cases::tool_cut_view(&case));
}

#[test]
fn bug29807_b2() {
    run_body("modalg_display_regressions::bug29807_b2", bug29807_b2_body);
}

fn bug29807_b2_body(identity: &str) {
    let case = cases::tool_cut_spec(cases::SourceCut::Tilted);
    run(identity, || cases::tool_cut_view(&case));
}

#[test]
fn bug29807_b4a() {
    run_body(
        "modalg_display_regressions::bug29807_b4a",
        bug29807_b4a_body,
    );
}

fn bug29807_b4a_body(identity: &str) {
    let case = cases::tool_cut_spec(cases::SourceCut::RaisedTodo);
    run_todo(identity, cases::TOOL_CUT_TODOS, || {
        cases::tool_cut_view(&case)
    });
}

#[test]
fn bug29807_b4b() {
    run_body(
        "modalg_display_regressions::bug29807_b4b",
        bug29807_b4b_body,
    );
}

fn bug29807_b4b_body(identity: &str) {
    let case = cases::tool_cut_spec(cases::SourceCut::Tightened);
    run(identity, || cases::tool_cut_view(&case));
}

/// AMD: two snap-dependent coverage pixels (`snap_coverage`), the rest within one level.
#[test]
fn bug29807_b5a() {
    bug29807_b5a_body("modalg_display_regressions::bug29807_b5a");
}

fn bug29807_b5a_body(identity: &str) {
    let case = ToolCut {
        rotations: &[(FIRST_AXIS, 5.), (SECOND_AXIS, -38.)],
        lift: Some(2.),
        remove_location: false,
        tighten_vertex: false,
        area: 4013.42,
        volume: 9465.86,
        image: "bug29807_b5a.png",
    };
    run(identity, || cases::tool_cut_view(&case));
}

/// AMD: two snap-dependent coverage pixels (`snap_coverage`), the rest within one level.
#[test]
fn bug29807_b5b() {
    bug29807_b5b_body("modalg_display_regressions::bug29807_b5b");
}

fn bug29807_b5b_body(identity: &str) {
    let case = ToolCut {
        rotations: &[(FIRST_AXIS, 5.), (SECOND_AXIS, -38.)],
        lift: Some(2.),
        remove_location: false,
        tighten_vertex: true,
        area: 4013.42,
        volume: 9465.86,
        image: "bug29807_b5b.png",
    };
    run(identity, || cases::tool_cut_view(&case));
}

#[test]
fn bug23378_1() {
    run("modalg_display_regressions::bug23378_1", || {
        cases::blower_cut_view(1, 864274., "bug23378_1.png")
    });
}

#[test]
fn bug23378_2() {
    run("modalg_display_regressions::bug23378_2", || {
        cases::blower_cut_view(2, 2.77917, "bug23378_2.png")
    });
}

#[test]
fn bug27472() {
    run(
        "modalg_display_regressions::bug27472",
        cases::volume_changing_cut_view,
    );
}

#[test]
fn bug27760() {
    run(
        "modalg_display_regressions::bug27760",
        cases::parallel_common_view,
    );
}

#[test]
fn bug29807_b3a() {
    run("modalg_display_regressions::bug29807_b3a", || {
        cases::history_cut_view(false, "bug29807_b3a.png")
    });
}

#[test]
fn bug29807_b3b() {
    run("modalg_display_regressions::bug29807_b3b", || {
        cases::history_cut_view(true, "bug29807_b3b.png")
    });
}

/// `bug27383_<n>`: one OCC27383 regularity regression and its hidden-line view.
fn regularity(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    run(identity, || {
        banner_27383();
        body()
    });
}

fn banner_27383() {
    session_run::banner(&[
        "========",
        "0027383: Modeling - improve handling of regularity on edges",
        "========",
        "",
    ]);
}

#[test]
fn bug27383_1() {
    regularity(
        "modalg_display_regressions::bug27383_1",
        cases::fused_boxes_regularity,
    );
}

#[test]
fn bug27383_2() {
    regularity(
        "modalg_display_regressions::bug27383_2",
        cases::split_angle_regularity,
    );
}

#[test]
fn bug27383_3() {
    regularity(
        "modalg_display_regressions::bug27383_3",
        cases::sewn_spheres_regularity,
    );
}

#[test]
fn bug27383_4() {
    regularity(
        "modalg_display_regressions::bug27383_4",
        cases::closed_split_regularity,
    );
}

#[test]
fn bug27383_5() {
    regularity(
        "modalg_display_regressions::bug27383_5",
        cases::turned_copy_regularity,
    );
}

#[test]
fn bug27383_6() {
    regularity(
        "modalg_display_regressions::bug27383_6",
        cases::tangent_plane_regularity,
    );
}

#[test]
fn bug27383_7() {
    regularity(
        "modalg_display_regressions::bug27383_7",
        cases::fused_capsule_regularity,
    );
}

#[test]
fn bug28780() {
    run(
        "modalg_display_regressions::bug28780",
        cases::perspective_hidden_lines_view,
    );
}

#[test]
fn bug26180() {
    run(
        "modalg_display_regressions::bug26180",
        cases::overlapped_faces_view,
    );
}
