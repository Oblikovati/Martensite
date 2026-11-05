//! The parameter types of `XSAlgoShapeProcessor`, which the actors of the Transfer
//! package keep.
use martensite_tkernel::{ncollection::DataMap, tcollection::AsciiString};
use martensite_tkshhealing::shape_process::ShapeProcessOperationsFlags;

/// `XSAlgoShapeProcessor::ParameterMap`: parameter names to values.
pub type XSAlgoParameterMap = DataMap<AsciiString, AsciiString>;

/// `XSAlgoShapeProcessor::ProcessingData`: the parameters and the operations.
pub type XSAlgoProcessingData = (XSAlgoParameterMap, ShapeProcessOperationsFlags);

/// `XSAlgoShapeProcessor::ProcessingFlags`: the operations, and whether they were set.
pub type XSAlgoProcessingFlags = (ShapeProcessOperationsFlags, bool);
