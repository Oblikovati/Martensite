//! Adjacency primitive arrays against native dumps.
//!
//! the recorded adjacency_arrays capture draws one array of each
//! adjacency kind alone in a 200x200 orthographic view, in a group given an unlit red fill
//! aspect and then a green line aspect. GlGroup keeps the last aspect, and
//! GlPrimitiveArray draws GL_*_ADJACENCY with the face program and no geometry shader, so
//! GL rasterizes the inner segment or triangle of each group in green. Native llvmpipe
//! (SSE2/128) and radeonsi agree except for one edge pixel of the triangle strip, and the
//! programmable and fixed-function pipelines draw identical dumps on each driver
//! (fixtures/adjacency_arrays/<driver>.txt holds the probe's FNV lines). Lavapipe is graded
//! byte for byte; RADV against radeonsi keeps two AMD snapping residues, graded by their
//! approved cross-driver bounds: a line-strip column whose line passes 0.0006 px from the pixel
//! corner and a triangle-strip pixel whose edge passes 0.0006 px from its centre.
//!
//! The "-after-wide-segments" cases draw a group of blue width-4 segments first. Only GL_LINES
//! and GL_LINE_STRIP draws call GlContext::SetLineWidth, so the line adjacency after them
//! keeps width 4.
#[path = "support/native_dump_grading.rs"]
#[allow(dead_code)]
mod grading;
#[path = "support/snapped_tie_grade.rs"]
#[allow(dead_code)]
mod snapped_tie;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectWindowMembers as _;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectNeutralWindow, AspectTypeOfLine};
use martensite_tkservice::graphic3d::{Graphic3dTypeOfPrimitiveArray as Kind, *};
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline,
};

const SIZE: (usize, usize) = (200, 200);

/// The probe's arrays in dump order: name, kind, vertices (z = 0) and whether the blue
/// width-4 segments draw first.
const CASES: [(&str, Kind, &[[f32; 2]], bool); 6] = [
    (
        "lines-adjacency",
        Kind::LinesAdjacency,
        &[[-45., -40.], [-30.3, -20.6], [30.7, 25.2], [45., 40.]],
        false,
    ),
    (
        "line-strip-adjacency",
        Kind::LineStripAdjacency,
        LINE_STRIP,
        false,
    ),
    (
        "triangles-adjacency",
        Kind::TrianglesAdjacency,
        &[
            [-30.3, -20.6],
            [0., -45.],
            [30.7, -20.1],
            [45., 10.],
            [0.4, 25.2],
            [-45., 10.],
        ],
        false,
    ),
    (
        "triangle-strip-adjacency",
        Kind::TriangleStripAdjacency,
        &[
            [-40.3, -20.6],
            [-45., 30.],
            [-10.2, -20.1],
            [-45., -45.],
            [-25.4, 25.2],
            [45., -45.],
            [20.6, 24.8],
            [45., 45.],
        ],
        false,
    ),
    (
        "lines-adjacency-after-wide-segments",
        Kind::LinesAdjacency,
        &[[-45., -40.], [-30.3, -20.6], [30.7, 25.2], [45., 40.]],
        true,
    ),
    (
        "line-strip-adjacency-after-wide-segments",
        Kind::LineStripAdjacency,
        LINE_STRIP,
        true,
    ),
];
const LINE_STRIP: &[[f32; 2]] = &[
    [-45., 0.],
    [-30.3, -20.6],
    [0.4, 25.2],
    [30.7, -20.1],
    [45., 0.],
];
/// The blue width-4 segments of the "-after-wide-segments" cases.
const WIDE_SEGMENTS: [[f32; 2]; 4] = [[-45., -44.3], [45., -44.3], [-45., 44.3], [45., 44.3]];

const LLVMPIPE: [&[u8]; 6] = [
    include_bytes!("fixtures/adjacency_arrays/llvmpipe/lines-adjacency.ppm"),
    include_bytes!("fixtures/adjacency_arrays/llvmpipe/line-strip-adjacency.ppm"),
    include_bytes!("fixtures/adjacency_arrays/llvmpipe/triangles-adjacency.ppm"),
    include_bytes!("fixtures/adjacency_arrays/llvmpipe/triangle-strip-adjacency.ppm"),
    include_bytes!("fixtures/adjacency_arrays/llvmpipe/lines-adjacency-after-wide-segments.ppm"),
    include_bytes!(
        "fixtures/adjacency_arrays/llvmpipe/line-strip-adjacency-after-wide-segments.ppm"
    ),
];
/// radeonsi equals llvmpipe byte for byte apart from the triangle strip.
const AMD: [&[u8]; 6] = [
    LLVMPIPE[0],
    LLVMPIPE[1],
    LLVMPIPE[2],
    include_bytes!("fixtures/adjacency_arrays/amd/triangle-strip-adjacency.ppm"),
    LLVMPIPE[4],
    LLVMPIPE[5],
];

/// The probe's view and the viewer and window it keeps alive.
struct AdjacencyView {
    view: Handle<dyn V3dViewSource>,
    viewer: Handle<V3dViewer>,
    _window: Handle<AspectNeutralWindow>,
}

