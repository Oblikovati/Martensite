//! Every published public type has its thread-transfer contract checked at compile time.
use martensite_tksketch::surface_sketch::*;

static_assertions::assert_impl_all!(SurfaceRegionId: Send, Sync);
static_assertions::assert_impl_all!(SurfaceCurveId: Send, Sync);
static_assertions::assert_impl_all!(SurfaceVertexId: Send, Sync);
static_assertions::assert_impl_all!(NormalSense: Send, Sync);
static_assertions::assert_impl_all!(CylinderSupport: Send, Sync);
static_assertions::assert_impl_all!(CylinderRectangle: Send, Sync);
static_assertions::assert_impl_all!(ConeSupport: Send, Sync);
static_assertions::assert_impl_all!(ConeSector: Send, Sync);
static_assertions::assert_impl_all!(NormalExtrusionOptions: Send, Sync);
static_assertions::assert_impl_all!(NormalExtrusionLimits: Send, Sync);
static_assertions::assert_impl_all!(SurfaceStage: Send, Sync);
static_assertions::assert_impl_all!(SurfaceFailureCause: Send, Sync);
static_assertions::assert_impl_all!(SurfaceSketchFailure: Send, Sync);
static_assertions::assert_impl_all!(HistorySource: Send, Sync);
static_assertions::assert_impl_all!(HistoryRole: Send, Sync);
static_assertions::assert_impl_all!(HistoryKey: Send, Sync);
static_assertions::assert_impl_all!(SurfaceDiagnostic: Send, Sync);
static_assertions::assert_impl_all!(DetachedNormalBody: Send, Sync);
static_assertions::assert_impl_all!(VolumeErrorEstimates: Send, Sync);
static_assertions::assert_impl_all!(NormalExtrusionResult: Send, Sync);
