//! Resolve the caller's directed outer and hole loops against the exact planar region.
use super::*;
use martensite_tkbo::profile_extrusion::PlanarRegion;
use std::collections::{BTreeMap, BTreeSet};

type Use<'a> = (LoopUse, &'a ProfileCurve);
pub(super) type VertexMap = BTreeMap<ProfileVertexId, [f64; 2]>;

pub(super) fn selected<'a>(
    profile: &SketchProfile,
    selections: &'a [WrappedRegionSelection],
) -> Checked<&'a WrappedRegionSelection> {
    let [selection] = selections else {
        return Err(invalid(
            format!("selected regions={}", selections.len()),
            "one requested rational material region",
        ));
    };
    let loops = boundaries(selection);
    let total: usize = loops.map(|boundary| boundary.uses.len()).sum();
    if total > 64 || boundaries(selection).any(|boundary| boundary.uses.is_empty()) {
        return Err(invalid(
            format!(
                "selected directed uses={total}, holes={}",
                selection.holes.len()
            ),
            "one through 64 directed uses and nonempty outer and hole boundaries",
        ));
    }
    if profile.plane != ProfilePlane::world_xy() {
        return Err(invalid(
            format!("authored plane={:?}", profile.plane),
            "world-XY developed coordinates",
        ));
    }
    Ok(selection)
}

pub(super) fn resolve_uses<'a>(
    selection: &WrappedRegionSelection,
    profile: &'a SketchProfile,
) -> Checked<Vec<Use<'a>>> {
    let mut curves = BTreeMap::new();
    for curve in &profile.curves {
        if curves.insert(curve.id(), curve).is_some() {
            return Err(invalid(
                format!("duplicate profile curve={:?}", curve.id()),
                "unique authored curve identifiers",
            ));
        }
    }
    let mut seen = BTreeSet::new();
    boundaries(selection)
        .flat_map(|boundary| &boundary.uses)
        .map(|&used| {
            let curve = curves
                .get(&used.curve)
                .copied()
                .filter(|_| seen.insert(used.curve))
                .ok_or_else(|| {
                    invalid(
                        format!("missing or repeated selected curve={:?}", used.curve),
                        "one existing curve per selected use",
                    )
                })?;
            Ok((used, curve))
        })
        .collect()
}

pub(super) fn vertices(profile: &SketchProfile) -> Checked<VertexMap> {
    let mut indexed = BTreeMap::new();
    for vertex in &profile.vertices {
        if indexed.insert(vertex.id, vertex.point).is_some() {
            return Err(invalid(
                format!("duplicate profile vertex={:?}", vertex.id),
                "unique authored vertex identifiers",
            ));
        }
    }
    Ok(indexed)
}

pub(super) fn point(vertices: &VertexMap, vertex: ProfileVertexId) -> Checked<[f64; 2]> {
    vertices
        .get(&vertex)
        .copied()
        .filter(|point| point.iter().all(|coordinate| coordinate.is_finite()))
        .ok_or_else(|| {
            invalid(
                format!(
                    "profile vertex={vertex:?}, position={:?}",
                    vertices.get(&vertex)
                ),
                "one finite authored endpoint",
            )
        })
}

pub(super) fn selected_profile(uses: &[Use<'_>], vertices: &VertexMap) -> Checked<SketchProfile> {
    let ids: BTreeSet<_> = uses
        .iter()
        .flat_map(|(_, c)| c.ends().into_iter().flatten())
        .collect();
    let vertices = ids
        .into_iter()
        .map(|id| {
            Ok(ProfileVertex {
                id,
                point: point(vertices, id)?,
            })
        })
        .collect::<Checked<_>>()?;
    let curves = uses.iter().map(|(_, curve)| (**curve).clone()).collect();
    Ok(SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    })
}

pub(in super::super) fn select_cell(
    selection: &WrappedRegionSelection,
    regions: &[PlanarRegion],
) -> Checked<()> {
    let count = regions
        .iter()
        .filter(|region| matches_cell(selection, region))
        .count();
    if count == 1 {
        return Ok(());
    }
    Err(invalid(
        format!(
            "selected region={:?}, matching planar regions={count}",
            selection.region
        ),
        "exactly one certified region with all original directed boundaries",
    ))
}

fn matches_cell(selection: &WrappedRegionSelection, region: &PlanarRegion) -> bool {
    same_cycle(&selection.outer, region.outer())
        && selection.holes.len() == region.holes().len()
        && selection
            .holes
            .iter()
            .all(|wanted| region.holes().iter().any(|found| same_cycle(wanted, found)))
}

fn same_cycle(wanted: &ProfileLoop, actual: &ProfileLoop) -> bool {
    let Some(first) = wanted.uses.first() else {
        return false;
    };
    wanted.uses.len() == actual.uses.len()
        && actual
            .uses
            .iter()
            .position(|used| used == first)
            .is_some_and(|at| {
                actual
                    .uses
                    .iter()
                    .cycle()
                    .skip(at)
                    .take(wanted.uses.len())
                    .eq(&wanted.uses)
            })
}

fn boundaries(selection: &WrappedRegionSelection) -> impl Iterator<Item = &ProfileLoop> {
    std::iter::once(&selection.outer).chain(&selection.holes)
}
