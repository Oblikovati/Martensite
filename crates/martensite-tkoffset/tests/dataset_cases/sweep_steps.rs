//! `thrusections`, the `mksweep` family and `depouille`, with the arguments the commands pass.
use crate::more_steps::{MakeShapeHistory, remember_history};
use crate::script::{OffsetScript, command_stopped};
use crate::steps::{Draft, Loft, PipeShell, SweepBuild, SweepSection};
use martensite_tkbool::brep_fill::{ThruSectionErrorStatus, TypeOfContact};
use martensite_tkbrep::{
    brep_tools::History,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    ncollection::Array1,
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::law::{LawFunction, LawInterpol};
use martensite_tkmath::gp::{Axis2, Direction3, Plane3, Point2, Point3, Vector3};
use martensite_tkoffset::{
    brep_offset_api::{
        BRepOffsetAPIDraftAngle, BRepOffsetAPIMakePipeShell, BRepOffsetAPIThruSections,
    },
    draft::DraftErrorStatus,
};
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIPipeError, BRepBuilderAPITransitionMode,
};
use std::f64::consts::PI;

fn edge_count(shape: &Shape) -> Result<usize, Failure> {
    let mut count = 0;
    let mut explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while explorer.more() {
        count += 1;
        explorer.next()?;
    }
    Ok(count)
}

fn loft_status_text(status: ThruSectionErrorStatus) -> &'static str {
    match status {
        ThruSectionErrorStatus::NotDone => "Algorithm is not done\n",
        ThruSectionErrorStatus::NotSameTopology => {
            "The input profiles should be all closed or all opened\n"
        }
        ThruSectionErrorStatus::ProfilesInconsistent => "Profiles inconsistent\n",
        ThruSectionErrorStatus::WrongUsage => "Wrong usage of punctual sections\n",
        ThruSectionErrorStatus::Null3DCurve => "Some edges have null 3d curve\n",
        ThruSectionErrorStatus::Failed => "Algorithm has failed\n",
        ThruSectionErrorStatus::Done => "",
    }
}

/// A section that is a wire and one that is a vertex differ in how they count toward "the same
/// number of edges".
fn add_section(generator: &mut BRepOffsetAPIThruSections, shape: &Shape) -> Result<bool, Failure> {
    if shape.shape_type()? == ShapeEnum::Wire {
        generator.add_wire(&topods::wire(shape)?);
        return Ok(true);
    }
    generator.add_vertex(&topods::vertex(shape)?)?;
    Ok(false)
}

/// `thrusections [-N] result issolid isruled shape... [-safe]`.
pub fn loft(script: &mut OffsetScript<'_>, step: &Loft) -> Result<(), Failure> {
    let mut generator = BRepOffsetAPIThruSections::new(step.solid, step.ruled, 1.0e-06);
    let (mut first_wire, mut edges, mut same_number) = (false, 0, true);
    for name in step.sections {
        let shape = script.required(name)?;
        let is_wire = add_section(&mut generator, &shape)?;
        first_wire ^= is_wire;
        let count = edge_count(&shape)?;
        if first_wire {
            edges = count;
        } else if is_wire && count != edges {
            same_number = false;
        }
    }
    generator.set_mutable_input(step.mutable_input);
    generator.check_compatibility(step.check || !same_number);
    generator.build(&Default::default())?;
    if generator.is_done() {
        script.set_shape(step.result, generator.shape()?.clone());
        let wires: Vec<Shape> = generator.wires().iter().cloned().collect();
        remember_history(script, &wires, &mut MakeShapeHistory(&mut generator))?;
    } else {
        print!("{}", loft_status_text(generator.get_status()));
    }
    Ok(())
}

fn sweep_of<'a>(
    script: &'a mut OffsetScript<'_>,
) -> Result<&'a mut BRepOffsetAPIMakePipeShell, Failure> {
    script
        .commands
        .sweep
        .as_mut()
        .ok_or_else(|| command_stopped("You have forgotten the <<mksweep>> command  !"))
}

fn corrected_spine(
    spine: &Shape,
    angle: f64,
    linear: f64,
) -> Result<(Shape, Handle<History>), Failure> {
    let unifier = ShapeUpgradeUnifySameDomain::new(spine, true, false, true);
    unifier.set_angular_tolerance(angle * PI / 180.0);
    unifier.set_linear_tolerance(linear);
    unifier.build()?;
    Ok((unifier.shape().clone(), unifier.history()))
}

