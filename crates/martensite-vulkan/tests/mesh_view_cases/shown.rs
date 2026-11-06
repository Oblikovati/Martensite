//! Shapes cleaned and meshed, or meshed by their shaded presentation, then shown and dumped.
use super::prelude::*;
use crate::primitive_shapes::cone;
use crate::shape_checks::check_shape_valid;
use martensite_tkv3d::v3d::V3dTypeOfOrientation as O;

/// One cleaned-and-meshed regression: its banner, input, mesh, expected counts and image.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CleanedMesh {
    pub(crate) heading: &'static [&'static str],
    pub(crate) input: &'static str,
    pub(crate) checked_input: bool,
    pub(crate) request: MeshRequest,
    pub(crate) expected: Option<TriangulationExpectation>,
    pub(crate) image: &'static str,
}

/// `bug23184_1/2`, `bug24938`: a restored shape cleaned, meshed, measured and shown shaded.
pub(crate) fn cleaned_mesh_shaded(case: CleanedMesh) -> Result<(), Box<dyn Error>> {
    banner(case.heading);
    let result = read_dataset_shape(case.input)?;
    if case.checked_input {
        check_shape_valid(&result)?;
    }
    remeshed(&result, case.request)?;
    if let Some(expected) = case.expected {
        check_triangulation(&result, &expected)?;
    }
    let mut run = shaded_view(&result, true)?;
    run.dump(case.image)?;
    run.finish()
}

const COARSE_EDGES_VIEW: ViewParameters = ViewParameters {
    scale: Some(44.578446),
    proj: Some([8.964134e-4, 0.055037, -0.998484]),
    up: Some([-0.951475, 0.307306, 0.016085]),
    at: Some([44.487383, 37.608231, -3.232527]),
    eye: Some([44.517406, 37.608231, -3.232527]),
};

/// `bug26928`: a compound shown with its automatic mesh, then remeshed at four deflections
/// with an 11.459 degree angle and redisplayed from its own triangulation.
pub(crate) fn coarse_edges_remeshed() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=========",
        "0026928: Edges are meshed too coarsly",
        "=========",
        "",
    ]);
    let c = read_dataset_shape("bug27356_comp.brep")?;
    remove_triangulation(&c)?;
    let mut run = SessionRun::new()?;
    run.session().display("c", &c, &DisplayRequest::default())?;
    run.session().set_view_parameters(&COARSE_EDGES_VIEW)?;
    run.session().set_object_display_mode("c", 1)?;
    run.dump("bug26928-vdisplay.png")?;
    run.session().set_auto_triangulation(false);
    for (deflection, image) in [
        (0.715, "bug26928-0715.png"),
        (0.5, "bug26928-05.png"),
        (0.3, "bug26928-03.png"),
        (0.1, "bug26928-01.png"),
    ] {
        remeshed(&c, MeshRequest::new(deflection).with_angle_degrees(11.459))?;
        run.session().redisplay("c")?;
        // The red loose edges tie on one-pixel lines; each image's uncovered-surface control
        // shows c's face alone (the recording tool, user decision 2026-10-28).
        run.dump_with_faces_only_control(image, "c", &c, 1)?;
    }
    run.finish()
}

/// `bug29641`: an extruded spline shaded with deviation coefficient 1e-4, seen from the top
/// turned about the origin, keeps its counts.
pub(crate) fn deviation_coefficient_top() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0029641: Mesher produce 'bad' result for extruded spline with given deviation coefficient",
        "========",
        "",
    ]);
    let result = read_dataset_shape("bug29641.brep")?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session().set_default_deviation_coefficient(0.0001);
    run.session().set_display_mode(1)?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_standard_projection(O::ZUP_TOP)?;
    run.session().rotate_about([-0.1, 0.1, 1.], [0., 0., 0.])?;
    run.session().fit()?;
    check_triangulation(&result, &counts(29040., 15358.))?;
    run.dump("bug29641.png")?;
    run.finish()
}

/// `bug28500`: a shape meshed at 0.01 has no faulty mesh and shows shaded without artifacts.
pub(crate) fn mesh_faulties() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "CR28500: Artifact in shaded view of the shape",
        "=======",
        "",
    ]);
    let result = read_dataset_shape("bug28500_shape_mesh_artifact.brep")?;
    remeshed(&result, MeshRequest::new(0.01))?;
    let mut run = shaded_view(&result, false)?;
    if !crate::mesh_topology_check::check_mesh_topology(&result)?.is_clean() {
        println!("Error : Mesh contains faulties");
    }
    run.dump("bug28500.png")?;
    run.finish()
}

/// `bug28379`: a thin cone meshed at 1e-3 seen shaded from left, right, front and back.
pub(crate) fn side_views_of_thin_cone() -> Result<(), Box<dyn Error>> {
    banner(&["========", "OCC28379", "========", ""]);
    let s = cone(0., 0.1, 1.)?;
    mesh_shape(&s, &MeshRequest::new(1e-3))?;
    let mut run = shaded_view(&s, true)?;
    let sides = [
        (O::ZUP_LEFT, "bug28379-left.png"),
        (O::ZUP_RIGHT, "bug28379-right.png"),
        (O::ZUP_FRONT, "bug28379-front.png"),
        (O::ZUP_BACK, "bug28379-back.png"),
    ];
    for (orientation, image) in sides {
        run.session().set_standard_projection(orientation)?;
        run.session().fit()?;
        run.dump(image)?;
    }
    run.finish()
}

/// `bug28719`: a cleaned shape shaded by its presentation has every face triangulated.
pub(crate) fn shaded_after_tricheck() -> Result<(), Box<dyn Error>> {
    banner(&["========", "OCC28719", "========", ""]);
    let result = read_dataset_shape("bug28719_display_issue.brep")?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_object_display_mode("result", 1)?;
    run.session().fit()?;
    if crate::mesh_topology_check::check_mesh_topology(&result)?.is_clean() {
        println!("Mesh is OK");
    } else {
        println!("ERROR: OCC28719 is reproduced. Nontriangulated faces have been detected.");
    }
    run.dump("bug28719.png")?;
    run.finish()
}
