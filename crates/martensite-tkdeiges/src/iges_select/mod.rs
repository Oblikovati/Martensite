//! The tools of a work session on models of the norm: selections, signatures, modifiers and the
//! work library.
mod counter_of_level_number;
mod edit_dir_part;
mod edit_header;
mod edit_header_fields;
mod explores;
mod explores_more;
mod extracts;
mod float_format;
pub(crate) mod modifier_bases;
mod modifiers_header;
mod modifiers_levels;
mod rebuild;
mod remove_curves;
#[cfg(test)]
mod select_tests;
#[cfg(test)]
mod select_tests_editors;
mod sign_color;
mod signatures;
mod spline_to_bspline;
mod view_sorter;
mod views;
mod work_library;

pub use counter_of_level_number::IgesSelectCounterOfLevelNumber;
pub use edit_dir_part::IgesSelectEditDirPart;
pub use edit_header::IgesSelectEditHeader;
pub use explores::IgesSelectSelectBasicGeom;
pub use explores_more::{
    IgesSelectSelectBypassGroup, IgesSelectSelectBypassSubfigure, IgesSelectSelectFaces,
    IgesSelectSelectPCurves,
};
pub use extracts::{
    IgesSelectSelectLevelNumber, IgesSelectSelectName, IgesSelectSelectSubordinate,
    IgesSelectSelectVisibleStatus,
};
pub use float_format::{IgesSelectFloatFormat, IgesSelectFloatFormatValues};
pub use modifier_bases::{
    IgesSelectFileModifier, IgesSelectModelModifier, iges_model_modifier_perform,
};
pub use modifiers_header::{
    IgesSelectAutoCorrect, IgesSelectComputeStatus, IgesSelectSetGlobalParameter,
    IgesSelectSetLabel, IgesSelectSetVersion5, IgesSelectUpdateCreationDate,
    IgesSelectUpdateFileName, IgesSelectUpdateLastChange,
};
pub use modifiers_levels::{IgesSelectChangeLevelList, IgesSelectChangeLevelNumber};
pub use rebuild::{IgesSelectRebuildDrawings, IgesSelectRebuildGroups};
pub use remove_curves::IgesSelectRemoveCurves;
pub use sign_color::IgesSelectSignColor;
pub(crate) use signatures::IgesSelectClassType;
pub use signatures::{
    IgesSelectIGESName, IgesSelectIGESTypeForm, IgesSelectSignLevelNumber, IgesSelectSignStatus,
};
pub use spline_to_bspline::IgesSelectSplineToBSpline;
pub use view_sorter::IgesSelectViewSorter;
pub use views::{
    IgesSelectDispPerDrawing, IgesSelectDispPerSingleView, IgesSelectSelectDrawingFrom,
    IgesSelectSelectFromDrawing, IgesSelectSelectFromSingleView, IgesSelectSelectSingleViewFrom,
};
pub use work_library::IgesSelectWorkLibrary;
