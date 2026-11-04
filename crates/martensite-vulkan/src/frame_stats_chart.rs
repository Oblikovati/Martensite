//! GlFrameStatsPrs::updateChart: stacked timer bins of the data-frame history,
//! the white frame lines and the three time labels, as retained Graphic3d content.
use crate::RenderFrameStats;
use martensite_tkernel::ncollection::Vec4;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfPrimitives, Graphic3dFrameStatsMembers as _, Graphic3dFrameStatsTimer as Timer,
    Graphic3dHorizontalTextAlignment as HAlign, Graphic3dRenderingParams,
    Graphic3dTransformPersMembers as _, Graphic3dTypeOfPrimitiveArray as Kind,
    Graphic3dVerticalTextAlignment as VAlign,
};

/// Timers stacked in each bin, bottom first, with their native RGBA colors.
const CHART_TIMERS: [(Timer, [u8; 4]); 4] = [
    (Timer::CpuDynamics, [255, 0, 0, 127]),
    (Timer::CpuPicking, [255, 127, 39, 127]),
    (Timer::CpuCulling, [255, 0, 0, 127]),
    (Timer::ElapsedFrame, [0, 255, 0, 127]),
];

/// The chart content in corner-relative pixels; `ChartContent::build(&stats, &params, [w, h])`.
pub(crate) struct ChartContent {
    pub bins: Handle<Graphic3dArrayOfPrimitives>,
    pub frame_lines: Handle<Graphic3dArrayOfPrimitives>,
    /// Label text, position and horizontal alignment, top label first.
    pub labels: [(String, Point3, HAlign); 3],
    pub label_vertical: VAlign,
}

/// The chart extent, offset and bin size in pixels.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartLayout {
    pub size: [i32; 2],
    pub offset: [i32; 2],
    pub bin: [f64; 2],
    pub right_labels: bool,
}

impl ChartContent {
    /// The chart of a multi-frame history; `None` with one data frame, as native.
    pub fn build(
        stats: &RenderFrameStats,
        params: &Graphic3dRenderingParams,
        viewport: [i32; 2],
    ) -> Option<Self> {
        let frames = stats.data_frames();
        if frames.len() <= 1 {
            return None;
        }
        let layout = chart_layout(params, viewport, frames.len());
        let max_duration = chart_max_duration(params, stats);
        Some(Self {
            bins: chart_bins(stats, &layout, max_duration),
            frame_lines: chart_frame_lines(&layout),
            labels: chart_labels(&layout, max_duration),
            label_vertical: VAlign::Center,
        })
    }
}

/// `StatsMaxChartTime`, or the history maximum rounded to 10 ms within [5 ms, 100 ms].
pub(crate) fn chart_max_duration(
    params: &Graphic3dRenderingParams,
    stats: &RenderFrameStats,
) -> f64 {
    let configured = f64::from(params.stats.stats_max_chart_time);
    if configured > 0.0 {
        return configured;
    }
    let longest = stats
        .data_frames()
        .iter()
        .map(|frame| frame[Timer::ElapsedFrame])
        .fold(configured, f64::max);
    let rounded = (longest * 1000.0 * 0.1).ceil() * 0.001 * 10.0;
    rounded.clamp(0.005, 0.1)
}

/// The native chart size defaults (half width, 15% height) and corner offsets.
pub(crate) fn chart_layout(
    params: &Graphic3dRenderingParams,
    viewport: [i32; 2],
    bins: usize,
) -> ChartLayout {
    let requested = params.stats.chart_size;
    let width = if requested.x() <= 0 {
        viewport[0] / 2
    } else {
        requested.x()
    };
    let height = if requested.y() <= 0 {
        (0.15 * f64::from(viewport[1])) as i32
    } else {
        requested.y()
    };
    let corner = chart_corner(&params.stats.chart_position);
    ChartLayout {
        size: [width, height],
        offset: chart_offset(corner, [width, height]),
        bin: [
            f64::from(width) / bins as f64,
            0.15 * f64::from(viewport[1]),
        ],
        right_labels: corner.is_some_and(|c| c.0 & Corner::RIGHT.0 != 0),
    }
}

fn chart_corner(
    position: &Handle<martensite_tkservice::graphic3d::Graphic3dTransformPers>,
) -> Option<Corner> {
    let pers = position.get().filter(|pers| pers.is_trihedron_or_2d())?;
    pers.corner_2d().ok()
}

fn chart_offset(corner: Option<Corner>, [width, height]: [i32; 2]) -> [i32; 2] {
    let Some(corner) = corner else {
        return [0, 0];
    };
    let x = match corner.0 {
        c if c & Corner::LEFT.0 != 0 => 0,
        c if c & Corner::RIGHT.0 != 0 => -width,
        _ => -width / 2,
    };
    let y = match corner.0 {
        c if c & Corner::BOTTOM.0 != 0 => height,
        c if c & Corner::TOP.0 != 0 => 0,
        _ => height / 2,
    };
    [x, y]
}

