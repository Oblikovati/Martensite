//! Linear edge parameter transfer.
use super::ShapeAnalysisEdge;
use crate::shape_build::ShapeBuildEdge;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle, SharedState, Transient},
};
use martensite_tkmath::gp::resolution;
mod operations;
/// Protected transfer state for projection-based subclasses; `TransferParameterFields::default()`.
#[derive(Default)]
pub struct TransferParameterFields {
    /// Source spatial range start.
    pub first: f64,
    /// Source spatial range end.
    pub last: f64,
    /// Source edge retained for copying all curve ranges.
    pub edge: Edge,
    /// Tolerance retained for derived projection policies.
    pub max_tolerance: f64,
    shift: f64,
    scale: f64,
    first_2d: f64,
    last_2d: f64,
    face: Face,
}
/// Shared transfer state preserves subclass identity; `TransferParameterData::default()`.
pub struct TransferParameterData {
    fields: SharedState<TransferParameterFields>,
}
impl Default for TransferParameterData {
    fn default() -> Self {
        Self {
            fields: SharedState::new(TransferParameterFields {
                scale: 1.,
                ..TransferParameterFields::default()
            }),
        }
    }
}
impl TransferParameterData {
    pub(crate) fn set_max_tolerance(&self, tolerance: f64) {
        self.with_mut(|fields| fields.max_tolerance = tolerance);
    }

    pub(in crate::shape_analysis) fn init(&self, edge: &Edge, face: &Face) -> Result<(), Failure> {
        operations::initialize(self, edge, face)
    }

    /// Inspect protected state; `state.with(|fields| fields.first)`.
    pub fn with<R>(&self, read: impl FnOnce(&TransferParameterFields) -> R) -> R {
        read(&self.fields.read())
    }
    /// Update subclass projection state; `state.with_mut(|fields| fields.max_tolerance=1e-6)`.
    pub fn with_mut<R>(&self, write: impl FnOnce(&mut TransferParameterFields) -> R) -> R {
        write(&mut self.fields.write())
    }
    /// Transform one parameter, preserving arithmetic order; `tool.perform_parameter(0.5,true)?`.
    pub fn perform_parameter(&self, parameter: f64, to_2d: bool) -> Result<f64, Failure> {
        Ok(self.with(|fields| {
            if to_2d {
                return fields.shift + parameter * fields.scale;
            }
            -fields.shift / fields.scale + parameter * 1. / fields.scale
        }))
    }
    /// Copy every matching representation range; `tool.transfer_range(&edge,0.,1.,false)?`.
    pub(in crate::shape_analysis) fn transfer_range(
        &self,
        edge: &Edge,
        previous: f64,
        current: f64,
        is_2d: bool,
    ) -> Result<(), Failure> {
        let (source, alpha, beta) = operations::range_fractions(self, previous, current, is_2d);
        ShapeBuildEdge.copy_ranges(edge, &source, alpha, beta)
    }
    /// Compare exact linear coefficients; `tool.is_same_range()`.
    pub(in crate::shape_analysis) fn is_same_range(&self) -> bool {
        self.with(|fields| fields.shift == 0. && fields.scale == 1.)
    }
    pub(in crate::shape_analysis) fn perform(
        &self,
        parameters: &Handle<HSequence<f64>>,
        to_2d: bool,
    ) -> Result<Handle<HSequence<f64>>, Failure> {
        operations::transfer_sequence(parameters, |value| self.perform_parameter(value, to_2d))
    }
}
/// Virtual transfer operations retain derived scalar dispatch; `tool.perform_parameter(0.5,true)?`.
pub trait TransferParameters: Transient {
    /// Shared protected storage; `tool.transfer_parameter_data()`.
    fn transfer_parameter_data(&self) -> &TransferParameterData;
    /// Stores the upper bound used by projection-based implementations; `tool.set_max_tolerance(1e-6)`.
    fn set_max_tolerance(&self, tolerance: f64) {
        self.transfer_parameter_data().set_max_tolerance(tolerance);
    }
    /// Discover unoriented curve ranges; `tool.init(&edge,&face)?`.
    fn init(&self, edge: &Edge, face: &Face) -> Result<(), Failure> {
        self.transfer_parameter_data().init(edge, face)
    }
    /// Transform an independent sequence through virtual scalar transfer; `tool.perform(&parameters,true)?`.
    fn perform(
        &self,
        parameters: &Handle<HSequence<f64>>,
        to_2d: bool,
    ) -> Result<Handle<HSequence<f64>>, Failure> {
        operations::transfer_sequence(parameters, |value| self.perform_parameter(value, to_2d))
    }
    /// Transform one parameter, preserving arithmetic order; `tool.perform_parameter(0.5,true)?`.
    fn perform_parameter(&self, parameter: f64, to_2d: bool) -> Result<f64, Failure> {
        self.transfer_parameter_data()
            .perform_parameter(parameter, to_2d)
    }
    /// Copy every matching representation range; `tool.transfer_range(&edge,0.,1.,false)?`.
    fn transfer_range(
        &self,
        edge: &Edge,
        previous: f64,
        current: f64,
        is_2d: bool,
    ) -> Result<(), Failure> {
        self.transfer_parameter_data()
            .transfer_range(edge, previous, current, is_2d)
    }
    /// Compare exact linear coefficients; `tool.is_same_range()`.
    fn is_same_range(&self) -> bool {
        self.transfer_parameter_data().is_same_range()
    }
}
/// Base linear parameter transfer; `ShapeAnalysisTransferParameters::new()`.
#[derive(Default)]
pub struct ShapeAnalysisTransferParameters {
    state: TransferParameterData,
}
impl_transient!(ShapeAnalysisTransferParameters,"ShapeAnalysis_TransferParameters",dyn Transient,interfaces:[dyn TransferParameters]);
parent_interface!(dyn TransferParameters, ShapeAnalysisTransferParameters, []);
impl TransferParameters for ShapeAnalysisTransferParameters {
    fn transfer_parameter_data(&self) -> &TransferParameterData {
        &self.state
    }
}
impl ShapeAnalysisTransferParameters {
    /// Create the identity transform; `ShapeAnalysisTransferParameters::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Initialize a transform from an edge and face; `ShapeAnalysisTransferParameters::from_edge(&edge,&face)?`.
    pub fn from_edge(edge: &Edge, face: &Face) -> Result<Self, Failure> {
        let tool = Self::new();
        tool.init(edge, face)?;
        Ok(tool)
    }
    /// Store the tolerance used by subclasses; `tool.set_max_tolerance(1e-6)`.
    pub fn set_max_tolerance(&self, tolerance: f64) {
        self.state.set_max_tolerance(tolerance);
    }
}
