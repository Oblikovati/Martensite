//! Meshing regressions of the bugs and perf mesh groups, each in an isolated worker under its
//! group's policy.
use super::*;

use mesh_view_cases as cases;
use shape_mesh::MeshRequest;
use std::error::Error;

/// A regression of the bugs group (1000 s CPU limit, bugs and common rules).
fn bugs(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_bugs_process_case::run_bugs_case(identity, body);
}

/// A regression of the perf group (500 s CPU limit, perf and common rules).
fn perf(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_perf_process_case::run_perf_case(identity, body);
}

const CR23184: &[&str] = &[
    "========================",
    "CR23184",
    "========================",
    "",
];

#[test]
fn bug23184_1() {
    bugs("mesh_view_regressions::bug23184_1", || {
        cases::cleaned_mesh_shaded(cases::CleanedMesh {
            heading: CR23184,
            input: "bug23184_tessface2.brep",
            checked_input: true,
            request: MeshRequest::new(0.045),
            expected: None,
            image: "bug23184_1.png",
        })
    });
}

#[test]
fn bug23184_2() {
    bugs("mesh_view_regressions::bug23184_2", || {
        cases::cleaned_mesh_shaded(cases::CleanedMesh {
            heading: CR23184,
            input: "bug23184_tessface.brep",
            checked_input: true,
            request: MeshRequest::new(0.045),
            expected: None,
            image: "bug23184_2.png",
        })
    });
}

#[test]
fn bug24938() {
    case_bugs_process_case::run_bugs_case_within("mesh_view_regressions::bug24938", 100, || {
        cases::cleaned_mesh_shaded(cases::CleanedMesh {
            heading: &[
                "==========",
                "OCC24938: BRepMesh_IncrementalMesh hangs",
                "==========",
                "",
            ],
            input: "bug24938_27773.brep",
            checked_input: false,
            request: MeshRequest {
                relative: true,
                ..MeshRequest::new(1.5)
            },
            expected: Some(cases::counts_deflection(
                8.,
                10.,
                3.3489133970888776e-05,
                1e-6,
            )),
            image: "bug24938.png",
        })
    });
}

#[test]
fn bug25042() {
    bugs("mesh_view_regressions::bug25042", cases::cone_relative_mesh);
}

#[test]
fn bug25157() {
    bugs(
        "mesh_view_regressions::bug25157",
        cases::cut_cone_face_wireframe,
    );
}

#[test]
fn bug25503_1() {
    bugs("mesh_view_regressions::bug25503_1", || {
        cases::banner(&[
            "============",
            "CR25503",
            "============",
            "",
            "===> meshing segment of sphere",
        ]);
        cases::remeshed_sphere_time(true, 0.003)
    });
}

#[test]
fn bug25503_2() {
    bugs("mesh_view_regressions::bug25503_2", || {
        cases::banner(&[
            "============",
            "CR25503: BRepMesh - IncrementalMesh always re-meshes the shape even if existing triangulation satisfies the given deflection",
            "============",
            "",
            "",
        ]);
        cases::remeshed_sphere_time(false, 0.015)
    });
}

#[test]
fn bug25594() {
    bugs("mesh_view_regressions::bug25594", cases::fixed_face_front);
}

/// lavapipe: `attribute_rounding`, the 0.5 image `edge_tie`. AMD: `shaded_silhouette`, the
/// 0.715, 0.3 and 0.1 images `line_snap_tie` (user decision 2026-10-29).
#[test]
fn bug26928() {
    bugs(
        "mesh_view_regressions::bug26928",
        cases::coarse_edges_remeshed,
    );
}

#[test]
fn bug27845() {
    bugs("mesh_view_regressions::bug27845", cases::spline_cavity_mesh);
}

/// The faces-only control of bug26928 keeps the compound's one face and its four edges and
/// drops the five loose edges (`nbshapes c`: 9 edges, `explode c`: one face, five edges).
#[test]
fn bug26928_faces_only_control_drops_the_loose_edges() -> Result<(), Box<dyn Error>> {
    use boolean_typed_explode::all_distinct;
    use martensite_tkg3d::top_abs::ShapeEnum;
    let c = dataset_shape::read_dataset_shape("bug27356_comp.brep")?;
    let faces = faces_only_control::faces_compound(&c)?;
    assert_eq!(all_distinct(&c, ShapeEnum::Edge).len(), 9);
    assert_eq!(all_distinct(&faces, ShapeEnum::Face).len(), 1);
    assert_eq!(all_distinct(&faces, ShapeEnum::Edge).len(), 4);
    Ok(())
}

#[test]
fn bug28379() {
    bugs(
        "mesh_view_regressions::bug28379",
        cases::side_views_of_thin_cone,
    );
}

#[test]
fn bug28500() {
    bugs("mesh_view_regressions::bug28500", cases::mesh_faulties);
}

#[test]
fn bug28719() {
    bugs(
        "mesh_view_regressions::bug28719",
        cases::shaded_after_tricheck,
    );
}

#[test]
fn bug29641() {
    bugs(
        "mesh_view_regressions::bug29641",
        cases::deviation_coefficient_top,
    );
}

#[test]
fn bug29751() {
    bugs("mesh_view_regressions::bug29751", cases::hollow_cut_sphere);
}

#[test]
fn bug31449_2() {
    bugs(
        "mesh_view_regressions::bug31449_2",
        cases::loopback_face_mesh,
    );
}

#[test]
fn bug31865() {
    bugs("mesh_view_regressions::bug31865", cases::terrain_free_mesh);
}

#[test]
fn bug23650() {
    perf("mesh_view_regressions::bug23650", cases::meshed_shaded);
}

#[test]
fn bug23795() {
    perf(
        "mesh_view_regressions::bug23795",
        cases::meshed_shaded_counts,
    );
}

#[test]
fn bug25264() {
    perf("mesh_view_regressions::bug25264", cases::revolved_meshes);
}

#[test]
fn bug26965() {
    perf(
        "mesh_view_regressions::bug26965",
        cases::display_mesh_counts,
    );
}

#[test]
fn bug30511() {
    perf(
        "mesh_view_regressions::bug30511",
        cases::assembly_mesh_counts,
    );
}
