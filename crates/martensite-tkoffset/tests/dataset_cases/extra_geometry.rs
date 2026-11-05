//! The commands of `extra_steps` that build geometry from curves, fill surfaces, or read a mesh or
//! a hidden-line projection, each a thin form of its command.
use crate::extra_steps::{Filling, curve_of, surface_of};
use crate::more_steps::compound_of;
use crate::script::{OffsetScript, command_stopped};
use crate::support::tolerance_check::checkreal;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    ncollection::Array1,
    precision,
    standard::{Failure, Handle},
    storage::format_general,
};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::{
    geom::{GeomBSplineSurface, GeomPlane},
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::{
    app_blend::AppBlendApprox,
    geom_api,
    geom_fill::{GeomFillAppSurf, GeomFillLine, GeomFillSectionGenerator},
};
use martensite_tkgeombase::gc::MakeArcOfCircle;
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{Algo, HLRToShape, PolyAlgo, PolyHLRToShape},
};
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Axis2, Axis3, Direction3, Point3, Vector3, xoy},
};
use martensite_tkoffset::brep_offset_api::{
    BRepOffsetAPIDraftAngle, BRepOffsetAPIMakeFilling, FillingParameters,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeSweep};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
    },
    brep_lib,
};
use martensite_tkv3d::{prs3d::Prs3dDrawer, std_prs::StdPrsToolTriangulatedShape};
use std::f64::consts::PI;

/// The command's own defaults, which are not the builder's.
const FILLING_COMMAND_PARAMETERS: FillingParameters = FillingParameters {
    degree: 3,
    nb_pts_on_cur: 10,
    nb_iter: 3,
    anisotropie: false,
    tol2d: 0.00001,
    tol3d: 0.0001,
    tol_ang: 0.01,
    tol_curv: 0.1,
    max_deg: 8,
    max_segments: 9,
};

/// The plane a `to3d` names, XOY when it names none.
pub(crate) fn to_3d(
    script: &mut OffsetScript<'_>,
    name: &str,
    curve: &str,
    plane: Option<&str>,
) -> Result<(), Failure> {
    let curve = script.curve_2d(curve)?;
    let plane = match plane {
        Some(plane) => surface_of(script, plane)?.cast::<GeomPlane>(),
        None => Handle::new(GeomPlane::new(Axis3::from_axis2(xoy()))),
    };
    let plane = plane
        .get()
        .ok_or_else(|| command_stopped("to3d: expected a plane"))?
        .pln();
    let built = geom_api::to_3d(&curve, &plane)?;
    script.set_curve(name, built.cast());
    Ok(())
}

pub(crate) fn appro(
    script: &mut OffsetScript<'_>,
    name: &str,
    count: i32,
    curve: &str,
) -> Result<(), Failure> {
    let curve = curve_of(script, curve)?;
    let live = curve
        .get()
        .ok_or_else(|| command_stopped("appro: null curve"))?;
    let (u1, u2) = (live.first_parameter(), live.last_parameter());
    let delta = (u2 - u1) / f64::from(count - 1);
    let points = (0..count)
        .map(|i| live.value(u1 + f64::from(i) * delta))
        .collect::<Result<Vec<_>, _>>()?;
    let approximation = geom_api::PointsToBSpline::with_points(
        &Array1::from_shared(&points, 1),
        3,
        8,
        Continuity::C2,
        1.0e-3,
    )?;
    script.set_curve(name, approximation.curve()?.cast());
    println!("{name}");
    Ok(())
}

pub(crate) fn app_surf(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    curves: &[&str],
) -> Result<(), Failure> {
    let mut section = GeomFillSectionGenerator::new();
    for curve in curves {
        section.add_curve(&curve_of(script, curve)?)?;
    }
    section.perform(precision::p_confusion())?;
    let line = GeomFillLine::with_nb_points(curves.len() as i32);
    let mut app = GeomFillAppSurf::new(
        3,
        8,
        precision::confusion(),
        precision::p_confusion(),
        0,
        false,
    );
    app.perform(Some(&line), &mut section, false)?;
    if !app.is_done() {
        println!("    Approximation aux fraises ");
    }
    app.surf_shape()?;
    let surface = GeomBSplineSurface::with_weights(
        app.surf_poles()?,
        app.surf_weights()?,
        (app.surf_u_knots()?, app.surf_u_mults()?),
        (app.surf_v_knots()?, app.surf_v_mults()?),
        (app.u_degree()?, app.v_degree()?),
        (false, false),
    )?;
    script.set_surface(name, Handle::new(surface).cast());
    Ok(())
}

fn order(value: i32) -> Continuity {
    match value {
        i32::MIN..=0 => Continuity::C0,
        1 => Continuity::G1,
        _ => Continuity::G2,
    }
}

