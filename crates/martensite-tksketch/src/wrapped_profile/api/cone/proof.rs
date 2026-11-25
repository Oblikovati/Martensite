//! Cone proof settings are explicit and retain finite whole-operation limits.
use martensite_tkmath::predicates::roots::metric_bounds::{
    ConeClearanceOptions, CylindricalFluxOptions,
};

/// Requested cone proof settings; `let proof = WrappedConeProofOptions::default();`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedConeProofOptions {
    /// Requested retained-UV material proof engine; defaults to `FilteredBernsteinV4`.
    pub planar_proof: martensite_tkbo::profile_extrusion::PlanarProofVersion,
    /// Fixed-grid precision and aggregate charged arithmetic across the complete operation.
    pub arithmetic: ConeClearanceOptions,
    /// Finite integration limits shared by enabled planar programs, reference volume and proxy flux.
    pub integration: CylindricalFluxOptions,
    /// Independent authored normal-volume enclosure settings.
    pub authored_volume: WrappedConeAuthoredVolumeOptions,
}
impl Default for WrappedConeProofOptions {
    fn default() -> Self {
        Self {
            planar_proof:
                martensite_tkbo::profile_extrusion::PlanarProofVersion::FilteredBernsteinV4,
            arithmetic: ConeClearanceOptions {
                fractional_bits: 128,
                max_operations: 1_000_000,
            },
            integration: CylindricalFluxOptions::default(),
            authored_volume: WrappedConeAuthoredVolumeOptions::default(),
        }
    }
}

/// Finite authored normal-volume proof settings; `WrappedConeAuthoredVolumeOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedConeAuthoredVolumeOptions {
    /// Fixed dyadic subdivision level for the authored boundary integral.
    pub subdivision_level: u8,
    /// Fixed dyadic subdivision level for authored axis-clearance certification.
    pub clearance_level: u8,
    /// Positive finite requested volume-enclosure width in cubic model units.
    pub enclosure_width: f64,
}
impl Default for WrappedConeAuthoredVolumeOptions {
    fn default() -> Self {
        Self {
            subdivision_level: 6,
            clearance_level: 0,
            enclosure_width: 1e-7,
        }
    }
}
