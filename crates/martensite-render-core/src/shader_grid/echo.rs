//! GlShaderGrid::Echo and SnapPoint: the grid node nearest a pixel or a world point.
use super::{GridFrame, RenderShaderGrid, is_finite_point, local_point, to_point};
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkservice::aspect::AspectGridDrawMode;
use martensite_tkservice::graphic3d::Graphic3dCamera;

/// A snapped grid point and the display point at the view's middle depth.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderGridEcho {
    /// The snapped grid node.
    pub point: Point3,
    /// The node moved to the middle of the view's depth range, for echo markers.
    pub display: Point3,
}

/// The pixel query of an echo: viewport size and window pixel (top-left origin).
#[derive(Clone, Copy, Debug)]
pub struct RenderGridPixel {
    /// Window width and height in pixels.
    pub size: [i32; 2],
    /// Window pixel, origin top-left.
    pub pixel: [i32; 2],
}

/// The best screen-space candidate so far.
#[derive(Default)]
struct EchoChoice {
    best: Option<(f64, Xyz)>,
}

impl RenderShaderGrid {
    fn snaps(&self) -> bool {
        self.is_shown
            && self.params.draw_mode() != AspectGridDrawMode::None
            && !self.params.is_background()
    }
    fn positive_scale(&self, camera: &Graphic3dCamera) -> Option<(f64, f64)> {
        let (x, y) = self.effective_scale(camera);
        let tiny = martensite_tkernel::precision::confusion();
        (x > tiny && y > tiny).then_some((x, y))
    }
    /// `SnapPoint`: round a world point to the nearest grid node inside the domain.
    pub fn snap_point(&self, camera: &Graphic3dCamera, point: Xyz) -> Option<Point3> {
        if !self.snaps() {
            return None;
        }
        let scale = self.positive_scale(camera)?;
        let frame = self.frame();
        let local = point - frame.origin;
        let (x, y) = (local.dot(frame.x), local.dot(frame.y));
        let [x, y] = self.rounded_node(x, y, scale);
        self.is_point_in_bounds(x, y)
            .then(|| to_point(local_point(&frame, x, y)))
    }
    /// The nearest node in polar (radius/angle) or rectangular steps.
    fn rounded_node(&self, x: f64, y: f64, (scale_x, scale_y): (f64, f64)) -> [f64; 2] {
        if self.params.is_circular() {
            let (radius_step, angle_step) = self.polar_steps(scale_x);
            let radius = ((x * x + y * y).sqrt() / radius_step).round() * radius_step;
            let angle = (martensite_cmath::atan2(y, x) / angle_step).round() * angle_step;
            return [
                radius * martensite_cmath::cos(angle),
                radius * martensite_cmath::sin(angle),
            ];
        }
        let (step_x, step_y) = (1.0 / scale_x, 1.0 / scale_y);
        [(x / step_x).round() * step_x, (y / step_y).round() * step_y]
    }
    fn polar_steps(&self, scale_x: f64) -> (f64, f64) {
        let divisions = self.params.angular_divisions();
        let pi = std::f64::consts::PI;
        let angle = if divisions > 0 {
            pi / f64::from(divisions)
        } else {
            pi
        };
        (1.0 / scale_x, angle)
    }
    /// `Echo`: the node under a window pixel; points mode picks the closest cell corner on screen.
    pub fn echo(&self, camera: &Graphic3dCamera, query: RenderGridPixel) -> Option<RenderGridEcho> {
        if !self.snaps() || query.size[0] <= 0 || query.size[1] <= 0 {
            return None;
        }
        let ndc = echo_ndc(query);
        let hit = self.plane_local_hit(camera, ndc, true)?;
        let [x, y] = hit.local;
        if !self.is_point_in_bounds(x, y) {
            return None;
        }
        let scale = self.positive_scale(camera)?;
        let frame = self.frame();
        let mut choice = EchoChoice::default();
        for [cx, cy] in self.echo_candidates(x, y, scale) {
            self.accept_candidate(camera, query, &frame, [cx, cy], &mut choice);
        }
        let (_, snapped) = choice.best?;
        let display = echo_display_point(camera, snapped)?;
        Some(RenderGridEcho {
            point: to_point(snapped),
            display,
        })
    }
    /// One rounded node, or the four corners of the enclosing cell in points mode.
    fn echo_candidates(&self, x: f64, y: f64, scale: (f64, f64)) -> Vec<[f64; 2]> {
        if self.params.draw_mode() != AspectGridDrawMode::Points {
            return vec![self.rounded_node(x, y, scale)];
        }
        if self.params.is_circular() {
            let (radius_step, angle_step) = self.polar_steps(scale.0);
            let radius = ((x * x + y * y).sqrt() / radius_step).floor();
            let angle = (martensite_cmath::atan2(y, x) / angle_step).floor();
            let radii = [radius, radius + 1.0].map(|index| (index * radius_step).max(0.0));
            let angles = [angle, angle + 1.0].map(|index| index * angle_step);
            return cell_corners(radii, angles)
                .map(|[r, a]| [r * martensite_cmath::cos(a), r * martensite_cmath::sin(a)])
                .to_vec();
        }
        let (step_x, step_y) = (1.0 / scale.0, 1.0 / scale.1);
        let xs = [(x / step_x).floor(), (x / step_x).floor() + 1.0].map(|i| i * step_x);
        let ys = [(y / step_y).floor(), (y / step_y).floor() + 1.0].map(|i| i * step_y);
        cell_corners(xs, ys).to_vec()
    }
    /// `acceptEchoCandidate`: inside the domain, in front of the camera and closest on screen.
    fn accept_candidate(
        &self,
        camera: &Graphic3dCamera,
        query: RenderGridPixel,
        frame: &GridFrame,
        [x, y]: [f64; 2],
        choice: &mut EchoChoice,
    ) {
        if !self.is_point_in_bounds(x, y) {
            return;
        }
        let candidate = local_point(frame, x, y);
        if !is_finite_point(candidate) || behind_camera(camera, candidate) {
            return;
        }
        let Some(distance) = pixel_distance(camera, query, candidate) else {
            return;
        };
        if choice.best.is_none_or(|(best, _)| distance < best) {
            choice.best = Some((distance, candidate));
        }
    }
}

