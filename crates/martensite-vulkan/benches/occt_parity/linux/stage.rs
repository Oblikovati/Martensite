//! The stage every case draws on, twin of `occt/stage.cpp`: one mapped, unmanaged 1280x720
//! window, one lit `V3d` view, and a camera that orbits the scene one degree per frame so
//! that every redraw renders the whole scene again.
use super::native_window::NativeWindow;
use martensite_cmath::sincos;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection, Graphic3dFrameStatsCounter as Counter,
    Graphic3dFrameStatsMembers as _, Graphic3dRenderingParamsPerfCounters as Perf,
    Graphic3dStructure,
};
use martensite_tkv3d::v3d::{V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView};

/// The window size of every case.
pub const WIDTH: i32 = 1280;
/// The window height of every case.
pub const HEIGHT: i32 = 720;
/// The orbit's start angle and per-frame step, in degrees, and its elevation.
const ORBIT_START: f64 = 30.0;
const ORBIT_STEP: f64 = 1.0;
const ORBIT_ELEVATION: f64 = 35.264;

/// A live view on the shared driver; `Stage::new(&driver, [0.; 3], 10.)`.
pub struct Stage {
    /// The X11 window the view presents to.
    pub native: NativeWindow,
    /// The viewer that owns the structures of the case.
    pub viewer: Handle<V3dViewer>,
    /// The one view of the viewer.
    pub view: Handle<dyn V3dViewSource>,
    center: [f64; 3],
    radius: f64,
}

impl Stage {
    /// A lit view of a scene inside the sphere `(center, radius)`.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>, center: [f64; 3], radius: f64) -> Self {
        let native = NativeWindow::new(WIDTH, HEIGHT);
        let viewer = V3dViewer::new(&driver.cast()).expect("viewer");
        viewer.set_default_lights().expect("default lights");
        viewer.set_light_on().expect("lights on");
        let view = viewer.create_view().expect("view");
        view.set_immediate_update(false);
        view.set_window(&native.window.cast(), 0).expect("window");
        let black = Color::new(0., 0., 0., ColorSpace::Rgb).expect("black");
        view.set_background_color(black);
        let stage = Self {
            native,
            viewer,
            view,
            center,
            radius,
        };
        stage.frame_camera();
        if std::env::var_os("MARTENSITE_594_DISABLE_BATCH").is_some() {
            stage.render_view().enable_fused_replay(false);
        }
        stage
    }

    fn frame_camera(&self) {
        let camera = self.view.camera();
        let [x, y, z] = self.center;
        camera.set_center(&Point3::new(x, y, z)).expect("center");
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        camera.set_scale(2.2 * self.radius);
        self.orbit(0);
    }

    /// Moves the eye to the orbit position of `frame`.
    pub fn orbit(&self, frame: usize) {
        let angle = (ORBIT_START + ORBIT_STEP * frame as f64).to_radians();
        let (sine, cosine) = sincos(angle);
        let (rise, run) = sincos(ORBIT_ELEVATION.to_radians());
        let distance = 4.0 * self.radius;
        let [x, y, z] = self.center;
        let eye = Point3::new(
            x + distance * cosine * run,
            y + distance * sine * run,
            z + distance * rise,
        );
        let camera = self.view.camera();
        camera.set_eye(&eye).expect("eye");
        camera.set_up(&Direction3::new(0., 0., 1.).expect("up"));
    }

    /// Orbits to `frame` and redraws the view.
    pub fn redraw_orbit(&self, frame: usize) {
        self.orbit(frame);
        self.view.redraw();
    }

    /// A new empty structure of this viewer.
    pub fn structure(&self) -> Handle<Graphic3dStructure> {
        Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default())
            .expect("structure")
    }

    /// The Vulkan view behind the `V3d` view.
    pub fn render_view(&self) -> Handle<Graphic3dRenderView> {
        self.view.get().expect("view").view().cast()
    }

    /// Fails when the last redraw failed; the void `Redraw` keeps its error in the view.
    pub fn require_rendered(&self) {
        if let Err(error) = self.render_view().capture() {
            panic!("redraw failed: {error:?}; expected a rendered frame");
        }
    }

    /// Redraws the start position with primitive counting on and folds the counters.
    pub fn counted_frame(&self) -> i64 {
        self.orbit(0);
        self.set_counting(COUNTED, 0.0);
        self.view.redraw();
        self.require_rendered();
        let stats = self.render_view().frame_stats();
        let stats = stats.get().expect("frame statistics");
        let counts = COUNTERS.map(|counter| stats.counter_value(counter) as i64);
        self.set_counting(Perf::BASIC, 1.0);
        eprintln!("counters {counts:?}");
        fold_counts(&counts)
    }

    fn set_counting(&self, counters: Perf, interval: f32) {
        self.view.change_rendering_params(&mut |params| {
            params.stats.collected_stats = counters;
            params.stats.stats_update_interval = interval;
        });
    }

    /// Removes the view from the driver.
    pub fn close(self) {
        self.view.remove();
    }
}

/// The counter groups both renderers fill from their retained layers.
const COUNTED: Perf = Perf(
    Perf::STRUCTURES.0
        | Perf::GROUPS.0
        | Perf::GROUP_ARRAYS.0
        | Perf::TRIANGLES.0
        | Perf::LINES.0
        | Perf::POINTS.0,
);

/// The rendered-primitive counters of the checksum, in this order on both sides.
const COUNTERS: [Counter; 8] = [
    Counter::NbStructsNotCulled,
    Counter::NbElemsFillNotCulled,
    Counter::NbElemsLineNotCulled,
    Counter::NbElemsPointNotCulled,
    Counter::NbElemsTextNotCulled,
    Counter::NbTrianglesNotCulled,
    Counter::NbLinesNotCulled,
    Counter::NbPointsNotCulled,
];

/// Folds counters into one checksum, as `parity::fold` does; `fold_counts(&[1, 2])`.
pub fn fold_counts(counts: &[i64]) -> i64 {
    counts.iter().fold(0i64, |sum, count| {
        sum.wrapping_mul(1_000_003).wrapping_add(*count)
    })
}
