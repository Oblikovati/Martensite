//! One regression run on a viewer session: the commands, its image dumps and its completion.
//!
//! The first dump publishes the session's capture, later dumps add to it, and completion
//! retires the views after the regression's end, keeping every image dump required.
#[path = "light_dot_selection.rs"]
mod light_dot_selection;
#[path = "line_clipping_selection.rs"]
mod line_clipping_selection;
#[cfg(feature = "vertex-output-diagnostics")]
#[path = "vertex_output_selection.rs"]
mod vertex_output_selection;
#[path = "world_clipping_selection.rs"]
mod world_clipping_selection;
use crate::case_view::viewer_session::ViewerSession;
use crate::hatch_surface::scene::SourceViewOutput as _;
use crate::source_output::{self, SourceCompletion};
use crate::tie_edges::{SubmittedDraws, tie_edges_json};
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{
    Graphic3dRenderDriver, Graphic3dRenderView, RenderCoordinateConvention, RenderLineWidthPolicy,
    RenderPipelinePolicies, RenderRasterPolicy,
};
use std::error::Error;

// One module instance keeps all diagnostic role scopes on the same exact-handle implementation.
#[path = "angle_role_observer.rs"]
mod angle_role_observer;
#[path = "capture_witness.rs"]
pub(crate) mod capture_witness;
#[path = "../support/presentation_boundary_scope.rs"]
#[allow(dead_code)]
pub(crate) mod presentation_role_scope;
#[path = "sized_dump.rs"]
mod sized_dump;
#[path = "stereo_dump_scope.rs"]
mod stereo_dump_scope;

/// A session before its first dump, or a published capture after it.
enum Stage {
    Open(Handle<Graphic3dRenderDriver>, ViewerSession),
    Captured(SourceCompletion<ViewerSession>),
}

/// The session of one regression with its dumps; `let mut run = SessionRun::new()?;`.
pub(crate) struct SessionRun {
    #[cfg(feature = "vertex-output-diagnostics")]
    vertex_output: Option<vertex_output_selection::CaseVertexOutput>,
    stage: Option<Stage>,
    pending_witness_error: Option<std::io::Error>,
    observed_dump_ordinal: usize,
}

