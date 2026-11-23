//! Redraw cases, twins of `occt/frames.cpp`: a static scene while the camera orbits one degree
//! per frame. One operation is one complete frame.
use super::geometry::{self, Array, Aspect};
use super::stage::Stage;
use super::{Bench, Case, Measured, Mode, timed_runs};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dGroupMembers as _, Graphic3dStructure, Graphic3dStructureMembers as _,
    Graphic3dStructureSource as _,
};

/// The structures a case keeps displayed until it ends.
pub type Kept = Vec<Handle<Graphic3dStructure>>;

pub const CASES: &[Case] = &[
    Case {
        name: "frame_empty",
        run: frame_empty,
    },
    Case {
        name: "frame_mesh_1m",
        run: frame_mesh_1m,
    },
    Case {
        name: "frame_structures_10k",
        run: frame_structures_10k,
    },
    Case {
        name: "frame_segments_1m",
        run: frame_segments_1m,
    },
    Case {
        name: "frame_points_1m",
        run: frame_points_1m,
    },
    Case {
        name: "frame_text_1k",
        run: frame_text_1k,
    },
    Case {
        name: "frame_transparent_1k",
        run: frame_transparent_1k,
    },
    Case {
        name: "frame_msaa4_100k",
        run: frame_msaa4_100k,
    },
    Case {
        name: "frame_structures_100k",
        run: frame_structures_100k,
    },
    Case {
        name: "frame_mesh_10m",
        run: frame_mesh_10m,
    },
];

/// The columns and rows of the box grid of `frame_structures_10k` and `close_10k`.
pub const GRID_10K: [i32; 2] = [100, 100];
/// The columns and rows of the box grid of `frame_structures_100k`.
const GRID_100K: [i32; 2] = [400, 250];

/// Times a stage: `measure_orbit` or `measure_heavy_orbit`.
type Measure = fn(&Bench, &Stage, &Mode) -> Measured;

/// Warms up, then times `mode.frames` orbit frames per run and counts the start frame.
pub fn measure_orbit(bench: &Bench, stage: &Stage, mode: &Mode) -> Measured {
    measure_orbit_frames(bench, stage, mode, mode.frames)
}

/// `measure_orbit` with a tenth of the frames per run, for the scenes whose Vulkan frame
/// takes seconds, so that a round stays within minutes; `measure_heavy_orbit(bench, &stage, mode)`.
pub fn measure_heavy_orbit(bench: &Bench, stage: &Stage, mode: &Mode) -> Measured {
    measure_orbit_frames(bench, stage, mode, mode.scaled(10))
}

/// Warms up, then times `frames` orbit frames per run and counts the start frame.
fn measure_orbit_frames(bench: &Bench, stage: &Stage, mode: &Mode, frames: usize) -> Measured {
    for frame in 0..mode.warmup.min(frames) {
        stage.redraw_orbit(frame);
    }
    let nanos = timed_runs(bench, mode, frames, |frame| stage.redraw_orbit(frame));
    stage.require_rendered();
    Measured {
        nanos,
        checksum: stage.counted_frame(),
    }
}

/// Builds a stage around `(center, radius)`, fills it with `build` and measures its frames.
fn orbit_case(
    bench: &Bench,
    mode: &Mode,
    scene: ([f64; 3], f64),
    build: impl FnOnce(&Stage) -> Kept,
) -> Measured {
    staged_case(bench, mode, measure_orbit, scene, build)
}

/// `orbit_case` with the frames measured by `measure`.
fn staged_case(
    bench: &Bench,
    mode: &Mode,
    measure: Measure,
    (center, radius): ([f64; 3], f64),
    build: impl FnOnce(&Stage) -> Kept,
) -> Measured {
    let stage = Stage::new(&bench.driver, center, radius);
    let kept = build(&stage);
    let measured = measure(bench, &stage, mode);
    drop(kept);
    stage.close();
    measured
}

/// Displays one structure whose one group draws `array` with `aspect`.
pub fn display_array(stage: &Stage, array: &Array, aspect: &Aspect) -> Handle<Graphic3dStructure> {
    let structure = stage.structure();
    let group = structure.new_group().expect("group");
    group.set_group_primitives_aspect(aspect);
    group.add_primitive_array(&array.upcast(), true);
    structure.display();
    structure
}

fn frame_empty(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 10.), |_| Vec::new())
}

fn frame_mesh_1m(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 10.), |stage| {
        lit_sphere(stage, [1000, 500])
    })
}

