//! GlGraduatedTrihedron::renderLine, renderGridPlane, renderAxis and the tickmarks of
//! renderTickmarkLabels. Each native `Render` of a shared primitive under a
//! translated or scaled world-view matrix becomes one placed copy of the lowered primitive whose
//! clip matrix is `P * W' * M`, composed in float like the native shader uniforms.
use super::grid_axes::{GridAxes, box_corner, edge};
use super::trihedron_frame::TrihedronFrame;
use crate::{PrimitiveDraw, RenderError};
use martensite_tkernel::ncollection::{Mat4, Vec3};
use martensite_tkernel::quantity::Color;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::transform_utils::{project, scale, translate, un_project};
use martensite_tkservice::graphic3d::*;

/// The axis directions of `myAxes`.
pub(super) const DIRECTIONS: [[f32; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
/// The arrow plane normals passed to `InitArrow` for X, Y and Z.
const ARROW_NORMALS: [[f32; 3]; 3] = [[0., 0., 1.], [0., 0., 1.], [1., 0., 0.]];
/// The grid edges in native drawing order: edge flag, axis line and corner of its start.
const GRID_EDGES: [(u16, usize, usize); 9] = [
    (edge::XOO_XYO, 1, 4),
    (edge::XOO_XOZ, 2, 4),
    (edge::OYO_OYZ, 2, 2),
    (edge::OYO_XYO, 0, 2),
    (edge::OOZ_XOZ, 0, 1),
    (edge::OOZ_OYZ, 1, 1),
    (edge::OYZ_XYZ, 0, 3),
    (edge::XOZ_XYZ, 1, 5),
    (edge::XYO_XYZ, 2, 6),
];

pub(super) fn direction(index: usize) -> Vec3<f32> {
    let [x, y, z] = DIRECTIONS[index];
    Vec3::new(x, y, z)
}

fn component(vector: Vec3<f32>, index: usize) -> f32 {
    [vector.x, vector.y, vector.z][index]
}

/// `Translate` by a vector; `translated(&m, offset)`.
fn translated(matrix: &Mat4<f32>, offset: Vec3<f32>) -> Mat4<f32> {
    let mut moved = *matrix;
    translate(&mut moved, [offset.x, offset.y, offset.z]);
    moved
}

impl TrihedronFrame<'_> {
    /// The grid edges seen from the grid corner (native `Render` with `ToDrawGrid`).
    pub fn grid_edges(&mut self, edges: u16) -> Result<(), RenderError> {
        for (flag, axis, corner) in GRID_EDGES {
            if edges & flag == 0 {
                continue;
            }
            let line = self.axis_line(axis, self.data.grid_color)?;
            let start = box_corner(corner, self.min, self.max);
            self.place(&line, &translated(&self.world_view, start), None);
        }
        Ok(())
    }

    /// `renderGridPlane`: the grid lines across the two other axes at each tickmark of `index`.
    pub fn grid_plane(&mut self, index: usize, grid: &GridAxes) -> Result<(), RenderError> {
        let ticks = self.data.axes[index].tickmarks_number;
        if ticks <= 0 {
            return Ok(());
        }
        let step = self.tick_step(index, grid);
        let step_vector = direction(index) * step;
        let first = if self.data.to_draw_axes { 1 } else { 0 };
        for turn in 1..=2 {
            let other = (index + turn) % 3;
            let line = self.axis_line(other, self.data.grid_color)?;
            let mut start = grid.origin;
            if component(grid.axes[other], other) < 0.0 {
                set_component(&mut start, other, component(self.min, other));
            }
            let mut matrix = translated(&self.world_view, start);
            for _ in first..ticks {
                matrix = translated(&matrix, step_vector);
                self.place(&line, &matrix, None);
            }
        }
        Ok(())
    }

    /// The signed tick spacing of an axis: `Axes[i][i] * (max - min)[i] / TickmarksNumber`.
    pub fn tick_step(&self, index: usize, grid: &GridAxes) -> f32 {
        let span = component(self.max, index) - component(self.min, index);
        component(grid.axes[index], index) * span / self.data.axes[index].tickmarks_number as f32
    }

    /// `renderAxis`: the zoom-persistent arrow at the axis end, then the axis line scaled to
    /// reach the arrow base.
    pub fn axis(&mut self, index: usize) -> Result<(), RenderError> {
        let color = self.data.axes[index].color;
        let span = self.max - self.min;
        let arrow_point = self.min + direction(index) * span;
        let model = self.zoom_persistence(arrow_point)?;
        if let Some(arrow) = self.arrow(index, color)? {
            let world_view = self.world_view;
            self.place(&arrow, &world_view, Some(&model));
        }
        let start = self.arrow_start(index, &model);
        let mut matrix = translated(&self.world_view, self.min);
        let factor =
            ((start - self.min) * direction(index)).modulus() / (direction(index) * span).modulus();
        let factors = direction(index) * factor;
        scale(&mut matrix, [factors.x, factors.y, factors.z]);
        let line = self.axis_line(index, color)?;
        self.place(&line, &matrix, None);
        Ok(())
    }

    /// The tickmarks of an axis along its tick edge (renderTickmarkLabels).
    pub fn tickmarks(
        &mut self,
        index: usize,
        grid: &GridAxes,
        outward: Vec3<f32>,
        dpix: f32,
    ) -> Result<(), RenderError> {
        let aspect = &self.data.axes[index];
        if !aspect.to_draw_tickmarks || aspect.tickmarks_number <= 0 {
            return Ok(());
        }
        let tip = outward * aspect.tickmarks_length as f32 * dpix;
        let ticks = aspect.tickmarks_number;
        let tick = self.segment(Vec3::new(0., 0., 0.), tip, self.data.grid_color)?;
        let step_vector = direction(index) * self.tick_step(index, grid);
        let mut matrix = translated(&self.world_view, grid.ticks[index]);
        for _ in 0..=ticks {
            self.place(&tick, &matrix, None);
            matrix = translated(&matrix, step_vector);
        }
        Ok(())
    }

    /// The model matrix of `Graphic3dTransformPers(Graphic3dTMF_ZoomPers, point).Compute`.
    fn zoom_persistence(&self, point: Vec3<f32>) -> Result<Mat4<f32>, RenderError> {
        let anchor = Point3::new(f64::from(point.x), f64::from(point.y), f64::from(point.z));
        let persistence =
            Graphic3dTransformPers::with_point(Graphic3dTransModeFlags::ZOOM_PERS, &anchor)
                .map_err(|error| crate::error::rejected(error, "a zoom persistence anchor"))?;
        let size = self.extent.map(|value| value as i32);
        persistence
            .compute(
                &self.camera,
                &self.projection,
                &self.world_view,
                size,
                false,
            )
            .map_err(|error| crate::error::rejected(error, "a computable zoom persistence"))
    }

    /// The world point under the window projection of the arrow base, through the plain
    /// world view (a failed projection keeps the native zero point).
    fn arrow_start(&self, index: usize, model: &Mat4<f32>) -> Vec3<f32> {
        let viewport = self.viewport();
        let end = -direction(index) * self.data.arrows_length;
        let model_view = self.world_view * *model;
        let window = project(
            [end.x, end.y, end.z],
            &model_view,
            &self.projection,
            viewport,
        )
        .unwrap_or([0.0; 3]);
        let start =
            un_project(window, &self.world_view, &self.projection, viewport).unwrap_or([0.0; 3]);
        Vec3::new(start[0], start[1], start[2])
    }

    /// `InitArrow`: the closed arrow head polyline, none when the axis lies on the normal.
    fn arrow(&self, index: usize, color: Color) -> Result<Option<Vec<PrimitiveDraw>>, RenderError> {
        let length = self.data.arrows_length;
        let [x, y, z] = ARROW_NORMALS[index];
        let along = -direction(index) * length;
        let radial = Vec3::cross(&direction(index), &Vec3::new(x, y, z));
        if radial.modulus() < martensite_tkernel::precision::confusion() as f32 {
            return Ok(None);
        }
        let radial = radial.normalized() * length * 0.2;
        let points = [
            radial + along,
            Vec3::new(0., 0., 0.),
            -radial + along,
            radial + along,
        ];
        let kind = Graphic3dTypeOfPrimitiveArray::Polylines;
        self.lower(kind, &points, color).map(Some)
    }

    /// `InitLine` of an axis over the extent the lines were initialized with.
    fn axis_line(&self, index: usize, color: Color) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let Some(extent) = self.lines else {
            // Lines stay uninitialized until the cubic axes callback ran (native IsInitialized).
            return Ok(Vec::new());
        };
        let tip = direction(index) * component(extent, index);
        self.segment(Vec3::new(0., 0., 0.), tip, color)
    }

    fn segment(
        &self,
        from: Vec3<f32>,
        to: Vec3<f32>,
        color: Color,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        self.lower(Graphic3dTypeOfPrimitiveArray::Segments, &[from, to], color)
    }
}

fn set_component(vector: &mut Vec3<f32>, index: usize, value: f32) {
    match index {
        0 => vector.x = value,
        1 => vector.y = value,
        _ => vector.z = value,
    }
}
