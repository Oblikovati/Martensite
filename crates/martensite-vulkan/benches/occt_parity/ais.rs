//! Interactive-context cases, twins of `occt/ais.cpp`: the sphere grid of OCCT's
//! `perf/vis/bug24623_1` at 20 x 20, shaded. These include the presentation and selection
//! layers above the renderer, as an application drives them.
use super::frames::measure_orbit;
use super::stage::{HEIGHT, Stage, WIDTH, fold_counts};
use super::{Bench, Case, Measured, Mode, timed_runs};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Transform3;
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::PrsMgrDisplayStatus;

pub const CASES: &[Case] = &[
    Case {
        name: "frame_ais_spheres_400",
        run: frame_ais_spheres_400,
    },
    Case {
        name: "highlight_ais_spheres_400",
        run: highlight_ais_spheres_400,
    },
];

const SIDE: i32 = 20;
const RADIUS: f64 = 100.;
const PITCH: f64 = 110.;

/// The stage and context with the shaded sphere grid displayed.
fn sphere_grid(bench: &Bench) -> (Stage, Handle<AisInteractiveContext>) {
    let half = PITCH * f64::from(SIDE - 1) / 2.;
    let radius = half * std::f64::consts::SQRT_2 + RADIUS;
    let stage = Stage::new(&bench.driver, [half, -half, 0.], radius);
    let context = AisInteractiveContext::new(&stage.viewer).expect("context");
    let sphere = BRepPrimAPIMakeSphere::new(RADIUS)
        .expect("sphere")
        .shape()
        .expect("sphere shape")
        .clone();
    for index in 0..SIDE * SIDE {
        let object: Handle<dyn AisInteractiveObject> =
            AisShape::new(&placed(&sphere, index)).cast();
        context
            .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
            .expect("display");
    }
    (stage, context)
}

/// The sphere of grid cell `index`, moved as `ttranslate` moves it.
fn placed(sphere: &Shape, index: i32) -> Shape {
    let (column, row) = (f64::from(index / SIDE), f64::from(index % SIDE));
    let offset = Transform3::translation([column * PITCH, -row * PITCH, 0.]);
    sphere
        .moved(&Location::new(offset), false)
        .expect("moved sphere")
}

fn frame_ais_spheres_400(bench: &Bench, mode: &Mode) -> Measured {
    let (stage, context) = sphere_grid(bench);
    let measured = measure_orbit(&stage, mode);
    drop(context);
    stage.close();
    measured
}

/// One operation moves the cursor onto the grid centre, then off the grid: two
/// detections, each with a dynamic-highlight redraw.
fn highlight_ais_spheres_400(bench: &Bench, mode: &Mode) -> Measured {
    let (stage, context) = sphere_grid(bench);
    stage.redraw_orbit(0);
    let mut detections = [0i64; 2];
    let mut hover = |_| {
        let points = [(WIDTH / 2, HEIGHT / 2), (2, 2)];
        for (count, (x, y)) in detections.iter_mut().zip(points) {
            context.move_to(x, y, &stage.view, true).expect("move to");
            *count += i64::from(context.has_detected());
        }
    };
    for index in 0..mode.warmup {
        hover(index);
    }
    let nanos = timed_runs(mode, mode.frames, hover);
    stage.require_rendered();
    eprintln!("detections {detections:?}");
    let checksum = fold_counts(&detections);
    drop(context);
    stage.close();
    Measured { nanos, checksum }
}