impl AdjacencyView {
    fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        let v3d = view.get().unwrap();
        v3d.set_auto_z_fit_mode(false, 1.).unwrap();
        let camera = v3d.camera();
        camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
        camera.set_eye(&Point3::new(0., 0., 10.)).unwrap();
        camera.set_center(&Point3::new(0., 0., 0.)).unwrap();
        camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
        camera.set_scale(100.);
        camera.set_z_range(1., 20.).unwrap();
        Self {
            view,
            viewer,
            _window: window,
        }
    }

    /// The array in a group with an unlit red fill aspect, then a green line aspect, after the
    /// group of wide segments when `wide_first`.
    fn adjacency(
        &self,
        (kind, points, wide_first): (Kind, &[[f32; 2]], bool),
    ) -> Handle<Graphic3dStructure> {
        let structure =
            Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap();
        if wide_first {
            wide_segments(&structure);
        }
        let group = structure.new_group().unwrap();
        let fill = Handle::new(Graphic3dAspectFillArea3d::default());
        fill.set_interior_style(AspectInteriorStyle::Solid);
        fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
        fill.set_interior_color(Color::named(NamedColor::RED));
        group.set_group_primitives_aspect(&fill.cast());
        let line = Graphic3dAspectLine3d::new(
            Color::named(NamedColor::GREEN),
            AspectTypeOfLine::Solid,
            1.,
        )
        .unwrap();
        group.set_group_primitives_aspect(&Handle::new(line).cast());
        let array = Handle::new(
            Graphic3dArrayOfPrimitives::new(kind, points.len() as i32, 0, 0, 0).unwrap(),
        );
        for [x, y] in points {
            array.add_vertex(*x, *y, 0.).unwrap();
        }
        group.add_primitive_array(&array, true);
        structure
    }

    fn dump(&self) -> Vec<u8> {
        let v3d = self.view.get().unwrap();
        v3d.redraw();
        let image = ImagePixMap::default();
        image.set_top_down(true);
        let options = V3dImageDumpOptions {
            width: SIZE.0 as i32,
            height: SIZE.1 as i32,
            ..Default::default()
        };
        assert!(v3d.view_base().to_pix_map(&image, &options).unwrap());
        let storage = image.data();
        (0..SIZE.1)
            .flat_map(|y| storage.data.row(y).unwrap()[..SIZE.0 * 3].to_vec())
            .collect()
    }
}

/// A group of blue width-4 `GL_LINES` segments (`WIDE_SEGMENTS`).
fn wide_segments(structure: &Handle<Graphic3dStructure>) {
    let group = structure.new_group().unwrap();
    let blue = Color::named(NamedColor::BLUE1);
    let line = Graphic3dAspectLine3d::new(blue, AspectTypeOfLine::Solid, 4.).unwrap();
    group.set_group_primitives_aspect(&Handle::new(line).cast());
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Segments, 4, 0, 0, 0).unwrap());
    for [x, y] in WIDE_SEGMENTS {
        array.add_vertex(x, y, 0.).unwrap();
    }
    group.add_primitive_array(&array, true);
}

/// Each case displayed alone and dumped, in `CASES` order.
fn adjacency_dumps(driver: &Handle<Graphic3dRenderDriver>) -> Vec<Vec<u8>> {
    let scene = AdjacencyView::new(driver);
    CASES
        .iter()
        .map(|(_, kind, points, wide_first)| {
            let structure = scene.adjacency((*kind, points, *wide_first));
            structure.display();
            let dump = scene.dump();
            structure.erase();
            dump
        })
        .collect()
}

/// (case, differing pixels) of every dump that differs from the device's native dump.
/// (case, failure) of every dump the device's grading rejects: lavapipe against llvmpipe byte
/// for byte; RADV against radeonsi under the AMD snapping bounds (`amd_failure`).
fn differing(
    driver: &Handle<Graphic3dRenderDriver>,
    dumps: &[Vec<u8>],
) -> Vec<(&'static str, String)> {
    let software = driver.device_name().contains("llvmpipe");
    let native = if software { LLVMPIPE } else { AMD };
    let names = CASES.map(|(name, _, _, _)| name);
    grading::write_dumps("MARTENSITE_ADJACENCY_DUMP_DIR", &names, dumps, SIZE);
    CASES
        .iter()
        .zip(dumps)
        .zip(native)
        .filter_map(|((case, rust), expected)| {
            let expected = grading::ppm_rgb(expected, SIZE);
            let failure = match software {
                true => exact_failure(rust, expected),
                false => amd_failure(case.1, case.2, rust, expected),
            };
            failure.map(|failure| (case.0, failure))
        })
        .collect()
}

fn exact_failure(rust: &[u8], native: &[u8]) -> Option<String> {
    let count = rust
        .chunks(3)
        .zip(native.chunks(3))
        .filter(|(a, b)| a != b)
        .count();
    (count > 0).then(|| format!("{count} pixels differ"))
}

