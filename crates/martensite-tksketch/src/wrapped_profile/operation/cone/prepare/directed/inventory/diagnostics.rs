//! Pair refusal labels are mapped only through the issued authored-source inventory.
use super::*;
use crate::surface_sketch::HistorySource;
impl Inventory {
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn math_cause(
        &self,
        cause: ConicalActualPatchError,
    ) -> ConeCause {
        let mut entities = Vec::new();
        self.pair_entities(&cause.cause, &mut entities);
        if entities.is_empty() {
            return ConeCause::ActualPatch(Box::new(cause));
        }
        ConeCause::ActualPair {
            cause: Box::new(cause),
            entities,
        }
    }
    fn pair_entities(&self, cause: &ConicalActualPatchFailure, entities: &mut Vec<HistorySource>) {
        use ConicalActualPatchFailure::*;
        match cause {
            UnresolvedSidePair { sides, .. } => self.side_entities(sides, entities),
            SidePair { sides, cause } => {
                self.side_entities(sides, entities);
                self.pair_entities(cause, entities);
            }
            CapPair { caps, cause } => {
                self.cap_entities(caps, entities);
                self.pair_entities(cause, entities);
            }
            Policy { .. }
            | Shape { .. }
            | Binding { .. }
            | Value { .. }
            | Unresolved { .. }
            | Arithmetic(_) => {}
        }
    }
    fn side_entities(&self, sides: &[ConicalActualSideId; 2], entities: &mut Vec<HistorySource>) {
        for side in sides {
            if let Some(((region, span), _)) = self.sides.iter().find(|(_, id)| **id == *side) {
                entities.extend([
                    HistorySource::Region(*region),
                    HistorySource::Curve(span.curve),
                ]);
            }
        }
    }
    fn cap_entities(&self, caps: &[ConicalActualCapId; 2], entities: &mut Vec<HistorySource>) {
        for cap in caps {
            if let Some(((region, _), _)) = self.caps.iter().find(|(_, id)| **id == *cap) {
                entities.push(HistorySource::Region(*region));
            }
        }
    }
}
