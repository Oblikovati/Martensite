//! Stable meshlet construction for the indexed surface path (#594).
use super::meshlet_range::MeshletRange;
use crate::RenderVertex;
use std::collections::HashMap;

pub(super) const MAX_VERTICES: usize = 64;
pub(super) const MAX_TRIANGLES: usize = 96;

#[derive(Default)]
pub(super) struct MeshletGeometry {
    pub(super) groups: Vec<MeshletRange>,
    pub(super) vertices: Vec<u32>,
    pub(super) triangles: Vec<[u8; 3]>,
}

pub(super) fn build(vertices: &[RenderVertex], indices: &[u32]) -> Option<MeshletGeometry> {
    if indices.is_empty() || !indices.len().is_multiple_of(3) || !valid_vertices(vertices) {
        return None;
    }
    let mut result = MeshletGeometry::default();
    let mut part = MeshletPart::default();
    for triangle in indices.as_chunks::<3>().0 {
        if triangle
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            return None;
        }
        if !part.fits(*triangle) {
            part.finish(&mut result, vertices);
        }
        part.push(*triangle);
    }
    part.finish(&mut result, vertices);
    Some(result)
}

fn valid_vertices(vertices: &[RenderVertex]) -> bool {
    !vertices.is_empty()
        && vertices
            .iter()
            .all(|vertex| vertex.weight == 1. && vertex.position.into_iter().all(f32::is_finite))
}

#[derive(Default)]
struct MeshletPart {
    ordinals: HashMap<u32, u8>,
    vertices: Vec<u32>,
    triangles: Vec<[u8; 3]>,
}

impl MeshletPart {
    fn fits(&self, triangle: [u32; 3]) -> bool {
        let new_vertices = triangle
            .iter()
            .enumerate()
            .filter(|&(index, vertex)| {
                !self.ordinals.contains_key(vertex) && !triangle[..index].contains(vertex)
            })
            .count();
        self.triangles.len() < MAX_TRIANGLES && self.vertices.len() + new_vertices <= MAX_VERTICES
    }

    fn push(&mut self, triangle: [u32; 3]) {
        let ordinals = triangle.map(|vertex| {
            *self.ordinals.entry(vertex).or_insert_with(|| {
                let index = self.vertices.len() as u8;
                self.vertices.push(vertex);
                index
            })
        });
        self.triangles.push(ordinals);
    }

    fn finish(&mut self, output: &mut MeshletGeometry, geometry: &[RenderVertex]) {
        let range = MeshletRange::capture(
            (output.vertices.len(), output.triangles.len()),
            &self.vertices,
            &self.triangles,
            geometry,
        );
        output.groups.push(range);
        output.vertices.append(&mut self.vertices);
        output.triangles.append(&mut self.triangles);
        self.ordinals.clear();
    }
}

#[cfg(test)]
#[path = "meshlet_geometry_tests.rs"]
mod tests;
