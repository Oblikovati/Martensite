//! Camera changes of the active view: projection, rotation, panning and the privileged plane.
use super::*;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};

impl ViewerSession {
    /// Switch between orthographic and perspective projection; `session.set_projection(P::Perspective)?`.
    pub fn set_projection(&self, projection: Graphic3dCameraProjection) -> Result<(), Failure> {
        let view = self.view().view_base();
        let camera = view.camera();
        if let Some(camera) = camera.get() {
            camera.set_projection_type(projection);
        }
        view.redraw();
        Ok(())
    }

    /// Rotate the camera by angles about the view axes, in radians; `session.rotate(0., 0., 0.5)?`.
    pub fn rotate(&self, ax: f64, ay: f64, az: f64) -> Result<(), Failure> {
        catch_native_failure(|| self.view().view_base().rotate(ax, ay, az, true))
    }

    /// Rotate the camera by angles in radians about the view axes through the point `about`
    /// Example: `session.rotate_about([-0.1, 0.1, 1.], [0.; 3])?`.
    pub fn rotate_about(&self, angles: [f64; 3], about: [f64; 3]) -> Result<(), Failure> {
        let [ax, ay, az] = angles;
        let [x, y, z] = about;
        catch_native_failure(|| {
            self.view()
                .view_base()
                .rotate_about(ax, ay, az, x, y, z, true)
        })
    }

    /// Start an interactive rotation at `from`
    /// (no Z-rotation threshold) and continue it to `to`; `session.mouse_rotate((400, 200), (300, 300))?`.
    pub fn mouse_rotate(&self, from: (i32, i32), to: (i32, i32)) -> Result<(), Failure> {
        let view = self.view().view_base();
        catch_native_failure(|| view.start_rotation(from.0, from.1, 0.0))?;
        catch_native_failure(|| view.rotation(to.0, to.1))
    }

    /// Fit the depth range to the displayed objects, then redraw; `session.z_fit()?`.
    pub fn z_fit(&self) -> Result<(), Failure> {
        let view = self.view().view_base();
        catch_native_failure(|| view.z_fit_all(1.0))?;
        view.redraw();
        Ok(())
    }

    /// Pan the view by a pixel offset; `session.pan(80, 40)?`.
    pub fn pan(&self, dx: i32, dy: i32) -> Result<(), Failure> {
        catch_native_failure(|| self.view().view_base().pan(dx, dy, 1.0, true))
    }

    /// Set the viewer's privileged plane from origin, normal and X direction.
    /// Example: `session.set_privileged_plane([0.; 3], [1., 0., 0.], [0., 1., 0.])?`.
    pub fn set_privileged_plane(
        &self,
        origin: [f64; 3],
        normal: [f64; 3],
        x_dir: [f64; 3],
    ) -> Result<(), Failure> {
        let point = |v: [f64; 3]| Point3::new(v[0], v[1], v[2]);
        let direction = |v: [f64; 3]| Direction3::new(v[0], v[1], v[2]);
        let plane = Axis3::new(point(origin), direction(normal)?, direction(x_dir)?)?;
        self.viewer.set_privileged_plane(&plane)
    }
}

impl ViewerSession {
    /// Look along a standard orientation with Z up, as the named view shortcuts do.
    /// Example: `session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?`.
    pub fn set_standard_projection(
        &self,
        orientation: V3dTypeOfOrientation,
    ) -> Result<(), Failure> {
        catch_native_failure(|| self.view().set_proj_orientation(orientation, false))
    }
}
