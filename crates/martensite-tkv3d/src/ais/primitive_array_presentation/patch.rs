//! Mutable array resizing preserves original mutation and invalidation order.
use super::*;
impl PrimitiveArrayPresentation {
    /// Resize/invalidate compatible mutable storage; `Self::patch_counts(&array, kind, [6, 0, 0])?`.
    pub fn patch_counts(
        array: &Graphic3dArrayOfPrimitives,
        kind: Graphic3dTypeOfPrimitiveArray,
        counts: [i32; 3],
    ) -> Result<bool, Failure> {
        validate_public_patch_counts(counts)?;
        Self::patch_counts_legacy_raw(array, kind, counts)
    }

    /// Preserve the descriptor parser's low-level partial-error behavior during migration.
    /// This compatibility entry may mutate counts before returning an error, including zero
    /// or negative vertices; prefer checked `Self::patch_counts(&array, kind, counts)?`.
    pub fn patch_counts_legacy_raw(
        array: &Graphic3dArrayOfPrimitives,
        kind: Graphic3dTypeOfPrimitiveArray,
        counts: [i32; 3],
    ) -> Result<bool, Failure> {
        if !patch_storage_accepts(array, kind, counts) {
            return Ok(false);
        }
        resize_patch_storage(array, counts)?;
        Ok(true)
    }
}
fn patch_storage_accepts(
    array: &Graphic3dArrayOfPrimitives,
    kind: Graphic3dTypeOfPrimitiveArray,
    [vertices, bounds, edges]: [i32; 3],
) -> bool {
    array.primitive_type() == kind
        && vertices <= array.vertex_number_allocated()
        && edges <= array.edge_number_allocated()
        && bounds <= array.bound_number_allocated()
        && array.attributes().get().is_some_and(|a| a.is_mutable())
        && array.indices().get().is_none_or(|i| i.is_mutable())
}
fn resize_patch_storage(
    array: &Graphic3dArrayOfPrimitives,
    [vertices, bounds, edges]: [i32; 3],
) -> Result<(), Failure> {
    let attributes = array.attributes();
    if let Some(value) = attributes.get() {
        value.set_nb_elements(vertices);
    }
    if let Some(buffer) = attributes.cast::<Graphic3dAttribBuffer>().get() {
        buffer.invalidate_vertices(0, vertices - 1)?;
    }
    if let Some(value) = array.indices().get() {
        value.set_nb_elements(edges);
    }
    if let Some(value) = array.bounds().get() {
        value.set_nb_bounds(bounds);
    }
    Ok(())
}

#[cfg(test)]
#[path = "negative_counts.rs"]
mod negative_counts;

fn validate_public_patch_counts(counts: [i32; 3]) -> Result<(), Failure> {
    if counts[0] > 0 && counts[1] >= 0 && counts[2] >= 0 {
        return Ok(());
    }
    Err(Failure::out_of_range(&format!(
        "PrimitiveArrayPresentation::patch_counts counts {counts:?}; expected [vertices > 0, bounds >= 0, edges >= 0]"
    )))
}