/// One 10,000,000-triangle mesh: the vertex throughput of the GPU.
fn frame_mesh_10m(bench: &Bench, mode: &Mode) -> Measured {
    let build = |stage: &Stage| lit_sphere(stage, [4000, 1250]);
    staged_case(bench, mode, measure_heavy_orbit, ([0.; 3], 10.), build)
}

/// Displays the orange sphere of radius 10 and `[lon, lat]` quads, `2 * lon * lat` triangles.
fn lit_sphere(stage: &Stage, [lon, lat]: [i32; 2]) -> Kept {
    let aspect = geometry::lit_aspect([0.8, 0.5, 0.2], 0.);
    vec![display_array(
        stage,
        &geometry::uv_sphere(lon, lat, 10.),
        &aspect,
    )]
}

/// A 100 x 100 grid of boxes, one structure each: the per-draw overhead.
fn frame_structures_10k(bench: &Bench, mode: &Mode) -> Measured {
    box_grid_case(bench, mode, measure_orbit, GRID_10K)
}

/// A 400 x 250 grid of boxes, one structure each: the per-draw overhead of an assembly.
fn frame_structures_100k(bench: &Bench, mode: &Mode) -> Measured {
    box_grid_case(bench, mode, measure_heavy_orbit, GRID_100K)
}

/// Measures the box grid of `[columns, rows]` with `measure`.
fn box_grid_case(bench: &Bench, mode: &Mode, measure: Measure, grid: [i32; 2]) -> Measured {
    let scene = ([0.; 3], box_grid_radius(grid));
    staged_case(bench, mode, measure, scene, |stage| box_grid(stage, grid))
}

/// Displays `[columns, rows]` boxes of half size 1 three units apart, centred on the origin,
/// one structure each; `box_grid(&stage, GRID_10K)`.
pub fn box_grid(stage: &Stage, [columns, rows]: [i32; 2]) -> Kept {
    let aspect = geometry::lit_aspect([0.2, 0.6, 0.9], 0.);
    let center = |index: i32| {
        [
            cell(index % columns, columns),
            cell(index / columns, rows),
            0.,
        ]
    };
    (0..columns * rows)
        .map(|index| display_array(stage, &geometry::box_array(center(index), 1.), &aspect))
        .collect()
}

/// The radius that frames `box_grid`: the half diagonal of its cells, rounded up (213 for
/// 100 x 100, 708 for 400 x 250); `box_grid_radius(GRID_10K)`.
pub fn box_grid_radius([columns, rows]: [i32; 2]) -> f64 {
    let (half_width, half_depth) = (1.5 * f64::from(columns), 1.5 * f64::from(rows));
    (half_width * half_width + half_depth * half_depth)
        .sqrt()
        .ceil()
}

/// The centre of cell `index` of a row of `count` cells three units apart.
fn cell(index: i32, count: i32) -> f64 {
    3.0 * f64::from(index) - 1.5 * f64::from(count - 1)
}

fn frame_segments_1m(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 708.), |stage| {
        let array = geometry::segment_field(1_000_000);
        vec![display_array(stage, &array, &geometry::line_aspect())]
    })
}

fn frame_points_1m(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 708.), |stage| {
        let array = geometry::point_field(1_000_000);
        vec![display_array(stage, &array, &geometry::point_aspect())]
    })
}

/// One thousand 16-pixel labels in one group.
fn frame_text_1k(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 240.), |stage| {
        let structure = stage.structure();
        let group = structure.new_group().expect("group");
        group.set_group_primitives_aspect(&geometry::text_aspect());
        for index in 0..1000 {
            group.add_text(&geometry::text_label(index), true);
        }
        structure.display();
        vec![structure]
    })
}

/// A 10 x 10 x 10 block of half-transparent boxes, one structure each.
fn frame_transparent_1k(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 27.), |stage| {
        let aspect = geometry::lit_aspect([0.9, 0.3, 0.3], 0.5);
        let center = |index: i32| {
            [
                cell(index % 10, 10),
                cell(index / 10 % 10, 10),
                cell(index / 100, 10),
            ]
        };
        (0..1000)
            .map(|index| display_array(stage, &geometry::box_array(center(index), 1.), &aspect))
            .collect()
    })
}

/// A 102,000-triangle sphere drawn with four samples per pixel.
fn frame_msaa4_100k(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 10.), |stage| {
        stage
            .view
            .change_rendering_params(&mut |params| params.nb_msaa_samples = 4);
        lit_sphere(stage, [300, 170])
    })
}
