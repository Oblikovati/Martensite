//! Public dump fields keep duplicate keys visible; pointer text is never parsed into handles.
use super::*;
use martensite_tkernel::standard::dump::{DumpValues, split_json};
pub(super) type Fields = Vec<(Vec<u8>, Vec<u8>)>;

/// Read one public dump level; `fields(bytes)?`.
pub(super) fn fields(value: &[u8]) -> io::Result<Fields> {
    let value = value
        .strip_prefix(b"{")
        .and_then(|v| v.strip_suffix(b"}"))
        .unwrap_or(value);
    let mut parsed = DumpValues::default();
    if !split_json(value, &mut parsed).map_err(|e| refused(e.to_string()))? {
        return Err(refused(format!(
            "dump {value:?}; expected complete public dump fields"
        )));
    }
    parsed_fields(&parsed)
}
fn parsed_fields(parsed: &DumpValues) -> io::Result<Fields> {
    (1..=parsed.extent())
        .map(|index| {
            let key = parsed
                .find_key(index)
                .map_err(|error| refused(error.to_string()))?;
            let value = parsed
                .find_from_index(index)
                .map_err(|error| refused(error.to_string()))?;
            Ok((key.as_bytes().to_vec(), value.value.as_bytes().to_vec()))
        })
        .collect()
}

/// Refuse missing or repeated semantic fields; `unique(bytes,b"Group")?`.
pub(super) fn unique(value: &[u8], key: &[u8]) -> io::Result<Vec<u8>> {
    let fields = fields(value)?;
    let matched: Vec<_> = fields
        .iter()
        .filter(|(name, _)| name == key || name.starts_with(&[key, b"_"].concat()))
        .collect();
    if matched.len() != 1 || matched[0].0 != key {
        return Err(refused(format!(
            "dump field {key:?} matches {}; expected one exact field",
            matched.len()
        )));
    }
    Ok(matched[0].1.clone())
}

/// Snapshot every field outside an explicit role; `without(bytes,&[b"Aspects"])?`.
pub(super) fn without(value: &[u8], keys: &[&[u8]]) -> io::Result<Fields> {
    Ok(fields(value)?
        .into_iter()
        .filter(|(key, _)| !keys.contains(&key.as_slice()))
        .collect())
}

/// Format an owned handle address without parsing text; `pointer(address)?`.
pub(super) fn pointer(address: usize) -> io::Result<Vec<u8>> {
    martensite_tkernel::standard::dump::get_pointer_info(Some(address), true)
        .map(|value| value.as_bytes().to_vec())
        .map_err(|error| refused(error.to_string()))
}
