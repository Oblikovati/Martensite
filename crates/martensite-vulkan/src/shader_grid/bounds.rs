//! Grid domain tests, camera-ray plane hits and the Z-fit bounds of GlShaderGrid.
use super::{GridFrame, RenderShaderGrid, angle_in_arc, is_finite_point, local_point, to_point};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkservice::aspect::AspectGridDrawMode;
use martensite_tkservice::graphic3d::Graphic3dCamera;

/// A ray hit in plane-local coordinates and its world point.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PlaneHit {
    pub local: [f64; 2],
    pub world: Xyz,
}

impl RenderShaderGrid {
    /// `isPointInBounds`: radius and arc for circular grids, half sizes for rectangular ones.
    pub(crate) fn is_point_in_bounds(&self, x: f64, y: f64) -> bool {
        let params = &self.params;
        if params.is_circular() {
            if params.radius() > 0.0 && (x * x + y * y).sqrt() > params.radius() {
                return false;
            }
            return !params.is_arc()
                || angle_in_arc(
                    params.angle_start(),
                    params.angle_end(),
                    martensite_cmath::atan2(y, x),
                );
        }
        (params.size_x() <= 0.0 || x.abs() <= params.size_x() * 0.5)
            && (params.size_y() <= 0.0 || y.abs() <= params.size_y() * 0.5)
    }
    /// `planeLocalHit`: the camera ray through an NDC point meets the grid plane.
    pub(crate) fn plane_local_hit(
        &self,
        camera: &Graphic3dCamera,
        ndc: [f64; 2],
        reject_behind: bool,
    ) -> Option<PlaneHit> {
        let frame = self.frame();
        let near_z = if camera.is_zero_to_one_depth() {
            0.0
        } else {
            -1.0
        };
        let near = camera
            .un_project(&Point3::new(ndc[0], ndc[1], near_z))
            .xyz();
        let far = camera.un_project(&Point3::new(ndc[0], ndc[1], 1.0)).xyz();
        let perspective = !camera.is_orthographic();
        let origin = if perspective {
            camera.eye().xyz()
        } else {
            near
        };
        let ray = far - origin;
        let length = ray.modulus();
        if length <= martensite_tkernel::precision::confusion() {
            return None;
        }
        let denominator = frame.normal.dot(ray);
        if (denominator / length).abs() <= martensite_tkernel::precision::angular() {
            return None;
        }
        let t = frame.normal.dot(frame.origin - origin) / denominator;
        if reject_behind && perspective && t < 0.0 {
            return None;
        }
        let world = origin + ray * t;
        let local = world - frame.origin;
        Some(PlaneHit {
            local: [local.dot(frame.x), local.dot(frame.y)],
            world,
        })
    }
    /// `AddZFitBounds`: finite bounds, or the visible footprint of an unbounded grid.
    pub fn add_z_fit_bounds(&self, graphic: &mut BoundingBox3, camera: &Graphic3dCamera) {
        let params = &self.params;
        if !self.is_shown
            || params.draw_mode() == AspectGridDrawMode::None
            || params.is_background()
        {
            return;
        }
        let mut grid = BoundingBox3::default();
        if params.is_bounded() {
            self.add_finite_bounds(&mut grid);
        } else {
            self.add_view_footprint_bounds(&mut grid, camera);
        }
        if grid.is_void() {
            self.add_finite_bounds(&mut grid);
        }
        if !grid.is_void() {
            graphic.add_box(&grid);
        }
    }
    fn add_view_footprint_bounds(&self, bounds: &mut BoundingBox3, camera: &Graphic3dCamera) {
        let samples = [
            [-1.0, -1.0],
            [1.0, -1.0],
            [1.0, 1.0],
            [-1.0, 1.0],
            [0.0, 0.0],
        ];
        for sample in samples {
            let Some(hit) = self.plane_local_hit(camera, sample, true) else {
                continue;
            };
            if self.is_point_in_bounds(hit.local[0], hit.local[1]) && is_finite_point(hit.world) {
                bounds.add_point(to_point(hit.world));
            }
        }
    }
    fn add_finite_bounds(&self, bounds: &mut BoundingBox3) {
        let frame = self.frame();
        add_local(bounds, &frame, 0.0, 0.0);
        if self.params.is_circular() {
            self.add_circular_bounds(bounds, &frame);
            return;
        }
        let half = |size: f64| if size > 0.0 { size * 0.5 } else { 0.0 };
        let (x, y) = (half(self.params.size_x()), half(self.params.size_y()));
        let corners: &[[f64; 2]] = match (x > 0.0, y > 0.0) {
            (true, true) => &[[-x, -y], [x, -y], [x, y], [-x, y]],
            (true, false) => &[[-x, 0.0], [x, 0.0]],
            (false, true) => &[[0.0, -y], [0.0, y]],
            (false, false) => &[],
        };
        for [px, py] in corners {
            add_local(bounds, &frame, *px, *py);
        }
    }
    fn add_circular_bounds(&self, bounds: &mut BoundingBox3, frame: &GridFrame) {
        let radius = self.params.radius();
        if radius <= 0.0 {
            return;
        }
        let half_pi = std::f64::consts::FRAC_PI_2;
        if !self.params.is_arc() {
            for [x, y] in [[radius, 0.0], [-radius, 0.0], [0.0, radius], [0.0, -radius]] {
                add_local(bounds, frame, x, y);
            }
            return;
        }
        let (start, end) = (self.params.angle_start(), self.params.angle_end());
        let arc_point = |angle: f64| {
            [
                radius * martensite_cmath::cos(angle),
                radius * martensite_cmath::sin(angle),
            ]
        };
        let mut angles = vec![start, end];
        angles.extend(
            [0.0, half_pi, std::f64::consts::PI, -half_pi]
                .into_iter()
                .filter(|angle| angle_in_arc(start, end, *angle)),
        );
        for [x, y] in angles.into_iter().map(arc_point) {
            add_local(bounds, frame, x, y);
        }
    }
}

fn add_local(bounds: &mut BoundingBox3, frame: &GridFrame, x: f64, y: f64) {
    let point = local_point(frame, x, y);
    if is_finite_point(point) {
        bounds.add_point(to_point(point));
    }
}
