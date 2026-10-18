// the implementation and the implementation at
// .

//! `Criterion`: an extra condition an approximation must satisfy.
use super::context::AdvApp2VarContext;
use super::patch::AdvApp2VarPatch;
use martensite_tkernel::standard::Failure;

/// How the criterion drives the cutting (`CriterionType`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AdvApp2VarCriterionType {
    /// Cut whenever the criterion is not satisfied, without the error bound
    /// (`Absolute`).
    #[default]
    Absolute,
    /// Cut when the error bound fails, or when it holds and the criterion does not
    /// (`Relative`).
    Relative,
}

/// How the cuttings are placed (`CriterionRepartition`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AdvApp2VarCriterionRepartition {
    /// A new regular grid of N+1 intervals at each step (`Regular`).
    #[default]
    Regular,
    /// One new cutting point at each step (`Incremental`).
    Incremental,
}

/// A criterion evaluated on each approximated patch (`Criterion`).
///
/// The retained base class stores the maximum value, the type and the repartition in
/// protected fields read by non-virtual getters; here an implementer returns them.
/// `struct Flat; impl AdvApp2VarCriterion for Flat { ... }`.
pub trait AdvApp2VarCriterion {
    /// Computes the criterion on the patch and stores it with `SetCritValue`
    /// (`Value`); `crit.value(&patch, &context)?`.
    fn value(&self, patch: &AdvApp2VarPatch, context: &AdvApp2VarContext) -> Result<(), Failure>;

    /// Whether the patch satisfies the criterion (`IsSatisfied`); `crit.is_satisfied(&patch)`.
    fn is_satisfied(&self, patch: &AdvApp2VarPatch) -> bool;

    /// The bound of the criterion (`MaxValue`); `crit.max_value()`.
    fn max_value(&self) -> f64;

    /// Absolute or relative (`Type`); `crit.criterion_type()`.
    fn criterion_type(&self) -> AdvApp2VarCriterionType;

    /// Regular or incremental cutting (`Repartition`); `crit.repartition()`.
    fn repartition(&self) -> AdvApp2VarCriterionRepartition;
}
