//! The commands only the sample scripts and `bugs/heal/bug26244` use, and the `BUC60811` QA command.
use crate::script::{FilletScript, command_stopped};
use crate::steps::Sample;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Edge, Shape, Shell, Solid},
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::Array2,
    precision,
    standard::{Failure, Handle},
    storage::format_general,
};
use martensite_tkfillet::brep_fillet_api::BRepFilletAPIMakeFillet2d;
use martensite_tkg2d::{
    geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkg3d::{
    geom::{
        GeomBezierSurface, GeomCurve, GeomOffsetSurface, GeomPlane, GeomSurface, GeomTrimmedCurve,
    },
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::{
    geom_api,
    geom2d_gcc::{self, Lin2d2Tan},
};
use martensite_tkgeombase::geom_proj_lib;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmath::{
    gp::{Axis2, Axis3, Axis22d, Direction3, Point2, Point3, Transform3, Vector3, dx2d},
    toploc::Location,
};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeWire, BRepBuilderAPITransform,
};

/// Run one sample command; `apply(&mut script, &Sample::ReverseCurve("c"))?`.
pub fn apply(script: &mut FilletScript<'_>, step: &Sample) -> Result<(), Failure> {
    match step {
        Sample::Circle2d(name, values) => circle_2d(script, name, *values),
        Sample::TangentLines(name, first, second) => tangent_lines(script, name, first, second),
        Sample::Trim(name, base, range) => trim(script, name, base, *range),
        Sample::To3d(name, curve, plane) => to_3d(script, name, curve, plane),
        Sample::ReverseCurve(name) => reverse_curve(script, name),
        Sample::VertexOnCurve(name, curve, parameter) => {
            vertex_on_curve(script, name, curve, *parameter)
        }
        _ => apply_to_shapes(script, step),
    }
}

/// The sample commands that work on shapes rather than curves.
fn apply_to_shapes(script: &mut FilletScript<'_>, step: &Sample) -> Result<(), Failure> {
    match step {
        Sample::ResetLocation(name) => {
            let located = script.required(name)?.located(&Location::default(), true)?;
            script.set_shape(name, located);
            Ok(())
        }
        Sample::Mirror(name, v) => mirror(script, name, v),
        Sample::UnifySameDomain(result, shape) => unify(script, result, shape),
        Sample::Common(result, a, b) => common(script, result, a, b),
        Sample::Mesh(name, deflection) => mesh(script, name, *deflection),
        Sample::CheckDeflection(name, value, absolute, relative) => {
            check_deflection(script, name, *value, *absolute, *relative)
        }
        Sample::Buc60811 => buc60811(script),
        _ => unreachable!("apply dispatches the curve commands"),
    }
}

fn circle_2d(
    script: &mut FilletScript<'_>,
    name: &str,
    [x, y, r]: [f64; 3],
) -> Result<(), Failure> {
    let frame = Axis22d::new(Point2::new(x, y), dx2d(), true)?;
    let circle: Handle<dyn Geom2dCurve> = Handle::new(Geom2dCircle::new(frame, r)?).cast();
    script.set_curve_2d(name, circle);
    Ok(())
}

fn tangent_lines(
    script: &mut FilletScript<'_>,
    name: &str,
    first: &str,
    second: &str,
) -> Result<(), Failure> {
    let (c1, c2) = (script.curve_2d(first)?, script.curve_2d(second)?);
    let lines = Lin2d2Tan::with_parameters(
        &geom2d_gcc::unqualified(&Geom2dAdaptorCurve::with_curve(&c1)?)?,
        &geom2d_gcc::unqualified(&Geom2dAdaptorCurve::with_curve(&c2)?)?,
        precision::angular(),
        middle_parameter(&c1)?,
        middle_parameter(&c2)?,
    )?;
    publish_tangent_lines(script, name, &lines)
}

fn publish_tangent_lines(
    script: &mut FilletScript<'_>,
    name: &str,
    lines: &Lin2d2Tan,
) -> Result<(), Failure> {
    if !lines.is_done() {
        println!("Lin2d2Tan Not done");
        return Ok(());
    }
    for index in 1..=lines.nb_solutions() {
        let line = Geom2dLine::from_lin2d(lines.this_solution(index)?);
        script.set_curve_2d(&format!("{name}_{index}"), Handle::new(line).cast());
    }
    Ok(())
}

fn middle_parameter(curve: &Handle<dyn Geom2dCurve>) -> Result<f64, Failure> {
    let live = curve
        .get()
        .ok_or_else(|| command_stopped("lintan: null curve"))?;
    Ok((live.first_parameter() + live.last_parameter()) / 2.0)
}

fn trim(
    script: &mut FilletScript<'_>,
    name: &str,
    base: &str,
    [u1, u2]: [f64; 2],
) -> Result<(), Failure> {
    if let Some(curve) = script.curve(base) {
        let trimmed: Handle<dyn GeomCurve> =
            Handle::new(GeomTrimmedCurve::new(&curve, u1, u2, true, true)?).cast();
        script.set_curve(name, trimmed);
        return Ok(());
    }
    let curve = script.curve_2d(base)?;
    let trimmed = Geom2dTrimmedCurve::new(&curve, u1, u2, true, true)?;
    script.set_curve_2d(name, Handle::new(trimmed).cast());
    Ok(())
}

fn to_3d(
    script: &mut FilletScript<'_>,
    name: &str,
    curve: &str,
    plane: &[f64],
) -> Result<(), Failure> {
    let (frame, _) = crate::input_shapes::conic_frame(plane, 0)?;
    let plane = GeomPlane::new(Axis3::from_axis2(frame)).pln();
    let result = geom_api::to_3d(&script.curve_2d(curve)?, &plane)?;
    script.set_curve(name, result.cast());
    Ok(())
}

fn reverse_curve(script: &mut FilletScript<'_>, name: &str) -> Result<(), Failure> {
    if let Some(curve) = script.curve(name) {
        curve
            .get()
            .ok_or_else(|| command_stopped("reverse: null curve"))?
            .reverse()?;
        return Ok(());
    }
    let curve = script.curve_2d(name)?;
    curve
        .get()
        .ok_or_else(|| command_stopped("reverse: null curve"))?
        .reverse()?;
    Ok(())
}

fn vertex_on_curve(
    script: &mut FilletScript<'_>,
    name: &'static str,
    curve: &str,
    parameter: f64,
) -> Result<(), Failure> {
    let handle = script
        .curve(curve)
        .ok_or_else(|| command_stopped("cvalue: no curve"))?;
    let live = handle
        .get()
        .ok_or_else(|| command_stopped("cvalue: null curve"))?;
    let point = live.value(parameter)?;
    script.apply_vertex(name, point)
}

fn mirror(script: &mut FilletScript<'_>, name: &str, v: &[f64; 6]) -> Result<(), Failure> {
    let normal = Direction3::from_vector(Vector3::new(v[3], v[4], v[5]))?;
    let mut trsf = Transform3::default();
    trsf.set_mirror_plane(Axis2::new(Point3::new(v[0], v[1], v[2]), normal)?);
    let mut transformer = BRepBuilderAPITransform::new(trsf);
    transformer.perform(&script.required(name)?, false, false)?;
    script.set_shape(name, transformer.shape()?.clone());
    Ok(())
}

fn unify(script: &mut FilletScript<'_>, result: &str, shape: &str) -> Result<(), Failure> {
    let unifier = ShapeUpgradeUnifySameDomain::new(&script.required(shape)?, true, true, false);
    unifier.set_safe_input_mode(true);
    unifier.allow_internal_edges(false);
    unifier.set_linear_tolerance(precision::confusion());
    unifier.set_angular_tolerance(precision::angular());
    unifier.build()?;
    script.set_shape(result, unifier.shape());
    Ok(())
}

/// `bcommon`: `bsmt` with its default options.
fn common(
    script: &mut FilletScript<'_>,
    result: &'static str,
    a: &str,
    b: &str,
) -> Result<(), Failure> {
    crate::fillet_steps::bsmt(
        script,
        result,
        (a, b),
        martensite_tkbo::bop_algo::Operation::Common,
    )
}

fn mesh(script: &mut FilletScript<'_>, name: &str, deflection: f64) -> Result<(), Failure> {
    BRepMeshIncrementalMesh::with_shape(&script.required(name)?, deflection, false, 0.5, false)?;
    println!("Incremental Mesh, multi-threading OFF");
    println!("Meshing statuses: NoError");
    Ok(())
}

/// The largest deflection of the faces' triangulations, as `trinfo` reports it.
fn maximal_deflection(shape: &Shape) -> Result<f64, Failure> {
    let mut maximal = 0.0_f64;
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        let (triangulation, _) =
            BRepTool::triangulation(&topods::face(faces.current())?, mesh_purpose::NONE)?;
        if let Some(mesh) = triangulation.get() {
            maximal = maximal.max(mesh.deflection());
        }
        faces.next()?;
    }
    Ok(maximal)
}