impl SessionRun {
    /// A validation-enabled driver and a session with its first 409x409 view.
    /// Example: `SessionRun::new()?.session().fit()?`.
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        Self::with_view_size(409, 409)
    }

    /// A validation-enabled driver and a session with a first view of the given size.
    /// Example: `SessionRun::with_view_size(1024, 512)?`.
    pub(crate) fn with_view_size(width: i32, height: i32) -> Result<Self, Box<dyn Error>> {
        Self::with_view_configuration(width, height, RenderLineWidthPolicy::default())
    }

    /// Select raster widths before creating a view; `SessionRun::with_line_width_policy(policy)?`.
    pub(crate) fn with_line_width_policy(
        policy: RenderLineWidthPolicy,
    ) -> Result<Self, Box<dyn Error>> {
        Self::with_view_configuration(409, 409, policy)
    }

    fn with_view_configuration(
        width: i32,
        height: i32,
        line_widths: RenderLineWidthPolicy,
    ) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "vertex-output-diagnostics")]
        let vertex_output = vertex_output_selection::CaseVertexOutput::environment()?;
        #[cfg(feature = "vertex-output-diagnostics")]
        let driver = match &vertex_output {
            Some(request) => request.driver(line_widths)?,
            None => run_driver(line_widths)?,
        };
        #[cfg(not(feature = "vertex-output-diagnostics"))]
        let driver = run_driver(line_widths)?;
        let session = ViewerSession::with_view_size(&driver, width, height)?;
        Ok(Self {
            #[cfg(feature = "vertex-output-diagnostics")]
            vertex_output,
            stage: Some(Stage::Open(driver, session)),
            pending_witness_error: None,
            observed_dump_ordinal: 0,
        })
    }

    /// The live session; `run.session().move_cursor(0, 0)?`.
    pub(crate) fn session(&mut self) -> &mut ViewerSession {
        match self
            .stage
            .as_mut()
            .expect("a run is never left without a stage")
        {
            Stage::Open(_, session) => session,
            Stage::Captured(completion) => &mut completion.scene,
        }
    }

    /// Dump the active view to image `basename`, with the tie edges and one-pixel lines of the
    /// frame it drew (`<basename>.ties.json`), and the supersampled grid control when the
    /// harness requests it (`<basename>.ss15.png`, supersample_control.rs); `run.dump("case_1.png")?`.
    pub(crate) fn dump(&mut self, basename: &'static str) -> Result<(), Box<dyn Error>> {
        self.dump_using(basename, None, |run| run.capture(basename))
    }

    fn dump_using(
        &mut self,
        basename: &'static str,
        options: Option<&martensite_tkv3d::v3d::V3dImageDumpOptions>,
        capture: impl FnOnce(&mut Self) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = self.vertex_output.clone() {
            let observation = request.bind(self.session());
            capture_witness::retain_error(&mut self.pending_witness_error, observation);
        }
        let view = render_view(self.session())?;
        let view = view
            .get()
            .ok_or("a Vulkan render view behind the active view")?;
        let record = SubmittedDraws::install(view);
        self.capture_observed_dump(view, basename, capture)?;
        self.dump_observation(view, &record, basename, options)
    }

    fn capture_observed_dump(
        &mut self,
        view: &Graphic3dRenderView,
        _basename: &'static str,
        capture: impl FnOnce(&mut Self) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "command-state-diagnostics")]
        let scope = capture_witness::DumpScope::begin(view, _basename);
        let result = capture_then_clear(view, || capture(self));
        #[cfg(feature = "command-state-diagnostics")]
        scope.finish(view, result.is_ok());
        result
    }

    fn dump_witnesses(
        &mut self,
        view: &Graphic3dRenderView,
        record: &SubmittedDraws,
        basename: &'static str,
        size: (i32, i32),
    ) -> Result<(), Box<dyn Error>> {
        self.observe_dump(view, record, basename);
        write_ties_sidecar(record, basename, size)?;
        if crate::supersample_control::requested(basename) {
            let session = self.session();
            let draws = record.draws();
            crate::supersample_control::supersample_dump(&*session, view, basename, &draws)?;
        }
        Ok(())
    }

    /// Apply one dump's stereo projection without losing a returned error; `run.dump_stereo("a.png", option)?`.
    pub(crate) fn dump_stereo(
        &mut self,
        basename: &'static str,
        option: martensite_tkv3d::v3d::V3dStereoDumpOptions,
    ) -> Result<(), Box<dyn Error>> {
        stereo_dump_scope::with_stereo_dump(self, basename, option)
    }

    fn session_if_present(&mut self) -> Option<&mut ViewerSession> {
        match self.stage.as_mut()? {
            Stage::Open(_, session) => Some(session),
            Stage::Captured(completion) => Some(&mut completion.scene),
        }
    }

    fn capture(&mut self, basename: &'static str) -> Result<(), Box<dyn Error>> {
        let stage = self
            .stage
            .take()
            .expect("a run is never left without a stage");
        let captured = match stage {
            Stage::Open(driver, session) => source_output::capture_case(driver, session, basename)?,
            Stage::Captured(mut completion) => {
                completion.capture_additional(basename)?;
                completion
            }
        };
        self.stage = Some(Stage::Captured(captured));
        Ok(())
    }

    /// Defer diagnostics so authored checks and END still execute; `run.retain_observation(result)`.
    pub(crate) fn retain_observation(&mut self, result: Result<(), Box<dyn Error>>) {
        capture_witness::retain_error(&mut self.pending_witness_error, result);
    }

    /// Report completion, then retire the views; every dump must have been written.
    /// Example: `run.finish()?`.
    pub(crate) fn finish(mut self) -> Result<(), Box<dyn Error>> {
        println!("TEST COMPLETED");
        let source = match self
            .stage
            .take()
            .expect("a run is never left without a stage")
        {
            Stage::Captured(completion) => completion.finish_after_end(),
            Stage::Open(..) => Err(std::io::Error::other(
                "regression finished without an image dump; expected at least one dump",
            )
            .into()),
        };
        capture_witness::after_end(source, self.pending_witness_error.take())?;
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.vertex_output {
            request.finish()?;
        }
        Ok(())
    }
}

/// The harness's switch for the lavapipe attribute-rounding control run (user decision
/// 2026-10-28): the whole regression renders in the GL-equivalent bottom-up frame.
const BOTTOM_UP_CONTROL: &str = "MARTENSITE_CASE_BOTTOM_UP_CONTROL";

#[path = "coordinate_policy_selection.rs"]
mod coordinate_policy_selection;

#[path = "session_driver.rs"]
mod session_driver;
pub(crate) use session_driver::run_driver;

fn run_coordinate_policies(
    bottom_up: bool,
) -> (RenderPipelinePolicies, RenderCoordinateConvention) {
    if !bottom_up {
        return (Default::default(), Default::default());
    }
    let policies = RenderPipelinePolicies {
        raster: RenderRasterPolicy::LastProvokingVertexV2,
        ..Default::default()
    };
    (policies, RenderCoordinateConvention::LegacyBottomUpV1)
}