/// `filling`: it writes the errors it reached and fills the name.
pub(crate) fn fill(script: &mut OffsetScript<'_>, step: &Filling) -> Result<(), Failure> {
    let mut filling = BRepOffsetAPIMakeFilling::new(FILLING_COMMAND_PARAMETERS);
    for (edge, face, degree) in step.bounds {
        let edge = topods::edge(&script.required(edge)?)?;
        let face = topods::face(&script.required(face)?)?;
        filling.add_edge_on_face(&edge, &face, order(*degree), true);
    }
    for point in step.points {
        let at = script.extras.points.get(point).copied();
        let at = at.ok_or_else(|| command_stopped(&format!("{point} is not a point")))?;
        filling.add_point(at)?;
    }
    filling.build(&Default::default())?;
    if !filling.is_done() {
        println!("filling failed");
        return Ok(());
    }
    println!(
        " dist. max = {} ; angle max = {} ; diffcurv max = {}",
        format_general(filling.g0_error()?, 17),
        format_general(filling.g1_error()?, 17),
        format_general(filling.g2_error()?, 17),
    );
    let built = filling.shape()?.clone();
    script.set_shape(step.result, built);
    Ok(())
}

/// The default view looks along (1, -1, 1) with Z up; `vcomputehlr` takes its frame from it. The
/// frame's location is the view centre, which moves the projection only by a translation.
pub(crate) fn hidden_lines(script: &mut OffsetScript<'_>, poly: bool) -> Result<(), Failure> {
    let shape = script.required("a")?;
    StdPrsToolTriangulatedShape::tessellate(&shape, &Handle::new(Prs3dDrawer::default()))?;
    let direction = Direction3::from_vector(Vector3::new(1., -1., 1.))?;
    let right = Direction3::from_vector(Vector3::new(1., 1., 0.))?;
    let frame = Axis2::with_x_direction(Point3::new(0., 0., 0.), direction, right)?;
    let projector = Projector::with_frame(frame)?;
    let parts = if poly {
        let algo = Handle::new(PolyAlgo::new()?);
        let live = algo.get().expect("just made");
        live.set_projector(&projector);
        live.load(&shape);
        live.update()?;
        let mut output = PolyHLRToShape::new();
        output.update(&algo)?;
        vec![
            output.v_compound()?,
            output.out_line_v_compound()?,
            output.rg1_line_v_compound()?,
        ]
    } else {
        let algo = Handle::new(Algo::new()?);
        let live = algo.get().expect("just made");
        live.add(&shape, 0);
        live.set_projector(&projector);
        live.update()?;
        live.hide()?;
        let output = HLRToShape::new(&algo);
        vec![
            output.v_compound()?,
            output.out_line_v_compound()?,
            output.rg1_line_v_compound()?,
            output.iso_line_v_compound()?,
        ]
    };
    let kept: Vec<Shape> = parts.into_iter().filter(|part| !part.is_null()).collect();
    let result = compound_of(&[compound_of(&kept)?, compound_of(&[])?])?;
    if !poly {
        brep_lib::build_curves3d_default(&result)?;
    }
    script.set_shape("result", result);
    Ok(())
}

/// `checktrinfo`: `trinfo`'s totals over the faces, against what the script expects.
pub(crate) fn triangle_counts(
    script: &mut OffsetScript<'_>,
    name: &str,
    triangles: usize,
    nodes: usize,
) -> Result<(), Failure> {
    let shape = script.required(name)?;
    let (mut found_triangles, mut found_nodes) = (0_usize, 0_usize);
    let mut faces = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    let mut count = 0;
    while faces.more() {
        let face = topods::face(faces.current())?;
        count += 1;
        let (mesh, _) =
            BRepTool::triangulation(&face, martensite_tkmath::poly::mesh_purpose::NONE)?;
        if let Some(mesh) = mesh.get() {
            found_triangles += mesh.nb_triangles() as usize;
            found_nodes += mesh.nb_nodes() as usize;
        }
        faces.next()?;
    }
    println!("This shape contains {count} faces.");
    println!("                    {found_triangles} triangles.");
    println!("                    {found_nodes} nodes.");
    for (what, value, expected) in [
        ("triangles", found_triangles, triangles),
        ("nodes", found_nodes, nodes),
    ] {
        checkreal(
            &format!("Number of {what}"),
            &value.to_string(),
            &expected.to_string(),
            0.,
            0.,
        );
    }
    Ok(())
}

