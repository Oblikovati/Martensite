//! Registered later stages may emit lines irrespective of the retained input topology.
use crate::{PrimitiveTopology, user_program::UserStage};
pub(super) fn line_stage_requires_admission(
    topology: PrimitiveTopology,
    mut stages: impl Iterator<Item = UserStage>,
) -> bool {
    topology == PrimitiveTopology::Lines
        || stages.any(|stage| {
            matches!(
                stage,
                UserStage::Geometry | UserStage::TessControl | UserStage::TessEvaluation
            )
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retained_lines_and_registered_later_stages_require_explicit_adaptation() {
        for topology in [
            PrimitiveTopology::Triangles,
            PrimitiveTopology::Points,
            PrimitiveTopology::Lines,
        ] {
            for later in [
                UserStage::Geometry,
                UserStage::TessControl,
                UserStage::TessEvaluation,
            ] {
                assert!(line_stage_requires_admission(
                    topology,
                    [UserStage::Vertex, later, UserStage::Fragment].into_iter()
                ));
            }
            let ordinary = [UserStage::Vertex, UserStage::Fragment];
            assert_eq!(
                line_stage_requires_admission(topology, ordinary.into_iter()),
                topology == PrimitiveTopology::Lines
            );
        }
        assert!(!line_stage_requires_admission(
            PrimitiveTopology::Triangles,
            [].into_iter()
        ));
        assert!(line_stage_requires_admission(
            PrimitiveTopology::Lines,
            [].into_iter()
        ));
    }
}
