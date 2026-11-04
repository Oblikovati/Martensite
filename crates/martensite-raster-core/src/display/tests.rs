use super::*;
use crate::color::{MarkerShape, PaletteKind};
use martensite_tkmath::gp::{Circle2, Transform3};

/// A view that records what the display sends to its window.
struct RecordingSurface {
    zoom: f64,
    matrix: Transform3,
    perspective: Option<f64>,
    offset: (i32, i32),
    frame: (i32, i32, i32, i32),
    calls: Vec<String>,
    segments: Vec<PixelSegment>,
}

impl RecordingSurface {
    fn new(zoom: f64) -> Self {
        Self {
            zoom,
            matrix: Transform3::default(),
            perspective: None,
            offset: (100, -100),
            frame: (-1000, -1000, 1000, 1000),
            calls: Vec::new(),
            segments: Vec::new(),
        }
    }
}

impl DisplaySurface for RecordingSurface {
    fn zoom(&self) -> f64 {
        self.zoom
    }
    fn matrix(&self) -> Transform3 {
        self.matrix
    }
    fn is_perspective(&self) -> bool {
        self.perspective.is_some()
    }
    fn focal_distance(&self) -> f64 {
        self.perspective.unwrap_or(0.0)
    }
    fn dx(&self) -> i32 {
        self.offset.0
    }
    fn dy(&self) -> i32 {
        self.offset.1
    }
    fn frame(&mut self) -> (i32, i32, i32, i32) {
        self.frame
    }
    fn viewer_frame(&self) -> (i32, i32, i32, i32) {
        self.frame
    }
    fn view_type(&self) -> String {
        "+X+Y".to_string()
    }
    fn set_color(&mut self, color: PaletteKind) {
        self.calls.push(format!("color {}", color.index()));
    }
    fn set_mode(&mut self, mode: i32) {
        self.calls.push(format!("mode {mode}"));
    }
    fn draw_segments(&mut self, segments: &[PixelSegment]) {
        self.calls.push(format!("segments {}", segments.len()));
        self.segments.extend_from_slice(segments);
    }
    fn draw_string(&mut self, x: i32, y: i32, text: &str) {
        self.calls.push(format!("string {x} {y} {text}"));
    }
}

#[test]
fn a_2d_line_becomes_a_window_segment() {
    let mut surface = RecordingSurface::new(2.0);
    let mut state = DisplayState {
        bounds: RasterBounds::empty(),
        ..DisplayState::default()
    };
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    display.draw(Point2::new(1.0, 2.0), Point2::new(3.0, 4.0));
    display.flush();
    // x = 2 * x + dx, y = -(2 * y) - dy.
    assert_eq!(surface.segments, vec![PixelSegment::new(102, 96, 106, 92)]);
    assert_eq!(
        state.bounds,
        RasterBounds {
            x_min: 2.0,
            x_max: 6.0,
            y_min: 4.0,
            y_max: 8.0
        }
    );
}

