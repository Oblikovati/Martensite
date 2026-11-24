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

mod placement_verification;

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
const DISTRIBUTED_PARTS: i32 = 2_815;
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
    if std::env::var_os("MARTENSITE_594_VERIFY_COLD_MIXED").is_some() {
        check_cold_mixed_material(&grid);
    }
    let measured = if std::env::var_os("MARTENSITE_594_REPORT_PEAK").is_some()
        || std::env::var_os("MARTENSITE_594_FUSED_REPLAY").is_some()
    {
        measure_fused_replay(bench, &grid.stage, mode)
    } else {
        measure_orbit(bench, &grid.stage, mode)
    };
    if std::env::var_os("MARTENSITE_594_VERIFY_STREAM_EDITS").is_some() {
        check_fused_edit_fallback(&grid);
    }
    grid.close();
    if std::env::var_os("MARTENSITE_594_VERIFY_EXPIRED_BATCH").is_some() {
        check_expired_batch(bench);
    }
    measured
}

fn check_expired_batch(bench: &Bench) {
    use martensite_tkservice::graphic3d::Graphic3dCView;
    let grid = sphere_grid(bench);
    let view = grid.stage.render_view();
    grid.stage.redraw_orbit(17);
    grid.stage.redraw_orbit(17);
    grid.stage.redraw_orbit(18);
    let warm = view.last_frame_work();
    assert_eq!(warm.buffers_created, 0, "{warm:?}");
    assert_eq!(warm.geometry_uploaded_bytes, 0, "{warm:?}");
    assert!(view.fused_replay_hits() > 0);
    let layer = view
        .layers()
        .into_iter()
        .find(|layer| layer.nb_structures() > 0)
        .unwrap();
    let source = layer
        .array_of_structures()
        .iter()
        .flat_map(|sources| sources.iter())
        .next()
        .unwrap()
        .upgrade();
    source.clear();
    compare_fused_edit(&grid.stage, "destroyed group");
    assert_eq!(view.fused_replay_hits(), 0);
    grid.close();
}

fn check_cold_mixed_material(grid: &ShapeGrid) {
    use martensite_tkernel::quantity::{Color, NamedColor};
    let stage = &grid.stage;
    let view = stage.render_view();
    let object = &grid.objects[0];
    assert_eq!(view.fused_replay_hits(), 0);
    grid.context
        .set_color(object, Color::named(NamedColor::RED), false)
        .unwrap();
    compare_fused_edit(stage, "cold mixed material");
    grid.context.unset_color(object, false).unwrap();
    let retain = std::env::var_os("MARTENSITE_594_FUSED_REPLAY").is_some();
    view.enable_fused_replay(retain);
    stage.redraw_orbit(17);
    stage.redraw_orbit(17);
    assert_eq!(view.fused_replay_hits() > 0, retain);
}

fn check_fused_edit_fallback(grid: &ShapeGrid) {
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dNameOfMaterial};
    let stage = &grid.stage;
    let view = stage.render_view();
    let object = &grid.objects[0];
    stage.redraw_orbit(17);
    let original = view.capture().unwrap();
    check_fused_immediate_fallback(stage);
    check_fused_background_fallback(stage);
    check_fused_connection_fallback(stage);
    check_fused_priority_fallback(stage);
    view.enable_fused_replay(true);
    let accepted = view.fused_replay_hits();
    stage.redraw_orbit(17);
    assert_eq!(view.fused_replay_hits(), accepted + 1);
    view.enable_fused_replay(false);
    let gold = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
    grid.context.set_material(object, &gold, false).unwrap();
    let changed = compare_fused_edit(stage, "material coefficients");
    assert_ne!(changed.rgba, original.rgba);
    grid.context
        .set_color(object, Color::named(NamedColor::RED), false)
        .unwrap();
    let recolored = compare_fused_edit(stage, "interior color");
    assert_ne!(recolored.rgba, changed.rgba);
    grid.context.erase(object, false).unwrap();
    let hidden = compare_fused_edit(stage, "visibility");
    assert_ne!(hidden.rgba, recolored.rgba);
}

fn check_fused_immediate_fallback(stage: &Stage) {
    use martensite_tkservice::graphic3d::Graphic3dCView;
    stage.orbit(17);
    compare_fused_redraw(stage, "immediate redraw", || {
        stage.render_view().redraw_immediate();
    });
}

fn check_fused_background_fallback(stage: &Stage) {
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::{aspect::AspectBackground, graphic3d::Graphic3dCView};
    let view = stage.render_view();
    let original = view.background();
    view.set_background(&AspectBackground::new(Color::named(NamedColor::RED)));
    compare_fused_edit(stage, "background color");
    view.set_background(&original);
}

fn check_fused_connection_fallback(stage: &Stage) {
    use martensite_tkservice::graphic3d::Graphic3dCView;
    let layer = stage
        .render_view()
        .layers()
        .into_iter()
        .find(|layer| layer.nb_structures() > 1)
        .unwrap();
    let source_order = layer.array_of_structures();
    let mut sources = source_order
        .iter()
        .flat_map(|sources| sources.iter())
        .map(|link| link.upgrade());
    let first = sources.next().unwrap();
    let second = sources.next().unwrap();
    first.connect(&*second);
    compare_fused_edit(stage, "structure connection");
    first.disconnect(&*second);
}

