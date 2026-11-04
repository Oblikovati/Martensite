//! The horizontal triangle edges a dump submitted exactly on pixel-centre rows, written next
//! to its image (`<image>.ties.json`) so the harness can prove GL-vs-Vulkan exact ties from
//! the submitted geometry (the recording tool; exact-tie decision 2026-10-27).
#[path = "final_line_witness.rs"]
pub(crate) mod final_line_witness;
use martensite_tkservice::graphic3d::Graphic3dDataStructureManager;
use martensite_vulkan::{
    Graphic3dRenderView, LineStyle, PrimitiveDraw, PrimitiveTopology, RenderError, RenderFrameStep,
    RenderLayer, RenderVertex, RenderViewFrameHooks,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

/// Both drivers snap window coordinates to 1/256 pixel; an edge closer than half that step to
/// a pixel-centre row snaps onto it (less a margin for the float32 vertex transform).
const TIE_TOLERANCE: f64 = 1. / 512. - 1. / 16384.;

/// The draws of a view's last normal and immediate frame steps; installed as frame hooks.
#[derive(Default)]
pub(crate) struct SubmittedDraws {
    normal: Mutex<Vec<RenderLayer>>,
    immediate: Mutex<Vec<RenderLayer>>,
    observed_tile: AtomicBool,
}

impl RenderViewFrameHooks for SubmittedDraws {
    fn redraw(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.observe_tile(view);
        view.redraw_base(step)?;
        *self.normal.lock().expect("unpoisoned draw record") = step.layers().to_vec();
        Ok(())
    }
    fn redraw_immediate(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.observe_tile(view);
        view.redraw_immediate_base(step)?;
        *self.immediate.lock().expect("unpoisoned draw record") = step.layers().to_vec();
        Ok(())
    }
}

/// A triangle edge on a pixel-centre row: image row `y` (top-down), span `x0..=x1`, and the
/// side of its triangle (+1 below: a top edge, -1 above: a bottom edge).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TieEdge {
    pub y: f64,
    pub x0: f64,
    pub x1: f64,
    pub interior: i8,
}

impl SubmittedDraws {
    fn observe_tile(&self, view: &Graphic3dRenderView) {
        if view
            .camera()
            .get()
            .is_some_and(|camera| camera.tile().is_valid())
        {
            self.observed_tile.store(true, Ordering::Relaxed);
        }
    }

    /// Whether an actual observed frame used a tile camera; `record.observed_tile()`.
    pub(crate) fn observed_tile(&self) -> bool {
        self.observed_tile.load(Ordering::Relaxed)
    }

    /// Record the frames `view` draws from now on; `SubmittedDraws::install(&view)`.
    pub(crate) fn install(view: &Graphic3dRenderView) -> Arc<Self> {
        let record = Arc::new(Self::default());
        view.set_frame_hooks(Some(record.clone()));
        record
    }

    /// Tie edges of every recorded triangle in a `width` x `height` window.
    pub(crate) fn tie_edges(&self, width: f64, height: f64) -> Vec<TieEdge> {
        let triangles = self.triangles(width, height);
        triangles.iter().flat_map(triangle_tie_edges).collect()
    }

    /// Every recorded draw, normal frame first; `record.draws().len()`.
    pub(crate) fn draws(&self) -> Vec<PrimitiveDraw> {
        let normal = self.normal.lock().expect("unpoisoned draw record");
        let immediate = self.immediate.lock().expect("unpoisoned draw record");
        normal
            .iter()
            .chain(immediate.iter())
            .flat_map(|layer| layer.draws().iter().cloned())
            .collect()
    }

    /// Window positions of every recorded triangle; `record.triangles(409., 409.)`.
    pub(crate) fn triangles(&self, width: f64, height: f64) -> Vec<[[f64; 2]; 3]> {
        let normal = self.normal.lock().expect("unpoisoned draw record");
        let immediate = self.immediate.lock().expect("unpoisoned draw record");
        let draws = normal
            .iter()
            .chain(immediate.iter())
            .flat_map(|layer| layer.draws());
        draws
            .flat_map(|draw| window_triangles(draw, [width, height]))
            .collect()
    }
}

/// A submitted one-pixel line segment in window pixels (rows top-down), for the thin-line ties
/// of the recording tool (`ThinLine`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ThinLine {
    pub from: [f64; 2],
    pub to: [f64; 2],
    /// Normalized device depth at `from` and `to`, for the coincident-line proof of
    /// the recording tool (equal depths are an undefined locus).
    pub depth: [f64; 2],
}

