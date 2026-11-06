//! The user-program scenes of tests/user_programs_native.rs and
//! tests/user_program_stages_native.rs: the shaded 1x2x3 box under an axonometric view,
//! fitted and rotated 0.2 about X, programs built from inline GLSL stages,
//! and the grading of the dumps against the recorded captures.
use super::grading::{self, DumpReport};
use super::raytrace_scene::{RaytraceScene, SIZE};
use super::{shift_control, view_snap_pixels};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{Graphic3dRenderDriver, UserProgramKey};
use std::cell::RefCell;
use std::collections::BTreeSet;

/// A 1x2x3 box.
pub fn unit_box() -> martensite_tkbrep::topods::Shape {
    let made = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(0., 0., 0.),
        1.,
        2.,
        3.,
    );
    made.unwrap().shape().unwrap().clone()
}

/// The snap-dependent pixels of one dump.
type SnapPixels = BTreeSet<(i32, i32)>;

/// One run of a case: the camera shift of the cross-driver shift control (zero by default) and,
/// on AMD, the snap-dependent pixels of every dump (`view_snap_pixels`).
pub struct DumpRun {
    pub shift: [f64; 2],
    snaps: Option<RefCell<Vec<SnapPixels>>>,
}
impl DumpRun {
    /// A run without shift that records no snap pixels.
    pub fn plain() -> Self {
        Self::shifted([0., 0.])
    }
    fn shifted(shift: [f64; 2]) -> Self {
        Self { shift, snaps: None }
    }
    fn recording() -> Self {
        Self {
            shift: [0., 0.],
            snaps: Some(RefCell::default()),
        }
    }
    /// The checked frame, and its snap pixels when the run records them.
    pub fn dump(&self, scene: &RaytraceScene) -> Vec<u8> {
        self.dump_view(&scene.view)
    }
    /// A dump of `view`; `run.dump_view(&scene.view)`.
    pub fn dump_view(&self, view: &Handle<dyn V3dViewSource>) -> Vec<u8> {
        let rgb = checked_view_dump(view);
        if let Some(snaps) = &self.snaps {
            snaps
                .borrow_mut()
                .push(view_snap_pixels::view_snap_pixels(view, SIZE));
        }
        rgb
    }
}

/// A shaded axonometric view of `shape`, fitted and rotated 0.2 about X, then `run.shift`.
pub fn shaded_scene(
    driver: &Handle<Graphic3dRenderDriver>,
    shape: &martensite_tkbrep::topods::Shape,
    run: &DumpRun,
) -> (RaytraceScene, Handle<dyn AisInteractiveObject>) {
    let scene = RaytraceScene::new(driver);
    let orientation = V3dTypeOfOrientation::XposYnegZpos;
    scene
        .view()
        .set_proj_orientation(orientation, false)
        .unwrap();
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
    object.get().unwrap().set_display_mode(1);
    scene
        .context
        .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
        .unwrap();
    scene.fit();
    scene.view().rotate(0.2, 0., 0., true).unwrap();
    shift_control::shift_camera(scene.view(), run.shift, SIZE.0);
    (scene, object)
}

/// A program of `stages` under `header`, attaching inline sources.
pub fn source_program(
    header: &str,
    stages: &[(Graphic3dTypeOfShaderObject, &str)],
) -> Graphic3dShaderProgram {
    let program = Graphic3dShaderProgram::default();
    program.set_header(header.into());
    for &(kind, source) in stages {
        let shader = Graphic3dShaderObject::create_from_source(kind, source.into());
        assert!(program.attach_shader(&shader));
    }
    program
}

/// Register the offline-compiled stages of `program` under its key, which must be `expected`
/// (the name the recording tool gave the SPIR-V files).
pub fn register(
    driver: &Handle<Graphic3dRenderDriver>,
    program: &Graphic3dShaderProgram,
    expected: &str,
    spirv: &[&[u8]],
) {
    let key = UserProgramKey::of(program);
    assert_eq!(
        key.to_string(),
        expected,
        "program key of the compiled fixture"
    );
    let words = |bytes: &[u8]| -> Vec<u32> {
        bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|w| u32::from_le_bytes(*w))
            .collect()
    };
    let modules = spirv.iter().map(|bytes| words(bytes)).collect();
    driver.user_programs().insert(key, modules).unwrap();
}

/// Set the program on the fill-area aspects, then redisplay.
pub fn apply_program(
    scene: &RaytraceScene,
    object: &Handle<dyn AisInteractiveObject>,
    program: &Handle<Graphic3dShaderProgram>,
) {
    let drawer = object.attributes();
    let changed = drawer.set_shader_program(program, Graphic3dGroupAspect::FillArea, true);
    if changed.unwrap() {
        scene.context.redisplay(object, false, false).unwrap();
    }
}

/// The recorded captures of one case per device profile.
pub struct NativeDumps<'a> {
    pub names: &'a [&'static str],
    pub llvmpipe: &'a [&'static [u8]],
    pub amd: &'a [&'static [u8]],
    /// Environment variable naming a folder for the Rust dumps.
    pub dump_dir_variable: &'a str,
}

/// A case run on a driver; `run(&driver, &DumpRun::plain())`.
pub type ScriptRun = fn(&Handle<Graphic3dRenderDriver>, &DumpRun) -> Vec<Vec<u8>>;

/// Run the case, the lavapipe rounding control and the AMD half-step shift controls, and
/// grade every dump (with its snap pixels on AMD); `(device, report)`.
pub fn graded_dumps(native: &NativeDumps, run: ScriptRun) -> (String, Vec<DumpReport>) {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let device = driver.device_name();
    let recorded = if device.contains("llvmpipe") {
        DumpRun::plain()
    } else {
        DumpRun::recording()
    };
    let actual = run(&driver, &recorded);
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    let control = grading::rounding_control(&driver, |d| run(d, &DumpRun::plain()));
    let shifts =
        shift_control::shift_controls(&driver, |d, shift| run(d, &DumpRun::shifted(shift)));
    let snaps = recorded.snaps.map(RefCell::into_inner);
    let dumps = if device.contains("llvmpipe") {
        native.llvmpipe
    } else {
        native.amd
    };
    grading::write_dumps(native.dump_dir_variable, native.names, &actual, SIZE);
    let report: Vec<DumpReport> = actual
        .iter()
        .enumerate()
        .map(|(index, rgb)| {
            let control = control.as_ref().map(|dumps| dumps[index].as_slice());
            let expected = grading::ppm_rgb(dumps[index], SIZE);
            let snaps = snaps.as_ref().map(|snaps| &snaps[index]);
            let shifted = shift_control::dump_shifts(&shifts, index);
            let name = native.names[index];
            grading::grade_dump(name, rgb, expected, control, snaps, &shifted, SIZE.0)
        })
        .collect();
    println!("{{\"user_program_device\":{device:?},\"dumps\":{report:?}}}");
    (device, report)
}

/// A dump of `view`, failing with the typed render error when the frame fails.
pub fn checked_view_dump(view: &Handle<dyn V3dViewSource>) -> Vec<u8> {
    let base = view.get().unwrap();
    let render: Handle<martensite_vulkan::Graphic3dRenderView> = base.view().cast();
    render.try_redraw().expect("a rendered user-program frame");
    grading::view_rgb(base.view_base(), SIZE)
}
