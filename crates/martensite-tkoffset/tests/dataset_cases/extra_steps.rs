//! The commands only a handful of scripts use, each a thin Rust form of its command in the same order
//! of calls: healing, deformations, surfaces and curves built on others, the medial axis, drafts and
//! the rolling ball.
use crate::extra_geometry;
use crate::script::{OffsetScript, command_stopped};
use martensite_tkbo::bop_algo::BOPAlgoMakerVolume;
use martensite_tkbrep::topods;
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::{Array1, List},
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomOffsetCurve, GeomOffsetSurface, GeomSurface};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::{
    geom_abs::JoinType,
    gp::{Axis1, Direction3, GeneralTransform3, Matrix3, Point2, Point3, Transform3, Vector3},
};
use martensite_tkoffset::{
    bi_tgte::BiTgteBlend,
    brep_offset::{BRepOffsetTool, EnlargeFaceOptions},
    brep_offset_api::{BRepOffsetAPIMakeDraft, BRepOffsetAPIMakePipe},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tkshhealing::{
    shape_extend::ShapeExtendStatus,
    shape_fix::{ShapeFix, ShapeFixShape},
    shape_upgrade::ShapeUpgradeShapeDivideContinuity,
};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIGTransform, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeShell,
    },
    brep_mat2d::{BRepMAT2dBisectingLocus, BRepMAT2dExplorer},
    mat::MatSide,
};
use martensite_tkv3d::{prs3d::Prs3dDrawer, std_prs::StdPrsToolTriangulatedShape};
use std::collections::HashMap;
use std::f64::consts::PI;