fn cell_corners(first: [f64; 2], second: [f64; 2]) -> [[f64; 2]; 4] {
    [
        [first[0], second[0]],
        [first[0], second[1]],
        [first[1], second[0]],
        [first[1], second[1]],
    ]
}

fn echo_ndc(query: RenderGridPixel) -> [f64; 2] {
    let [width, height] = query.size.map(f64::from);
    let [x, y] = query.pixel.map(f64::from);
    [
        2.0 * x / width - 1.0,
        2.0 * (height - 1.0 - y) / height - 1.0,
    ]
}

fn behind_camera(camera: &Graphic3dCamera, candidate: Xyz) -> bool {
    !camera.is_orthographic()
        && camera.direction().xyz().dot(candidate - camera.eye().xyz())
            <= martensite_tkernel::precision::confusion()
}

/// The squared pixel distance from the query to the candidate's projection.
fn pixel_distance(camera: &Graphic3dCamera, query: RenderGridPixel, candidate: Xyz) -> Option<f64> {
    let projected = camera.project(&to_point(candidate)).xyz();
    if !is_finite_point(projected) {
        return None;
    }
    let [width, height] = query.size.map(f64::from);
    let px = (projected.x() + 1.0) * 0.5 * width;
    let py = (height - 1.0) - (projected.y() + 1.0) * 0.5 * height;
    let dx = px - f64::from(query.pixel[0]);
    let dy = py - f64::from(query.pixel[1]);
    let distance = dx * dx + dy * dy;
    distance.is_finite().then_some(distance)
}

/// `echoDisplayPoint`: the snapped point moved to the middle of the depth range.
fn echo_display_point(camera: &Graphic3dCamera, snapped: Xyz) -> Option<Point3> {
    let projected = camera.project(&to_point(snapped)).xyz();
    if !is_finite_point(projected) {
        return None;
    }
    let depth = if camera.is_zero_to_one_depth() {
        0.5
    } else {
        0.0
    };
    let display = camera.un_project(&Point3::new(projected.x(), projected.y(), depth));
    is_finite_point(display.xyz()).then_some(display)
}
