//! Exact direct-child multisets; no recursive explorer can hide unexpected levels.
use super::*;
use martensite_tkbrep::topods::ShapeIterator;
pub(super) fn children(
    owner: impl Into<CoverageOwner> + Copy,
    parent: &Shape,
    kind: ShapeEnum,
    maximum: usize,
) -> Checked<Vec<Shape>> {
    let mut iterator = ShapeIterator::new(parent, true, true);
    let mut result = Vec::new();
    while iterator.more() {
        capacity(owner, result.len(), maximum)?;
        let child = native(owner, iterator.value())?.clone();
        registry::kind_check(owner, &child, kind)?;
        result.push(child);
        iterator.next();
    }
    Ok(result)
}
fn capacity(owner: impl Into<CoverageOwner> + Copy, actual: usize, maximum: usize) -> Checked<()> {
    if actual == maximum {
        return Err(fail(
            owner,
            CoverageCause::Count {
                role: "direct children",
                actual: actual + 1,
                expected: maximum,
            },
        ));
    }
    Ok(())
}

pub(super) fn uses(
    owner: impl Into<CoverageOwner> + Copy,
    parent: &Shape,
    kind: ShapeEnum,
    expected: &[BoundUse],
    registry: &Registry<'_>,
) -> Checked<Vec<BoundUse>> {
    let shapes = children(owner, parent, kind, expected.len())?;
    count(owner, "direct child uses", shapes.len(), expected.len())?;
    let actual = bound_uses(owner, &shapes, registry)?;
    compare(owner, &actual, expected)?;
    Ok(actual)
}
pub(super) fn bound_uses(
    owner: impl Into<CoverageOwner> + Copy,
    shapes: &[Shape],
    registry: &Registry<'_>,
) -> Checked<Vec<BoundUse>> {
    shapes
        .iter()
        .map(|shape| {
            Ok(BoundUse::new(
                registry.resolve(owner, shape)?,
                shape.orientation(),
            ))
        })
        .collect()
}
pub(super) fn signature(uses: &[BoundUse]) -> Vec<(BindingKey, u8)> {
    let mut keys: Vec<_> = uses
        .iter()
        .map(|use_| (use_.key, use_.orientation as u8))
        .collect();
    keys.sort();
    keys
}
pub(super) fn compare(
    owner: impl Into<CoverageOwner> + Copy,
    actual: &[BoundUse],
    expected: &[BoundUse],
) -> Checked<()> {
    if signature(actual) != signature(expected) {
        return Err(fail(
            owner,
            CoverageCause::Uses {
                actual: actual.to_vec(),
                expected: expected.to_vec(),
            },
        ));
    }
    Ok(())
}
pub(super) fn closed(owner: impl Into<CoverageOwner> + Copy, shape: &Shape) -> Checked<()> {
    if !native(owner, shape.closed())? {
        return Err(fail(owner, CoverageCause::ClosedFlag));
    }
    Ok(())
}
