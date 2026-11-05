//! The typed steps a case runs, one per source script command.
//!
//! Tables hold no script text and nothing evaluates expressions at run time: the
//! generator resolves `SCALE1*0.5`, `pi/2` and data file names when it writes them.
use crate::groups::FilletGroup;
use crate::support::{checkview::CheckView, properties::Prop, session::ViewStep};

/// One case: its source script, the group it inherits from and its commands.
pub struct BlendCase {
    pub source: &'static str,
    pub group: FilletGroup,
    /// The `TODO` that native does not detect either on this platform, if there is one: the case
    /// must end as that same improvement.
    pub native_improvement: Option<&'static str>,
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
    Fillet(Fillet),
    Chamfer(Chamfer),
    Planar(Planar),
    History(HistoryQuery),
    Check(Check),
    /// A `puts` of the script: the parse rules read it like any other line.
    Puts(&'static str),
    /// `if [catch {step}] {puts failed} else {puts done}`; an empty text prints nothing.
    Caught(&'static Step, &'static str, &'static str),
    /// `if [catch {step}] {puts failed} else {steps}`.
    IfDone(&'static Step, &'static str, &'static [Step]),
    /// `smallview`, `clear`, `donly`, `fit` and `xwd`.
    View(ViewStep<'static>),
    /// `checkview -display name -2d`; `-3d` in a script goes to the same memory raster.
    CheckView(CheckView<'static>),
    /// `checkview -screenshot -2d -path image`.
    Screenshot(&'static str),
    /// The commands of the sample scripts, in `sample_steps`.
    Sample(Sample),
    /// A command of the 3D viewer; the cases that use one run in `fillet_view_cases`.
    V3d(V3d),
}

/// `lintan`, `trim`, `to3d`, `reverse`, `cvalue`, `reset`, `tmirror`, `unifysamedom`, `bcommon`, `incmesh`,
/// `checktrinfo` and the QA command `BUC60811`.
pub enum Sample {
    /// `circle name x y r`, a 2D circle.
    Circle2d(&'static str, [f64; 3]),
    /// `lintan name c1 c2`: the 2D lines `name_1`, `name_2`, ...
    TangentLines(&'static str, &'static str, &'static str),
    /// `trim name base u1 u2`, of a 3D or 2D curve.
    Trim(&'static str, &'static str, [f64; 2]),
    /// `to3d name curve2d plane`, the plane given by `x y z [nx ny nz [xx xy xz]]`.
    To3d(&'static str, &'static str, &'static [f64]),
    /// `reverse name` of a curve.
    ReverseCurve(&'static str),
    /// `cvalue curve u x y z` and `vertex name x y z`.
    VertexOnCurve(&'static str, &'static str, f64),
    /// `reset name`.
    ResetLocation(&'static str),
    /// `tmirror name x y z nx ny nz`.
    Mirror(&'static str, [f64; 6]),
    /// `unifysamedom result shape`.
    UnifySameDomain(&'static str, &'static str),
    /// `bcommon result a b`.
    Common(&'static str, &'static str, &'static str),
    /// `incmesh name deflection`.
    Mesh(&'static str, f64),
    /// `checktrinfo name -defl value -tol_abs_defl abs -tol_rel_defl rel`.
    CheckDeflection(&'static str, f64, f64, f64),
    /// `BUC60811`: the QA command's whole body.
    Buc60811,
}

/// `chamf`, `chamf_throat` and `chamf_throat_with_penetration`.
pub enum Chamfer {
    Mixed(&'static str, &'static str, &'static [ChamferEdge]),
    Throat(&'static str, &'static str, &'static [(&'static str, f64)]),
    Penetration(
        &'static str,
        &'static str,
        &'static [(&'static str, &'static str, f64, f64)],
    ),
}

/// One edge of `chamf`; the grids only use the distance and angle form.
pub enum ChamferEdge {
    DistanceAngle(&'static str, &'static str, f64, f64),
}

/// `fillet2d` and `chamfer2d`, of two edges or of a wire.
pub enum Planar {
    FilletEdges(&'static str, &'static str, &'static str, f64),
    FilletWire(&'static str, &'static str, f64),
    ChamferEdges(&'static str, &'static str, &'static str, [f64; 2]),
    /// `chfi2d result face e1 e2 F radius`.
    FaceFillet(&'static str, &'static str, &'static str, &'static str, f64),
}

/// `savehistory`, `generated` and `modified`.
pub enum HistoryQuery {
    Save,
    Generated(&'static str, &'static str),
    Modified(&'static str, &'static str),
}

/// The checks the scripts call.
pub enum Check {
    /// `checkshape name`.
    Shape(&'static str),
    /// `checknbshapes name -kind count ...`.
    Counts(&'static str, &'static [(&'static str, usize)]),
    /// `checknbshapes name -t -ref {...}`: each type mapped on its own, the `-t` of `nbshapes`.
    TypedCounts(&'static str, &'static [(&'static str, usize)]),
    /// `checknbshapes name ... -t -m message`.
    TypedCountsNamed(&'static str, &'static [(&'static str, usize)], &'static str),
    /// `set t [checkmaxtol name]; if { $t > limit } { puts message }`.
    MaxTolerance(&'static str, f64, &'static str),
    /// `checkmaxtol name -ref value`.
    MaxToleranceRef(&'static str, &'static str),
    /// `checkmaxtol name -min_tol value`.
    MaxToleranceMin(&'static str, f64, &'static str),
    /// `if { [tolerance name] > limit } { puts over } else { puts under }`.
    ToleranceBranch(&'static str, f64, &'static str, &'static str),
    /// `checkprops name -l|-s|-v value [-deps relative]`.
    Props(Prop, &'static str, f64, f64),
    /// `if { [checkshape name] is not valid } { puts message }`.
    ValidElse(&'static str, &'static str),
    /// `if { [bopcheck name] is not OK } { puts message }`.
    BopcheckElse(&'static str, &'static str),
    /// `if { [bopargcheck name] has faulties } { puts message }`.
    ArgcheckElse(&'static str, &'static str),
    /// `if { [whatis name] is no shape } { puts message }`.
    BoundElse(&'static str, &'static str),
    /// `if { [info exist name] == 0 } { puts missing } else { puts present }`.
    BoundBranch(&'static str, &'static str, &'static str),
    /// `checksection name`.
    Section(&'static str),
    /// `if` the V multiplicities of the first B-spline surface of the edge are these, `puts`.
    VKnotMultiplicities(&'static str, &'static [(i32, i32)], &'static str),
}

/// `box`, `wedge`, `pcylinder`, `pcone`, `psphere`, `ptorus`, with their numeric arguments.
pub enum Prim {
    Box(&'static str, &'static [f64]),
    Wedge(&'static str, &'static [f64]),
    Cylinder(&'static str, &'static [f64]),
    /// `pcylinder name plane R H`, the plane given by `x y z nx ny nz [xx xy xz]`.
    CylinderOnPlane(&'static str, &'static [f64], [f64; 2]),
    Cone(&'static str, &'static [f64]),
    Sphere(&'static str, &'static [f64]),
    Torus(&'static str, &'static [f64]),
}

/// `ttranslate`, `trotate` and `tscale` of the named shapes.
pub enum Place {
    Translate(&'static [&'static str], [f64; 3]),
    Rotate(&'static [&'static str], [f64; 7]),
    Scale(&'static [&'static str], [f64; 4]),
}

/// `circle`, `ellipse`, `parabola`, `beziercurve` and `bsplinecurve`.
pub enum Curve {
    Line(&'static str, &'static [f64]),
    Circle(&'static str, &'static [f64]),
    Ellipse(&'static str, &'static [f64]),
    Parabola(&'static str, &'static [f64]),
    Bezier(&'static str, &'static [f64]),
    BSpline(&'static str, &'static [f64]),
}

/// `polyline`, `mkedge`, `wire`, `mkplane`, `vertex` and `edge`.
pub enum Profile {
    Vertex(&'static str, [f64; 3]),
    EdgeOfVertices(&'static str, &'static str, &'static str),
    Polyline(&'static str, &'static [f64]),
    Edge(&'static str, &'static str, Option<[f64; 2]>),
    Wire(&'static str, &'static [&'static str]),
    Plane(&'static str, &'static str),
}

/// `prism`, `revol` and `pipe`.
pub enum Sweep {
    Prism(&'static str, &'static str, [f64; 3]),
    Revol(&'static str, &'static str, [f64; 7]),
    Pipe(&'static str, &'static str, &'static str),
}

/// The command-host topology commands the grids use.
pub enum Topo {
    /// `restore`: a path under the test dataset and the variable it fills.
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
}

/// `bfuse`, `bcut`, `bfuseblend` and `bcutblend`.
pub enum Bool {
    /// `bop a b`: the pave filler the next `bopfuse` uses.
    PrepareTwoStep(&'static str, &'static str),
    /// `bopfuse result`.
    FuseTwoStep(&'static str),
    CutBlend(&'static str, &'static str, &'static str, f64),
    Fuse(&'static str, &'static str, &'static str),
    Cut(&'static str, &'static str, &'static str),
    FuseBlend(&'static str, &'static str, &'static str, f64),
}

/// The commands of the 3D viewer a script writes.
#[derive(Clone, Copy, Debug)]
pub enum V3d {
    /// `vinit`.
    Init,
    /// `vsetdispmode mode`.
    DisplayMode(i32),
    /// `vdisplay [-dispMode mode] name`.
    Display(&'static str, Option<i32>),
    /// `vfit`.
    Fit,
    /// `vdump image`, or `checkview -screenshot -3d`.
    Dump(&'static str),
    /// `checkview -display name -3d -path image`.
    ResultView(&'static str, &'static str),
    /// `vbackground -color rrggbb`.
    Background(&'static str),
    /// `vaspects name -setFaceBoundaryDraw 1 -setFaceBoundaryColor WHITE -isoontriangulation 1`.
    WhiteBoundaries(&'static str),
    /// `vaspects name -setFaceBoundaryDraw 1 -mostContinuity c2`.
    BoundariesUpToC2(&'static str),
    /// `trinfo name`.
    MeshReport(&'static str),
    /// What the `BUC60811` command shows once it has built its shapes.
    ShowBuc60811,
}

/// `tolblend`, `blend`, `mkevol`, `updatevol` and `buildevol`.
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