/// Write `<basename>.ties.json`: the tie edges, one-pixel lines, dashes, points and snap
/// pixels of the recorded frame in a `size` window; `write_ties_sidecar(&record, "a.png", (409, 409))?`.
pub(crate) fn write_ties_sidecar(
    record: &SubmittedDraws,
    basename: &str,
    (width, height): (i32, i32),
) -> Result<(), Box<dyn Error>> {
    let (w, h) = (f64::from(width), f64::from(height));
    let triangles = record.triangles(w, h);
    let snaps = crate::snap_pixels::snap_pixels(&triangles, [width, height]);
    let (edges, lines) = (record.tie_edges(w, h), record.thin_lines(w, h)?);
    let (points, dashed) = (record.one_pixel_points(w, h), record.dashed_lines(w, h));
    let json = tie_edges_json(&edges, &lines, &dashed, &points, &snaps, [width, height]);
    std::fs::write(
        source_output::capture_path(&format!("{basename}.ties.json"))?,
        json,
    )?;
    Ok(())
}

/// The Vulkan view behind the session's active view; `render_view(session)?`.
pub(crate) fn render_view(
    session: &ViewerSession,
) -> Result<Handle<Graphic3dRenderView>, Box<dyn Error>> {
    let view = session.view().get().ok_or("an active V3d view")?;
    Ok(view.view().cast())
}

/// A same-driver control of an image: the view dumped again to `<image>.<suffix>.png` with
/// the `erased` objects hidden (`uncovered`, `locus-1`, ...), as the native control run does.
pub(crate) struct ImageControl<'a> {
    pub suffix: &'a str,
    pub erased: &'a [&'a str],
}

impl SessionRun {
    /// Dump `basename` with its tie sidecar, then each control capture of it, showing the
    /// erased objects again after each; `run.dump_with_controls("a.png", &[control])?`.
    pub(crate) fn dump_with_controls(
        &mut self,
        basename: &'static str,
        controls: &[ImageControl<'_>],
    ) -> Result<(), Box<dyn Error>> {
        self.dump(basename)?;
        for control in controls {
            let session = self.session();
            session.named_objects(control.erased)?;
            session.erase(control.erased)?;
            source_output::control_capture::control_dump(&*session, basename, control.suffix)?;
            session.show_erased(control.erased)?;
        }
        Ok(())
    }
}

/// A scene change of a scripted control: applied before the control dump, undone after it.
pub(crate) type ControlChange<'a> = &'a dyn Fn(&mut ViewerSession) -> Result<(), Box<dyn Error>>;

/// A same-driver control that changes the scene by code where erasing whole objects cannot
/// express it (the view cube without its axes), dumped to `<image>.<suffix>.png`; the batch's
/// recorded control evaluates the matching commands.
pub(crate) struct ScriptedControl<'a> {
    pub suffix: &'a str,
    pub apply: ControlChange<'a>,
    pub restore: ControlChange<'a>,
}

impl SessionRun {
    /// Dump `basename` with its tie sidecar, then each scripted control of it, restoring the
    /// scene after each; `run.dump_with_scripted_controls("a.png", &[control])?`.
    pub(crate) fn dump_with_scripted_controls(
        &mut self,
        basename: &'static str,
        controls: &[ScriptedControl<'_>],
    ) -> Result<(), Box<dyn Error>> {
        self.dump(basename)?;
        for control in controls {
            (control.apply)(self.session())?;
            let session = self.session();
            source_output::control_capture::control_dump(&*session, basename, control.suffix)?;
            (control.restore)(self.session())?;
        }
        Ok(())
    }
}

/// The four axis-aligned camera shifts of the cross-driver shift control, in pixels: half the
/// 1/256 px snapping step (D2, AMD only, accepted 2026-10-28).
const HALF_STEP_SHIFTS: [[f64; 2]; 4] = [
    [1. / 512., 0.],
    [-1. / 512., 0.],
    [0., 1. / 512.],
    [0., -1. / 512.],
];

impl SessionRun {
    /// Dump `basename` (with the shift controls when `shifts`), then finish the run;
    /// `run.dump_and_finish("a.png", false)`.
    pub(crate) fn dump_and_finish(
        mut self,
        basename: &'static str,
        shifts: bool,
    ) -> Result<(), Box<dyn Error>> {
        if shifts {
            self.dump_with_shift_controls(basename)?;
        } else {
            self.dump(basename)?;
        }
        self.finish()
    }

