//! What a case body needs from its session besides the kernel: shape variables and the
//! view commands of its script.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::gp::{Point2, Point3};

/// One view command of a case script, in the script's terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewStep<'a> {
    /// `smallview`: delete every view, then open the 400x400 axonometric view 1.
    SmallView,
    /// `smallview -2D-`, `v2d` or `view 1 -2D- ...`: the same, with a 400x400 planar view 1.
    PlaneView,
    /// `clear`: erase every displayed variable.
    Clear,
    /// `donly names`: erase everything, then display `names`.
    Donly(&'a [&'a str]),
    /// `disp names`: display `names` as well, erasing nothing.
    Disp(&'a [&'a str]),
    /// `fit`: fit view 1 to what it displays.
    Fit,
    /// `xwd image`: capture view 1 under the name `image`.
    Xwd(&'a str),
}

/// A script variable that is no shape: what the session shows under its name.
#[derive(Clone, Copy)]
pub enum Drawing<'a> {
    /// A curve in space, as `intersect` and `bopcurves` bind it.
    Curve(&'a Handle<dyn GeomCurve>),
    /// A curve in the plane, as `bopcurves -2d` and `pcurve` bind it.
    PlaneCurve(&'a Handle<dyn Geom2dCurve>),
    /// A surface, as `restore` and `mksurface` bind it.
    Surface(&'a Handle<dyn GeomSurface>),
    /// A point in space, as `intersect` and `bopcurves` bind it.
    Point(Point3),
    /// A point in the plane, as `point name u v` binds it.
    PlanePoint(Point2),
}

/// The variables and the view of one running case.
pub trait Session {
    /// `name` is (re)bound to `shape`, which is displayed as a script variable is when set.
    /// Example: `session.bind("b1", &shape)`.
    fn bind(&mut self, name: &str, shape: &Shape);
    /// `name` is (re)bound to a drawing, displayed as a script variable is when set. A session
    /// that only records its view steps does not need them.
    /// Example: `session.bind_drawing("c_1", Drawing::curve(&curve))`.
    fn bind_drawing(&mut self, _name: &str, _drawing: Drawing<'_>) {}
    /// The shape bound to `name`; `none` is the script's `isdraw name` being false.
    /// Example: `session.shape("result").is_some()`.
    fn shape(&self, name: &str) -> Option<&Shape>;
    /// Run one view command; a capture that shows nothing panics.
    /// Example: `session.view(ViewStep::fit)`.
    fn view(&mut self, step: ViewStep<'_>);
}
