//! Interactive-context cases, twins of `occt/ais.cpp`: the sphere grid of OCCT's
//! `perf/vis/bug24623_1` at 20 x 20, shaded, a selection change in a grid of 10,000 boxes and
//! an assembly of 30,000 connected instances. These include the presentation and selection
//! layers above the renderer, as an application drives them.
use super::frames::{measure_heavy_orbit, measure_orbit};
use super::stage::{HEIGHT, Stage, WIDTH, fold_counts};
use super::{Bench, Case, Measured, Mode, timed_runs};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::ais::{
    AisConnectedInteractive, AisInteractiveContext, AisInteractiveObject, AisShape,
};
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrTypeOfPresentation3d};

pub const CASES: &[Case] = &[
    Case {
        name: "frame_ais_spheres_400",
        run: frame_ais_spheres_400,
    },
    Case {
        name: "highlight_ais_spheres_400",
        run: highlight_ais_spheres_400,
    },
    Case {
        name: "select_10k",
        run: select_10k,
    },
    Case {
        name: "frame_connected_30k",
        run: frame_connected_30k,
    },
];

/// A square grid of `side` x `side` cells `pitch` apart, for a shape that lies inside the
/// sphere of radius `reach` around its origin.
#[derive(Clone, Copy)]
struct GridLayout {
    side: i32,
    pitch: f64,
    reach: f64,
}

/// The spheres of radius 100 of `bug24623_1`.
const SPHERES: GridLayout = GridLayout {
    side: 20,
    pitch: 110.,
    reach: 100.,
};
/// Boxes of half size 1; 2 bounds their half diagonal of 1.73.
const BOXES: GridLayout = GridLayout {
    side: 100,
    pitch: 3.,
    reach: 2.,
};

/// The instances of `frame_connected_30k`: 30 parts on a 200 x 150 grid, 10 units apart.
const PARTS: i32 = 30;
const INSTANCE_COLUMNS: i32 = 200;
const INSTANCE_ROWS: i32 = 150;
const INSTANCE_PITCH: f64 = 10.;
/// Bounds the distance of a part from its origin; the tallest cylinder reaches 6.3.
const PART_REACH: f64 = 8.;

/// A stage, its context and the displayed objects of a shape grid.
struct ShapeGrid {
    stage: Stage,
    context: Handle<AisInteractiveContext>,
    objects: Vec<Handle<dyn AisInteractiveObject>>,
}

impl ShapeGrid {
    /// Copies of `shape` on `layout`, displayed shaded and selectable (selection mode 0).
    fn new(bench: &Bench, shape: &Shape, layout: GridLayout) -> Self {
        let half = layout.pitch * f64::from(layout.side - 1) / 2.;
        let radius = half * std::f64::consts::SQRT_2 + layout.reach;
        let stage = Stage::new(&bench.driver, [half, -half, 0.], radius);
        let context = AisInteractiveContext::new(&stage.viewer).expect("context");
        let objects = (0..layout.side * layout.side)
            .map(|index| display_selectable(&context, &placed(shape, index, layout)))
            .collect();
        Self {
            stage,
            context,
            objects,
        }
    }

    /// Releases the objects and the context, then closes the stage.
    fn close(self) {
        drop(self.objects);
        drop(self.context);
        self.stage.close();
    }
}

/// The shape a primitive maker builds; `made(BRepPrimAPIMakeCylinder::new(1., 2.))`.
fn made(mut maker: impl BRepBuilderAPIMakeShape) -> Shape {
    match maker.shape() {
        Ok(shape) => shape.clone(),
        Err(failure) => panic!("primitive maker: {failure:?}; expected a built solid"),
    }
}

/// `shape` moved to grid cell `index` of `layout`, as `ttranslate` moves it.
fn placed(shape: &Shape, index: i32, layout: GridLayout) -> Shape {
    let (column, row) = (
        f64::from(index / layout.side),
        f64::from(index % layout.side),
    );
    let offset = Transform3::translation([column * layout.pitch, -row * layout.pitch, 0.]);
    shape
        .moved(&Location::new(offset), false)
        .expect("moved shape")
}

/// Displays `shape` shaded with selection mode 0, which `set_selected_object` needs.
fn display_selectable(
    context: &Handle<AisInteractiveContext>,
    shape: &Shape,
) -> Handle<dyn AisInteractiveObject> {
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
    context
        .display_with_modes(&object, 1, 0, false, PrsMgrDisplayStatus::None)
        .expect("display");
    object
}

/// The stage, context and objects of the shaded sphere grid.
fn sphere_grid(bench: &Bench) -> ShapeGrid {
    let sphere = made(BRepPrimAPIMakeSphere::new(SPHERES.reach).expect("sphere"));
    ShapeGrid::new(bench, &sphere, SPHERES)
}

fn frame_ais_spheres_400(bench: &Bench, mode: &Mode) -> Measured {
    let grid = sphere_grid(bench);
    let measured = measure_orbit(bench, &grid.stage, mode);
    grid.close();
    measured
}