fn check_deflection(
    script: &mut FilletScript<'_>,
    name: &str,
    expected: f64,
    absolute: f64,
    relative: f64,
) -> Result<(), Failure> {
    let value = maximal_deflection(&script.required(name)?)?;
    let printed = format_general(value, 17);
    let expected_text = format_general(expected, 17);
    if (value - expected).abs() > absolute + relative * expected.abs() {
        println!("Error: Maximal deflection = {printed} is not equal to expected {expected_text}");
    } else {
        println!("Check of Maximal deflection OK: value = {printed}, expected = {expected_text}");
    }
    Ok(())
}

const PATCH_CORNERS: [[f64; 2]; 6] = [
    [150., 150.],
    [350., 150.],
    [350., 250.],
    [300., 250.],
    [200., 200.],
    [150., 200.],
];

/// The Bezier surface of step 1 of the command.
fn bezier_patch() -> Result<Handle<dyn GeomSurface>, Failure> {
    let rows: [[[f64; 3]; 3]; 3] = [
        [[0., 100., 0.], [200., 100., 0.], [400., 100., 0.]],
        [[0., 200., 100.], [200., 200., 100.], [400., 200., 100.]],
        [[0., 300., 0.], [200., 300., 0.], [400., 300., 0.]],
    ];
    let mut poles = Array2::with_bounds(1, 3, 1, 3).map_err(|_| command_stopped("poles"))?;
    for (i, row) in (1..).zip(rows) {
        for (j, [x, y, z]) in (1..).zip(row) {
            poles
                .set_value(i, j, Point3::new(x, y, z))
                .map_err(|_| command_stopped("poles"))?;
        }
    }
    Ok(Handle::new(GeomBezierSurface::new(&poles)?).cast())
}

