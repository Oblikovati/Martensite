//! One frame of the graduated trihedron: the native matrices of GlGraduatedTrihedron::Render
//! and the ordered draws it emits. Primitives are lowered through a scratch group as driver
//! arrays, which take no primitive-array id: native trihedron arrays are not group elements.
use super::grid_axes::{box_corner, distance_to_corner, grid_axes, view_normal};
use crate::{Graphic3dDrawGroup, PrimitiveDraw, RenderError};
use martensite_tkernel::ncollection::{Mat4, Vec3};
use martensite_tkernel::quantity::Color;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// The inputs and output of one trihedron render; `frame.render(&labels)?`.
pub(super) struct TrihedronFrame<'a> {
    /// The scratch group lowering one primitive at a time.
    pub primitives: &'a Graphic3dDrawGroup,
    pub camera: Handle<Graphic3dCamera>,
    /// `ProjectionState`: the float camera projection.
    pub projection: Mat4<f32>,
    /// `theMat`: the float world view current when the trihedron renders.
    pub world_view: Mat4<f32>,
    pub extent: [u32; 2],
    pub rendering: &'a Graphic3dRenderingParams,
    /// The fields of the view's scratch structure, which carries the label groups.
    pub fields: Graphic3dCStructureFields,
    pub data: &'a Graphic3dGraduatedTrihedron,
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
    /// The `max - min` extent the axis lines were last initialized with, if they were.
    pub lines: Option<Vec3<f32>>,
    pub draws: Vec<PrimitiveDraw>,
}

impl TrihedronFrame<'_> {
    /// Native `Render`: grid, axes, then per axis its tickmarks, name and values.
    /// Example: `frame.render(&label_groups)?; let draws = frame.draws;`.
    pub fn render(&mut self, labels: &[Handle<dyn Graphic3dGroup>; 3]) -> Result<(), RenderError> {
        let (mut normal, dpix) = view_normal(&self.world_view, &self.projection, self.viewport());
        normal.normalize();
        let center = (self.min + self.max) * 0.5;
        let corners: [f32; 8] = std::array::from_fn(|index| {
            distance_to_corner(normal, center, box_corner(index, self.min, self.max))
        });
        let (grid, edges) = grid_axes(&corners, self.min, self.max);
        if self.data.to_draw_grid {
            self.grid_edges(edges)?;
            for index in 0..3 {
                self.grid_plane(index, &grid)?;
            }
        }
        if self.data.to_draw_axes {
            for index in 0..3 {
                self.axis(index)?;
            }
        }
        for (index, group) in labels.iter().enumerate() {
            self.tickmark_labels(index, &grid, dpix, group)?;
        }
        Ok(())
    }

    /// `[0, 0, width, height]`, the context viewport.
    pub fn viewport(&self) -> [i32; 4] {
        [0, 0, self.extent[0] as i32, self.extent[1] as i32]
    }

    /// Lower an unlit-by-default primitive of the default aspects in one colour.
    pub fn lower(
        &self,
        kind: Graphic3dTypeOfPrimitiveArray,
        points: &[Vec3<f32>],
        color: Color,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let array = Graphic3dArrayOfPrimitives::new(kind, points.len() as i32, 0, 0, 0)
            .map_err(|error| crate::error::rejected(error, "a trihedron primitive array"))?;
        for point in points {
            array
                .add_vertex(point.x, point.y, point.z)
                .map_err(|error| crate::error::rejected(error, "a trihedron vertex"))?;
        }
        let aspect = Handle::new(Graphic3dAspects::default());
        aspect.set_interior_color(color);
        let group = self.primitives;
        group.clear(false);
        group.set_group_primitives_aspect(&aspect.cast());
        group.add_driver_primitive_array(&array);
        self.group_draws(group)
    }

    /// The draws of a group rendered with the frame's camera, viewport and parameters.
    pub fn group_draws(
        &self,
        group: &Graphic3dDrawGroup,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let clipping = Handle::default();
        group.draws_in_structure(
            &self.camera,
            Some(self.extent),
            &self.fields,
            self.rendering,
            &clipping,
        )
    }

    /// One native `Render` call: the lowered draws under `P * world_view * model`.
    pub fn place(
        &mut self,
        lowered: &[PrimitiveDraw],
        world_view: &Mat4<f32>,
        model: Option<&Mat4<f32>>,
    ) {
        let model = model.copied().unwrap_or_else(Mat4::identity);
        let clip = (self.projection * *world_view * model).data();
        for draw in lowered {
            let mut placed = draw.clone();
            placed.clip_transform = clip;
            self.draws.push(placed);
        }
    }
}