impl SubmittedDraws {
    /// Derive one-pixel loci from recorded inputs; crossing-eye unit lines replay frame
    /// preparation, not GPU output. Unsupported contexts refuse; `record.thin_lines(409.,409.)?`.
    pub(crate) fn thin_lines(&self, width: f64, height: f64) -> Result<Vec<ThinLine>, RenderError> {
        let normal = self.normal.lock().expect("unpoisoned draw record");
        let immediate = self.immediate.lock().expect("unpoisoned draw record");
        let layers = normal.iter().chain(immediate.iter()).cloned().collect();
        let draws = final_line_witness::prepare_layers(layers, [width, height])?;
        Ok(draws
            .iter()
            .flat_map(|draw| window_segments(draw, [width, height]))
            .collect())
    }
}

/// The `thin_lines` sidecar entries; `thin_lines_json(&lines)`.
fn thin_lines_json(lines: &[ThinLine]) -> String {
    let rows: Vec<String> = lines
        .iter()
        .map(|l| {
            let ([x0, y0], [x1, y1], [z0, z1]) = (l.from, l.to, l.depth);
            format!(
                "{{\"x0\": {x0:?}, \"y0\": {y0:?}, \"x1\": {x1:?}, \"y1\": {y1:?}, \"z0\": {z0:?}, \"z1\": {z1:?}}}"
            )
        })
        .collect();
    rows.join(", ")
}

/// Window segments of a line draw one pixel wide; other draws have none.
fn window_segments(draw: &PrimitiveDraw, size: [f64; 2]) -> Vec<ThinLine> {
    let one_pixel = draw
        .line_style()
        .is_some_and(|style| style.parameters().0 == 1.);
    if draw.topology() != PrimitiveTopology::Lines || !one_pixel {
        return vec![];
    }
    let vertices = draw.vertices();
    let ends: Vec<usize> = if draw.indices().is_empty() {
        (0..vertices.len()).collect()
    } else {
        draw.indices().iter().map(|&i| i as usize).collect()
    };
    let project = |i: usize| window_point_depth(draw.clip_transform(), vertices.get(i)?, size);
    let pairs = ends.as_chunks::<2>().0.iter();
    let segment = |a: usize, b: usize| {
        let ((from, z0), (to, z1)) = (project(a)?, project(b)?);
        Some(ThinLine {
            from,
            to,
            depth: [z0, z1],
        })
    };
    let segments = pairs.map(|[a, b]| segment(*a, *b));
    segments.flatten().collect()
}

/// A submitted one-pixel dashed segment in window pixels with its stipple, for the dash-phase
/// loci of the recording tool (`DashedLine`): dashed line draws and the triangle
/// edges of dashed polygon-edge passes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DashedLine {
    pub segment: [[f64; 2]; 2],
    pub pattern: u16,
    pub factor: u16,
}

impl SubmittedDraws {
    /// Every one-pixel dashed segment of the recorded frames; `record.dashed_lines(409., 409.)`.
    pub(crate) fn dashed_lines(&self, width: f64, height: f64) -> Vec<DashedLine> {
        let normal = self.normal.lock().expect("unpoisoned draw record");
        let immediate = self.immediate.lock().expect("unpoisoned draw record");
        let draws = normal
            .iter()
            .chain(immediate.iter())
            .flat_map(|layer| layer.draws());
        draws
            .flat_map(|draw| window_dashed_lines(draw, [width, height]))
            .collect()
    }
}

/// Dashed segments of one draw: a dashed one-pixel line draw, or the triangle edges of a dashed
/// one-pixel polygon-edge pass.
fn window_dashed_lines(draw: &PrimitiveDraw, size: [f64; 2]) -> Vec<DashedLine> {
    let dashed = |style: Option<LineStyle>| {
        style.filter(|s| s.parameters().0 == 1. && s.parameters().1 != u16::MAX)
    };
    let (segments, style): (Vec<[[f64; 2]; 2]>, _) = if let Some(style) = dashed(draw.line_style())
    {
        let lines = window_segments(draw, size).into_iter();
        (lines.map(|l| [l.from, l.to]).collect(), style)
    } else if let Some(style) = dashed(draw.polygon_edge_line_style()) {
        let edges = window_triangles(draw, size)
            .into_iter()
            .flat_map(|t| (0..3).map(move |k| [t[k], t[(k + 1) % 3]]));
        (edges.collect(), style)
    } else {
        return vec![];
    };
    let (_, pattern, factor) = style.parameters();
    let lines = segments.into_iter();
    lines
        .map(|segment| DashedLine {
            segment,
            pattern,
            factor,
        })
        .collect()
}