fn patch_face() -> Result<Shape, Failure> {
    let mut wire = BRepBuilderAPIMakeWire::new();
    for (index, [x, y]) in PATCH_CORNERS.iter().enumerate() {
        let [nx, ny] = PATCH_CORNERS[(index + 1) % PATCH_CORNERS.len()];
        let edge = BRepBuilderAPIMakeEdge::from_points(
            Point3::new(*x, *y, 260.),
            Point3::new(nx, ny, 260.),
        )?;
        wire.add_edge(&edge.edge()?)?;
    }
    let face = BRepBuilderAPIMakeFace::from_wire(&wire.wire()?, false)?;
    Ok(face.face()?.into_shape())
}

/// The first two vertices of the patch, the status of each fillet printed, then the result.
fn filleted_patch(patch: &Shape) -> Result<Shape, Failure> {
    let face = topods::face(patch)?;
    let mut fillet = BRepFilletAPIMakeFillet2d::with_face(&face)?;
    let mut vertices = Explorer::new(patch, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    for _ in 0..2 {
        fillet.add_fillet(&topods::vertex(vertices.current())?, 20.)?;
        println!("\nError is {}", fillet.status() as i32);
        vertices.next()?;
    }
    fillet.build(&ProgressRange::default())?;
    Ok(fillet.shape()?.clone())
}

fn projected_edge(edge: &Edge, surface: &Handle<dyn GeomSurface>) -> Result<Edge, Failure> {
    let (curve, first, last) = BRepTool::curve(edge)?;
    let trimmed: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&curve, first, last, true, true)?).cast();
    BRepBuilderAPIMakeEdge::from_curve(&geom_proj_lib::project(&trimmed, surface)?)?.edge()
}

/// The command's steps 1 to 4, each shape displayed as its `AisShape` was; the solid of step 1
/// is built as there and then not used.
fn buc60811(script: &mut FilletScript<'_>) -> Result<(), Failure> {
    let surface = bezier_patch()?;
    let first = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
    solid_of_face(first.face()?.as_ref())?;
    script.set_shape("F1", first.face()?.into_shape());
    let patch = patch_face()?;
    script.set_shape("FP", patch.clone());
    let offset: Handle<dyn GeomSurface> =
        Handle::new(GeomOffsetSurface::new(&surface, -100.0, false)?).cast();
    let second = BRepBuilderAPIMakeFace::from_surface(&offset, precision::confusion())?;
    script.set_shape("F2", second.face()?.into_shape());
    let filleted = filleted_patch(&patch)?;
    script.set_shape("FP1", filleted.clone());
    let wire = wire_on_surface(&filleted, &offset)?;
    script.set_shape("Wire", wire);
    Ok(())
}

/// The shell and solid the QA command builds around its face and then does not use.
fn solid_of_face(face: &Shape) -> Result<Solid, Failure> {
    let builder = BRepBuilder::default();
    let (mut shell, mut solid) = (Shell::default(), Solid::default());
    builder.make_shell(&mut shell);
    builder.add(&mut shell, face)?;
    builder.make_solid(&mut solid);
    builder.add(&mut solid, shell.as_ref())?;
    Ok(solid)
}

/// The edges of `shape` projected on `surface`, in one wire.
fn wire_on_surface(shape: &Shape, surface: &Handle<dyn GeomSurface>) -> Result<Shape, Failure> {
    let mut wire = BRepBuilderAPIMakeWire::new();
    let mut edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        wire.add_edge(&projected_edge(&topods::edge(edges.current())?, surface)?)?;
        edges.next()?;
    }
    Ok(wire.wire()?.into_shape())
}
