//! Bind public keys to the same semantic references retained in the frozen table.
use super::*;
use crate::surface_sketch::{HistoryRole, NormalSense};
use std::collections::{BTreeMap, BTreeSet};
type Checked<T> = Result<T, HistoryMapFailure>;
type Directions = BTreeMap<SurfaceRegionId, profile_input::CurveDirections>;
pub(in crate::surface_sketch::wrapped_profile) struct HistoryPublication {
    pub(in crate::surface_sketch::wrapped_profile) records: Vec<WrappedHistoryRecord>,
    pub(in crate::surface_sketch::wrapped_profile) bindings: Vec<WrappedHistoryKey>,
}
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum HistoryMapFailure {
    Owner(SurfaceRegionId),
    Parameter {
        owner: SurfaceRegionId,
        cause: profile_input::ProfileInputFailure,
    },
    Duplicate(WrappedHistoryKey),
    Binding {
        key: BindingKey,
        count: usize,
    },
    Count {
        actual: usize,
        expected: usize,
    },
}
impl std::fmt::Display for HistoryMapFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owner(owner) => write!(
                f,
                "history owner={owner:?}; expected one source direction map"
            ),
            Self::Parameter { owner, cause } => write!(f, "history owner={owner:?}: {cause}"),
            Self::Duplicate(key) => {
                write!(f, "history key={key:?}; expected unique authored identity")
            }
            Self::Binding { key, count } => write!(
                f,
                "history binding={key:?}, records={count}; expected one retained source record"
            ),
            Self::Count { actual, expected } => write!(
                f,
                "history bindings={actual}; expected {expected} unique targets"
            ),
        }
    }
}
#[derive(Default)]
struct Projection {
    records: BTreeMap<WrappedHistoryKey, WrappedHistoryRecord>,
    bindings: BTreeMap<BindingKey, WrappedHistoryKey>,
}

// Native cylinder archives retain increasing-radius order. Normalize their identities
// before the shared caller-relative projection (#522).
pub(in crate::surface_sketch::wrapped_profile) fn prepare_physical(
    keys: &[BindingKey],
    history: &history::HistoryEvidence,
    directions: &Directions,
    normal: NormalSense,
) -> Checked<HistoryPublication> {
    let records = history.records.iter().map(|record| CallerDepthRecord {
        native_binding: record.result,
        caller_result: caller_key(record.result, normal),
        original: record,
    });
    prepare_normalized(keys, records, directions)
}

#[cfg(test)]
pub(in crate::surface_sketch::wrapped_profile) fn prepare(
    keys: &[BindingKey],
    history: &history::HistoryEvidence,
    directions: &Directions,
) -> Checked<HistoryPublication> {
    let records = history.records.iter().map(|record| CallerDepthRecord {
        native_binding: record.result,
        caller_result: record.result,
        original: record,
    });
    prepare_normalized(keys, records, directions)
}

struct CallerDepthRecord<'a> {
    native_binding: BindingKey,
    caller_result: BindingKey,
    original: &'a history::HistoryRecord,
}

fn prepare_normalized<'a>(
    keys: &[BindingKey],
    records: impl Iterator<Item = CallerDepthRecord<'a>>,
    directions: &Directions,
) -> Checked<HistoryPublication> {
    let mut projection = Projection::default();
    for record in records {
        projection.insert(record, directions)?;
    }
    projection.finish(keys)
}

fn caller_key(mut key: BindingKey, normal: NormalSense) -> BindingKey {
    key.role = role(key.role, normal);
    key
}

impl Projection {
    fn insert(&mut self, record: CallerDepthRecord<'_>, directions: &Directions) -> Checked<()> {
        let target = record.native_binding;
        let projected = project_record(record, directions)?;
        let key = projected.result;
        if self.records.insert(key, projected).is_some() {
            return Err(HistoryMapFailure::Duplicate(key));
        }
        if self.bindings.insert(target, key).is_some() {
            return Err(HistoryMapFailure::Binding {
                key: target,
                count: 2,
            });
        }
        Ok(())
    }
    fn binding(&self, key: BindingKey) -> Checked<WrappedHistoryKey> {
        self.bindings
            .get(&key)
            .copied()
            .ok_or(HistoryMapFailure::Binding { key, count: 0 })
    }
    fn finish(self, keys: &[BindingKey]) -> Checked<HistoryPublication> {
        let unique: BTreeSet<_> = keys.iter().copied().collect();
        count(unique.len(), keys.len())?;
        count(self.bindings.len(), keys.len())?;
        let bindings = keys
            .iter()
            .map(|key| self.binding(*key))
            .collect::<Checked<_>>()?;
        Ok(HistoryPublication {
            records: self.records.into_values().collect(),
            bindings,
        })
    }
}