/// The `dashed_lines` sidecar entries; `dashed_lines_json(&dashed)`.
fn dashed_lines_json(lines: &[DashedLine]) -> String {
    let rows: Vec<String> = lines
        .iter()
        .map(|l| {
            let ([x0, y0], [x1, y1]) = (l.segment[0], l.segment[1]);
            let (pattern, factor) = (l.pattern, l.factor);
            format!(
                "{{\"x0\": {x0:?}, \"y0\": {y0:?}, \"x1\": {x1:?}, \"y1\": {y1:?}, \"pattern\": {pattern}, \"factor\": {factor}}}"
            )
        })
        .collect();
    rows.join(", ")
}

/// The JSON sidecar of `edges`, `lines`, `dashed` lines, one-pixel `points` and the snap
/// pixels; `tie_edges_json(&edges, &lines, &dashed, &points, &snaps, [409, 409])`.
pub(crate) fn tie_edges_json(
    edges: &[TieEdge],
    lines: &[ThinLine],
    dashed: &[DashedLine],
    points: &[[f64; 2]],
    snaps: &std::collections::BTreeSet<(i32, i32)>,
    size: [i32; 2],
) -> String {
    let rows: Vec<String> = edges
        .iter()
        .map(|e| {
            let (y, x0, x1, interior) = (e.y, e.x0, e.x1, e.interior);
            format!("{{\"y\": {y:?}, \"x0\": {x0:?}, \"x1\": {x1:?}, \"interior\": {interior}}}")
        })
        .collect();
    let [width, height] = size;
    format!(
        "{{\"size\": [{width}, {height}], \"horizontal_fill_edges\": [{}], \"thin_lines\": [{}], \"dashed_lines\": [{}], \"points\": [{}], {}}}\n",
        rows.join(", "),
        thin_lines_json(lines),
        dashed_lines_json(dashed),
        points_json(points),
        super::snap_pixels::snap_pixels_json(snaps)
    )
}

/// The `points` sidecar entries; `points_json(&[[3.0, 4.5]])`.
fn points_json(points: &[[f64; 2]]) -> String {
    let rows: Vec<String> = points
        .iter()
        .map(|[x, y]| format!("{{\"x\": {x:?}, \"y\": {y:?}}}"))
        .collect();
    rows.join(", ")
}

impl SubmittedDraws {
    /// The window position of every one-pixel point of the recorded frames (point draws whose
    /// style is one pixel without a sprite larger than one texel), for the point ties of
    /// the recording tool (`OnePixelPoint`); `record.one_pixel_points(409., 409.)`.
    pub(crate) fn one_pixel_points(&self, width: f64, height: f64) -> Vec<[f64; 2]> {
        let normal = self.normal.lock().expect("unpoisoned draw record");
        let immediate = self.immediate.lock().expect("unpoisoned draw record");
        let draws = normal
            .iter()
            .chain(immediate.iter())
            .flat_map(|layer| layer.draws());
        draws
            .flat_map(|draw| window_points(draw, [width, height]))
            .collect()
    }
}

/// Window positions of the vertices of a one-pixel point draw; other draws have none.
fn window_points(draw: &PrimitiveDraw, size: [f64; 2]) -> Vec<[f64; 2]> {
    let one_pixel = draw
        .point_style()
        .is_some_and(|style| style.parameters() == (1., [1, 1]));
    if draw.topology() != PrimitiveTopology::Points || !one_pixel {
        return vec![];
    }
    let vertices = draw.vertices();
    let project = |vertex| window_point(draw.clip_transform(), vertex, size);
    if draw.indices().is_empty() {
        return vertices.iter().filter_map(project).collect();
    }
    let indexed = draw
        .indices()
        .iter()
        .filter_map(|&i| vertices.get(i as usize));
    indexed.filter_map(project).collect()
}

/// Window positions (image rows top-down) of each triangle of a triangle draw.
fn window_triangles(draw: &PrimitiveDraw, size: [f64; 2]) -> Vec<[[f64; 2]; 3]> {
    if draw.topology() != PrimitiveTopology::Triangles || draw.rasterizes_polygon_points() {
        return vec![];
    }
    let vertices = draw.vertices();
    let corners: Vec<usize> = if draw.indices().is_empty() {
        (0..vertices.len()).collect()
    } else {
        draw.indices().iter().map(|&i| i as usize).collect()
    };
    let project = |i: usize| window_point(draw.clip_transform(), vertices.get(i)?, size);
    let triangles = corners.as_chunks::<3>().0.iter();
    let windowed = triangles.map(|[a, b, c]| Some([project(*a)?, project(*b)?, project(*c)?]));
    windowed.flatten().collect()
}