/// `bug712`: its own slab's draft. The result it sets is named after its first word, not `result`.
pub(crate) fn slab_draft(angle: f64, thickness: f64) -> Result<(), Failure> {
    let (f1, f2) = (75., 35.);
    let point = |x: f64, y: f64| Point3::new(x, y, 0.);
    let mut wire = BRepBuilderAPIMakeWire::new();
    for (a, b, c) in [
        (point(-f2, f2), point(0., f1), point(f2, f2)),
        (point(f2, f2), point(f1, 0.), point(f2, -f2)),
        (point(f2, -f2), point(0., -f1), point(-f2, -f2)),
        (point(-f2, -f2), point(-f1, 0.), point(-f2, f2)),
    ] {
        let arc = MakeArcOfCircle::through_points(a, b, c)?;
        let edge = BRepBuilderAPIMakeEdge::from_curve(&arc.value()?.cast())?.edge()?;
        wire.add_edge(&edge)?;
    }
    let maker = BRepBuilderAPIMakeFace::from_wire(&wire.wire()?, false)?;
    let face = maker.face()?;
    let surface = BRepTool::surface(&face)?;
    let plane = surface
        .cast::<GeomPlane>()
        .get()
        .expect("a planar face")
        .pln();
    let direction = Direction3::from_vector(Vector3::new(0., 0., 1.))?;
    let vector = Vector3::new(0., 0., thickness);
    let mut slab = BRepPrimAPIMakePrism::new(face.as_ref(), vector, true, true)?;
    let solid = slab.shape()?.clone();
    let (first, last) = (slab.first_shape()?, slab.last_shape()?);
    let mut draft = BRepOffsetAPIDraftAngle::with_shape(&solid)?;
    let mut faces = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        let current = faces.current();
        if !current.is_same(&first) && !current.is_same(&last) {
            let side = topods::face(current)?;
            draft.add(&side, direction, angle * (PI / 180.), &plane, true)?;
            if !draft.add_done()? {
                println!(" Error in Add ");
                return Err(command_stopped(" Error in Add "));
            }
        }
        faces.next()?;
    }
    println!("All Faces added. Building... ");
    draft.build(&Default::default())?;
    println!("Build done...");
    if !draft.is_done() {
        println!(" Error in Build ");
        return Err(command_stopped(" Error in Build "));
    }
    Ok(())
}

/// The fixed columns of a coordinate file's line: `x` in the first 20, `y` in the next 20.
fn line_point(line: &str) -> Point3 {
    let bytes = line.as_bytes();
    let column = |range: std::ops::Range<usize>| -> f64 {
        let end = range.end.min(bytes.len());
        let start = range.start.min(end);
        std::str::from_utf8(&bytes[start..end])
            .unwrap_or("")
            .trim()
            .parse()
            .unwrap_or(0.)
    };
    Point3::new(column(0..20), column(20..40), 0.)
}

pub(crate) fn coord_load(
    script: &mut OffsetScript<'_>,
    name: &str,
    path: &str,
) -> Result<(), Failure> {
    let bytes = crate::support::restore::input_bytes(path.rsplit('/').next().unwrap_or(path));
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let mut lines = text.split('\n');
    let mut previous =
        BRepBuilderAPIMakeVertex::new(line_point(lines.next().unwrap_or("")))?.vertex()?;
    let mut wire = BRepBuilderAPIMakeWire::new();
    // `getline` fails only past the last newline: the empty piece after it is not a line.
    let rest: Vec<&str> = lines.collect();
    let count = rest.len().saturating_sub(usize::from(text.ends_with('\n')));
    for line in &rest[..count] {
        let next = BRepBuilderAPIMakeVertex::new(line_point(line))?.vertex()?;
        let edge = BRepBuilderAPIMakeEdge::from_vertices(&previous, &next)?.edge()?;
        wire.add_edge(&edge)?;
        previous = next;
    }
    if !wire.is_done() {
        script.set_shape(name, Shape::default());
        return Ok(());
    }
    let face = BRepBuilderAPIMakeFace::from_wire(&wire.wire()?, false)?;
    let shape = if face.is_done() {
        face.face()?.as_ref().clone()
    } else {
        Shape::default()
    };
    script.set_shape(name, shape);
    Ok(())
}

/// `drawcont` reads each contour's curves to draw them; a failing read still ends the command.
pub(crate) fn read_contours(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let explorer = &mut script.extras.explorer;
    for contour in 1..=explorer.number_of_contours() {
        explorer.init(contour);
        while explorer.more()? {
            let _curve: Handle<dyn Geom2dCurve> = explorer.value()?;
            explorer.next();
        }
    }
    Ok(())
}

/// `result`: every arc's bisector of the graph, as the command reads them to draw.
pub(crate) fn bisectors(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let locus = &script.extras.locus;
    let graph = locus.graph();
    let graph = graph.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            "result: no bisecting locus; expected mat first",
        )
    })?;
    for arc in 1..=graph.number_of_arcs() {
        locus.geom_bis(&graph.arc(arc)?)?;
    }
    Ok(())
}