/// The probe's view maps world (x, y) to top-down window (100 + 2x, 100 - 2y).
fn window([x, y]: [f32; 2]) -> [f64; 2] {
    [100. + 2. * f64::from(x), 100. - 2. * f64::from(y)]
}

/// The inner primitives of a case as window segments: the drawn lines of the line kinds, the
/// triangle edges of the triangle kinds (the GL adjacency rules of primitive_topology.rs).
fn window_segments(kind: Kind, points: &[[f32; 2]]) -> Vec<[[f64; 2]; 2]> {
    let pairs: Vec<[usize; 2]> = match kind {
        Kind::LinesAdjacency => vec![[1, 2]],
        Kind::LineStripAdjacency => (1..points.len() - 2).map(|i| [i, i + 1]).collect(),
        Kind::TrianglesAdjacency => vec![[0, 2], [2, 4], [4, 0]],
        _ => (0..(points.len() - 4) / 2)
            .flat_map(|i| {
                [
                    [2 * i, 2 * i + 2],
                    [2 * i + 2, 2 * i + 4],
                    [2 * i + 4, 2 * i],
                ]
            })
            .collect(),
    };
    let segment = |[a, b]: [usize; 2]| [window(points[a]), window(points[b])];
    pairs.into_iter().map(segment).collect()
}

/// AMD: one-pixel line columns follow the reviewed snapped-tie bound (support/snapped_tie_grade.rs),
/// and a fill pixel passes under the "edge through the pixel centre" rule (user decision
/// 2026-10-28): a triangle edge within RADV's 2^-8 px vertex snap of the pixel centre, with a
/// colour within one level of a colour within one pixel in the other image.
fn amd_failure(kind: Kind, points: &[[f32; 2]], rust: &[u8], native: &[u8]) -> Option<String> {
    let segments = window_segments(kind, points);
    let lines: Vec<snapped_tie::WindowLine> = match kind {
        Kind::LinesAdjacency | Kind::LineStripAdjacency => segments
            .iter()
            .map(|[from, to]| snapped_tie::WindowLine {
                from: *from,
                to: *to,
            })
            .collect(),
        _ => vec![],
    };
    let mut fill_edge =
        |pixel: (usize, usize), _margin: f64| edge_through_centre(&segments, pixel, rust, native);
    snapped_tie::grade_with(rust, native, SIZE.0, &lines, &mut fill_edge).err()
}

fn edge_through_centre(
    segments: &[[[f64; 2]; 2]],
    (column, row): (usize, usize),
    rust: &[u8],
    native: &[u8],
) -> Result<(), String> {
    let centre = [column as f64 + 0.5, row as f64 + 0.5];
    let distance = segments
        .iter()
        .map(|segment| segment_distance(segment, centre))
        .fold(f64::MAX, f64::min);
    if distance > 1. / 256. {
        return Err(format!(
            "pixel ({column}, {row}) is {distance} px from every edge"
        ));
    }
    let pixel = |image: &[u8], c: usize, r: usize| image[(r * SIZE.0 + c) * 3..][..3].to_vec();
    let near = |image: &[u8], colour: &[u8]| {
        (row.saturating_sub(1)..=(row + 1).min(SIZE.1 - 1)).any(|r| {
            (column.saturating_sub(1)..=(column + 1).min(SIZE.0 - 1)).any(|c| {
                let other = pixel(image, c, r);
                colour.iter().zip(&other).all(|(a, b)| a.abs_diff(*b) <= 1)
            })
        })
    };
    let (mine, theirs) = (pixel(rust, column, row), pixel(native, column, row));
    match near(native, &mine) && near(rust, &theirs) {
        true => Ok(()),
        false => Err(format!(
            "pixel ({column}, {row}) has no colour within one pixel"
        )),
    }
}

fn segment_distance([a, b]: &[[f64; 2]; 2], p: [f64; 2]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let t = (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / (dx * dx + dy * dy)).clamp(0., 1.);
    (martensite_cmath::powi(a[0] + t * dx - p[0], 2)
        + martensite_cmath::powi(a[1] + t * dy - p[1], 2))
    .sqrt()
}

#[test]
fn programmable_adjacency_arrays_draw_their_inner_primitives_as_natively() {
    let driver = Graphic3dRenderDriver::new(grading::validation_options()).unwrap();
    let dumps = adjacency_dumps(&driver);
    let differing = differing(&driver, &dumps);
    let device = driver.device_name();
    assert!(
        differing.is_empty(),
        "device={device}; differing (case, pixels): {differing:?}"
    );
}

#[test]
fn fixed_function_adjacency_arrays_draw_their_inner_primitives_as_natively() {
    let driver = Graphic3dRenderDriver::with_render_pipelines(
        grading::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::default(),
        RenderLightingPipeline::FixedFunction,
    )
    .unwrap();
    let dumps = adjacency_dumps(&driver);
    let differing = differing(&driver, &dumps);
    let device = driver.device_name();
    assert!(
        differing.is_empty(),
        "device={device}; differing (case, pixels): {differing:?}"
    );
}
