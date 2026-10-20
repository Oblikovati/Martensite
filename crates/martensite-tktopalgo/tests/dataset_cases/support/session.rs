//! The case session a case body talks to: every variable the script sets (the viewer displays
//! each one as it is set) and every view step (smallview, fit, donly, xwd, ...).
//!
//! The tests run the bodies with [`NoView`], which only checks the geometry.
use super::restore::Restored;

/// One view command of a case script, in the script's terms.
#[derive(Clone, Copy, Debug)]
pub enum ViewStep<'a> {
    /// `smallview [type]`: delete every view, then `view 1 type 728 450 400 400`.
    SmallView(&'a str),
    /// `view 1 -2D- x y w h` (the scripts' `#v2d2` substitute).
    View2d([i32; 4]),
    /// `v2d`: delete every view, then `view 1 -2D- 465 20 800 800`.
    V2d,
    /// `fit` (3D views) or `2dfit` (2D views): both fit view 1 here.
    Fit,
    /// `clear`: erase every displayed variable.
    Clear,
    /// `donly names`: erase everything, then display `names`.
    Donly(&'a [&'a str]),
    /// `don names` / `display names`: display `names` when they are hidden.
    Display(&'a [&'a str]),
    /// `isos name n`: `ChangeNbIsos(n)` of the shape variable `name`.
    Isos(&'a str, i32),
    /// `u`: rotate every AXON/PERS view up by DEFROTATE (5 degrees) about screen X.
    RotateUp,
    /// `r`: rotate every AXON/PERS view right by DEFROTATE about screen Y.
    RotateRight,
    /// `normals shape length`: display the face normals of the shape.
    Normals(&'a martensite_tkbrep::topods::Shape, f64),
    /// `reverse c` / a variable redraw: repaint view 1 with the current geometry.
    Repaint,
    /// `xwd file` (`checkview -screenshot -2d`): capture view 1.
    Xwd(&'a str),
}

/// What a case body needs from the case harness besides the kernel.
pub trait Session {
    /// `Script::Set(name, value)`: the variable is (re)defined and displayed.
    fn set(&mut self, name: &str, value: &Restored);
    /// One view command.
    fn step(&mut self, step: ViewStep<'_>);
}

/// A session without a viewer: variables and view steps are accepted and ignored.
pub struct NoView;

impl Session for NoView {
    fn set(&mut self, _name: &str, _value: &Restored) {}
    fn step(&mut self, _step: ViewStep<'_>) {}
}

/// `checkview -display shape -2d -path image`: smallview, clear, donly shape (only when
/// it is a defined variable), fit and xwd.
pub fn checkview_display(session: &mut dyn Session, shape: Option<&str>, image: &str) {
    session.step(ViewStep::SmallView("AXON"));
    session.step(ViewStep::Clear);
    if let Some(shape) = shape {
        session.step(ViewStep::Donly(&[shape]));
    }
    session.step(ViewStep::Fit);
    session.step(ViewStep::Xwd(image));
}

/// `explode name ...`: sets `name_1`, `name_2`, ... to `parts`, which it returns.
pub fn set_exploded(
    session: &mut dyn Session,
    name: &str,
    parts: Vec<martensite_tkbrep::topods::Shape>,
) -> Vec<martensite_tkbrep::topods::Shape> {
    for (index, part) in parts.iter().enumerate() {
        session.set(
            &format!("{name}_{}", index + 1),
            &Restored::Shape(part.clone()),
        );
    }
    parts
}

/// `set` then return the value, for script lines that both define and use a variable.
pub fn set<T>(session: &mut dyn Session, name: &str, value: T) -> T
where
    T: Clone + Into<Restored>,
{
    session.set(name, &value.clone().into());
    value
}
