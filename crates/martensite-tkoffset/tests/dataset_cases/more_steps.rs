//! Commands few scripts use: surfaces, faces on them, history queries, the general-fuse sequence,
//! evolved solids, middle paths, normal projection, fillets, trims and scale changes.
use crate::script::{OffsetScript, command_stopped};
use crate::steps::{General, HistoryStep, More, Surface};
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{History, HistoryAlgorithm},
    topods::{self, CompSolid, Compound, Shape, Shell, Solid, Wire},
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::{Array1, Array2, List},
    precision,
    standard::{Failure, Handle},
};
use martensite_tkfeat::brep_feat::{self, BRepFeatMakeLinearForm, StatusError};
use martensite_tkg2d::geom2d::Geom2dTrimmedCurve;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
    GeomSphericalSurface, GeomSurface, GeomToroidalSurface, GeomTrimmedCurve,
};
use martensite_tkmath::{
    geom_abs::{JoinType, Shape as Continuity},
    gp::{Axis2, Axis3, Direction3, Point3, Transform3, Vector3, dx, dz},
};
use martensite_tkoffset::brep_offset_api::{
    BRepOffsetAPIMakeEvolved, BRepOffsetAPIMiddlePath, BRepOffsetAPINormalProjection,
    EvolvedOptions,
};
use martensite_tkshhealing::shape_custom::ShapeCustom;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPISewing, BRepBuilderAPITransform,
};
use std::f64::consts::PI;

/// The history of a builder, for `savehistory`.
pub struct MakeShapeHistory<'a, T: BRepBuilderAPIMakeShape + ?Sized>(pub &'a mut T);

impl<T: BRepBuilderAPIMakeShape + ?Sized> HistoryAlgorithm for MakeShapeHistory<'_, T> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }

    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }

    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

fn point(v: &[f64]) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}

fn direction(v: &[f64]) -> Result<Direction3, Failure> {
    Direction3::from_vector(Vector3::new(v[0], v[1], v[2]))
}

/// `[x y z [dx dy dz [ux uy uz]]]` of `plane`, `cylinder` and the others.
fn frame(v: &[f64]) -> Result<Axis3, Failure> {
    match v.len() {
        0 => Axis3::new(Point3::new(0., 0., 0.), dz(), dx()),
        3 => Axis3::new(point(v), dz(), dx()),
        6 => Axis3::from_normal(point(v), direction(&v[3..])?),
        9 => Axis3::new(point(v), direction(&v[3..])?, direction(&v[6..])?),
        count => Err(command_stopped(&format!(
            "frame of {count} values; expected 0, 3, 6 or 9"
        ))),
    }
}

fn analytic(step: &Surface) -> Result<Handle<dyn GeomSurface>, Failure> {
    Ok(match step {
        Surface::Plane(_, v) => Handle::new(GeomPlane::new(frame(v)?)).cast(),
        Surface::Cylinder(_, v, r) => {
            Handle::new(GeomCylindricalSurface::new(frame(v)?, *r)?).cast()
        }
        Surface::Sphere(_, v, r) => Handle::new(GeomSphericalSurface::new(frame(v)?, *r)?).cast(),
        Surface::Cone(_, v, [angle, r]) => {
            Handle::new(GeomConicalSurface::new(frame(v)?, angle * (PI / 180.), *r)?).cast()
        }
        Surface::Torus(_, v, [a, b]) => {
            Handle::new(GeomToroidalSurface::new(frame(v)?, *a, *b)?).cast()
        }
        Surface::Bezier(_, v) => bezier(v)?,
        Surface::BSpline(_, v) => bspline(v)?,
    })
}

/// `beziersurf name nu nv pole [weight] ...`: three or four values per pole, by the word count.
fn pole_grid(
    v: &[f64],
    nu: i32,
    nv: i32,
) -> Result<(Array2<'static, Point3>, Array2<'static, f64>), Failure> {
    let stride = v.len() / (nu * nv) as usize;
    let mut poles = Array2::with_bounds(1, nu, 1, nv).map_err(grid_failure)?;
    let mut weights = Array2::with_bounds(1, nu, 1, nv).map_err(grid_failure)?;
    let mut rows = v.chunks_exact(stride);
    for j in 1..=nv {
        for i in 1..=nu {
            let row = rows.next().expect("a value row per pole");
            poles.set_value(i, j, point(row)).map_err(grid_failure)?;
            let weight = if stride == 4 { row[3] } else { 1. };
            weights.set_value(i, j, weight).map_err(grid_failure)?;
        }
    }
    Ok((poles, weights))
}

