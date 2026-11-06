//! Shapes meshed explicitly, then measured, checked for mesh topology and shown.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::SurfaceAspects;
use crate::common_operation;
use crate::primitive_shapes::{cone, cylinder, sphere, translated};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCone, BRepPrimAPIMakeSphere};
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use std::time::Instant;

/// `bug25042`: a cone meshed relatively in parallel keeps its counts and a valid mesh.
pub(crate) fn cone_relative_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "==========",
        "0025042: Cone Meshing: Missing two connecting edges.",
        "==========",
        "",
    ]);
    let result = cone(200., 0., 500.)?;
    let angle = 0.2 * 180.0 / std::f64::consts::PI;
    let request = MeshRequest {
        relative: true,
        parallel: true,
        ..MeshRequest::new(0.2).with_angle_degrees(angle)
    };
    mesh_shape(&result, &request)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().set_auto_triangulation(false);
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(1.81755),
        proj: Some([0.88572, 0.104526, 0.452299]),
        up: Some([-0.0339444, 0.986295, -0.16146]),
        at: Some([15.3368, 42.2711, 210.324]),
        eye: Some([679.928, 120.701, 549.702]),
    })?;
    run.dump("bug25042.png")?;
    check_triangulation(
        &result,
        &counts_deflection(509., 327., 1.5916215272983552, 1e-6),
    )?;
    report_mesh_validity(&result)?;
    run.finish()
}

/// `bug25157`: a face of a half cone cut by a cylinder meshes with triangles and nodes.
pub(crate) fn cut_cone_face_wireframe() -> Result<(), Box<dyn Error>> {
    banner(&["=========", "CR25157", "=========", ""]);
    let half_cone = BRepPrimAPIMakeCone::with_angle(6., 0., 10., std::f64::consts::PI)?
        .shape()?
        .clone();
    let tool = translated(&cylinder(1., 6.)?, [0., 3., 1.])?;
    let cut = common_operation::direct(&half_cone, &tool, Operation::Cut);
    let face = all_distinct(&cut, ShapeEnum::Face)[0].clone();
    remove_triangulation(&face)?;
    mesh_shape(&face, &MeshRequest::new(0.1))?;
    let present = TriangulationExpectation {
        triangles: CountExpectation::Present,
        nodes: CountExpectation::Present,
        ..TriangulationExpectation::default()
    };
    check_triangulation(&face, &present)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("Cut_1", &face, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug25157.png")?;
    run.finish()
}

/// `bug25594`: a healed face shows shaded from the front with a valid mesh of known counts.
pub(crate) fn fixed_face_front() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0025594: Valid shape is not visualize in shading mode.",
        "=======",
        "",
    ]);
    let fixer = ShapeFixShape::default();
    fixer.init(read_dataset_shape("bug25594_face1.brep")?);
    fixer.perform()?;
    let result = fixer.shape();
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    run.session().fit()?;
    report_mesh_validity(&result)?;
    check_triangulation(&result, &counts(374., 224.))?;
    run.dump("bug25594.png")?;
    run.finish()
}

const HOLLOW_CUT_HEADING: &[&str] = &[
    "=======",
    "0029751: Incremental mesh produces different meshes for windows and linux (debian 8)",
    "=======",
    "",
];

/// Create the authored sphere before its cylinder, cut and rotate the actual result.
/// Example: `let result = hollow_cut_result()?;`.
pub(crate) fn hollow_cut_result() -> Result<Shape, martensite_tkernel::standard::Failure> {
    let object = sphere(10.)?;
    let tool = translated(&cylinder(2., 10.)?, [0., 0., -10.])?;
    let cut = common_operation::direct(&object, &tool, Operation::Cut);
    crate::primitive_shapes::rotated_x(&cut, 45.)
}

