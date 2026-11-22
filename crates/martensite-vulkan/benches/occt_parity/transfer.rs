//! Start-up, upload, readback and resize cases, twins of `occt/transfer.cpp`.
use super::frames::display_array;
use super::geometry::{self, Array, Aspect};
use super::native_window::NativeWindow;
use super::stage::{HEIGHT, Stage, WIDTH, fold_counts};
use super::{Bench, Case, Measured, Mode, new_driver, timed_runs};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dBufferType, Graphic3dStructureNetwork as _, Graphic3dStructureSource as _,
};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::Graphic3dRenderView;

pub const STARTUP: &[Case] = &[Case {
    name: "startup_first_frame",
    run: startup_first_frame,
}];

pub const CASES: &[Case] = &[
    Case {
        name: "upload_mesh_100k",
        run: upload_mesh_100k,
    },
    Case {
        name: "dump_1080p",
        run: dump_1080p,
    },
    Case {
        name: "resize_redraw",
        run: resize_redraw,
    },
];

/// One operation opens a window and a new driver, draws the first (empty) frame and closes
/// everything again: what an application pays before its first picture.
fn startup_first_frame(_: &Bench, mode: &Mode) -> Measured {
    let mut started = 0i64;
    let nanos = timed_runs(mode, 1, |_| started += i64::from(start_once()));
    Measured {
        nanos,
        checksum: started,
    }
}

/// Opens, draws and closes one viewer on its own driver; true when the frame rendered.
fn start_once() -> bool {
    let native = NativeWindow::new(WIDTH, HEIGHT);
    let driver = new_driver();
    let viewer = V3dViewer::new(&driver.cast()).expect("viewer");
    viewer.set_default_lights().expect("default lights");
    viewer.set_light_on().expect("lights on");
    let view = viewer.create_view().expect("view");
    view.set_window(&native.window.cast(), 0).expect("window");
    view.redraw();
    let rendered: Handle<Graphic3dRenderView> = view.get().expect("view").view().cast();
    let started = rendered.capture().is_ok();
    view.remove();
    started
}

/// A 102,000-triangle sphere, the scene of the transfer cases.
fn transfer_scene() -> (Array, Aspect) {
    let array = geometry::uv_sphere(300, 170, 10.);
    let aspect = geometry::lit_aspect([0.8, 0.5, 0.2], 0.);
    (array, aspect)
}

/// One operation displays a new structure holding the sphere, draws it and removes it:
/// the vertex upload and release of a presentation.
fn upload_mesh_100k(bench: &Bench, mode: &Mode) -> Measured {
    let stage = Stage::new(&bench.driver, [0.; 3], 10.);
    let (array, aspect) = transfer_scene();
    let nanos = timed_runs(mode, mode.scaled(4), |frame| {
        let structure = display_array(&stage, &array, &aspect);
        stage.redraw_orbit(frame);
        structure.erase();
        structure.remove();
    });
    let kept = display_array(&stage, &array, &aspect);
    let checksum = stage.counted_frame();
    drop(kept);
    stage.close();
    Measured { nanos, checksum }
}

/// One operation renders the sphere offscreen at 1920 x 1080 and reads the RGB image back.
fn dump_1080p(bench: &Bench, mode: &Mode) -> Measured {
    let stage = Stage::new(&bench.driver, [0.; 3], 10.);
    let (array, aspect) = transfer_scene();
    let kept = display_array(&stage, &array, &aspect);
    stage.redraw_orbit(0);
    let image = ImagePixMap::default();
    let mut dumped = 0i64;
    let nanos = timed_runs(mode, mode.scaled(4), |_| {
        let done = stage
            .view
            .to_pix_map_sized(&image, 1920, 1080, Graphic3dBufferType::Rgb, true);
        dumped += i64::from(done.expect("dump"));
    });
    let size = (image.width() * image.height()) as i64;
    let checksum = fold_counts(&[dumped, size, stage.counted_frame()]);
    drop(kept);
    stage.close();
    Measured { nanos, checksum }
}

/// Resizes the X window and tells the view; `resize(&stage, (1024, 576))`.
fn resize(stage: &Stage, (width, height): (i32, i32)) {
    stage.native.resize(width, height);
    stage.view.must_be_resized();
}

/// One operation resizes the window between 1280 x 720 and 1024 x 576 and redraws.
fn resize_redraw(bench: &Bench, mode: &Mode) -> Measured {
    let stage = Stage::new(&bench.driver, [0.; 3], 10.);
    let (array, aspect) = transfer_scene();
    let kept = display_array(&stage, &array, &aspect);
    stage.redraw_orbit(0);
    let nanos = timed_runs(mode, mode.scaled(2), |index| {
        let small = index % 2 == 0;
        resize(&stage, if small { (1024, 576) } else { (WIDTH, HEIGHT) });
        stage.view.redraw();
    });
    resize(&stage, (WIDTH, HEIGHT));
    let checksum = stage.counted_frame();
    drop(kept);
    stage.close();
    Measured { nanos, checksum }
}
