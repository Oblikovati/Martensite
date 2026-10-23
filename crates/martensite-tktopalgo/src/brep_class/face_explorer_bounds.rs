//! Reuse geometry bounds only within a newly constructed classification operation.
use super::{BRepClassFaceExplorer, BndBoxState};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::bnd::BoundingBox2;

impl BRepClassFaceExplorer {
    // #242: the boxes already contain the projected curves; never cache in topology.
    pub(in crate::brep_class) fn reuse_planar_operation_bounds(&mut self) {
        let Some(bounds) = self.ready_planar_operation_bounds() else {
            return;
        };
        let Ok([xmin, ymin, xmax, ymax]) = bounds.get() else {
            return;
        };
        (self.my_u_min, self.my_u_max, self.my_v_min, self.my_v_max) = (xmin, xmax, ymin, ymax);
    }

    fn ready_planar_operation_bounds(&self) -> Option<BoundingBox2> {
        let (surface, _) = BRepTool::surface_with_location(&self.my_face).ok()?;
        let plane_handle = surface.cast::<GeomPlane>();
        let plane = plane_handle.get()?;
        if self.my_probe_edges.is_empty() {
            return None;
        }
        let natural = plane.bounds();
        let mut result = BoundingBox2::default();
        for index in &self.my_probe_edges {
            let edge = &self.my_edges[*index];
            if edge.bounding_box_state() != BndBoxState::Ready {
                return None;
            }
            let coordinates = edge.bounding_box().get().ok()?;
            append_natural_plane_box(&mut result, coordinates, natural)?;
        }
        Some(result)
    }
}

fn append_natural_plane_box(
    result: &mut BoundingBox2,
    coordinates: [f64; 4],
    natural: (f64, f64, f64, f64),
) -> Option<()> {
    let [xmin, ymin, xmax, ymax] = coordinates;
    if !coordinates.iter().all(|value| value.is_finite())
        || xmin < natural.0
        || xmax > natural.1
        || ymin < natural.2
        || ymax > natural.3
    {
        return None;
    }
    // Flatten each gap before union, exactly as the general face-bounds path does.
    let mut flattened = BoundingBox2::default();
    flattened.update_bounds([xmin, ymin], [xmax, ymax]);
    result.add_box(&flattened);
    Some(())
}

#[cfg(test)]
#[path = "face_explorer_bounds_tests.rs"]
mod tests;