    /// Dump `basename` with its tie sidecar, then the view again with the camera shifted by
    /// each half step (`<image>.shift-1.png` .. `shift-4.png`, `HALF_STEP_SHIFTS` order),
    /// restoring the camera after each; `run.dump_with_shift_controls("a.png")?`.
    pub(crate) fn dump_with_shift_controls(
        &mut self,
        basename: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        self.dump(basename)?;
        let session = self.session();
        let width = f64::from(session.source_window_size().0);
        for (index, [dx, dy]) in HALF_STEP_SHIFTS.into_iter().enumerate() {
            let source = session.view();
            let view = source.get().ok_or("an active V3d view")?.view_base();
            let camera = view.camera();
            let (eye, center) = (camera.eye(), camera.center());
            let pixel = view.size().0 / width;
            view.translate(dx * pixel, dy * pixel, 0., true)?;
            let suffix = format!("shift-{}", index + 1);
            source_output::control_capture::control_dump(&*session, basename, &suffix)?;
            camera.set_eye(&eye)?;
            camera.set_center(&center)?;
        }
        Ok(())
    }
}

impl SessionRun {
    /// The selection image of the active view, saved in
    /// this run's capture folder; `run.selection_dump("a_selnorm.png", kind)?`.
    pub(crate) fn selection_dump(
        &mut self,
        basename: &str,
        kind: martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage,
    ) -> Result<(), Box<dyn Error>> {
        let path = source_output::capture_path(basename)?;
        Ok(self.session().selection_dump(&path, kind)?)
    }

    /// Normalized depth in the default BGR
    /// image; `run.untyped_selection_dump("a.png")?`.
    pub(crate) fn untyped_selection_dump(&mut self, basename: &str) -> Result<(), Box<dyn Error>> {
        let path = source_output::capture_path(basename)?;
        Ok(self.session().untyped_selection_dump(&path)?)
    }
}

impl SessionRun {
    /// Report completion of a regression that dumps no image, then retire the views and
    /// report the validation layer's stored errors; `run.finish_without_image()?`.
    pub(crate) fn finish_without_image(mut self) -> Result<(), Box<dyn Error>> {
        use crate::case_view::SourceViewOutput as _;
        use std::io::Write as _;
        println!("TEST COMPLETED");
        let stage = self
            .stage
            .take()
            .expect("a run is never left without a stage");
        let Stage::Open(driver, session) = stage else {
            return Err(std::io::Error::other(
                "regression dumped an image; expected an image-less regression",
            )
            .into());
        };
        session.close();
        for diagnostic in driver.validation_errors().unwrap() {
            writeln!(std::io::stderr(), "Error: Vulkan validation: {diagnostic}")?;
        }
        #[cfg(feature = "vertex-output-diagnostics")]
        if let Some(request) = &self.vertex_output {
            request.finish()?;
        }
        Ok(())
    }
}

#[path = "../support/regression_banner.rs"]
mod regression_banner;
// Some bounded selection hosts use their inherited group banner directly.
#[allow(unused_imports)]
pub(crate) use regression_banner::banner;

/// Every coordinate within the tolerance, else an error line.
/// Example: `check_point("pp1", [0., 0., 1.], [0., 0., 1.], 1e-4)`.
pub(crate) fn check_point(label: &str, value: [f64; 3], expected: [f64; 3], tolerance: f64) {
    let text = |p: [f64; 3]| format!("{} {} {}", p[0], p[1], p[2]);
    if value
        .iter()
        .zip(expected)
        // RAY6-O1: original numeric captures cannot establish NaN/Inf coordinates.
        .any(|(v, e)| !v.is_finite() || !e.is_finite() || (v - e).abs() > tolerance)
    {
        println!(
            "Error: {label}, ({}) is not equal to expected ({})",
            text(value),
            text(expected)
        );
        return;
    }
    println!(
        "Check of {label} OK: value = ({}), expected = ({})",
        text(value),
        text(expected)
    );
}

impl stereo_dump_scope::StereoDumpTarget for SessionRun {
    type Output = ();

    fn projection(&mut self) -> Option<martensite_tkv3d::v3d::V3dStereoDumpOptions> {
        self.session_if_present()
            .map(|session| session.stereo_dump_option())
    }

    fn set_projection(&mut self, option: martensite_tkv3d::v3d::V3dStereoDumpOptions) {
        self.session().set_stereo_dump(option);
    }

    fn capture_dump(&mut self, basename: &'static str) -> Result<Self::Output, Box<dyn Error>> {
        self.dump(basename)
    }
}

#[cfg(test)]
#[path = "checkpoint_report_tests.rs"]
mod checkpoint_report_tests;

#[cfg(test)]
#[path = "session_policy_tests.rs"]
mod session_policy_tests;

#[path = "options_dump.rs"]
mod options_dump;
#[path = "session_restore_check.rs"]
mod session_restore_check;

/// Clear the installed observer before returning either capture outcome; `capture_then_clear(view, || capture())`.
pub(crate) fn capture_then_clear<T>(
    view: &Graphic3dRenderView,
    capture: impl FnOnce() -> Result<T, Box<dyn Error>>,
) -> Result<T, Box<dyn Error>> {
    let result = capture();
    view.set_frame_hooks(None);
    result
}

#[path = "options_observation.rs"]
mod options_observation;

#[cfg(test)]
#[path = "selection_completion_controls.rs"]
mod selection_completion_controls;

#[path = "session_dump_observation.rs"]
mod dump_observation;