/// A command few scripts call.
pub enum Extra {
    /// `fixshape result shape`, the command's default modes.
    FixShape(&'static str, &'static str),
    /// `mkshell name surface`.
    MkShell(&'static str, &'static str),
    /// `deform result shape cx cy cz` and `scalexyz`.
    Deform(&'static str, &'static str, [f64; 3]),
    /// `offset name surface distance`.
    OffsetSurface(&'static str, &'static str, f64),
    /// `offsetcurve name curve distance point`.
    OffsetCurve(&'static str, &'static str, f64, &'static str),
    /// `point name x y z`.
    Point(&'static str, [f64; 3]),
    /// `convert result curve`.
    ConvertCurve(&'static str, &'static str),
    /// `brollingball result shape radius @ face face ...`.
    RollingBall(&'static str, &'static str, f64, &'static [&'static str]),
    /// `draft result shape dx dy dz angle stop [-IN/-OUT]`; -1, 1 and 0 for the side.
    Draft(&'static str, &'static str, [f64; 3], f64, &'static str, i32),
    /// `mkvolume result shapes... [-ni]`: whether the arguments are intersected.
    MakerVolume(&'static str, &'static [&'static str], bool),
    /// `DT_ShapeDivide result shape tolerance`.
    ShapeDivide(&'static str, &'static str, f64),
    /// `bug30391 result face before-u after-u before-v after-v`: `en_large_face`.
    EnlargeFace(&'static str, &'static str, [f64; 4]),
    /// `bug24667 result spine profile`: the pipe, then its first and last shapes as `result_f` and `_l`.
    PipeWithEnds(&'static str, &'static str, &'static str),
    /// `coordload name file`: a closed planar polygon from a coordinate file.
    CoordLoad(&'static str, &'static str),
    /// `topoload face`, loading the medial axis explorer.
    TopoLoad(&'static str),
    /// `drawcont`, which only reads every contour's curves on a display.
    DrawContours,
    /// `side left/right`: `true` is right.
    Side(bool),
    /// `mat`: the bisecting locus of the loaded contour.
    Mat,
    /// `result` after `mat`: every arc's bisector.
    MatResult,
    /// `rotate names x y z dx dy dz degrees` on curve variables.
    CurveRotate(&'static [&'static str], [f64; 7]),
    /// `translate names dx dy dz` on curve variables.
    CurveTranslate(&'static [&'static str], [f64; 3]),
    /// `2dbeziercurve name count x y weight ...`.
    Bezier2d(&'static str, &'static [f64]),
    /// `to3d name curve [plane]`.
    To3d(&'static str, &'static str, Option<&'static str>),
    /// `appro name count curve`: a curve's points approximated.
    Appro(&'static str, i32, &'static str),
    /// `appsurf name curves...`.
    AppSurf(&'static str, &'static [&'static str]),
    /// `filling name bounds 0 points [edge face order]... [point]...`.
    Filling(Filling),
    /// `vinit`, `vdisplay a`, `vcomputehlr a result -algoType` in the default view, and `build3d`.
    Hlr(bool),
    /// `checkview -3d` meshing a shape as the viewer displays it, for `checktrinfo`.
    MeshAsViewer(&'static str),
    /// `checktrinfo name -tri count -nod count`.
    TriangleCounts(&'static str, usize, usize),
    /// `bug712 draft slab`: the slab's draft.
    SlabDraft(f64, f64),
    /// `mkedge name curve [surface] vertex vertex`.
    EdgeBetween(
        &'static str,
        &'static str,
        Option<&'static str>,
        &'static str,
        &'static str,
    ),
}

/// `filling`'s arguments.
pub struct Filling {
    pub result: &'static str,
    /// Edge, face and order of each boundary.
    pub bounds: &'static [(&'static str, &'static str, i32)],
    pub points: &'static [&'static str],
}

/// What these commands keep across steps.
#[derive(Default)]
pub struct ExtraState {
    pub points: HashMap<&'static str, Point3>,
    pub explorer: BRepMAT2dExplorer,
    pub locus: BRepMAT2dBisectingLocus,
    pub right_side: bool,
    /// `bfuzzyvalue`, which a following Boolean command reads.
    pub fuzzy: Option<f64>,
}

/// Runs one of them; `apply(script, &Extra::Mat)`.
pub fn apply(script: &mut OffsetScript<'_>, step: &Extra) -> Result<(), Failure> {
    match step {
        Extra::FixShape(result, shape) => fix_shape(script, result, shape),
        Extra::MkShell(name, surface) => make_shell(script, name, surface),
        Extra::Deform(result, shape, v) => deform(script, result, shape, *v),
        Extra::OffsetSurface(name, surface, distance) => {
            let basis = surface_of(script, surface)?;
            let built = GeomOffsetSurface::new(&basis, *distance, false)?;
            script.set_surface(name, Handle::new(built).cast());
            Ok(())
        }
        Extra::OffsetCurve(name, curve, distance, point) => {
            let basis = curve_of(script, curve)?;
            let at = script.extras.points.get(point).copied();
            let at = at.unwrap_or(Point3::new(0., 0., 0.));
            let built =
                GeomOffsetCurve::new(&basis, *distance, Direction3::from_xyz(at.xyz())?, false)?;
            script.set_curve(name, Handle::new(built).cast());
            Ok(())
        }
        Extra::Point(name, [x, y, z]) => {
            script.extras.points.insert(name, Point3::new(*x, *y, *z));
            Ok(())
        }
        Extra::ConvertCurve(result, curve) => {
            let basis = curve_of(script, curve)?;
            let converted = geom_convert::curve_to_bspline_curve(&basis, convert_kind(curve))?;
            script.set_curve(result, converted.cast());
            Ok(())
        }
        Extra::RollingBall(result, shape, radius, faces) => {
            rolling_ball(script, result, shape, *radius, faces)
        }
        Extra::Draft(result, shape, direction, angle, stop, side) => {
            draft(script, result, shape, *direction, *angle, stop, *side)
        }
        Extra::MakerVolume(result, shapes, intersect) => {
            maker_volume(script, result, shapes, *intersect)
        }
        Extra::ShapeDivide(result, shape, tolerance) => {
            shape_divide(script, result, shape, *tolerance)
        }
        Extra::EnlargeFace(result, face, lengths) => enlarge(script, result, face, *lengths),
        Extra::PipeWithEnds(result, spine, profile) => {
            pipe_with_ends(script, result, spine, profile)
        }
        Extra::CoordLoad(name, path) => extra_geometry::coord_load(script, name, path),
        Extra::TopoLoad(face) => {
            let face = topods::face(&script.required(face)?)?;
            script.extras.explorer.perform(&face)
        }
        Extra::DrawContours => extra_geometry::read_contours(script),
        Extra::Side(right) => {
            script.extras.right_side = *right;
            Ok(())
        }
        Extra::Mat => {
            let extras = &mut script.extras;
            let side = if extras.right_side {
                MatSide::Right
            } else {
                MatSide::Left
            };
            extras
                .locus
                .compute(&mut extras.explorer, 1, side, JoinType::Arc, false)
        }
        Extra::MatResult => extra_geometry::bisectors(script),
        Extra::CurveRotate(names, v) => {
            let mut trsf = Transform3::default();
            let direction = Direction3::from_vector(Vector3::new(v[3], v[4], v[5]))?;
            trsf.set_rotation(
                Axis1::new(Point3::new(v[0], v[1], v[2]), direction),
                v[6] * (PI / 180.),
            )?;
            move_curves(script, names, trsf)
        }
        Extra::CurveTranslate(names, v) => {
            let mut trsf = Transform3::default();
            trsf.set_translation(Vector3::new(v[0], v[1], v[2]));
            move_curves(script, names, trsf)
        }
        Extra::Bezier2d(name, v) => {
            let poles: Vec<_> = v.chunks_exact(3).map(|p| Point2::new(p[0], p[1])).collect();
            let weights: Vec<f64> = v.chunks_exact(3).map(|p| p[2]).collect();
            let curve = Geom2dBezierCurve::with_weights(
                &Array1::from_shared(&poles, 1),
                &Array1::from_shared(&weights, 1),
            )?;
            script.set_curve_2d(name, Handle::new(curve).cast());
            Ok(())
        }
        Extra::To3d(name, curve, plane) => extra_geometry::to_3d(script, name, curve, *plane),
        Extra::Appro(name, count, curve) => extra_geometry::appro(script, name, *count, curve),
        Extra::AppSurf(name, curves) => extra_geometry::app_surf(script, name, curves),
        Extra::Filling(filling) => extra_geometry::fill(script, filling),
        Extra::Hlr(poly) => extra_geometry::hidden_lines(script, *poly),
        Extra::MeshAsViewer(name) => {
            let shape = script.required(name)?;
            StdPrsToolTriangulatedShape::tessellate(&shape, &Handle::new(Prs3dDrawer::default()))?;
            Ok(())
        }
        Extra::TriangleCounts(name, triangles, nodes) => {
            extra_geometry::triangle_counts(script, name, *triangles, *nodes)
        }
        Extra::SlabDraft(angle, thickness) => extra_geometry::slab_draft(*angle, *thickness),
        Extra::EdgeBetween(name, curve, surface, first, last) => {
            edge_between(script, name, curve, *surface, first, last)
        }
    }
}

pub(crate) fn curve_of(
    script: &OffsetScript<'_>,
    name: &str,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    script
        .curve(name)
        .ok_or_else(|| command_stopped(&format!("{name} is not a defined curve")))
}

pub(crate) fn surface_of(
    script: &OffsetScript<'_>,
    name: &str,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    script
        .surface(name)
        .ok_or_else(|| command_stopped(&format!("{name} is not a surface")))
}

fn move_curves(
    script: &mut OffsetScript<'_>,
    names: &[&str],
    trsf: Transform3,
) -> Result<(), Failure> {
    for name in names {
        let moved = curve_of(script, name)?.transformed(trsf)?;
        script.set_curve(name, moved.cast());
        println!("{name}");
    }
    Ok(())
}

fn fix_shape(script: &mut OffsetScript<'_>, result: &str, shape: &str) -> Result<(), Failure> {
    let input = script.required(shape)?;
    let tool = ShapeFixShape::default();
    tool.init(input);
    tool.perform_with_progress(&ProgressRange::default())?;
    script.set_shape(result, tool.shape());
    Ok(())
}

fn make_shell(script: &mut OffsetScript<'_>, name: &str, surface: &str) -> Result<(), Failure> {
    let surface = surface_of(script, surface)?;
    let maker = BRepBuilderAPIMakeShell::with_surface(&surface, false)?;
    script.set_shape(name, maker.shell()?.as_ref().clone());
    Ok(())
}

/// The command reads its last word as a parameterisation before it reads it as a name, so
/// `convert c1 c1` is `RationalC1`, not the default `TgtThetaOver2`.
fn convert_kind(last_word: &str) -> martensite_tkmath::convert::ParameterisationType {
    use martensite_tkmath::convert::ParameterisationType as Kind;
    match last_word {
        "qa" => Kind::QuasiAngular,
        "c1" => Kind::RationalC1,
        "s1" => Kind::TgtThetaOver2_1,
        "s2" => Kind::TgtThetaOver2_2,
        "s3" => Kind::TgtThetaOver2_3,
        "s4" => Kind::TgtThetaOver2_4,
        "po" => Kind::Polynomial,
        _ => Kind::TgtThetaOver2,
    }
}

/// `deform`: an axis scaling by the general transformation builder.
fn deform(
    script: &mut OffsetScript<'_>,
    result: &str,
    shape: &str,
    [cx, cy, cz]: [f64; 3],
) -> Result<(), Failure> {
    let base = script.required(shape)?;
    let mut gtrsf = GeneralTransform3::default();
    gtrsf.set_vectorial_part(Matrix3::new([[cx, 0., 0.], [0., cy, 0.], [0., 0., cz]]));
    let mut maker = BRepBuilderAPIGTransform::with_shape(&base, gtrsf, false)?;
    let built = maker.shape()?.clone();
    script.set_shape(result, built);
    Ok(())
}

fn rolling_ball(
    script: &mut OffsetScript<'_>,
    result: &str,
    shape: &str,
    radius: f64,
    faces: &[&str],
) -> Result<(), Failure> {
    let input = script.required(shape)?;
    let tolerance = script.fillets.tolerance_3d();
    let mut roll = BiTgteBlend::new();
    roll.init(&input, radius, tolerance, false);
    for pair in faces.chunks_exact(2) {
        let first = topods::face(&script.required(pair[0])?)?;
        let second = topods::face(&script.required(pair[1])?)?;
        roll.set_faces(&first, &second);
    }
    roll.perform(true)?;
    if !roll.is_done() {
        return Err(command_stopped("brollingball not done"));
    }
    let built = roll.shape().clone();
    script.set_shape(result, built);
    Ok(())
}

fn draft(
    script: &mut OffsetScript<'_>,
    result: &str,
    shape: &str,
    [dx, dy, dz]: [f64; 3],
    angle: f64,
    stop: &str,
    side: i32,
) -> Result<(), Failure> {
    let input = script.required(shape)?;
    let direction = Direction3::from_vector(Vector3::new(dx, dy, dz))?;
    let mut maker = BRepOffsetAPIMakeDraft::new(&input, direction, angle)?;
    println!("External Draft : ");
    let stop_shape = script.shape(stop);
    if !stop_shape.is_null() {
        maker.perform_shape(&stop_shape, side != 0)?;
    } else if let Some(surface) = script.surface(stop) {
        maker.perform_surface(&surface, side != 1)?;
    } else {
        let length: f64 = stop.parse().unwrap_or(0.);
        if length <= 1.0e-7 {
            return Err(command_stopped("draft: nothing to stop on"));
        }
        maker.perform_length(length)?;
    }
    script.set_shape(result, maker.shape()?.clone());
    Ok(())
}

fn maker_volume(
    script: &mut OffsetScript<'_>,
    result: &str,
    shapes: &[&str],
    intersect: bool,
) -> Result<(), Failure> {
    let mut arguments = List::default();
    for name in shapes {
        arguments.append(script.required(name)?);
    }
    let mut volume = BOPAlgoMakerVolume::new();
    volume.set_arguments(&arguments);
    volume.set_intersect(intersect);
    volume.perform();
    script.set_shape(result, volume.shape().clone());
    Ok(())
}

/// `DT_ShapeDivide`: the division, then the command's same-parameter fix of what it made.
fn shape_divide(
    script: &mut OffsetScript<'_>,
    result: &str,
    shape: &str,
    tolerance: f64,
) -> Result<(), Failure> {
    let tool = ShapeUpgradeShapeDivideContinuity::new(script.required(shape)?);
    tool.set_tolerance(tolerance);
    tool.perform(true)?;
    let divided = tool.result();
    for (flag, name) in [
        (ShapeExtendStatus::Done1, "DONE1"),
        (ShapeExtendStatus::Done2, "DONE2"),
        (ShapeExtendStatus::Done3, "DONE3"),
        (ShapeExtendStatus::Done4, "DONE4"),
        (ShapeExtendStatus::Done5, "DONE5"),
        (ShapeExtendStatus::Done6, "DONE6"),
        (ShapeExtendStatus::Done7, "DONE7"),
        (ShapeExtendStatus::Done8, "DONE8"),
        (ShapeExtendStatus::Fail1, "FAIL1"),
        (ShapeExtendStatus::Fail2, "FAIL2"),
        (ShapeExtendStatus::Fail3, "FAIL3"),
        (ShapeExtendStatus::Fail4, "FAIL4"),
        (ShapeExtendStatus::Fail5, "FAIL5"),
        (ShapeExtendStatus::Fail6, "FAIL6"),
        (ShapeExtendStatus::Fail7, "FAIL7"),
        (ShapeExtendStatus::Fail8, "FAIL8"),
    ] {
        if tool.status_flag(flag) {
            println!("Status: {name}");
        }
    }
    ShapeFix::same_parameter(
        &divided,
        false,
        0.0,
        &ProgressRange::default(),
        &Handle::default(),
    )?;
    script.set_shape(result, divided);
    Ok(())
}

fn enlarge(
    script: &mut OffsetScript<'_>,
    result: &str,
    face: &str,
    lengths: [f64; 4],
) -> Result<(), Failure> {
    let face = topods::face(&script.required(face)?)?;
    let mut enlarged = Default::default();
    BRepOffsetTool::en_large_face(
        &face,
        &mut enlarged,
        EnlargeFaceOptions {
            can_extent_surface: true,
            update_pcurve: true,
            enlarge_u: true,
            enlarge_vfirst: true,
            enlarge_vlast: true,
            extension_mode: 1,
            lengths,
        },
    )?;
    let built: martensite_tkbrep::topods::Face = enlarged;
    script.set_shape(result, built.as_ref().clone());
    Ok(())
}

fn pipe_with_ends(
    script: &mut OffsetScript<'_>,
    result: &str,
    spine: &str,
    profile: &str,
) -> Result<(), Failure> {
    let spine = topods::wire(&script.required(spine)?)?;
    let profile = script.required(profile)?;
    let trihedron = martensite_tkgeomalgo::geom_fill::GeomFillTrihedron::IsCorrectedFrenet;
    let mut pipe = BRepOffsetAPIMakePipe::with_mode(&spine, &profile, trihedron, false)?;
    let shape = pipe.shape()?.clone();
    let (first, last) = (
        BRepPrimAPIMakeSweep::first_shape(&mut pipe)?,
        BRepPrimAPIMakeSweep::last_shape(&mut pipe)?,
    );
    script.set_shape(result, shape);
    script.set_shape(&format!("{result}_f"), first);
    script.set_shape(&format!("{result}_l"), last);
    Ok(())
}

fn edge_between(
    script: &mut OffsetScript<'_>,
    name: &str,
    curve: &str,
    surface: Option<&str>,
    first: &str,
    last: &str,
) -> Result<(), Failure> {
    let first = topods::vertex(&script.required(first)?)?;
    let last = topods::vertex(&script.required(last)?)?;
    let edge = match surface {
        Some(surface) => {
            let curve_2d = script.curve_2d(curve)?;
            let surface = surface_of(script, surface)?;
            BRepBuilderAPIMakeEdge::from_pcurve_vertices(&curve_2d, &surface, &first, &last)?
                .edge()?
        }
        None => {
            let curve = curve_of(script, curve)?;
            BRepBuilderAPIMakeEdge::from_curve_vertices(&curve, &first, &last)?.edge()?
        }
    };
    script.set_shape(name, edge.as_ref().clone());
    Ok(())
}
