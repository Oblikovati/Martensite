// SPDX-License-Identifier: GPL-2.0-only
//! Exact native incidence and source lookups for accepted cap edges.
use super::*;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use std::collections::BTreeMap;

pub(in super::super::super) struct SideRows {
    index: IndexedMapOfShape,
    links: Vec<Option<(u64, usize)>>,
}
impl SideRows {
    pub(in super::super::super) fn capture(sides: &[Side]) -> Self {
        let mut index = IndexedMapOfShape::default();
        let mut links = Vec::new();
        for side in sides {
            for (row, edge) in side.edges[..2].iter().enumerate() {
                let key = index.add(edge.clone());
                if key > links.len() {
                    links.push(Some((side.id, row)));
                } else {
                    links[key - 1] = None;
                }
            }
        }
        Self { index, links }
    }
    pub(super) fn find(&self, edge: &Shape) -> Result<(u64, usize), Failure> {
        let key = self.index.find_index(edge);
        key.checked_sub(1)
            .and_then(|index| self.links[index])
            .ok_or_else(|| rejected("cap edge requires exactly one actual side-row incidence"))
    }
}

pub(super) struct CapCurves<'a> {
    pub(super) material: &'a CertifiedPlanarMaterial,
    by_side: BTreeMap<u64, Option<ProfileCurveId>>,
}
impl<'a> CapCurves<'a> {
    pub(super) fn new(
        material: &'a CertifiedPlanarMaterial,
        curves: &[(ProfileCurveId, u64)],
    ) -> Self {
        let mut by_side = BTreeMap::new();
        for &(curve, side) in curves {
            by_side
                .entry(side)
                .and_modify(|old| *old = None)
                .or_insert(Some(curve));
        }
        Self { material, by_side }
    }
    pub(super) fn find(&self, side: u64) -> Result<(ProfileCurveId, &'a ProfileCurve), Failure> {
        let key = self
            .by_side
            .get(&side)
            .copied()
            .flatten()
            .ok_or_else(|| rejected("missing/ambiguous source curve-to-side binding"))?;
        let curve = self
            .material
            .curve(key)
            .ok_or_else(|| rejected("selected material curve missing"))?;
        Ok((key, curve))
    }
}

pub(super) struct WireEdges {
    index: IndexedMapOfShape,
    states: Vec<WireUse>,
}
#[derive(PartialEq)]
enum WireUse {
    Unvisited,
    Visited,
    Ambiguous,
}
impl WireEdges {
    pub(super) fn capture(edges: &[Shape]) -> Self {
        let mut index = IndexedMapOfShape::default();
        let mut states = Vec::new();
        for edge in edges {
            let key = index.add(edge.clone());
            if key > states.len() {
                states.push(WireUse::Unvisited);
            } else {
                states[key - 1] = WireUse::Ambiguous;
            }
        }
        Self { index, states }
    }
    pub(super) fn visit(&mut self, edge: &Shape) -> Result<(), Failure> {
        let slot = self.index.find_index(edge);
        let state = slot.checked_sub(1).map(|index| &mut self.states[index]);
        match state {
            Some(state @ WireUse::Unvisited) => {
                *state = WireUse::Visited;
                Ok(())
            }
            Some(WireUse::Visited) => Err(rejected("repeated actual cap wire edge")),
            _ => Err(rejected("ambiguous actual native cap edge occurrence")),
        }
    }
}
