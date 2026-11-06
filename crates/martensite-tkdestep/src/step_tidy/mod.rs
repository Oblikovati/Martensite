//! StepTidy: merges the duplicate geometric entities of a STEP model before it is written.
mod duplicate_cleaner;
mod entity_reducer;
mod hashers;
mod reducers;

pub use duplicate_cleaner::StepTidyDuplicateCleaner;
pub use entity_reducer::{StepTidyEntityReducer, StepTidyReplacer};
pub use hashers::{
    StepTidyAxis2Placement2dHasher, StepTidyAxis2Placement3dHasher, StepTidyCartesianPointHasher,
    StepTidyCircleHasher, StepTidyDirectionHasher, StepTidyLineHasher, StepTidyPlaneHasher,
    StepTidyVectorHasher,
};
pub use reducers::{
    StepTidyAxis2Placement3dReducer, StepTidyCartesianPointReducer, StepTidyCircleReducer,
    StepTidyDirectionReducer, StepTidyLineReducer, StepTidyPlaneReducer, StepTidyVectorReducer,
};