/// One operation moves the cursor onto the grid centre, then off the grid: two
/// detections, each with a dynamic-highlight redraw.
fn highlight_ais_spheres_400(bench: &Bench, mode: &Mode) -> Measured {
    let grid = sphere_grid(bench);
    grid.stage.redraw_orbit(0);
    let mut detections = [0i64; 2];
    let mut hover = |_| {
        let points = [(WIDTH / 2, HEIGHT / 2), (2, 2)];
        for (count, (x, y)) in detections.iter_mut().zip(points) {
            let context = &grid.context;
            context
                .move_to(x, y, &grid.stage.view, true)
                .expect("move to");
            *count += i64::from(context.has_detected());
        }
    };
    for index in 0..mode.warmup {
        hover(index);
    }
    let nanos = timed_runs(bench, mode, mode.frames, hover);
    grid.stage.require_rendered();
    eprintln!("detections {detections:?}");
    let checksum = fold_counts(&detections);
    grid.close();
    Measured { nanos, checksum }
}

/// One operation selects the other of two boxes of a 100 x 100 grid and updates the viewer,
/// so that the selection changes every time; a selection must not upload geometry again.
fn select_10k(bench: &Bench, mode: &Mode) -> Measured {
    let (low, high) = (Point3::new(-1., -1., -1.), Point3::new(1., 1., 1.));
    let shape = made(BRepPrimAPIMakeBox::from_points(low, high).expect("box"));
    let grid = ShapeGrid::new(bench, &shape, BOXES);
    grid.stage.redraw_orbit(0);
    let picks = [&grid.objects[0], &grid.objects[5050]];
    let mut changes = 0usize;
    let mut select = |_| {
        changes += 1;
        let pick = picks[changes % 2];
        grid.context
            .set_selected_object(pick, true)
            .expect("select");
    };
    for index in 0..mode.warmup {
        select(index);
    }
    let nanos = timed_runs(bench, mode, mode.frames, select);
    grid.stage.require_rendered();
    let selected = i64::from(grid.context.nb_selected());
    let checksum = fold_counts(&[selected, grid.stage.counted_frame()]);
    grid.close();
    Measured { nanos, checksum }
}

/// 30,000 connected instances of 30 parts, shaded and not selectable; the parts themselves
/// are not displayed. The occurrences of an assembly, sharing the presentations of their parts.
fn frame_connected_30k(bench: &Bench, mode: &Mode) -> Measured {
    let (center, radius) = instance_scene();
    let stage = Stage::new(&bench.driver, center, radius);
    let context = AisInteractiveContext::new(&stage.viewer).expect("context");
    let parts: Vec<Handle<dyn AisInteractiveObject>> = (0..PARTS)
        .map(|index| AisShape::new(&part(index)).cast())
        .collect();
    for (index, part) in (0..INSTANCE_COLUMNS * INSTANCE_ROWS).zip(parts.iter().cycle()) {
        display_instance(&context, part, index);
    }
    let measured = measure_heavy_orbit(bench, &stage, mode);
    drop(context);
    stage.close();
    measured
}

/// The sphere `(center, radius)` around the instance grid.
fn instance_scene() -> ([f64; 3], f64) {
    let half_width = INSTANCE_PITCH * f64::from(INSTANCE_COLUMNS - 1) / 2.;
    let half_depth = INSTANCE_PITCH * f64::from(INSTANCE_ROWS - 1) / 2.;
    let half_diagonal = (half_width * half_width + half_depth * half_depth).sqrt();
    ([half_width, half_depth, 0.], half_diagonal + PART_REACH)
}

/// Displays instance `index` of `part` at its grid cell, shaded, without selection.
fn display_instance(
    context: &Handle<AisInteractiveContext>,
    part: &Handle<dyn AisInteractiveObject>,
    index: i32,
) {
    let column = f64::from(index % INSTANCE_COLUMNS);
    let row = f64::from(index / INSTANCE_COLUMNS);
    let offset = Transform3::translation([column * INSTANCE_PITCH, row * INSTANCE_PITCH, 0.]);
    let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
    instance.connect_trsf(part, &offset).expect("connect");
    let instance: Handle<dyn AisInteractiveObject> = instance.cast();
    context
        .display_with_modes(&instance, 1, -1, false, PrsMgrDisplayStatus::None)
        .expect("display");
}

/// Part `index` of 30: ten boxes, ten cylinders and ten cones that grow with the index.
fn part(index: i32) -> Shape {
    let grow = f64::from(index % 10);
    match index / 10 {
        0 => box_part(grow),
        1 => made(BRepPrimAPIMakeCylinder::new(
            1. + 0.2 * grow,
            2. + 0.4 * grow,
        )),
        _ => made(BRepPrimAPIMakeCone::new(1.5 + 0.2 * grow, 0.5, 2. + 0.3 * grow).expect("cone")),
    }
}

/// A box standing on the origin, centred on the z axis, that grows with `grow` (0 to 9).
fn box_part(grow: f64) -> Shape {
    let (half_width, half_depth, height) = (1. + 0.15 * grow, 1.5 + 0.1 * grow, 1. + 0.4 * grow);
    let low = Point3::new(-half_width, -half_depth, 0.);
    let high = Point3::new(half_width, half_depth, height);
    made(BRepPrimAPIMakeBox::from_points(low, high).expect("box"))
}