fn project_record(
    record: CallerDepthRecord<'_>,
    directions: &Directions,
) -> Checked<WrappedHistoryRecord> {
    Ok(WrappedHistoryRecord {
        result: project_key(record.caller_result, record.original.owner, directions)?,
        owner: record.original.owner,
        relation: relation(record.original.relation),
        parents: record.original.parents.clone(),
    })
}

fn project_key(
    result: BindingKey,
    owner: SurfaceRegionId,
    directions: &Directions,
) -> Checked<WrappedHistoryKey> {
    let directions = directions
        .get(&owner)
        .ok_or(HistoryMapFailure::Owner(owner))?;
    let source = source(result.source, directions)
        .map_err(|cause| HistoryMapFailure::Parameter { owner, cause })?;
    Ok(WrappedHistoryKey {
        source,
        role: result.role,
    })
}

fn count(actual: usize, expected: usize) -> Checked<()> {
    if actual != expected {
        return Err(HistoryMapFailure::Count { actual, expected });
    }
    Ok(())
}

fn source(
    source: Source,
    directions: &profile_input::CurveDirections,
) -> Result<WrappedEntitySource, profile_input::ProfileInputFailure> {
    match source {
        Source::Region(region) => Ok(WrappedEntitySource::Region(region)),
        Source::Span(span) => span_source(span, directions),
        Source::Corner(CornerSource::Authored(vertex)) => Ok(WrappedEntitySource::Vertex(vertex)),
        Source::Corner(CornerSource::Split {
            curve,
            numerator,
            level,
        }) => split_source(curve, numerator, level, directions),
    }
}

fn span_source(
    span: CurveSpan,
    directions: &profile_input::CurveDirections,
) -> Result<WrappedEntitySource, profile_input::ProfileInputFailure> {
    let original = directions.source_interval(span)?;
    Ok(WrappedEntitySource::Curve {
        curve: original.curve,
        span: WrappedSourceSpan {
            level: original.level,
            index: original.index,
            reversed: original.reversed,
        },
    })
}

fn split_source(
    curve: SurfaceCurveId,
    numerator: u32,
    level: u8,
    directions: &profile_input::CurveDirections,
) -> Result<WrappedEntitySource, profile_input::ProfileInputFailure> {
    let original = directions.source_point(curve, numerator, level)?;
    Ok(WrappedEntitySource::SplitVertex {
        curve: original.curve,
        point: WrappedSourcePoint {
            level: original.level,
            numerator: original.numerator,
        },
    })
}

fn relation(relation: history::HistoryRelation) -> WrappedHistoryRelation {
    match relation {
        history::HistoryRelation::Derived => WrappedHistoryRelation::Derived,
        history::HistoryRelation::Split => WrappedHistoryRelation::Split,
        history::HistoryRelation::Created => WrappedHistoryRelation::Created,
    }
}

pub(super) fn role(role: HistoryRole, normal: NormalSense) -> HistoryRole {
    if normal == NormalSense::Outward {
        return role;
    }
    match role {
        HistoryRole::FirstDepthFace => HistoryRole::LastDepthFace,
        HistoryRole::LastDepthFace => HistoryRole::FirstDepthFace,
        HistoryRole::FirstDepthEdge => HistoryRole::LastDepthEdge,
        HistoryRole::LastDepthEdge => HistoryRole::FirstDepthEdge,
        HistoryRole::FirstDepthVertex => HistoryRole::LastDepthVertex,
        HistoryRole::LastDepthVertex => HistoryRole::FirstDepthVertex,
        other => other,
    }
}
