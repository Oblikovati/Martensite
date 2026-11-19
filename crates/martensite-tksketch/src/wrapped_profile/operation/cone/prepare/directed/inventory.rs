//! Internal numeric labels retain a complete authored-key bijection across native construction.
use super::*;
pub(in crate::surface_sketch::wrapped_profile) struct Inventory {
    pub sides: BTreeMap<(SurfaceRegionId, CurveSpan), ConicalActualSideId>,
    pub caps: BTreeMap<(SurfaceRegionId, usize), ConicalActualCapId>,
}
impl Inventory {
    pub(super) fn capture(prepared: &[Prepared]) -> ConeChecked<Self> {
        let mut sides = BTreeMap::new();
        let mut caps = BTreeMap::new();
        for member in prepared {
            capture_member(member, &mut sides, &mut caps)?;
        }
        assign(&mut sides, ConicalActualSideId)?;
        assign(&mut caps, ConicalActualCapId)?;
        Ok(Self { sides, caps })
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn side(
        &self,
        region: SurfaceRegionId,
        span: CurveSpan,
    ) -> ConeChecked<ConicalActualSideId> {
        self.sides
            .get(&(region, span))
            .copied()
            .ok_or_else(|| missing(region, Some(span), "directed side source inventory"))
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn cap_pair(
        &self,
        region: SurfaceRegionId,
    ) -> ConeChecked<[ConicalActualCapId; 2]> {
        Ok([self.cap(region, 0)?, self.cap(region, 1)?])
    }
    fn cap(&self, region: SurfaceRegionId, row: usize) -> ConeChecked<ConicalActualCapId> {
        self.caps
            .get(&(region, row))
            .copied()
            .ok_or_else(|| missing(region, None, "directed physical cap source inventory"))
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn owner(
        &self,
        cause: &ConicalActualPatchError,
    ) -> (Option<SurfaceRegionId>, Option<CurveSpan>) {
        if let Some(side) = cause.side
            && let Some(((region, span), _)) = self.sides.iter().find(|(_, id)| **id == side)
        {
            return (Some(*region), Some(*span));
        }
        let owner = cause
            .cap
            .and_then(|cap| self.caps.iter().find(|(_, id)| **id == cap));
        (owner.map(|((region, _), _)| *region), None)
    }
}
fn assign<K, V>(entries: &mut BTreeMap<K, V>, wrap: impl Fn(u64) -> V) -> ConeChecked<()> {
    for (index, value) in entries.values_mut().enumerate() {
        let id = u64::try_from(index)
            .ok()
            .and_then(|index| index.checked_add(1))
            .ok_or_else(|| {
                failure(
                    None,
                    None,
                    ConeCause::Scalar {
                        field: "directed source label count",
                        actual: index as f64,
                        expected: "u64 label without overflow",
                    },
                )
            })?;
        *value = wrap(id);
    }
    Ok(())
}
fn missing(region: SurfaceRegionId, span: Option<CurveSpan>, field: &'static str) -> ConeFailure {
    failure(
        Some(region),
        span,
        ConeCause::Binding {
            field,
            actual: vec![],
            expected: vec![1],
        },
    )
}

fn capture_member(
    member: &Prepared,
    sides: &mut BTreeMap<(SurfaceRegionId, CurveSpan), ConicalActualSideId>,
    caps: &mut BTreeMap<(SurfaceRegionId, usize), ConicalActualCapId>,
) -> ConeChecked<()> {
    let region = member.source.region;
    for piece in member.source.loops.iter().flatten() {
        if sides
            .insert((region, piece.span), ConicalActualSideId(0))
            .is_some()
        {
            return Err(duplicate(
                region,
                Some(piece.span),
                "unique directed side source",
            ));
        }
    }
    for row in [0, 1] {
        if caps.insert((region, row), ConicalActualCapId(0)).is_some() {
            return Err(duplicate(region, None, "unique directed cap source"));
        }
    }
    Ok(())
}
fn duplicate(region: SurfaceRegionId, span: Option<CurveSpan>, field: &'static str) -> ConeFailure {
    failure(
        Some(region),
        span,
        ConeCause::Binding {
            field,
            actual: vec![2],
            expected: vec![1],
        },
    )
}
#[cfg(test)]
mod tests;

mod diagnostics;