fn direction(v: &[f64]) -> Result<Direction3, Failure> {
    Direction3::from_vector(Vector3::new(v[0], v[1], v[2]))
}

/// Runs one step of a pipe shell; `shell(script, &PipeShell::frenet)`.
pub fn shell(script: &mut OffsetScript<'_>, step: &PipeShell) -> Result<(), Failure> {
    match step {
        PipeShell::Start(spine) => start(script, spine, None),
        PipeShell::StartCorrected(spine, angle, linear) => {
            start(script, spine, Some((*angle, *linear)))
        }
        PipeShell::Support(_) | PipeShell::Guide(..) => shell_on_shape(script, step),
        PipeShell::Add(section) => add(script, section),
        PipeShell::Delete(name) => {
            let section = script.required(name)?;
            sweep_of(script)?.delete(&section)
        }
        PipeShell::Build(build) => build_shell(script, build),
        _ => shell_option(sweep_of(script)?, step),
    }
}

fn start(
    script: &mut OffsetScript<'_>,
    spine: &str,
    corrected: Option<(f64, f64)>,
) -> Result<(), Failure> {
    let mut wire = script.required(spine)?;
    script.commands.sweep = None;
    if let Some((angle, linear)) = corrected {
        let (corrected, history) = corrected_spine(&wire, angle, linear)?;
        wire = corrected;
        script.last_history = Some(history);
    }
    script.commands.sweep = Some(BRepOffsetAPIMakePipeShell::new(&topods::wire(&wire)?)?);
    Ok(())
}

fn shell_on_shape(script: &mut OffsetScript<'_>, step: &PipeShell) -> Result<(), Failure> {
    match step {
        PipeShell::Support(surface) => {
            let surface = script.required(surface)?;
            sweep_of(script)?.set_mode_support(&surface)?;
            Ok(())
        }
        PipeShell::Guide(guide, curvilinear, contact) => {
            let guide = topods::wire(&script.required(guide)?)?;
            let contact = match contact {
                i32::MIN..=0 => TypeOfContact::NoContact,
                1 => TypeOfContact::Contact,
                _ => TypeOfContact::ContactOnBorder,
            };
            sweep_of(script)?.set_mode_auxiliary(&guide, *curvilinear, contact)
        }
        _ => unreachable!("shell sends only the steps that name a shape"),
    }
}

fn shell_option(sweep: &mut BRepOffsetAPIMakePipeShell, step: &PipeShell) -> Result<(), Failure> {
    match step {
        PipeShell::Frenet => sweep.set_mode_frenet(true),
        PipeShell::CorrectedFrenet => sweep.set_mode_frenet(false),
        PipeShell::Discrete => sweep.set_discrete_mode(),
        PipeShell::BiNormal(v) => sweep.set_mode_bi_normal(direction(v)?),
        PipeShell::Fixed(v) => {
            let origin = Point3::new(0., 0., 0.);
            let frame = if v.len() == 6 {
                Axis2::with_x_direction(origin, direction(v)?, direction(&v[3..])?)?
            } else {
                Axis2::new(origin, direction(v)?)?
            };
            sweep.set_mode_axe(&frame)
        }
        PipeShell::MaxDegree(value) => {
            sweep.set_max_degree(*value);
            Ok(())
        }
        PipeShell::MaxSegments(value) => {
            sweep.set_max_segments(*value);
            Ok(())
        }
        _ => unreachable!("shell sends only the options here"),
    }
}

fn law_of(values: &[f64]) -> Result<Option<Handle<dyn LawFunction>>, Failure> {
    if values.is_empty() {
        return Ok(None);
    }
    let points: Vec<Point2> = values
        .chunks_exact(2)
        .map(|p| Point2::new(p[0], p[1]))
        .collect();
    let periodic = (points[0].y() - points[points.len() - 1].y()).abs() < precision::confusion();
    let law = LawInterpol::new();
    law.set(&Array1::from_shared(&points, 1), periodic)?;
    Ok(Some(Handle::new(law).cast()))
}

