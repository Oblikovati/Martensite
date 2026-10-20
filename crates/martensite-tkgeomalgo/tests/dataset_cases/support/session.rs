//! The case viewer a case body talks to: every variable it sets (the viewer displays each one
//! as it is set), every marker a command draws and every view command.
//!
//! The tests run the bodies with [`NoView`], which only checks the kernel results and
//! the log.
use super::value::Value;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkmath::gp::{Point2, Point3};

/// One view command of a case script, in the script's terms.
#[derive(Clone, Copy, Debug)]
pub enum ViewStep<'a> {
    /// `smallview [type]`: delete every view, then `view 1 type 728 450 400 400`.
    SmallView(&'a str),
    /// `view 1 type x y w h`.
    View(&'a str, [i32; 4]),
    /// `v2d`: delete every view, then `view 1 -2D- 465 20 800 800`.
    V2d,
    /// `fit`: fit every 3D view.
    Fit,
    /// `2dfit`: fit every 2D view.
    Fit2d,
    /// `zoom z`: set the zoom of every view.
    Zoom(f64),
    /// `clear`: erase every displayed variable and drawn marker.
    Clear,
    /// `clear names` / `erase names`: erase the named variables.
    Erase(&'a [&'a str]),
    /// `donly names`: erase everything, then display `names`.
    Donly(&'a [&'a str]),
    /// `display names` / `don names`: display `names` when they are hidden.
    Display(&'a [&'a str]),
    /// `clpoles name`: hide the poles of a B-spline or Bezier variable.
    ClearPoles(&'a str),
    /// `clknots name`: hide the knots of a B-spline variable.
    ClearKnots(&'a str),
    /// `xwd file` (`checkview -screenshot -2d`): capture view 1.
    Xwd(&'a str),
}

/// A drawable a command displays without a variable.
#[derive(Clone)]
pub enum Marker {
    /// `xdistcs`: a red plus marker of size 3 at the point.
    SpacePlus(Point3),
    /// `2dintersect`: the cross of an intersection point.
    PlaneCross(Point2),
    /// `2dintersect`: the two curves of an overlap segment (blue and violet).
    PlaneSegments(Handle<dyn Geom2dCurve>, Handle<dyn Geom2dCurve>),
}

/// A curve color a command sets in the curve parameters before `Script::Set`
/// (the orientation color of an edge: red forward, blue reversed, pink external,
/// orange internal).
#[derive(Clone, Copy, Debug)]
pub enum CurveColor {
    Rouge,
    Bleu,
    Rose,
    Orange,
}

/// How the drawable of a variable is made.
#[derive(Clone, Copy, Debug)]
pub enum Look {
    /// Setting a variable with the default parameters: B-spline and Bezier
    /// geometry gets its specialized drawable (poles, knots).
    Default,
    /// The default drawable in a curve color the command set.
    Colored(CurveColor),
    /// The generic `DrawTrSurf_Curve`/`Curve2d`/`Surface` a restored file names.
    Generic,
}

/// What a case body needs from the case harness besides the kernel.
pub trait Session {
    /// `Script::Set(name, value)`: the variable is (re)defined and displayed.
    fn set(&mut self, name: &str, value: &Value);
    /// `Script::Set` with the drawable `look`; the default ignores the look.
    fn set_look(&mut self, name: &str, value: &Value, _look: Look) {
        self.set(name, value);
    }
    /// `unset name`: the variable is erased and forgotten.
    fn unset(&mut self, name: &str);
    /// `renamevar from to`: the drawable keeps its display under the new name.
    fn rename(&mut self, from: &str, to: &str);
    /// A marker displayed without a variable.
    fn marker(&mut self, marker: Marker);
    /// One view command.
    fn step(&mut self, step: ViewStep<'_>);
}

/// A session without a viewer: variables, markers and view steps are accepted and ignored.
pub struct NoView;

impl Session for NoView {
    fn set(&mut self, _name: &str, _value: &Value) {}
    fn unset(&mut self, _name: &str) {}
    fn rename(&mut self, _from: &str, _to: &str) {}
    fn marker(&mut self, _marker: Marker) {}
    fn step(&mut self, _step: ViewStep<'_>) {}
}
