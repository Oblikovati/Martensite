//! Verify semantic identities before publishing detached references (#519).
use super::*;
use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

type Checked<T> = Result<T, ExportBindingFailure>;

#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum ExportBindingFailure {
    Count {
        table: &'static str,
        actual: usize,
        expected: usize,
    },
    DuplicateBinding(BindingKey),
    DuplicateName(WrappedHistoryKey),
    MissingBinding(BindingKey),
}
impl std::fmt::Display for ExportBindingFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count {
                table,
                actual,
                expected,
            } => write!(f, "{table} entries={actual}; expected {expected}"),
            Self::DuplicateBinding(key) => write!(
                f,
                "export binding={key:?}; expected unique storage identity"
            ),
            Self::DuplicateName(key) => {
                write!(f, "export name={key:?}; expected unique authored identity")
            }
            Self::MissingBinding(key) => write!(
                f,
                "export binding={key:?}; expected one decoded shape, found none"
            ),
        }
    }
}
impl std::error::Error for ExportBindingFailure {}

pub(super) fn append(
    keys: &[BindingKey],
    names: &[WrappedHistoryKey],
    bindings: Vec<(BindingKey, Shape)>,
    entities: &mut BTreeMap<WrappedHistoryKey, Shape>,
) -> Checked<()> {
    count("history names", names.len(), keys.len())?;
    count("decoded bindings", bindings.len(), keys.len())?;
    let ordered = bindings.iter().map(|(key, _)| key).eq(keys);
    let mut pending = if ordered {
        resolve_ordered(keys, names, bindings, entities)?
    } else {
        resolve(keys, names, &mut index(bindings)?, entities)?
    };
    entities.append(&mut pending);
    Ok(())
}

// The decoder usually retains key order; verify it before avoiding a second shape index.
fn resolve_ordered(
    keys: &[BindingKey],
    names: &[WrappedHistoryKey],
    bindings: Vec<(BindingKey, Shape)>,
    entities: &BTreeMap<WrappedHistoryKey, Shape>,
) -> Checked<BTreeMap<WrappedHistoryKey, Shape>> {
    unique_bindings(keys)?;
    let pending: BTreeMap<_, _> = names
        .iter()
        .copied()
        .zip(bindings.into_iter().map(|(_, shape)| shape))
        .collect();
    unique_names(names, &pending)?;
    if !entities.is_empty()
        && let Some(name) = pending.keys().find(|name| entities.contains_key(name))
    {
        return Err(ExportBindingFailure::DuplicateName(*name));
    }
    Ok(pending)
}
fn unique_bindings(keys: &[BindingKey]) -> Checked<()> {
    let mut sorted = keys.to_vec();
    sorted.sort_unstable();
    if let Some(pair) = sorted.windows(2).find(|pair| pair[0] == pair[1]) {
        return Err(ExportBindingFailure::DuplicateBinding(pair[0]));
    }
    Ok(())
}
fn unique_names(
    names: &[WrappedHistoryKey],
    pending: &BTreeMap<WrappedHistoryKey, Shape>,
) -> Checked<()> {
    if pending.len() == names.len() {
        return Ok(());
    }
    let mut seen = BTreeSet::new();
    for name in names {
        if !seen.insert(*name) {
            return Err(ExportBindingFailure::DuplicateName(*name));
        }
    }
    count("unique history names", pending.len(), names.len())
}

fn index(bindings: Vec<(BindingKey, Shape)>) -> Checked<BTreeMap<BindingKey, Shape>> {
    let mut decoded = BTreeMap::new();
    for (key, shape) in bindings {
        match decoded.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(shape);
            }
            Entry::Occupied(_) => return Err(ExportBindingFailure::DuplicateBinding(key)),
        }
    }
    Ok(decoded)
}

fn resolve(
    keys: &[BindingKey],
    names: &[WrappedHistoryKey],
    decoded: &mut BTreeMap<BindingKey, Shape>,
    entities: &BTreeMap<WrappedHistoryKey, Shape>,
) -> Checked<BTreeMap<WrappedHistoryKey, Shape>> {
    let mut pending = BTreeMap::new();
    let mut expected = BTreeSet::new();
    for (key, name) in keys.iter().zip(names) {
        if !expected.insert(*key) {
            return Err(ExportBindingFailure::DuplicateBinding(*key));
        }
        let shape = decoded
            .remove(key)
            .ok_or(ExportBindingFailure::MissingBinding(*key))?;
        insert_name(&mut pending, entities, *name, shape)?;
    }
    Ok(pending)
}

fn insert_name(
    pending: &mut BTreeMap<WrappedHistoryKey, Shape>,
    entities: &BTreeMap<WrappedHistoryKey, Shape>,
    name: WrappedHistoryKey,
    shape: Shape,
) -> Checked<()> {
    if entities.contains_key(&name) || pending.contains_key(&name) {
        return Err(ExportBindingFailure::DuplicateName(name));
    }
    pending.insert(name, shape);
    Ok(())
}

fn count(table: &'static str, actual: usize, expected: usize) -> Checked<()> {
    if actual != expected {
        return Err(ExportBindingFailure::Count {
            table,
            actual,
            expected,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