fn grid_failure(failure: impl std::fmt::Debug) -> Failure {
    command_stopped(&format!("surface poles: {failure:?}"))
}

fn bezier(v: &[f64]) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (nu, nv) = (v[0] as i32, v[1] as i32);
    let (poles, weights) = pole_grid(&v[2..], nu, nv)?;
    Ok(Handle::new(GeomBezierSurface::with_weights(&poles, &weights)?).cast())
}

/// `bsplinesurf name du nku ku mu ... dv nkv kv mv ... pole weight ...`.
fn bspline(v: &[f64]) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (du, nku) = (v[0] as i32, v[1] as usize);
    let (knots_u, rest) = v[2..].split_at(2 * nku);
    let (dv, nkv) = (rest[0] as i32, rest[1] as usize);
    let (knots_v, poles) = rest[2..].split_at(2 * nkv);
    let count = |knots: &[f64], degree: i32| -> i32 {
        knots.chunks_exact(2).map(|k| k[1] as i32).sum::<i32>() - degree - 1
    };
    let (grid, weights) = pole_grid(poles, count(knots_u, du), count(knots_v, dv))?;
    let values = |knots: &[f64]| -> Vec<f64> { knots.chunks_exact(2).map(|k| k[0]).collect() };
    let mults =
        |knots: &[f64]| -> Vec<i32> { knots.chunks_exact(2).map(|k| k[1] as i32).collect() };
    let surface = GeomBSplineSurface::with_weights(
        &grid,
        &weights,
        (
            &Array1::from_shared(&values(knots_u), 1),
            &Array1::from_shared(&mults(knots_u), 1),
        ),
        (
            &Array1::from_shared(&values(knots_v), 1),
            &Array1::from_shared(&mults(knots_v), 1),
        ),
        (du, dv),
        (false, false),
    )?;
    Ok(Handle::new(surface).cast())
}

/// Runs a rarely used command; `apply(script, &More::scale("r", "a", 0.1))`.
pub fn apply(script: &mut OffsetScript<'_>, step: &More) -> Result<(), Failure> {
    match step {
        More::Surface(surface) => {
            let name = surface.name();
            let built = analytic(surface)?;
            script.set_surface(name, built);
            Ok(())
        }
        More::Face(name, surface, bounds) => face(script, name, surface, *bounds),
        More::FaceOnWire(name, surface, wire, keep) => {
            face_on_wire(script, name, surface, wire, *keep)
        }
        More::Moves(name, words) => crate::profile_command::profile(script, name, words),
        More::Middle(result, shape, start, end) => middle(script, result, shape, start, end),
        More::Evolved(evolved) => evolved_solid(script, evolved),
        More::Project(project) => project_normal(script, project),
        More::TrimCurve(name, curve, range) => trim_curve(script, name, curve, *range),
        More::Mirror(names, v) => mirror(script, names, v),
        More::Scale(result, shape, factor) => {
            let shape = script.required(shape)?;
            script.set_shape(result, ShapeCustom::scale_shape(&shape, *factor)?);
            Ok(())
        }
        More::Sewing(result, tolerance, shapes) => sewing(script, result, *tolerance, shapes),
        More::SurfaceOfFace(name, face) => {
            let face = topods::face(&script.required(face)?)?;
            let surface = martensite_tkbrep::brep::BRepTool::surface(&face)?;
            script.set_surface(name, surface);
            Ok(())
        }
        More::LinearForm(form) => linear_form(script, form),
        More::PerformLinearForm(result) => perform_linear_form(script, result),
        More::MultiOffset(multi) => crate::multi_offset::run(script, multi),
        More::Compound(parts, name) => compound(script, parts, name),
        More::EmptyShape(name, kind) => empty_shape(script, name, kind),
        More::History(history) => history_step(script, history),
        More::General(general) => general_step(script, general),
    }
}

fn surface_of(script: &OffsetScript<'_>, name: &str) -> Result<Handle<dyn GeomSurface>, Failure> {
    script
        .surface(name)
        .ok_or_else(|| command_stopped(&format!("{name} is not a surface")))
}

fn face(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    surface: &str,
    bounds: Option<[f64; 4]>,
) -> Result<(), Failure> {
    let surface = surface_of(script, surface)?;
    let shape = match bounds {
        None => BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?
            .shape()?
            .clone(),
        Some([a, b, c, d]) => BRepBuilderAPIMakeFace::from_surface_bounds(
            &surface,
            (a, b, c, d),
            precision::confusion(),
        )?
        .shape()?
        .clone(),
    };
    script.set_shape(name, shape);
    Ok(())
}

