//! `BRepBlendPointOnRst`: the crossing of a blend line with a restriction arc.
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkgeomalgo::int_surf::IntSurfTransition;

/// `BRepBlendPointOnRst`: the arc, the parameter on it, and the transitions of the line
/// and of the arc at the crossing.
///
/// ```
/// use martensite_tkfillet::brep_blend::BRepBlendPointOnRst;
/// assert_eq!(BRepBlendPointOnRst::new().parameter_on_arc(), 0.0);
/// ```
#[derive(Clone, Default)]
pub struct BRepBlendPointOnRst {
    arc: Handle<dyn Adaptor2dCurve2d>,
    traline: IntSurfTransition,
    traarc: IntSurfTransition,
    prm: f64,
}

impl BRepBlendPointOnRst {
    /// `BRepBlendPointOnRst()`: no arc, parameter 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// `BRepBlendPointOnRst(A, Param, TLine, TArc)`;
    /// `BRepBlendPointOnRst::with_arc(&arc, 0.5, t_line, t_arc)`.
    pub fn with_arc(
        a: &Handle<dyn Adaptor2dCurve2d>,
        param: f64,
        t_line: IntSurfTransition,
        t_arc: IntSurfTransition,
    ) -> Self {
        Self {
            arc: a.clone(),
            traline: t_line,
            traarc: t_arc,
            prm: param,
        }
    }

    /// `SetArc(A, Param, TLine, TArc)`; `point.set_arc(&arc, 0.5, t_line, t_arc)`.
    pub fn set_arc(
        &mut self,
        a: &Handle<dyn Adaptor2dCurve2d>,
        param: f64,
        t_line: IntSurfTransition,
        t_arc: IntSurfTransition,
    ) {
        *self = Self::with_arc(a, param, t_line, t_arc);
    }

    /// `Arc()`.
    pub fn arc(&self) -> &Handle<dyn Adaptor2dCurve2d> {
        &self.arc
    }

    /// `TransitionOnLine()`.
    pub fn transition_on_line(&self) -> &IntSurfTransition {
        &self.traline
    }

    /// `TransitionOnArc()`.
    pub fn transition_on_arc(&self) -> &IntSurfTransition {
        &self.traarc
    }

    /// `ParameterOnArc()`.
    pub fn parameter_on_arc(&self) -> f64 {
        self.prm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_arc_replaces_every_field() {
        let mut point = BRepBlendPointOnRst::new();
        assert!(point.arc().is_null());
        let line = IntSurfTransition::default();
        point.set_arc(&Handle::default(), 2.5, line, line);
        assert_eq!(point.parameter_on_arc(), 2.5);
        assert_eq!(point.transition_on_arc(), &line);
    }
}
