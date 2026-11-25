//! Temporary curve names never leave the wrapped operation's published history.
use super::*;
use crate::surface_sketch::HistorySource;

impl Proxy {
    pub(in super::super) fn history(
        &self,
        publication: &mut history_projection::HistoryPublication,
    ) -> Checked<()> {
        for record in &mut publication.records {
            self.record(record)?;
        }
        publication.records.sort_by_key(|record| record.result);
        let mut unique = BTreeSet::new();
        for key in &mut publication.bindings {
            *key = self.key(*key)?;
            if !unique.insert(*key) {
                return Err(invalid(
                    format!("history key={key:?}"),
                    "unique original curve and parameter bindings",
                ));
            }
        }
        Ok(())
    }

    fn record(&self, record: &mut WrappedHistoryRecord) -> Checked<()> {
        if let WrappedEntitySource::Curve { curve, .. } = record.result.source
            && self.spans.get(&curve).is_some_and(|base| base.level > 0)
            && record.relation == WrappedHistoryRelation::Derived
        {
            record.relation = WrappedHistoryRelation::Split;
        }
        record.result = self.key(record.result)?;
        for source in &mut record.parents {
            *source = self.parent(*source);
        }
        record.parents.sort();
        record.parents.dedup();
        Ok(())
    }

    fn parent(&self, source: HistorySource) -> HistorySource {
        match source {
            HistorySource::Curve(curve) if self.spans.contains_key(&curve) => {
                HistorySource::Curve(self.source)
            }
            HistorySource::Vertex(vertex) if self.joins.contains_key(&vertex) => {
                HistorySource::Curve(self.source)
            }
            other => other,
        }
    }

    fn key(&self, mut key: WrappedHistoryKey) -> Checked<WrappedHistoryKey> {
        key.source = match key.source {
            WrappedEntitySource::Curve { curve, span } => self.curve_key(curve, span)?,
            WrappedEntitySource::SplitVertex { curve, point } => self.split_key(curve, point)?,
            WrappedEntitySource::Vertex(vertex) => {
                self.joins
                    .get(&vertex)
                    .map_or(WrappedEntitySource::Vertex(vertex), |&point| {
                        WrappedEntitySource::SplitVertex {
                            curve: self.source,
                            point,
                        }
                    })
            }
            other => other,
        };
        Ok(key)
    }

    fn curve_key(
        &self,
        curve: SurfaceCurveId,
        span: WrappedSourceSpan,
    ) -> Checked<WrappedEntitySource> {
        let Some(base) = self.spans.get(&curve) else {
            return Ok(WrappedEntitySource::Curve { curve, span });
        };
        let (index, level) = compose(*base, span.index, span.level)?;
        Ok(WrappedEntitySource::Curve {
            curve: self.source,
            span: WrappedSourceSpan {
                level,
                index,
                reversed: span.reversed,
            },
        })
    }

    fn split_key(
        &self,
        curve: SurfaceCurveId,
        old: WrappedSourcePoint,
    ) -> Checked<WrappedEntitySource> {
        let Some(base) = self.spans.get(&curve) else {
            return Ok(WrappedEntitySource::SplitVertex { curve, point: old });
        };
        let (numerator, level) = compose(*base, old.numerator, old.level)?;
        Ok(WrappedEntitySource::SplitVertex {
            curve: self.source,
            point: point(numerator, level),
        })
    }
}

fn compose(base: WrappedSourceSpan, local: u64, level: u8) -> Checked<(u64, u8)> {
    let combined = u16::from(base.level) + u16::from(level);
    if combined >= 64 {
        return Err(invalid(
            format!("source span level={combined}"),
            "a combined exact dyadic level below 64",
        ));
    }
    Ok(((base.index << level) + local, combined as u8))
}

pub(super) fn point(numerator: u64, level: u8) -> WrappedSourcePoint {
    let shift = numerator.trailing_zeros().min(u32::from(level));
    WrappedSourcePoint {
        level: level - shift as u8,
        numerator: numerator >> shift,
    }
}
