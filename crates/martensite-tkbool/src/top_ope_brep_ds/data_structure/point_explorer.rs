use super::TopOpeBRepDSPoint;
use super::explorer_facade::geometry_explorer_facade;

geometry_explorer_facade!(
    TopOpeBRepDSPointExplorer,
    TopOpeBRepDSPoint,
    point,
    point_at,
    is_point,
    is_point_keep,
    nb_point
);

impl TopOpeBRepDSPointExplorer<'_> {
    /// Reads the current point or the empty point after exhaustion; `explorer.point()?`.
    /// Clearing an active source preserves More() but makes this lookup fail natively.
    pub fn point(&self) -> Result<&TopOpeBRepDSPoint, martensite_tkernel::standard::Failure> {
        match (self.cursor.source(), self.cursor.current_index()) {
            (Some(source), Some(index)) => source.point(index),
            _ => Ok(self.cursor.empty()),
        }
    }
}
