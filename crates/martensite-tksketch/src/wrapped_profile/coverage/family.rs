//! The compound is a transport container; each authored region still owns one solid.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in super::super) struct FamilyCoverage {
    pub members: usize,
    pub bindings: usize,
}
pub(in super::super) struct MemberView {
    pub region: SurfaceRegionId,
    pub shape: Shape,
    pub bindings: Vec<(BindingKey, Shape)>,
}
pub(in super::super) fn certify_family(
    requests: &[Request],
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<(FamilyCoverage, Vec<MemberView>)> {
    require_members(requests)?;
    let registry = Registry::build_many(requests, bindings)?;
    root_members(requests, root, &registry)?;
    let views = requests
        .iter()
        .map(|r| member_view(r, &registry))
        .collect::<Checked<Vec<_>>>()?;
    Ok((
        FamilyCoverage {
            members: requests.len(),
            bindings: registry.len(),
        },
        views,
    ))
}
fn member_view(request: &Request, registry: &Registry<'_>) -> Checked<MemberView> {
    let shape = registry
        .shape(expected::region(request, HistoryRole::Solid))?
        .clone();
    let definitions = expected::definitions(request)?;
    let bindings = definitions
        .keys()
        .map(|key| Ok((*key, registry.shape(*key)?.clone())))
        .collect::<Checked<_>>()?;
    Ok(MemberView {
        region: request.region,
        shape,
        bindings,
    })
}

fn require_members(requests: &[Request]) -> Checked<()> {
    if requests.is_empty() {
        return Err(fail(
            CoverageOwner::Family,
            CoverageCause::Count {
                role: "family members",
                actual: 0,
                expected: 1,
            },
        ));
    }
    Ok(())
}
fn root_members(requests: &[Request], root: &Shape, registry: &Registry<'_>) -> Checked<()> {
    let owner = CoverageOwner::Family;
    registry::canonical(owner, root)?;
    registry::kind_check(owner, root, ShapeEnum::Compound)?;
    let roots: Vec<_> = requests
        .iter()
        .map(|r| BoundUse::forward(expected::region(r, HistoryRole::Solid)))
        .collect();
    members::uses(owner, root, ShapeEnum::Solid, &roots, registry)?;
    Ok(())
}
