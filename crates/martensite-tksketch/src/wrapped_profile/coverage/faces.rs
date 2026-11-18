//! Match every actual wire to exactly one declared oriented contour.
use super::*;
pub(super) fn check(
    owner: BindingKey,
    expected: &[Vec<BoundUse>],
    orientation: Orientation,
    registry: &Registry<'_>,
) -> Checked<FaceEvidence> {
    let face = registry.shape(owner)?;
    let wires = members::children(owner, face, ShapeEnum::Wire, expected.len())?;
    count(owner, "face wires", wires.len(), expected.len())?;
    let mut remaining = contours(owner, expected)?;
    let mut edges = Vec::new();
    let maximum = expected.iter().map(Vec::len).max().unwrap_or(0);
    for wire in &wires {
        edges.extend(check_wire(owner, wire, maximum, &mut remaining, registry)?);
    }
    Ok(FaceEvidence::new(owner, orientation, wires.len(), edges))
}
fn contours(owner: BindingKey, expected: &[Vec<BoundUse>]) -> Checked<Contours<'_>> {
    let remaining: BTreeMap<_, _> = expected
        .iter()
        .map(|u| (members::signature(u), u))
        .collect();
    count(
        owner,
        "distinct declared contours",
        remaining.len(),
        expected.len(),
    )?;
    Ok(remaining)
}
type Contours<'a> = BTreeMap<Vec<(BindingKey, u8)>, &'a Vec<BoundUse>>;
fn check_wire(
    owner: BindingKey,
    wire: &Shape,
    maximum: usize,
    remaining: &mut Contours<'_>,
    registry: &Registry<'_>,
) -> Checked<Vec<BoundUse>> {
    registry::canonical(owner, wire)?;
    members::closed(owner, wire)?;
    let edges = members::children(owner, wire, ShapeEnum::Edge, maximum)?;
    let actual = members::bound_uses(owner, &edges, registry)?;
    if remaining.remove(&members::signature(&actual)).is_none() {
        return Err(fail(owner, CoverageCause::Contour { actual }));
    }
    Ok(actual)
}
pub(super) struct FaceEvidence {
    pub owner: BindingKey,
    pub orientation: Orientation,
    pub wire_count: usize,
    pub edges: Vec<BoundUse>,
}

impl FaceEvidence {
    fn new(
        owner: BindingKey,
        orientation: Orientation,
        wire_count: usize,
        edges: Vec<BoundUse>,
    ) -> Self {
        Self {
            owner,
            orientation,
            wire_count,
            edges,
        }
    }
}
