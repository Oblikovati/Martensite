//! Shared topology of rendered triangulation edges.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::poly::{Connect, Triangulation};

/// The edges of a triangulation as node pairs: the free edges (one adjacent
/// triangle) and the internal edges (listed by the triangle of lower index).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MeshEdgeSets {
    /// `myFree`: node pairs of the free edges.
    pub free: Vec<(i32, i32)>,
    /// `myInternals`: node pairs of the internal edges.
    pub internal: Vec<(i32, i32)>,
}

impl MeshEdgeSets {
    /// Classifies edge incidences; `let edges = MeshEdgeSets::compute(&mesh)?;`.
    /// The edges the constructor computes with `PolyConnect`: for each triangle and
    /// each side `j` (nodes `j`, `j + 1`), a free edge when there is no adjacent
    /// triangle, an internal edge when the adjacent triangle has a higher index.
    pub fn compute(triangulation: &Handle<Triangulation>) -> Result<Self, Failure> {
        let mut edges = Self::default();
        let Some(mesh) = triangulation.get() else {
            return Ok(edges);
        };
        let connect = Connect::new(triangulation)?;
        for index in 1..=mesh.nb_triangles() {
            edges.classify_triangle(mesh, &connect, index)?;
        }
        Ok(edges)
    }

    fn classify_triangle(
        &mut self,
        mesh: &Triangulation,
        connect: &Connect,
        index: i32,
    ) -> Result<(), Failure> {
        let (t1, t2, t3) = connect.triangles(index)?;
        let (n1, n2, n3) = mesh.triangle(index)?.get();
        let (adjacent, nodes) = ([t1, t2, t3], [n1, n2, n3]);
        for side in 0..3 {
            self.classify_side(index, adjacent[side], (nodes[side], nodes[(side + 1) % 3]));
        }
        Ok(())
    }

    fn classify_side(&mut self, index: i32, adjacent: i32, side: (i32, i32)) {
        if adjacent == 0 {
            self.free.push(side);
            return;
        }
        if index < adjacent {
            self.internal.push(side);
        }
    }
}
