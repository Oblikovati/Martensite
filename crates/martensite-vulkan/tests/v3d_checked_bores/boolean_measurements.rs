//! Measurements preserve estimate/unknown categories and reject modelling evidence from meshes.
use super::boolean_coverage::{CoverageStage, OperationCoverage};
use super::boolean_outcome::{BooleanIssue, BooleanRequest, RefusalClass, kernel_issue, refused};
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, SurfacePropertyOptions, SurfacePropertyResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AchievedVolumeError {
    Unknown,
}

pub(super) struct MeasurementReceipt {
    pub surface: SurfacePropertyResult,
    pub volume: f64,
    /// Strict volume completion currently exposes no achieved error scalar.
    pub volume_error: AchievedVolumeError,
}

pub(super) trait MeasurementSource {
    fn surface(
        &self,
        request: BooleanRequest,
        shape: &Shape,
    ) -> Result<SurfacePropertyResult, BooleanIssue>;
    fn volume(&self, request: BooleanRequest, shape: &Shape) -> Result<f64, BooleanIssue>;
}

pub(super) struct ActualMeasurements;
impl MeasurementSource for ActualMeasurements {
    fn surface(
        &self,
        request: BooleanRequest,
        shape: &Shape,
    ) -> Result<SurfacePropertyResult, BooleanIssue> {
        geometric_surface(request, shape)
    }
    fn volume(&self, request: BooleanRequest, shape: &Shape) -> Result<f64, BooleanIssue> {
        geometric_volume(request, shape)
    }
}

pub(super) fn measure_with(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    source: &impl MeasurementSource,
) -> Result<MeasurementReceipt, BooleanIssue> {
    let surface = observed_surface(request, shape, coverage, source)?;
    let volume = observed_volume(request, shape, coverage, source)?;
    finite_measurements(request, &surface, volume).inspect_err(|issue| {
        coverage.refused(
            CoverageStage::FiniteMeasurements,
            issue.class,
            issue.to_string(),
        );
    })?;
    coverage.completed(CoverageStage::FiniteMeasurements, 2);
    Ok(MeasurementReceipt {
        surface,
        volume,
        volume_error: AchievedVolumeError::Unknown,
    })
}

fn observed_surface(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    source: &impl MeasurementSource,
) -> Result<SurfacePropertyResult, BooleanIssue> {
    let surface = retained_query(
        source.surface(request, shape),
        CoverageStage::SurfaceMeasurement,
        coverage,
    )?;
    coverage.surface = Some(surface);
    retained_query(
        require_geometric_surface(request, &surface),
        CoverageStage::SurfaceMeasurement,
        coverage,
    )?;
    coverage.completed(CoverageStage::SurfaceMeasurement, surface.geometric_faces());
    Ok(surface)
}

fn observed_volume(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    source: &impl MeasurementSource,
) -> Result<f64, BooleanIssue> {
    let volume = retained_query(
        source.volume(request, shape),
        CoverageStage::VolumeMeasurement,
        coverage,
    )?;
    coverage.volume = Some(volume);
    coverage.completed(CoverageStage::VolumeMeasurement, 1);
    Ok(volume)
}

fn finite_measurements(
    request: BooleanRequest,
    surface: &SurfacePropertyResult,
    volume: f64,
) -> Result<(), BooleanIssue> {
    if volume.is_finite() && surface.properties().mass().is_finite() {
        return Ok(());
    }
    Err(refused(
        request,
        "Measurement",
        format!("volume={volume} area={}", surface.properties().mass()),
        "finite properties with retained error categories",
        RefusalClass::Failed,
    ))
}

fn geometric_surface(
    request: BooleanRequest,
    shape: &Shape,
) -> Result<SurfacePropertyResult, BooleanIssue> {
    BRepGProp::surface_properties_with_options(shape, SurfacePropertyOptions::default())
        .map_err(|cause| kernel_issue(request, "Measurement", cause))
}

fn require_geometric_surface(
    request: BooleanRequest,
    surface: &SurfacePropertyResult,
) -> Result<(), BooleanIssue> {
    if surface.triangulated_faces() != 0 || surface.geometric_faces() == 0 {
        let actual = format!(
            "geometry={} triangles={}",
            surface.geometric_faces(),
            surface.triangulated_faces()
        );
        return Err(refused(
            request,
            "Measurement",
            actual,
            "nonempty geometric faces without mesh fallback",
            RefusalClass::Incomplete,
        ));
    }
    Ok(())
}

fn geometric_volume(request: BooleanRequest, shape: &Shape) -> Result<f64, BooleanIssue> {
    let mut volume = GProps::new();
    BRepGProp::volume_properties(shape, &mut volume, false, false, false)
        .map_err(|cause| kernel_issue(request, "Measurement", cause))?;
    Ok(volume.mass())
}

pub(super) fn sphere_cube_expectation() -> (f64, f64) {
    let pi = std::f64::consts::PI;
    (1000. + 34. * pi, 600. + 6. * pi)
}

pub(super) fn blind_bore_expectation() -> (f64, f64) {
    let pi = std::f64::consts::PI;
    (2_000_000. - 25_000. * pi, 100_000. + 5_000. * pi)
}

fn retained_query<T>(
    result: Result<T, BooleanIssue>,
    stage: CoverageStage,
    coverage: &mut OperationCoverage,
) -> Result<T, BooleanIssue> {
    result.inspect_err(|issue| {
        coverage.refused(stage, issue.class, issue.to_string());
    })
}
