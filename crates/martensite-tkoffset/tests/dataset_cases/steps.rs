//! The typed steps a case runs, one per source script command.
//!
//! Tables hold no script text and nothing evaluates expressions at run time: the generator resolves
//! the scripts' arithmetic, variables and data file names when it writes them.
use crate::conditions::{Cond, Value};
use crate::groups::{End, Group};
use crate::support::{properties::Prop, session::ViewStep};

/// One case: its source script, what it inherits, and its commands.
pub struct OffsetCase {
    pub source: &'static str,
    pub group: Group,
    /// The budget the script chain sets last, in seconds.
    pub cpu_seconds: i64,
    /// The end script the chain runs after the body.
    pub end: End,
    /// The `TODO` that the reference run does not meet either, if there is one: the case must end
    /// as that same improvement.
    pub reference_improvement: Option<&'static str>,
    /// The reference run has no problem at all, so a declared problem must not appear either.
    pub reference_clean: bool,
    pub steps: &'static [Step],
}

/// A source script command, grouped by what it builds.
pub enum Step {
    Prim(Prim),
    Place(Place),
    Curve(Curve),
    Profile(Profile),
    Sweep(Sweep),
    Topo(Topo),
    Bool(Bool),
    Loft(Loft),
    Shell(PipeShell),
    Offset(Offset),
    Draft(Draft),
    Fillet(Fillet),
    More(More),
    Check(Check),
    /// A `puts` of the script: the parse rules read it like any other line.
    Puts(&'static str),
    /// `if {cond} {then} else {otherwise}` whose condition only the run can decide.
    If(Cond, &'static [Step], &'static [Step]),
    /// `set name [command ...]`: a value a later condition reads.
    Let(&'static str, Value),
    /// `catch {step}`: a failure of the step does not stop the script.
    Catch(&'static Step),
    /// `smallview`, `clear`, `donly`, `fit` and `xwd`.
    View(ViewStep<'static>),
    /// `checkview -display name -2d`; `-3d` in a script goes to the same memory raster.
    CheckView(&'static str),
    /// `checkview -display name -2d -otherwise {others} -s|-l|-v`.
    CheckViewOtherwise(&'static str, &'static [&'static str], Prop),
    /// `checkview -screenshot -2d -path image`.
    Screenshot(&'static str),
    /// Several commands where a form of the script takes one, as in `catch {a; b}`.
    Block(&'static [Step]),
    /// `catch {step} name`: the failure's text kept in the variable.
    CatchKept(&'static str, &'static Step),
    /// A command few scripts call.
    Extra(crate::extra_steps::Extra),
    /// One script's own block.
    Run(
        for<'a, 's> fn(
            &'a mut crate::script::OffsetScript<'s>,
        ) -> Result<(), martensite_tkernel::standard::Failure>,
    ),
    /// A command returned 1, or a script error: stops the script.
    Stop(&'static str),
}

/// `thrusections`: its sections are named variables, wires or vertices.
pub struct Loft {
    pub result: &'static str,
    pub solid: bool,
    pub ruled: bool,
    /// `-N` is absent: the sections' compatibility is checked.
    pub check: bool,
    /// `-safe` is absent: the input is mutable.
    pub mutable_input: bool,
    pub sections: &'static [&'static str],
}

/// `mksweep`, `setsweep`, `addsweep`, `deletesweep` and `buildsweep`.
pub enum PipeShell {
    /// `mksweep wire`.
    Start(&'static str),
    /// `mksweep wire -C angle_degrees linear`.
    StartCorrected(&'static str, f64, f64),
    Frenet,
    CorrectedFrenet,
    Discrete,
    /// `setsweep -DX shape`.
    Support(&'static str),
    /// `setsweep -CN dx dy dz`.
    BiNormal([f64; 3]),
    /// `setsweep -FX tx ty tz [nx ny nz]`.
    Fixed(&'static [f64]),
    /// `setsweep -G guide curvilinear contact`.
    Guide(&'static str, bool, i32),
    /// `setsweep -DM value`.
    MaxDegree(i32),
    /// `setsweep -SM value`.
    MaxSegments(i32),
    Add(SweepSection),
    Delete(&'static str),
    Build(SweepBuild),
}

/// `addsweep wire/vertex [Vertex] [-T] [-R] [u0 v0 u1 v1 ...]`.
pub struct SweepSection {
    pub section: &'static str,
    pub vertex: Option<&'static str>,
    pub translate: bool,
    pub rotate: bool,
    /// The interpolated law's `u v` pairs, flattened; empty for none.
    pub law: &'static [f64],
}

/// `buildsweep result [-M/-C/-R] [-S]`.
pub struct SweepBuild {
    pub result: &'static str,
    /// `'M'`, `'C'`, `'R'`, or `'-'` when the command gives no option and keeps the builder's mode.
    pub mode: char,
    pub solid: bool,
}

/// The offset commands.
pub enum Offset {
    /// `offsetparameter tolerance c|p a|i|t [r|k]`.
    Parameter(f64, bool, char, bool),
    /// `offsetload shape value faces...`, with the number of words after the shape's value.
    Load(&'static str, f64, &'static [&'static str]),
    /// `offsetonface face value ...`.
    OnFace(&'static [(&'static str, f64)]),
    /// `offsetperform result`.
    Perform(&'static str),
    /// `offsetshape` or `offsetcompshape result shape value [tol] [faces...]`.
    Shape(OffsetShape),
    /// `thickshell result shape value [join [tol]]`.
    Thick(&'static str, &'static str, f64, char, f64),
    /// `mkoffset result base count step [-approx] [join [alt]]`.
    Planar(PlanarOffset),
    /// `offsetshapesimple result shape value [tol]`.
    Simple(&'static str, &'static str, f64, f64),
}

/// `offsetshape` and `offsetcompshape`.
pub struct OffsetShape {
    pub result: &'static str,
    pub shape: &'static str,
    pub value: f64,
    pub tolerance: Option<f64>,
    pub complete: bool,
    pub faces: &'static [&'static str],
}

/// `mkoffset` and `openoffset`.
pub struct PlanarOffset {
    pub result: &'static str,
    pub base: &'static str,
    pub count: i32,
    pub step: f64,
    pub approx: bool,
    /// `'a'` or `'i'`.
    pub join: char,
    pub altitude: f64,
    pub open: bool,
}

/// `depouille result shape dx dy dz face angle x y z nx ny nz [...]`.
pub struct Draft {
    pub result: &'static str,
    pub shape: &'static str,
    pub direction: [f64; 3],
    pub faces: &'static [(&'static str, f64, [f64; 6])],
}

/// The checks the scripts call.
pub enum Check {
    /// `checkshape name`.
    Shape(&'static str),
    /// `puts [checkshape name]`.
    ShapePrinted(&'static str),
    /// `checknbshapes name -kind count ...`.
    Counts(&'static str, &'static [(&'static str, usize)]),
    /// `checknbshapes name -t -kind count ...`: each type mapped on its own.
    TypedCounts(&'static str, &'static [(&'static str, usize)]),
    /// `checknbshapes name -t -ref {counts} -m message`.
    TypedCountsNamed(&'static str, &'static [(&'static str, usize)], &'static str),
    /// `checknbshapes name -ref {counts of other}`.
    CountsLike(&'static str, &'static str),
    /// `checkprops name -l|-s|-v value [-deps relative] [-eps epsilon]`.
    Props(Prop, &'static str, f64, f64, Option<f64>),
    /// `checkprops` whose report quotes the script's word, which the number does not bring back.
    PropsAs(Prop, &'static str, f64, &'static str, f64, Option<f64>),
    /// `nbshapes name`.
    NbShapes(&'static str),
    /// `checknbshapes name -kind count ... -m message`.
    CountsNamed(&'static str, &'static [(&'static str, usize)], &'static str),
    /// `checkprops name -l|-s|-v empty`.
    PropsEmpty(Prop, &'static str),
    /// `checkprops name -equal other`.
    PropsEqual(&'static str, &'static str),
    /// `checkmaxtol name [-ref value] [-min_tol value]`.
    MaxTolerance(&'static str, Option<&'static str>, Option<f64>),
    /// `checksection name`.
    Section(&'static str),
    /// `set index [lsearch [whatis name] Closed]; if {$index == -1} {puts message}`.
    ClosedElse(&'static str, &'static str),
}

/// `box`, `wedge`, `pcylinder`, `pcone`, `psphere`, `ptorus`, with their numeric arguments.
pub enum Prim {
    Box(&'static str, &'static [f64]),
    Wedge(&'static str, &'static [f64]),
    Cylinder(&'static str, &'static [f64]),
    /// `pcylinder|pcone|psphere|ptorus name plane values...`, the plane a variable.
    OnPlane(PlaneKind, &'static str, &'static str, &'static [f64]),
    Cone(&'static str, &'static [f64]),
    Sphere(&'static str, &'static [f64]),
    Torus(&'static str, &'static [f64]),
}

/// What a primitive command builds on a plane.
#[derive(Clone, Copy)]
pub enum PlaneKind {
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

/// `ttranslate`, `trotate` and `tscale` of the named shapes.
pub enum Place {
    Translate(&'static [&'static str], [f64; 3]),
    Rotate(&'static [&'static str], [f64; 7]),
    Scale(&'static [&'static str], [f64; 4]),
}

/// `line`, `circle`, `ellipse`, `parabola`, `hyperbola`, `beziercurve` and `bsplinecurve`.
pub enum Curve {
    Line(&'static str, &'static [f64]),
    Circle(&'static str, &'static [f64]),
    Ellipse(&'static str, &'static [f64]),
    Parabola(&'static str, &'static [f64]),
    Hyperbola(&'static str, &'static [f64]),
    Bezier(&'static str, &'static [f64]),
    BSpline(&'static str, &'static [f64]),
    /// `pbsplinecurve`, the periodic form of `bsplinecurve`.
    PeriodicBSpline(&'static str, &'static [f64]),
}

/// `polyline`, `mkedge`, `wire`, `mkplane`, `vertex` and `edge`.
pub enum Profile {
    Vertex(&'static str, [f64; 3]),
    EdgeOfVertices(&'static str, &'static str, &'static str),
    Polyline(&'static str, &'static [f64]),
    Edge(&'static str, &'static str, Option<[f64; 2]>),
    Wire(&'static str, &'static [&'static str]),
    /// `mkplane name wire [1]`.
    Plane(&'static str, &'static str, bool),
}

/// `prism`, `revol` and `pipe`.
pub enum Sweep {
    /// `prism name base dx dy dz`, and whether `inf` or `SemiInf` is given is refused by the
    /// generator.
    Prism(&'static str, &'static str, [f64; 3]),
    Revol(&'static str, &'static str, [f64; 7]),
    /// `pipe result spine profile [mode [approx]]`.
    Pipe(&'static str, &'static str, &'static str, i32, bool),
}

/// The script topology commands the grids use.
pub enum Topo {
    /// `restore`: a file of the test dataset and the variable it fills.
    Restore(&'static str, &'static str),
    Explode(&'static str, &'static str),
    ExplodeChildren(&'static str),
    NumberedExplode(&'static str, &'static str),
    /// `tcopy from to`.
    GeometryCopy(&'static str, &'static str),
    /// `copy from to`: a new variable on the same shape.
    SharedCopy(&'static str, &'static str),
    Rename(&'static str, &'static str),
    OrientReversed(&'static str),
    Reverse(&'static str),
    /// `invert name`: the sub-shapes reversed in a copy.
    Invert(&'static str),
    NurbsConvert(&'static str, &'static str),
    /// `settolerance name value`.
    ForceTolerance(&'static str, f64),
    EmptySolid(&'static str),
    Add(&'static str, &'static str),
    SameParameter(&'static str, Option<f64>),
    EncodeRegularity(&'static str, Option<f64>),
    /// `setflags name flag...`: a flag name with its value.
    SetFlags(&'static str, &'static [(&'static str, bool)]),
    /// `orientation name F|R|E|I`.
    Orient(&'static str, char),
    /// `unifysamedom result shape`.
    UnifySameDomain(&'static str, &'static str),
    /// `fsameparameter shape [tol]`.
    ForceSameParameter(&'static str, Option<f64>),
}

/// `bfuse`, `bcut`, `bcommon` and `bsection`.
pub enum Bool {
    Fuse(&'static str, &'static str, &'static str),
    Cut(&'static str, &'static str, &'static str),
    Common(&'static str, &'static str, &'static str),
    Section(&'static str, &'static str, &'static str),
    /// `bcutblend result a b radius`.
    CutBlend(&'static str, &'static str, &'static str, f64),
    /// `bfuseblend result a b radius`.
    FuseBlend(&'static str, &'static str, &'static str, f64),
    /// `bfuzzyvalue value`, read by the following Boolean command.
    Fuzzy(f64),
    /// `bop a b`.
    Prepare(&'static str, &'static str),
    /// `bopcut result` and the others.
    TwoStep(&'static str, BopKind),
}

/// The operations of the two-step commands.
#[derive(Clone, Copy)]
pub enum BopKind {
    Common,
    Fuse,
    Cut,
    Tuc,
    Section,
}

/// The commands few scripts use, in `more_steps`.
pub enum More {
    Surface(Surface),
    /// `mkface name surface [u1 u2 v1 v2]`.
    Face(&'static str, &'static str, Option<[f64; 4]>),
    /// `mkface name surface wire [norient]`.
    FaceOnWire(&'static str, &'static str, &'static str, bool),
    /// `profile name words...`, its numbers resolved.
    Moves(&'static str, &'static [&'static str]),
    /// `middlepath result shape start end`.
    Middle(&'static str, &'static str, &'static str, &'static str),
    Evolved(Evolved),
    Project(Project),
    /// `trim name curve u1 u2`.
    TrimCurve(&'static str, &'static str, [f64; 2]),
    /// `tmirror names... x y z nx ny nz`.
    Mirror(&'static [&'static str], [f64; 6]),
    /// `scaleshape result shape factor`.
    Scale(&'static str, &'static str, f64),
    /// `sewing result [tol] shapes...`, with the command's default modes.
    Sewing(&'static str, f64, &'static [&'static str]),
    /// `mksurface name face`.
    SurfaceOfFace(&'static str, &'static str),
    /// `featlf base wire plane dx dy dz fx fy fz fuse modify`.
    LinearForm(LinearForm),
    /// `featperform lf result`.
    PerformLinearForm(&'static str),
    /// `perform_offset_multi_with_ref`, the procedure of `shape_type_i_c_multi`.
    MultiOffset(MultiOffset),
    /// `compound parts... name`.
    Compound(&'static [&'static str], &'static str),
    /// `shape name kind`.
    EmptyShape(&'static str, &'static str),
    History(HistoryStep),
    General(General),
}

/// `plane`, `cylinder`, `sphere`, `cone`, `torus`, `beziersurf` and `bsplinesurf`.
pub enum Surface {
    Plane(&'static str, &'static [f64]),
    Cylinder(&'static str, &'static [f64], f64),
    Sphere(&'static str, &'static [f64], f64),
    /// The angle in degrees, then the radius.
    Cone(&'static str, &'static [f64], [f64; 2]),
    Torus(&'static str, &'static [f64], [f64; 2]),
    Bezier(&'static str, &'static [f64]),
    BSpline(&'static str, &'static [f64]),
}

impl Surface {
    /// The variable it fills; `Surface::plane("p", &[]).name()`.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Plane(name, _)
            | Self::Cylinder(name, _, _)
            | Self::Sphere(name, _, _)
            | Self::Cone(name, _, _)
            | Self::Torus(name, _, _)
            | Self::Bezier(name, _)
            | Self::BSpline(name, _) => name,
        }
    }
}

/// `evolved result -s spine -p profile ...`.
pub struct Evolved {
    pub result: &'static str,
    pub spine: &'static str,
    pub profile: &'static str,
    pub solid: bool,
    pub compute_axes: bool,
    pub volume: bool,
    pub tolerance: f64,
    pub parallel: bool,
}

/// `nproject result shapes... surface ...`.
pub struct Project {
    pub result: &'static str,
    pub shapes: &'static [&'static str],
    pub surface: &'static str,
    pub unlimited: bool,
    pub max_distance: Option<f64>,
    pub tolerance: f64,
    pub continuity: i32,
    pub max_degree: i32,
    pub max_segments: i32,
}

/// `savehistory`, `generated` and `modified`.
pub enum HistoryStep {
    Save(&'static str),
    Generated(&'static str, &'static str, &'static str),
    Modified(&'static str, &'static str, &'static str),
}

/// The general-fuse sequence: `bclearobjects`, `bcleartools`, `baddobjects`, `baddtools`, `bfillds`, `bbuild`.
pub enum General {
    ClearObjects,
    ClearTools,
    AddObjects(&'static [&'static str]),
    AddTools(&'static [&'static str]),
    Fill,
    Build(&'static str),
    /// `bbop result operation`: 0 common, 1 fuse, 2 cut, 3 cut21, 4 section.
    Operation(&'static str, i32),
}

/// `perform_offset_multi_with_ref result shape value faces values refs unified`.
pub struct MultiOffset {
    pub result: &'static str,
    pub shape: &'static str,
    pub value: f64,
    pub faces: &'static [i32],
    pub face_values: &'static [f64],
    /// Area, volume, wires and faces; empty when the script gives none.
    pub refs: &'static [f64],
    pub unified: bool,
}

/// `featlf`'s words.
pub struct LinearForm {
    pub base: &'static str,
    pub wire: &'static str,
    pub plane: &'static str,
    pub direct: [f64; 3],
    pub other: [f64; 3],
    pub fuse: i32,
    pub modify: bool,
}

/// `tolblend`, `blend`, `fillet`, `mkevol`, `updatevol` and `buildevol`.
pub enum Fillet {
    /// `tolblend ang 3d 2d fleche`.
    Tolerances([f64; 4]),
    /// `blend result shape radius edge ...`: edges a script does not define are skipped.
    Blend(&'static str, &'static str, &'static [(f64, &'static str)]),
    /// `fillet result shape rad name ...`: the other command file's blend, which writes nothing,
    /// keeps its own tolerances and takes every edge a name holds.
    Plain(&'static str, &'static str, &'static [(f64, &'static str)]),
    /// `blend result shape radius edges`, the edges being all those `generated` from `source`'s faces.
    BlendGeneratedEdges(&'static str, &'static str, &'static str, f64),
    MakeEvolved(&'static str, &'static str),
    /// `updatevol edge par rad ...`.
    UpdateEvolved(&'static str, &'static [[f64; 2]]),
    BuildEvolved,
}