#[test]
fn a_color_change_flushes_then_switches() {
    let mut surface = RecordingSurface::new(1.0);
    let mut state = DisplayState::default();
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    display.draw(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
    display.set_color(PaletteColor::new(PaletteKind::Rouge));
    display.set_color(PaletteColor::new(PaletteKind::Rouge));
    display.set_mode(6);
    assert_eq!(
        surface.calls,
        vec!["segments 1", "color 1", "segments 0", "mode 6"]
    );
}

#[test]
fn markers_do_not_grow_the_bounds_and_end_at_the_point() {
    let mut surface = RecordingSurface::new(1.0);
    let mut state = DisplayState {
        bounds: RasterBounds::empty(),
        ..DisplayState::default()
    };
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    display.draw_marker(Point2::new(10.0, 10.0), MarkerShape::Square, 5);
    display.flush();
    assert_eq!(surface.segments.len(), 4);
    // Only the final MoveTo(pt) grows the bounds.
    assert_eq!(
        state.bounds,
        RasterBounds {
            x_min: 10.0,
            x_max: 10.0,
            y_min: 10.0,
            y_max: 10.0
        }
    );
    assert!(state.draw_bounds);
}

#[test]
fn a_full_circle_is_a_closed_polyline() {
    let mut surface = RecordingSurface::new(1.0);
    let mut state = DisplayState::default();
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    let mut circle = Circle2::default();
    circle.set_radius(1.0).unwrap();
    display.draw_circle(&circle, 0.0, 2.0 * std::f64::consts::PI, true);
    display.flush();
    // Radius 1 px: the 6-point minimum, 5 recurrence steps plus the closing point.
    assert_eq!(surface.segments.len(), 6);
}

#[test]
fn pick_mode_hits_a_segment_near_the_pick_point() {
    let mut surface = RecordingSurface::new(1.0);
    let mut state = DisplayState {
        mode: DrawingMode::Pick,
        ..DisplayState::default()
    };
    state.pick = PickState {
        x: 5,
        y: 0,
        precision: 2,
        ..PickState::default()
    };
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    display.draw_3d(Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0));
    assert!(display.has_picked());
    assert_eq!(state.pick.last_p2, Point3::new(10.0, 0.0, 0.0));
    assert!(surface.segments.is_empty());
}

#[test]
fn postscript_mode_writes_moves_and_lines() {
    let mut surface = RecordingSurface::new(1.0);
    let mut state = DisplayState {
        mode: DrawingMode::PostScript,
        ..DisplayState::default()
    };
    state.postscript.kx = 1.0;
    state.postscript.ky = 1.0;
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    display.draw(Point2::new(1.0, 2.0), Point2::new(3.0, 4.0));
    display.draw_string(Point2::new(5.0, 6.0), "t");
    let text = state.postscript.stream.text();
    assert_eq!(
        text,
        "stroke\nnewpath\n1 2 m\n3 4 l\nstroke\n5 6 m\n(t) show\nnewpath\n"
    );
}

#[test]
fn perspective_projects_points_in_front_of_the_eye() {
    let mut surface = RecordingSurface::new(1.0);
    surface.perspective = Some(10.0);
    let mut state = DisplayState::default();
    let display = RasterDisplay::new(&mut surface, &mut state);
    let projected = display.project(Point3::new(2.0, 4.0, 5.0));
    assert_eq!((projected.x(), projected.y()), (4.0, 8.0));
}

#[test]
fn the_batch_display_draws_nothing() {
    let mut display = RasterDisplay::batch();
    display.draw(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));
    display.draw_string(Point2::new(0.0, 0.0), "x");
    assert_eq!(display.zoom(), 1.0);
    assert_eq!(display.view_id(), 0);
    assert!(!display.has_picked());
}

fn recorded_orientation_arrow() -> (Vec<PixelSegment>, RasterBounds, RasterBounds) {
    let mut surface = RecordingSurface::new(2.0);
    let mut state = DisplayState::default();
    let before = state.bounds;
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    crate::sampling::draw_orientation_arrow(
        &mut display,
        Point2::new(30., 40.),
        martensite_tkmath::gp::Vector2::new(1., 0.),
    )
    .unwrap();
    display.flush();
    (surface.segments, before, state.bounds)
}

#[test]
fn orientation_arrow_uses_fixed_screen_size_without_growing_bounds() {
    let (segments, before, after) = recorded_orientation_arrow();
    assert_eq!(
        segments,
        vec![
            PixelSegment::new(140, 10, 160, 20),
            PixelSegment::new(160, 20, 140, 30),
        ]
    );
    assert_eq!(after, before);
}

#[test]
fn orientation_arrow_ignores_zero_and_nan_directions() {
    let mut surface = RecordingSurface::new(2.0);
    let mut state = DisplayState::default();
    let mut display = RasterDisplay::new(&mut surface, &mut state);
    for x in [0., f64::NAN] {
        crate::sampling::draw_direction_arrow(
            &mut display,
            Point2::new(30., 40.),
            martensite_tkmath::gp::Vector2::new(x, 0.),
        )
        .unwrap();
    }
    display.flush();
    assert!(surface.segments.is_empty());
}