/// Four quads per frame bin, oldest frame on the left; timers above the elapsed bar stack.
fn chart_bins(
    stats: &RenderFrameStats,
    layout: &ChartLayout,
    max_duration: f64,
) -> Handle<Graphic3dArrayOfPrimitives> {
    let frames = stats.data_frames();
    let count = frames.len();
    let quads = count * CHART_TIMERS.len();
    let array = Graphic3dArrayOfPrimitives::new(
        Kind::Triangles,
        (quads * 4) as i32,
        0,
        (quads * 6) as i32,
        martensite_tkservice::graphic3d::GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    )
    .expect("chart array within native index limits");
    let last = usize::try_from(stats.last_data_frame_index()).unwrap_or(0);
    for bin in 0..count {
        let frame = &frames[(last + 1 + bin) % count];
        let times = CHART_TIMERS.map(|(timer, _)| frame[timer]);
        add_bin(
            &array,
            layout,
            bin,
            stacked_heights(times, max_duration, layout.bin[1]),
        );
    }
    Handle::new(array)
}

/// Bottom and top of each timer bar: CPU timers stack, the elapsed bar starts at zero.
fn stacked_heights(times: [f64; 4], max_duration: f64, bin_height: f64) -> [[f64; 2]; 4] {
    let mut heights = [[0.0; 2]; 4];
    let (mut elapsed, mut current) = (0.0, 0.0);
    for (index, time) in times.iter().enumerate() {
        let is_elapsed = index == CHART_TIMERS.len() - 1;
        elapsed = if is_elapsed { *time } else { elapsed + time };
        let size = (elapsed / max_duration).min(1.2) * bin_height;
        heights[index] = [current, size];
        current = if is_elapsed { 0.0 } else { size };
    }
    heights
}

fn add_bin(
    array: &Graphic3dArrayOfPrimitives,
    layout: &ChartLayout,
    bin: usize,
    heights: [[f64; 2]; 4],
) {
    let x1 = f64::from(layout.offset[0]) + bin as f64 * layout.bin[0];
    let x2 = x1 + layout.bin[0];
    let base = f64::from(layout.offset[1]) - layout.bin[1];
    for ((_, color), [y1, y2]) in CHART_TIMERS.iter().zip(heights) {
        let first = add_colored(array, [x1, base + y2], *color);
        add_colored(array, [x1, base + y1], *color);
        add_colored(array, [x2, base + y1], *color);
        add_colored(array, [x2, base + y2], *color);
        array
            .add_edges3(first, first + 1, first + 3)
            .expect("chart edges");
        array
            .add_edges3(first + 1, first + 2, first + 3)
            .expect("chart edges");
    }
}

fn add_colored(array: &Graphic3dArrayOfPrimitives, [x, y]: [f64; 2], color: [u8; 4]) -> i32 {
    let [r, g, b, a] = color;
    array
        .add_vertex_pnt_color4(&Point3::new(x, y, 0.0), Vec4::new(r, g, b, a))
        .expect("chart vertex")
}

/// The top and bottom chart borders as two white segments.
fn chart_frame_lines(layout: &ChartLayout) -> Handle<Graphic3dArrayOfPrimitives> {
    let array =
        Graphic3dArrayOfPrimitives::new(Kind::Segments, 4, 0, 0, 0).expect("chart frame lines");
    let [x, y] = layout.offset.map(f64::from);
    let right = x + f64::from(layout.size[0]);
    let bottom = y - layout.bin[1];
    for [px, py] in [[x, y], [right, y], [x, bottom], [right, bottom]] {
        // GlFrameStatsPrs stores these coordinates as floats.
        let point = Point3::new(f64::from(px as f32), f64::from(py as f32), 0.0);
        array.add_vertex_pnt(&point).expect("chart frame vertex");
    }
    Handle::new(array)
}

fn chart_labels(layout: &ChartLayout, max_duration: f64) -> [(String, Point3, HAlign); 3] {
    let align = if layout.right_labels {
        HAlign::Right
    } else {
        HAlign::Left
    };
    let x = if layout.right_labels {
        f64::from(layout.offset[0])
    } else {
        f64::from(layout.offset[0] + layout.size[0])
    };
    let y = f64::from(layout.offset[1]);
    let label = |text: String, y: f64| (text, Point3::new(x, f64::from(y as f32), 0.0), align);
    [
        label(format_time_ms(max_duration), y),
        label(format_time_ms(max_duration * 0.5), y - layout.bin[1] / 2.0),
        label("0 ms".to_string(), y - layout.bin[1]),
    ]
}

/// Native `formatTimeMs`: whole milliseconds and the one-decimal frame rate.
pub(crate) fn format_time_ms(seconds: f64) -> String {
    let fps = if seconds != 0.0 { 1.0 / seconds } else { 0.0 };
    format!("{} ms ({fps:.1} FPS)", (seconds * 1000.0) as i32)
}

#[cfg(test)]
#[path = "frame_stats_chart_tests.rs"]
mod tests;