fn face_on_wire(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    surface: &str,
    wire: &str,
    keep_orientation: bool,
) -> Result<(), Failure> {
    let surface = surface_of(script, surface)?;
    let wire = topods::wire(&script.required(wire)?)?;
    let shape = BRepBuilderAPIMakeFace::from_surface_wire(&surface, &wire, keep_orientation)?
        .shape()?
        .clone();
    script.set_shape(name, shape);
    Ok(())
}

fn middle(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    shape: &str,
    start: &str,
    end: &str,
) -> Result<(), Failure> {
    let (shape, start, end) = (
        script.required(shape)?,
        script.required(start)?,
        script.required(end)?,
    );
    let mut builder = BRepOffsetAPIMiddlePath::new(&shape, &start, &end)?;
    builder.build(&Default::default())?;
    script.set_shape(result, builder.shape()?.clone());
    Ok(())
}

/// `evolved result -s spine -p profile [-solid] [-a] [-v] [-t tol] [-stm]`.
fn evolved_solid(
    script: &mut OffsetScript<'_>,
    step: &crate::steps::Evolved,
) -> Result<(), Failure> {
    let base = script.required(step.spine)?;
    let profile = topods::wire(&script.required(step.profile)?)?;
    let options = EvolvedOptions {
        join_type: JoinType::Arc,
        is_axe_prof: !step.compute_axes,
        is_solid: step.solid,
        is_prof_on_spine: false,
        tol: step.tolerance,
        is_volume: step.volume,
        run_in_parallel: step.parallel,
    };
    let mut builder = BRepOffsetAPIMakeEvolved::with_spine(&base, &profile, options)?;
    script.set_shape(step.result, builder.shape()?.clone());
    Ok(())
}

/// `nproject result e... surface [-g] [-d dmax] [tol [continuity [maxdeg [maxseg]]]]`.
fn project_normal(
    script: &mut OffsetScript<'_>,
    step: &crate::steps::Project,
) -> Result<(), Failure> {
    let surface = script.required(step.surface)?;
    let mut projection = BRepOffsetAPINormalProjection::with_shape(&surface);
    for name in step.shapes {
        projection.add(&script.required(name)?)?;
    }
    if step.unlimited {
        projection.set_limit(false);
    }
    if let Some(distance) = step.max_distance {
        projection.set_max_distance(distance);
    }
    let continuity = match step.continuity {
        0 => Continuity::C0,
        1 => Continuity::C1,
        _ => Continuity::C2,
    };
    let tolerance = step.tolerance;
    projection.set_params(
        tolerance,
        martensite_cmath::pow(tolerance, 2.0 / 3.0),
        continuity,
        step.max_degree,
        step.max_segments,
    );
    projection.build(&Default::default())?;
    let mut wires = List::default();
    if projection.build_wire(&mut wires)? {
        println!(" BuildWire OK ");
    }
    script.set_shape(step.result, projection.shape()?.clone());
    Ok(())
}

fn trim_curve(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    curve: &str,
    [u1, u2]: [f64; 2],
) -> Result<(), Failure> {
    if let Ok(planar) = script.curve_2d(curve) {
        let trimmed = Geom2dTrimmedCurve::new(&planar, u1, u2, true, true)?;
        script.set_curve_2d(name, Handle::new(trimmed).cast());
        return Ok(());
    }
    let basis = script
        .curve(curve)
        .ok_or_else(|| command_stopped(&format!("{curve} is not a defined curve")))?;
    let trimmed = GeomTrimmedCurve::new(&basis, u1, u2, true, true)?;
    script.set_curve(name, Handle::new(trimmed).cast());
    Ok(())
}

