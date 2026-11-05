//! Boolean algorithm infrastructure.
mod enums;
mod face_grouping_version;
mod options;
pub use enums::{Glue, Operation};
pub use face_grouping_version::FaceGroupingVersion;
mod context_tasks;
mod parallel;
pub(crate) use context_tasks::{run_context_tasks, run_launcher_context_tasks};
mod progress;
pub(crate) mod progress_result;
pub(crate) use progress_result::ProgressCancellation;
pub use progress_result::{
    ProgressCompletion, ProgressFailureClass, ProgressOperation, ProgressStage, ProgressWorksite,
};
mod section_attribute;
pub use options::Options;
pub use parallel::{ParallelAlgo, ParallelAlgoBase};
pub use progress::{Algo, PISteps};
pub use section_attribute::SectionAttribute;

pub(crate) mod alerts;
pub use alerts::*;
mod argument_analyzer;
mod bop;
mod cells_builder;
mod check_result;
mod checker_si;
pub use cells_builder::BOPAlgoCellsBuilder;
mod make_connected;
mod make_periodic;
mod maker_volume;
mod remove_features;
pub use argument_analyzer::BOPAlgoArgumentAnalyzer;
pub use check_result::{CheckResult, CheckStatus};
pub use checker_si::{
    BOPAlgoCheckerSI, SelfIntersectionFace, SelfIntersectionFaceStatus, SelfIntersectionObservation,
};
pub(crate) use checker_si::{
    FaceTermination, NativeAlertObservation, SelfCheckCompletion, SelfCheckPhase,
};
pub use make_connected::BOPAlgoMakeConnected;
pub use make_periodic::{BOPAlgoMakePeriodic, PeriodicityParams};
pub use maker_volume::BOPAlgoMakerVolume;
pub use remove_features::BOPAlgoRemoveFeatures;
mod builder;
pub use bop::{BOPAlgoBOP, BopHooks, build_bop_shape, perform_bop_internal1};
mod general_fuse;
mod section;
pub use general_fuse::GeneralFuseAlgorithm;
pub use section::BOPAlgoSection;
mod splitter;
pub use splitter::BOPAlgoSplitter;
mod pave_filler;
pub mod tools;
mod tools_provider;
pub use builder::{
    BOPAlgoBuilder, BuilderHooks, NbShapes, SameDomainShapes, ShapeImages, fill_images_solids_with,
    fill_images_solids_with_progress, perform_on, perform_with_filler_progress,
};
pub use pave_filler::{BOPAlgoPaveFiller, EdgeRangeDistance};
pub use tools_provider::BOPAlgoToolsProvider;
mod builder_area;
pub(crate) use builder_area::progress::{classify_area_progress, observe_user_break};
mod builder_shape;
pub use builder_shape::BuilderShape;
mod builder_face;
pub use builder_area::BuilderArea;
pub use builder_face::BuilderFace;
mod builder_solid;
mod shell_splitter;
pub use builder_solid::BuilderSolid;
pub use shell_splitter::ShellSplitter;
mod wire_edge_set;
mod wire_splitter;
pub use wire_edge_set::WireEdgeSet;
pub use wire_splitter::{
    EdgeInfo, WireOperationContext, WirePreparedExtension, WireSplitter, WireSplitterHooks,
};

#[cfg(test)]
mod progress_test_support;