fn check_fused_priority_fallback(stage: &Stage) {
    use martensite_tkservice::graphic3d::{Graphic3dCView, Graphic3dDisplayPriority};
    let layer = stage
        .render_view()
        .layers()
        .into_iter()
        .find(|layer| layer.nb_structures() > 0)
        .unwrap();
    let source = layer
        .array_of_structures()
        .iter()
        .flat_map(|sources| sources.iter())
        .last()
        .unwrap()
        .upgrade();
    let priority = layer.remove(&source, true);
    assert_ne!(priority, Graphic3dDisplayPriority::Invalid);
    assert_ne!(priority, Graphic3dDisplayPriority::Topmost);
    layer.add(&source, Graphic3dDisplayPriority::Topmost, true);
    compare_fused_edit(stage, "draw priority");
    layer.remove(&source, true);
    layer.add(&source, priority, true);
}

fn compare_fused_edit(stage: &Stage, edit: &str) -> martensite_vulkan::FrameCapture {
    compare_fused_redraw(stage, edit, || stage.redraw_orbit(17))
}

fn compare_fused_redraw(
    stage: &Stage,
    edit: &str,
    redraw: impl Fn(),
) -> martensite_vulkan::FrameCapture {
    let view = stage.render_view();
    let accepted = view.fused_replay_hits();
    view.enable_fused_replay(true);
    redraw();
    assert!(view.fused_replay_hits() <= accepted, "{edit}");
    let captured = view.capture().unwrap();
    view.enable_fused_replay(false);
    redraw();
    let normal = view.capture().unwrap();
    assert_eq!(captured.rgba, normal.rgba, "{edit}");
    assert!(
        captured
            .depth
            .iter()
            .map(|z| z.to_bits())
            .eq(normal.depth.iter().map(|z| z.to_bits()))
    );
    eprintln!("{edit} edit fell back to the exact ordinary result");
    captured
}

fn measure_fused_replay(bench: &Bench, stage: &Stage, mode: &Mode) -> Measured {
    for angle in 0..mode.warmup.min(mode.frames) {
        stage.redraw_orbit(angle);
    }
    let nanos = timed_runs(bench, mode, mode.frames, |angle| stage.redraw_orbit(angle));
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    let peak = status
        .lines()
        .find(|line| line.starts_with("VmHWM:"))
        .unwrap();
    eprintln!("end timed windows {peak}");
    let view = stage.render_view();
    if std::env::var_os("MARTENSITE_594_REPORT_PEAK").is_none() {
        for angle in [0, 17, 41, 47] {
            let accepted = view.fused_replay_hits();
            view.enable_fused_replay(true);
            stage.redraw_orbit(angle);
            assert!(view.fused_replay_hits() > accepted, "angle {angle}");
            let fused = view.capture().expect("retained fused probe frame");
            view.enable_fused_replay(false);
            stage.redraw_orbit(angle);
            let normal = view.capture().expect("ordinary frame");
            assert_eq!(fused.rgba, normal.rgba, "angle {angle}");
            assert!(
                fused
                    .depth
                    .iter()
                    .map(|z| z.to_bits())
                    .eq(normal.depth.iter().map(|z| z.to_bits())),
                "angle {angle}"
            );
            eprintln!("fused replay angle {angle} matched color and depth bytes");
        }
    }
    view.enable_fused_replay(false);
    stage.require_rendered();
    Measured {
        nanos,
        checksum: stage.counted_frame(),
    }
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
    let unique_parts = if std::env::var_os("MARTENSITE_594_DISTRIBUTED_PARTS").is_some() {
        DISTRIBUTED_PARTS
    } else {
        PARTS
    };
    let parts: Vec<Handle<dyn AisInteractiveObject>> = (0..unique_parts)
        .map(|index| AisShape::new(&part(index)).cast())
        .collect();
    let first = display_instance(&context, &parts[0], 0);
    for (index, part) in (1..INSTANCE_COLUMNS * INSTANCE_ROWS).zip(parts.iter().cycle().skip(1)) {
        display_instance(&context, part, index);
    }
    let measured = measure_heavy_orbit(bench, &stage, mode);
    if std::env::var_os("MARTENSITE_594_VERIFY_PLACEMENT_EDITS").is_some() {
        placement_verification::verify_placement_edits(&stage, &context, &parts[0], &first);
    }
    drop(first);
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
) -> Handle<dyn AisInteractiveObject> {
    let column = f64::from(index % INSTANCE_COLUMNS);
    let row = f64::from(index / INSTANCE_COLUMNS);
    let offset = Transform3::translation([column * INSTANCE_PITCH, row * INSTANCE_PITCH, 0.]);
    let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
    instance.connect_trsf(part, &offset).expect("connect");
    let instance: Handle<dyn AisInteractiveObject> = instance.cast();
    context
        .display_with_modes(&instance, 1, -1, false, PrsMgrDisplayStatus::None)
        .expect("display");
    instance
}

/// Cycle through the same three families, varying the dimensions in each additional batch.
fn part(index: i32) -> Shape {
    let grow = f64::from(index % 10) + 0.001 * f64::from(index / PARTS);
    match (index % PARTS) / 10 {
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
