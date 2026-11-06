use super::TopOpeBRepDSCurve;
use super::explorer_facade::geometry_explorer_facade;

geometry_explorer_facade!(
    TopOpeBRepDSCurveExplorer,
    TopOpeBRepDSCurve,
    curve,
    curve_at,
    is_curve,
    is_curve_keep,
    nb_curve
);

impl TopOpeBRepDSCurveExplorer<'_> {
    /// Returns the current curve or an empty curve after exhaustion; `explorer.curve()`.
    pub fn curve(&self) -> &TopOpeBRepDSCurve {
        match (self.cursor.source(), self.cursor.current_index()) {
            (Some(source), Some(index)) => source.curve(index),
            _ => self.cursor.empty(),
        }
    }
}
