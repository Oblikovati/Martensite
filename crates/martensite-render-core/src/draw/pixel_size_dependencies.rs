//! The draw state whose pixels depend on the window pixel size, named for the supersampled
//! grid control of the command-host harness (user decision 2026-10-28: the control is valid only where
//! nothing in the shading depends on screen derivatives, line widths or point sizes).
use super::{LineStyle, PrimitiveDraw};
use crate::RenderShading;

/// Whether a line style draws a screen-space dash pattern (anything but the solid 0xFFFF).
fn stippled(style: Option<LineStyle>) -> bool {
    style.is_some_and(|s| s.parameters().1 != u16::MAX)
}

impl PrimitiveDraw {
    /// Testing only, not product API: the names of this draw's pixel-size dependencies, in a
    /// fixed order. `lines`: line widths (line draws, face-program polygon lines, projected
    /// dashes); `points`: point sizes and markers; `polygon_edges`: polygon-mode edges;
    /// `stipple`: screen-space dash patterns; `mesh_edges`: edge widths in pixels;
    /// `derivatives`: faceted normals from `dFdx`/`dFdy`; `textures`: texture and glyph
    /// lookups (implicit derivatives, tangent-space normals); `hatch`: window-aligned hatch
    /// rows. Example: `draw.pixel_size_dependencies().is_empty()`.
    #[doc(hidden)]
    pub fn pixel_size_dependencies(&self) -> Vec<&'static str> {
        let facet = self
            .lighting
            .as_ref()
            .is_some_and(|l| l.shading() == RenderShading::PhongFacet);
        let lines = self.line_style.is_some() || self.polygon_face_lines;
        let named = [
            ("lines", lines || self.projected_dash.is_some()),
            ("points", self.point_style.is_some()),
            ("polygon_edges", self.polygon_edge_style.is_some()),
            (
                "stipple",
                stippled(self.line_style) || stippled(self.polygon_edge_style),
            ),
            ("mesh_edges", self.mesh_edges.is_some()),
            ("derivatives", facet),
            (
                "textures",
                self.textured_style.is_some() || self.surface_texture.is_some(),
            ),
            ("hatch", self.hatch_rows.is_some()),
        ];
        let present = named.into_iter().filter(|(_, present)| *present);
        present.map(|(name, _)| name).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        LineStyle, PrimitiveDraw, RenderLighting, RenderMaterial, RenderMeshEdges, RenderShading,
        RenderVertex, identity_clip,
    };

    fn vertex(position: [f32; 3]) -> RenderVertex {
        RenderVertex {
            position,
            weight: 1.,
            color: [1.; 4],
        }
    }

    fn triangle() -> PrimitiveDraw {
        let vertices = vec![vertex([0.; 3]), vertex([1., 0., 0.]), vertex([0., 1., 0.])];
        PrimitiveDraw::new(vertices, vec![0, 1, 2], identity_clip()).unwrap()
    }

    fn lit(shading: RenderShading) -> PrimitiveDraw {
        let normals = vec![[0., 0., 1.]; 3];
        let material = RenderMaterial::default();
        let lighting = RenderLighting::new(
            normals,
            material,
            identity_clip(),
            identity_clip(),
            false,
            shading,
        );
        triangle().with_lighting(lighting.unwrap()).unwrap()
    }

    #[test]
    fn smooth_shaded_triangles_have_no_pixel_size_dependency() {
        assert!(triangle().pixel_size_dependencies().is_empty());
        assert!(
            lit(RenderShading::Phong)
                .pixel_size_dependencies()
                .is_empty()
        );
        assert!(
            lit(RenderShading::Gouraud)
                .pixel_size_dependencies()
                .is_empty()
        );
    }

    #[test]
    fn facets_lines_stipple_and_mesh_edges_are_named() {
        assert_eq!(
            lit(RenderShading::PhongFacet).pixel_size_dependencies(),
            ["derivatives"]
        );
        let ends = vec![vertex([0.; 3]), vertex([1., 0., 0.])];
        let solid = LineStyle::new(1., u16::MAX, 1).unwrap();
        let line = PrimitiveDraw::segments(ends.clone(), vec![0, 1], identity_clip(), solid);
        assert_eq!(line.unwrap().pixel_size_dependencies(), ["lines"]);
        let dashed = LineStyle::new(1., 0xffc0, 1).unwrap();
        let line = PrimitiveDraw::segments(ends, vec![0, 1], identity_clip(), dashed);
        assert_eq!(
            line.unwrap().pixel_size_dependencies(),
            ["lines", "stipple"]
        );
        let edges = RenderMeshEdges::new([0., 0., 0., 1.], 1., false).unwrap();
        let meshed = triangle().with_mesh_edges(edges).unwrap();
        assert_eq!(meshed.pixel_size_dependencies(), ["mesh_edges"]);
    }
}