fn mirror(
    script: &mut OffsetScript<'_>,
    names: &[&'static str],
    v: &[f64; 6],
) -> Result<(), Failure> {
    let mut transform = Transform3::default();
    transform.set_mirror_plane(Axis2::new(point(v), direction(&v[3..])?)?);
    for name in names {
        let mut maker = BRepBuilderAPITransform::new(transform);
        maker.perform(&script.required(name)?, false, false)?;
        script.set_shape(name, maker.shape()?.clone());
    }
    Ok(())
}

fn compound(
    script: &mut OffsetScript<'_>,
    parts: &[&str],
    name: &'static str,
) -> Result<(), Failure> {
    let shapes = parts
        .iter()
        .map(|part| script.required(part))
        .collect::<Result<Vec<_>, _>>()?;
    script.set_shape(name, compound_of(&shapes)?);
    Ok(())
}

/// `shape name kind`: an empty shape of the kind.
fn empty_shape(
    script: &mut OffsetScript<'_>,
    name: &'static str,
    kind: &str,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let upper = kind.to_ascii_uppercase();
    let shape: Shape = match upper.as_str() {
        "C" | "CO" | "COMPOUND" => compound_of(&[])?,
        "CS" | "COMPSOLID" => {
            let mut made = CompSolid::default();
            builder.make_comp_solid(&mut made);
            made.as_ref().clone()
        }
        "SO" | "SOLID" => {
            let mut made = Solid::default();
            builder.make_solid(&mut made);
            made.as_ref().clone()
        }
        "SH" | "SHELL" => {
            let mut made = Shell::default();
            builder.make_shell(&mut made);
            made.as_ref().clone()
        }
        "W" | "WIRE" => {
            let mut made = Wire::default();
            builder.make_wire(&mut made);
            made.as_ref().clone()
        }
        other => {
            return Err(command_stopped(&format!(
                "shape {name} {other}: unknown kind"
            )));
        }
    };
    script.set_shape(name, shape);
    Ok(())
}

pub fn compound_of(parts: &[Shape]) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut made = Compound::default();
    builder.make_compound(&mut made);
    let mut shape = made.as_ref().clone();
    for part in parts {
        builder.add(&mut shape, part)?;
    }
    Ok(shape)
}

fn history_step(script: &mut OffsetScript<'_>, step: &HistoryStep) -> Result<(), Failure> {
    match step {
        HistoryStep::Save(name) => {
            let history = script.last_history.clone().unwrap_or_default();
            script.saved_histories.insert(name, history);
            Ok(())
        }
        HistoryStep::Generated(result, name, shape) => {
            history_query(script, result, name, shape, true)
        }
        HistoryStep::Modified(result, name, shape) => {
            history_query(script, result, name, shape, false)
        }
    }
}

fn history_query(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    name: &str,
    shape: &str,
    generated: bool,
) -> Result<(), Failure> {
    let history =
        script.saved_histories.get(name).cloned().ok_or_else(|| {
            command_stopped(&format!("History with the name {name} does not exist."))
        })?;
    let source = script.required(shape)?;
    let shapes = history.get().map_or(Ok(Vec::new()), |h| {
        if generated {
            h.generated(&source)
        } else {
            h.modified(&source)
        }
    })?;
    if shapes.is_empty() {
        println!(
            "{}",
            if generated {
                "The shape has not been generated."
            } else {
                "The shape has not been modified."
            }
        );
        return Ok(());
    }
    if shapes.len() == 1 {
        script.set_shape(result, shapes[0].clone());
        return Ok(());
    }
    script.set_shape(result, compound_of(&shapes)?);
    Ok(())
}

/// Records a builder's history as the last one, as each command that fills it does.
/// Example: `remember_history(script, &[profile], &mut MakeShapeHistory(&mut builder))`.
pub fn remember_history(
    script: &mut OffsetScript<'_>,
    arguments: &[Shape],
    algorithm: &mut dyn HistoryAlgorithm,
) -> Result<(), Failure> {
    script.last_history = Some(Handle::new(History::from_algorithm(arguments, algorithm)?));
    Ok(())
}

fn general_step(script: &mut OffsetScript<'_>, step: &General) -> Result<(), Failure> {
    match step {
        General::ClearObjects => script.general_objects.clear(),
        General::ClearTools => script.general_tools.clear(),
        General::AddObjects(names) => {
            for name in *names {
                let shape = script.required(name)?;
                script.general_objects.push(shape);
            }
        }
        General::AddTools(names) => {
            for name in *names {
                let shape = script.required(name)?;
                script.general_tools.push(shape);
            }
        }
        General::Fill => fill_arguments(script)?,
        General::Build(result) => build_general(script, result)?,
        General::Operation(result, operation) => general_operation(script, result, *operation)?,
    }
    Ok(())
}

fn fill_arguments(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let mut filler = BOPAlgoPaveFiller::new();
    let arguments = script
        .general_objects
        .iter()
        .chain(script.general_tools.iter())
        .cloned()
        .collect::<List<_>>();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    if filler.options().has_errors() {
        println!(" error reported by the pave filler");
    }
    script.general_filler = Some(filler);
    Ok(())
}