/// Clip transform (column-major) then the full-window viewport, rows counted top-down.
fn window_point(clip: &[f32; 16], vertex: &RenderVertex, size: [f64; 2]) -> Option<[f64; 2]> {
    window_point_depth(clip, vertex, size).map(|(point, _)| point)
}

/// `window_point` with the normalized device depth `z / w` of the vertex.
fn window_point_depth(
    clip: &[f32; 16],
    vertex: &RenderVertex,
    size: [f64; 2],
) -> Option<([f64; 2], f64)> {
    let [x, y, z] = vertex.position.map(f64::from);
    let point = [x, y, z, f64::from(vertex.weight)];
    let row = |r: usize| {
        (0..4)
            .map(|k| f64::from(clip[k * 4 + r]) * point[k])
            .sum::<f64>()
    };
    let w = row(3);
    if w <= 0. {
        return None;
    }
    let window = [
        (row(0) / w + 1.) * 0.5 * size[0],
        (1. - row(1) / w) * 0.5 * size[1],
    ];
    Some((window, row(2) / w))
}

/// The horizontal edges of `triangle` lying on a pixel-centre row.
fn triangle_tie_edges(triangle: &[[f64; 2]; 3]) -> Vec<TieEdge> {
    (0..3)
        .filter_map(|k| {
            let (a, b, apex) = (triangle[k], triangle[(k + 1) % 3], triangle[(k + 2) % 3]);
            let centre_row = (a[1] - 0.5).round() + 0.5;
            let on_row = |p: [f64; 2]| (p[1] - centre_row).abs() <= TIE_TOLERANCE;
            if !(on_row(a) && on_row(b)) || on_row(apex) {
                return None;
            }
            let interior = if apex[1] > centre_row { 1 } else { -1 };
            let (x0, x1) = (a[0].min(b[0]), a[0].max(b[0]));
            Some(TieEdge {
                y: a[1],
                x0,
                x1,
                interior,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_edges_on_pixel_centres_are_ties() {
        let triangle = [[10., 187.5], [30., 187.5], [20., 214.5]];
        let edges = triangle_tie_edges(&triangle);
        let top = TieEdge {
            y: 187.5,
            x0: 10.,
            x1: 30.,
            interior: 1,
        };
        assert_eq!(edges, vec![top]);
    }

    #[test]
    fn edges_between_pixel_centres_are_not_ties() {
        let triangle = [[10., 187.], [30., 187.], [20., 214.25]];
        assert!(triangle_tie_edges(&triangle).is_empty());
    }

    #[test]
    fn sidecar_names_size_and_edges() {
        let edge = TieEdge {
            y: 204.5,
            x0: 1.,
            x1: 2.,
            interior: -1,
        };
        let line = ThinLine {
            from: [1., 2.5],
            to: [9., 2.5],
            depth: [0.25, 0.5],
        };
        let dash = DashedLine {
            segment: [line.from, line.to],
            pattern: 0xFFC0,
            factor: 1,
        };
        let snaps = std::collections::BTreeSet::from([(5, 6)]);
        let json = tie_edges_json(&[edge], &[line], &[dash], &[[3., 4.5]], &snaps, [409, 409]);
        assert!(json.contains("\"size\": [409, 409]"), "{json}");
        assert!(json.contains("\"thin_lines\": [{\"x0\": 1.0, \"y0\": 2.5, \"x1\": 9.0, \"y1\": 2.5, \"z0\": 0.25, \"z1\": 0.5}]"), "{json}");
        assert!(
            json.contains("\"y\": 204.5, \"x0\": 1.0, \"x1\": 2.0, \"interior\": -1"),
            "{json}"
        );
        assert!(
            json.contains("\"points\": [{\"x\": 3.0, \"y\": 4.5}]"),
            "{json}"
        );
        let dashed = "\"dashed_lines\": [{\"x0\": 1.0, \"y0\": 2.5, \"x1\": 9.0, \"y1\": 2.5, \"pattern\": 65472, \"factor\": 1}]";
        assert!(json.contains(dashed), "{json}");
        assert!(json.contains("\"snap_pixels\": [[5, 6]]"), "{json}");
    }
}
