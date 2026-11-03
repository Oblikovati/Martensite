//! Discovery retains owners; complete reference closure is checked under every final lease.
use super::{
    cells::{Cell, Lease, owner_cells},
    owners::{self, Owner},
    types::{FaceCaptureIncomplete as Incomplete, FaceCaptureLimits},
};
use crate::topods::Face;
use std::collections::BTreeMap;

pub(super) struct Inventory {
    pub owners: Vec<Owner>,
    links: Vec<Vec<usize>>,
}
impl Inventory {
    pub(super) fn gather(face: &Face, limits: FaceCaptureLimits) -> Result<Self, Incomplete> {
        let mut result = Self {
            owners: vec![Owner::Topology(face.tshape().clone())],
            links: Vec::new(),
        };
        let mut index = 0;
        while index < result.owners.len() {
            result.discover(index, limits)?;
            index += 1;
        }
        Ok(result)
    }
    fn discover(&mut self, index: usize, limits: FaceCaptureLimits) -> Result<(), Incomplete> {
        let refs =
            owner_references(&self.owners[index], limits).map_err(|error| error.at_owner(index))?;
        self.links.push(refs.iter().map(Owner::key).collect());
        for reference in refs {
            if self
                .owners
                .iter()
                .any(|owner| owner.key() == reference.key())
            {
                continue;
            }
            check_limit(self.owners.len() + 1, limits.owners, "owners")?;
            self.owners.push(reference);
        }
        Ok(())
    }
    pub(super) fn cells(
        &self,
        limits: FaceCaptureLimits,
    ) -> Result<Vec<(usize, Cell<'_>)>, Incomplete> {
        let mut cells = BTreeMap::new();
        for (index, owner) in self.owners.iter().enumerate() {
            for cell in owner_cells(owner).map_err(|error| error.at_owner(index))? {
                cells.entry(cell.key()).or_insert((index, cell));
                check_limit(cells.len(), limits.cells, "cells")?;
            }
        }
        Ok(cells.into_values().collect())
    }
    pub(super) fn revalidate(
        &self,
        leases: &[(usize, Lease<'_>)],
        limits: FaceCaptureLimits,
    ) -> Result<(), Incomplete> {
        for index in 0..self.owners.len() {
            let actual = references_for_owner(index, leases, limits)?;
            if actual
                .iter()
                .map(Owner::key)
                .ne(self.links[index].iter().copied())
            {
                return Err(Incomplete::Changed { owner: index });
            }
        }
        Ok(())
    }
}

pub(super) fn check_limit(
    actual: usize,
    maximum: usize,
    dimension: &'static str,
) -> Result<(), Incomplete> {
    if actual > maximum {
        return Err(Incomplete::Limit {
            actual,
            maximum,
            dimension,
        });
    }
    Ok(())
}

fn owner_references(owner: &Owner, limits: FaceCaptureLimits) -> Result<Vec<Owner>, Incomplete> {
    let mut cells = owner_cells(owner)?;
    cells.sort_by_key(Cell::key);
    let mut leases = cells
        .iter()
        .map(|cell| cell.acquire(owner.key()))
        .collect::<Result<Vec<_>, _>>()?;
    leases.sort_by_key(super::cells::lease_order);
    let mut result = Vec::new();
    for lease in &leases {
        append_references(lease, &mut result, limits)?;
    }
    Ok(result)
}
fn references_for_owner(
    index: usize,
    leases: &[(usize, Lease<'_>)],
    limits: FaceCaptureLimits,
) -> Result<Vec<Owner>, Incomplete> {
    let mut selected: Vec<_> = leases
        .iter()
        .filter(|(owner, _)| *owner == index)
        .map(|(_, lease)| lease)
        .collect();
    selected.sort_by_key(|lease| super::cells::lease_order(lease));
    let mut result = Vec::new();
    for lease in selected {
        append_references(lease, &mut result, limits).map_err(|error| error.at_owner(index))?;
    }
    Ok(result)
}

fn append_references(
    lease: &Lease<'_>,
    result: &mut Vec<Owner>,
    limits: FaceCaptureLimits,
) -> Result<(), Incomplete> {
    match lease {
        Lease::Topology(fields) => topology_references(&fields.shapes, result, limits)?,
        Lease::Face(fields) => result.push(owners::surface_owner(&fields.surface)?),
        Lease::Edge(fields) => edge_references(&fields.curves, result, limits)?,
        _ => append_geometry_references(lease, result)?,
    }
    check_limit(result.len(), limits.owners, "references")
}
fn topology_references(
    shapes: &[crate::topods::Shape],
    result: &mut Vec<Owner>,
    limits: FaceCaptureLimits,
) -> Result<(), Incomplete> {
    check_limit(shapes.len(), limits.owners, "children")?;
    result.extend(
        shapes
            .iter()
            .map(|shape| Owner::Topology(shape.tshape().clone())),
    );
    Ok(())
}
fn edge_references(
    curves: &[martensite_tkernel::standard::Handle<dyn crate::brep::BRepCurveRepresentation>],
    result: &mut Vec<Owner>,
    limits: FaceCaptureLimits,
) -> Result<(), Incomplete> {
    check_limit(curves.len(), limits.owners, "representations")?;
    for curve in curves {
        result.push(owners::representation_owner(curve)?);
    }
    Ok(())
}

fn append_geometry_references(
    lease: &Lease<'_>,
    result: &mut Vec<Owner>,
) -> Result<(), Incomplete> {
    match lease {
        Lease::PCurve(fields) => {
            result.push(owners::planar_owner(&fields.pcurve)?);
            result.push(owners::surface_owner(&fields.surface)?);
        }
        Lease::Seam(fields) => result.push(owners::planar_owner(&fields.pcurve2)?),
        Lease::Spatial(curve) => result.push(owners::spatial_owner(curve)?),
        Lease::Extrusion(surface) => extrusion_reference(surface, result)?,
        Lease::Vertex(fields) if !fields.points.is_empty() => {
            return Err(unsupported("nonempty vertex point representations"));
        }
        _ => {}
    }
    Ok(())
}
fn extrusion_reference(
    surface: &martensite_tkg3d::geom::ExtrusionInputLease<'_>,
    result: &mut Vec<Owner>,
) -> Result<(), Incomplete> {
    if surface.has_evaluation_override() {
        return Err(unsupported("surface evaluation override"));
    }
    result.push(owners::spatial_owner(&surface.basis())?);
    Ok(())
}

fn unsupported(actual: &str) -> Incomplete {
    Incomplete::Unsupported {
        owner: 0,
        actual: actual.into(),
        expected: "stored evaluator and empty vertex point representations",
    }
}
