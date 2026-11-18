//! Per-loop receipts make containment decisions and aggregate winding work inspectable.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in super::super::super) struct MaterialOptions {
    pub probes: CylindricalProbeOptions,
    pub winding: RootIsolationOptions,
    pub max_locations: u32,
}
impl Default for MaterialOptions {
    fn default() -> Self {
        Self {
            probes: Default::default(),
            winding: RootIsolationOptions::new(1e-6),
            max_locations: 8192,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Job {
    pub source: SurfaceRegionId,
    pub target: SurfaceRegionId,
    pub witness: SurfaceCurveId,
}
#[derive(Debug)]
pub(in super::super::super) struct LoopLocation {
    pub target: SurfaceRegionId,
    pub contour: SurfaceCurveId,
    pub probe: usize,
    pub winding: i64,
    pub effective_bisections: u16,
    pub bisections: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super::super) enum OutsideReason {
    OutsideOuter,
    InsideHole(SurfaceCurveId),
}
#[derive(Debug)]
pub(in super::super::super) struct MaterialRelation {
    pub source: SurfaceRegionId,
    pub target: SurfaceRegionId,
    pub witness: SurfaceCurveId,
    pub probe: usize,
    pub reason: OutsideReason,
}
#[derive(Debug)]
pub(in super::super::super) struct PeriodicMaterialEvidence {
    pub probes: CertifiedCylindricalProbes,
    pub relations: Vec<MaterialRelation>,
    pub locations: Vec<LoopLocation>,
    pub bisections: u16,
    pub options: MaterialOptions,
}