fn add(script: &mut OffsetScript<'_>, step: &SweepSection) -> Result<(), Failure> {
    let section = script.shape(step.section);
    let kind = if section.is_null() {
        None
    } else {
        Some(section.shape_type()?)
    };
    if !matches!(kind, Some(ShapeEnum::Wire | ShapeEnum::Vertex)) {
        return Err(command_stopped(&format!(
            "{} is not a wire and is not a vertex!",
            step.section
        )));
    }
    let vertex = step.vertex.map(|name| script.required(name)).transpose()?;
    let vertex = vertex.as_ref().map(topods::vertex).transpose()?;
    let law = law_of(step.law)?;
    let sweep = sweep_of(script)?;
    let (t, r) = (step.translate, step.rotate);
    match (law, vertex) {
        (None, Some(v)) => sweep.add_at(&section, &v, t, r),
        (None, None) => sweep.add(&section, t, r),
        (Some(l), Some(v)) => sweep.set_law_at(&section, &l, &v, t, r),
        (Some(l), None) => sweep.set_law(&section, &l, t, r),
    }
}

fn build_shell(script: &mut OffsetScript<'_>, step: &SweepBuild) -> Result<(), Failure> {
    let sweep = sweep_of(script)?;
    if !sweep.is_ready() {
        return Err(command_stopped(
            "You have forgotten the <<addsweep>> command  !",
        ));
    }
    match step.mode {
        'C' => sweep.set_transition_mode(BRepBuilderAPITransitionMode::RightCorner),
        'R' => sweep.set_transition_mode(BRepBuilderAPITransitionMode::RoundCorner),
        'M' => sweep.set_transition_mode(BRepBuilderAPITransitionMode::Transformed),
        _ => {}
    }
    sweep.build(&Default::default())?;
    if !sweep.is_done() {
        print!("{}", not_done_text(sweep.get_status()));
        return Ok(());
    }
    if step.solid && !sweep.make_solid()? {
        println!(" BuildSweep : It is impossible to make a solid !");
    }
    let shape = sweep.shape()?.clone();
    let mut arguments: Vec<Shape> = sweep.profiles().iter().cloned().collect();
    arguments.push(sweep.spine().clone().into_shape());
    let history = History::from_algorithm(&arguments, &mut MakeShapeHistory(sweep))?;
    script.last_history = Some(Handle::new(history));
    script.set_shape(step.result, shape);
    Ok(())
}

fn not_done_text(status: BRepBuilderAPIPipeError) -> &'static str {
    match status {
        BRepBuilderAPIPipeError::PlaneNotIntersectGuide => {
            "Buildsweep : Not Done\nBuildsweep : One Plane not intersect the guide\n"
        }
        BRepBuilderAPIPipeError::ImpossibleContact => {
            "Buildsweep : Not Done\nBuildSweep : One section can not be in contact with the guide\n"
        }
        _ => "Buildsweep : Not Done\n",
    }
}

fn draft_status_text(status: DraftErrorStatus) -> &'static str {
    match status {
        DraftErrorStatus::NoError => "No error",
        DraftErrorStatus::FaceRecomputation => "Impossible face recomputation",
        DraftErrorStatus::EdgeRecomputation => "Impossible edge recomputation",
        DraftErrorStatus::VertexRecomputation => "Impossible vertex recomputation",
    }
}

fn draft_stop(draft: &BRepOffsetAPIDraftAngle, message: &str) -> Result<(), Failure> {
    let status = draft_status_text(draft.status()?);
    Err(command_stopped(&format!(
        "{message}  Error Status : {status}"
    )))
}

/// `depouille result shape dx dy dz face angle x y z nx ny nz ...`.
pub fn draft(script: &mut OffsetScript<'_>, step: &Draft) -> Result<(), Failure> {
    let source = script.required(step.shape)?;
    let mut builder = BRepOffsetAPIDraftAngle::with_shape(&source)?;
    let extraction = direction(&step.direction)?;
    for (name, degrees, plane) in step.faces {
        let face = topods::face(&script.shape(name))?;
        let plane = Plane3::new(
            Point3::new(plane[0], plane[1], plane[2]),
            direction(&plane[3..])?,
        )?;
        builder.add(&face, extraction, degrees * PI / 180.0, &plane, true)?;
        if !builder.add_done()? {
            break;
        }
    }
    if !builder.add_done()? {
        script.set_shape("bugdep", builder.problematic_shape()?);
        return draft_stop(&builder, "Bad shape in variable bugdep ");
    }
    builder.build(&Default::default())?;
    if !builder.is_done() {
        script.set_shape("bugdep", builder.problematic_shape()?);
        return draft_stop(
            &builder,
            "Problem encountered during the reconstruction : bad shape in variable bugdep; ",
        );
    }
    script.set_shape(step.result, builder.shape()?.clone());
    Ok(())
}
