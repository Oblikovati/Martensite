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
];

/// Warms up, then times `mode.frames` orbit frames per run and counts the start frame.
pub fn measure_orbit(stage: &Stage, mode: &Mode) -> Measured {
    for frame in 0..mode.warmup {
        stage.redraw_orbit(frame);
    }
    let nanos = timed_runs(mode, mode.frames, |frame| stage.redraw_orbit(frame));
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
    (center, radius): ([f64; 3], f64),
    build: impl FnOnce(&Stage) -> Kept,
) -> Measured {
    let stage = Stage::new(&bench.driver, center, radius);
    let kept = build(&stage);
    let measured = measure_orbit(&stage, mode);
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
        let aspect = geometry::lit_aspect([0.8, 0.5, 0.2], 0.);
        vec![display_array(
            stage,
            &geometry::uv_sphere(1000, 500, 10.),
            &aspect,
        )]
    })
}

/// A 100 x 100 grid of boxes, one structure each: the per-draw overhead.
fn frame_structures_10k(bench: &Bench, mode: &Mode) -> Measured {
    orbit_case(bench, mode, ([0.; 3], 213.), |stage| {
        let aspect = geometry::lit_aspect([0.2, 0.6, 0.9], 0.);
        let center = |index: i32| [cell(index % 100, 100), cell(index / 100, 100), 0.];
        (0..10_000)
            .map(|index| display_array(stage, &geometry::box_array(center(index), 1.), &aspect))
            .collect()
    })
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
        let aspect = geometry::lit_aspect([0.8, 0.5, 0.2], 0.);
        vec![display_array(
            stage,
            &geometry::uv_sphere(300, 170, 10.),
            &aspect,
        )]
    })
}
