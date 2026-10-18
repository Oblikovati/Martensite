//! Field incidence shared by integer and real callback storage.
use martensite_tkernel::standard::Failure;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub(super) struct ScalarIncidence<Field> {
    pub(super) slot: usize,
    pub(super) fields: Vec<Field>,
}
pub(super) fn scalar_incidence<Field: Copy + Debug>(
    fields: impl IntoIterator<Item = (Field, usize)>,
    length: usize,
    label: &str,
) -> Result<Vec<ScalarIncidence<Field>>, Failure> {
    let mut groups = BTreeMap::<usize, Vec<Field>>::new();
    for (field, slot) in fields {
        validate_scalar_slot(field, slot, length, label)?;
        groups.entry(slot).or_default().push(field);
    }
    Ok(groups
        .into_iter()
        .map(|(slot, fields)| ScalarIncidence { slot, fields })
        .collect())
}
fn validate_scalar_slot<Field: Debug>(
    field: Field,
    slot: usize,
    length: usize,
    label: &str,
) -> Result<(), Failure> {
    if slot < length {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_OutOfRange",
        &format!("{label}: {field:?} slot={slot}; expected slot in 0..{length}"),
    ))
}
