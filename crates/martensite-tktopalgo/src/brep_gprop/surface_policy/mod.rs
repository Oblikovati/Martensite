//! Independent integration and representation choices for surface measurements (#277).
mod observations;
mod report;
use super::BRepGProp;
use super::package::{located_origin, surface_props};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
pub(super) use observations::SurfaceObservations;
pub use report::SurfacePropertyResult;

/// Geometry integration policy; an adaptive target requests refinement, not a bound.
///
/// Example: `let policy = SurfaceIntegration::FixedOrder;`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceIntegration {
    /// Existing fixed Gauss orders; no integration error estimate is available.
    FixedOrder,
    /// Existing adaptive evaluation with a finite target strictly between 0 and 0.001.
    AdaptiveEstimate {
        /// Requested relative target; convergence and an effective target are not guaranteed.
        requested_relative_target: f64,
    },
}

/// Preferred face representation; either choice falls back to the only usable source.
///
/// Example: `let preference = SurfaceRepresentation::PreferExistingMesh;`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceRepresentation {
    /// Use geometry when present, otherwise existing triangles.
    PreferGeometry,
    /// Use existing triangles when present, otherwise geometry.
    PreferExistingMesh,
}

/// Surface measurement controls; the default preserves established adaptive geometry.
///
/// ```
/// use martensite_tktopalgo::brep_gprop::{SurfacePropertyOptions, SurfaceIntegration};
/// let options = SurfacePropertyOptions::default();
/// assert_eq!(options.integration, SurfaceIntegration::AdaptiveEstimate { requested_relative_target: 1e-5 });
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfacePropertyOptions {
    /// Geometry integration used when a face is measured geometrically.
    pub integration: SurfaceIntegration,
    /// Representation preference, applied independently to each face.
    pub representation: SurfaceRepresentation,
    /// Count each shared face once, retaining the established identity/location rule.
    pub skip_shared: bool,
}

impl Default for SurfacePropertyOptions {
    fn default() -> Self {
        Self {
            integration: SurfaceIntegration::AdaptiveEstimate {
                requested_relative_target: 1e-5,
            },
            representation: SurfaceRepresentation::PreferGeometry,
            skip_shared: false,
        }
    }
}

impl SurfacePropertyOptions {
    fn integration_epsilon(self) -> Result<f64, Failure> {
        let SurfaceIntegration::AdaptiveEstimate {
            requested_relative_target: target,
        } = self.integration
        else {
            return Ok(1.0);
        };
        if target.is_finite() && target > 0.0 && target < 0.001 {
            return Ok(target);
        }
        Err(Failure::with_type(
            "Standard_DomainError",
            &format!(
                "surface_properties_with_options: requested relative target={target}; expected finite 0 < target < 0.001"
            ),
            "",
        ))
    }
}

/// Numerical evidence for the complete surface measurement, separate from its value.
///
/// Example: `let evidence = SurfaceErrorEvidence::Unknown;`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceErrorEvidence {
    /// No aggregate estimate: fixed orders, triangles, empty contributions or unavailable estimates.
    Unknown,
    /// Maximum geometric face estimate; neither a certified bound nor a convergence guarantee.
    EstimatedRelative {
        /// Raw maximum relative face error reported by the adaptive engine.
        max_face_relative_error: f64,
    },
}

impl BRepGProp {
    /// Measure surfaces with explicit independent controls and publish an owned result.
    /// Inputs are only read. Invalid targets or failed measurements return no partial result.
    ///
    /// ```
    /// use martensite_tkbrep::topods::Shape;
    /// use martensite_tktopalgo::brep_gprop::{BRepGProp, SurfacePropertyOptions};
    /// let result = BRepGProp::surface_properties_with_options(&Shape::default(), SurfacePropertyOptions::default())?;
    /// assert_eq!(result.properties().mass(), 0.0);
    /// # Ok::<(), martensite_tkernel::standard::Failure>(())
    /// ```
    pub fn surface_properties_with_options(
        shape: &Shape,
        options: SurfacePropertyOptions,
    ) -> Result<SurfacePropertyResult, Failure> {
        let epsilon = options.integration_epsilon()?;
        let mut props = GProps::with_location(located_origin(shape));
        let mut observations = SurfaceObservations::default();
        let prefer_mesh = options.representation == SurfaceRepresentation::PreferExistingMesh;
        let error = surface_props(
            shape,
            &mut props,
            epsilon,
            (options.skip_shared, prefer_mesh),
            &mut observations,
        )?;
        observations.finish(props, options, error)
    }
}