/// Apply the source hollow alias and visible mesh edges to the actual scene.
/// Example: `hollow_cut_scene(&mut session, &shape)?;`.
pub(crate) fn hollow_cut_scene(
    session: &mut crate::case_view::viewer_session::ViewerSession,
    result: &Shape,
) -> Result<(), martensite_tkernel::standard::Failure> {
    session.set_axonometric()?;
    session.set_auto_triangulation(false);
    session.display(
        "result",
        result,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    let hollow_with_edges = SurfaceAspects {
        interior_style: Some(AspectInteriorStyle::Empty),
        draw_edges: Some(true),
        ..SurfaceAspects::default()
    };
    session.set_surface_aspects("result", &hollow_with_edges)?;
    session.fit()
}

/// `bug29751`: preserve actual cut/mesh checks and the hollow presentation.
/// The source initial vclear has no objects in the isolated fresh namespace.
/// Example: `hollow_cut_sphere()?`.
pub(crate) fn hollow_cut_sphere() -> Result<(), Box<dyn Error>> {
    banner(HOLLOW_CUT_HEADING);
    let result = hollow_cut_result()?;
    mesh_shape(&result, &MeshRequest::new(0.1))?;
    let mut run = SessionRun::new()?;
    hollow_cut_scene(run.session(), &result)?;
    report_mesh_validity(&result)?;
    check_triangulation(
        &result,
        &counts_deflection(1013., 578., 0.1164052220738387, 1e-6),
    )?;
    run.dump("bug29751.png")?;
    run.finish()
}

/// `bug31449_2`: a face with a loop-back boundary meshes at 1e-4 without free nodes or links.
pub(crate) fn loopback_face_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "0031449: Mesh - BRepMesh works too long and produces many free nodes on a valid face",
        "================",
        "",
    ]);
    let result = read_dataset_shape("bug31449_face_loopback_M.brep")?;
    mesh_shape(&result, &MeshRequest::new(0.0001))?;
    if !crate::mesh_topology_check::check_mesh_topology(&result)?.is_clean() {
        println!("Error : something weird has been produced, like free nodes, or links");
    }
    check_triangulation(&result, &counts(65., 67.))?;
    checkview_3d(&result, "bug31449_2.png")
}

/// `bug25503_1/2`: meshing a sphere again at the same deflection reuses its triangulation,
/// so the second meshing takes under a second (whole seconds, as `time` reports).
pub(crate) fn remeshed_sphere_time(segment: bool, deflection: f64) -> Result<(), Box<dyn Error>> {
    let a = if segment {
        let quarter = std::f64::consts::FRAC_PI_4;
        BRepPrimAPIMakeSphere::with_latitudes_angle(100., -quarter, quarter, std::f64::consts::PI)?
            .shape()?
            .clone()
    } else {
        sphere(100.)?
    };
    for pass in ["first", "second"] {
        println!("incmesh {pass} time");
        let start = Instant::now();
        mesh_shape(&a, &MeshRequest::new(deflection))?;
        let seconds = start.elapsed().as_secs();
        println!("spent {seconds} sec");
        if pass == "second" && seconds != 0 {
            println!("Error: second time must be quicker");
        }
    }
    println!("TEST COMPLETED");
    Ok(())
}

/// `bug27845`: a spline cavity meshed at 10 with 180 degrees keeps its counts and is valid.
pub(crate) fn spline_cavity_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "==========",
        "0027845: BRepMesh produces invalid result on spline cavity",
        "==========",
        "",
    ]);
    let result = read_dataset_shape("bug27845_shape.brep")?;
    remeshed(&result, MeshRequest::new(10.).with_angle_degrees(180.))?;
    let mut run = shaded_view(&result, false)?;
    run.dump("bug27845.png")?;
    check_triangulation(
        &result,
        &counts_deflection(3834., 4777., 3.0544822246414993, 1e-6),
    )?;
    report_mesh_validity(&result)?;
    run.finish()
}

/// `bug31865`: a shape meshed at a large deflection triangulates without failure.
pub(crate) fn terrain_free_mesh() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0031865: Mesh - triangulation fails with large deflection values due to unhandled Standard_OutOfRange, BRepMesh_PairOfIndex::Append()",
        "========",
        "",
    ]);
    let result = read_dataset_shape("bug31865.brep")?;
    remeshed(&result, MeshRequest::new(0.2))?;
    let mut run = SessionRun::new()?;
    run.session().set_auto_triangulation(false);
    crate::shape_view_checks::checkview_in(&mut run, &result, "bug31865.png")?;
    report_mesh_validity(&result)?;
    run.finish()
}