fn build_general(script: &mut OffsetScript<'_>, result: &'static str) -> Result<(), Failure> {
    let Some(mut filler) = script.general_filler.take() else {
        return Err(command_stopped(" prepare PaveFiller first"));
    };
    let mut builder = BOPAlgoBuilder::new();
    for shape in script
        .general_objects
        .iter()
        .chain(script.general_tools.iter())
    {
        builder.add_argument(shape);
    }
    builder.options_mut().set_run_parallel(false);
    builder.set_to_fill_history(true);
    builder.perform_with_filler(&mut filler);
    script.general_filler = Some(filler);
    if builder.options().has_errors() {
        println!(" error reported by the builder");
        return Ok(());
    }
    let shape = builder.shape().clone();
    let history = builder.history();
    script.set_shape(result, shape);
    script.last_history = history;
    Ok(())
}

fn status_text(error: StatusError) -> String {
    let mut text = Vec::new();
    brep_feat::print(error, &mut text).expect("a vector takes the status text");
    String::from_utf8_lossy(&text).into_owned()
}

/// `featlf base wire plane dx dy dz fx fy fz fuse modify`.
fn linear_form(
    script: &mut OffsetScript<'_>,
    step: &crate::steps::LinearForm,
) -> Result<(), Failure> {
    let base = script.required(step.base)?;
    let wire = topods::wire(&script.required(step.wire)?)?;
    let plane = surface_of(script, step.plane)?.cast::<GeomPlane>();
    if plane.is_null() {
        return Err(command_stopped("null plane to transform"));
    }
    let vector = |v: [f64; 3]| Vector3::new(v[0], v[1], v[2]);
    let mut feature = BRepFeatMakeLinearForm::new();
    feature.init(
        &base,
        &wire,
        &plane,
        vector(step.direct),
        vector(step.other),
        step.fuse,
        step.modify,
    )?;
    if !feature.is_done() {
        return Err(command_stopped(&status_text(
            feature.rib().current_status_error(),
        )));
    }
    script.linear_form = Some(feature);
    Ok(())
}

/// `featperform lf result`.
fn perform_linear_form(script: &mut OffsetScript<'_>, result: &'static str) -> Result<(), Failure> {
    let Some(mut feature) = script.linear_form.take() else {
        return Err(command_stopped("Form not defined"));
    };
    feature.perform()?;
    if !feature.is_done() {
        return Err(command_stopped(&status_text(
            feature.rib().current_status_error(),
        )));
    }
    script.set_shape(result, feature.shape()?.clone());
    Ok(())
}

/// `bbop result operation`, on the sequence's arguments and pave filler.
fn general_operation(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    operation: i32,
) -> Result<(), Failure> {
    let Some(mut filler) = script.general_filler.take() else {
        return Err(command_stopped(" prepare PaveFiller first"));
    };
    let kind = match operation {
        0 => Operation::Common,
        1 => Operation::Fuse,
        2 => Operation::Cut,
        3 => Operation::Cut21,
        _ => Operation::Section,
    };
    let mut builder = BOPAlgoBOP::new();
    for shape in &script.general_objects {
        builder.add_argument(shape);
    }
    for shape in &script.general_tools {
        builder.add_tool(shape);
    }
    builder.set_operation(kind);
    builder.options_mut().set_run_parallel(false);
    builder.set_check_inverted(true);
    builder.set_to_fill_history(true);
    builder.perform_with_filler(&mut filler);
    script.general_filler = Some(filler);
    crate::support::common_operation::report_result_alerts(builder.options());
    if !builder.options().has_errors() && !builder.shape().is_null() {
        script.set_shape(result, builder.shape().clone());
        script.last_history = builder.history();
    }
    Ok(())
}

/// `sewing result [tol] shapes...`; the dump goes to the stream, as the command writes it.
fn sewing(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    tolerance: f64,
    shapes: &[&str],
) -> Result<(), Failure> {
    let working = tolerance.max(precision::confusion());
    let mut sewer = BRepBuilderAPISewing::default();
    sewer.init(working, true, true, true, false);
    sewer.set_same_parameter_mode(true);
    sewer.set_floating_edges_mode(false);
    sewer.set_face_mode(true);
    sewer.set_min_tolerance((tolerance * 1e-4).max(precision::confusion()));
    sewer.set_max_tolerance(precision::infinite());
    for name in shapes {
        sewer.add(&script.required(name)?)?;
    }
    sewer.perform(&ProgressRange::default())?;
    let mut dump = Vec::new();
    sewer.dump(&mut dump)?;
    print!("{}", String::from_utf8_lossy(&dump));
    let sewn = sewer.sewed_shape();
    if !sewn.is_null() {
        script.set_shape(result, sewn.clone());
    }
    Ok(())
}
