//! Explicit real-role identities reuse the common operation-local caller owner.
use super::mzsnorm_;
use super::truncation::mmtrpjj_with_real_access;
use super::truncation_access::TruncationRealAccess;
use crate::adv_app2var::real_slots::{
    CallerRealTableRole, OwnedCallerTableSlots, checked_real_range,
};
use martensite_tkernel::standard::Failure;
use std::sync::{Arc, Mutex};

/// Origins of four real roles inside one initialized operation-local owner.
/// Equal or overlapping ranges retain identity; no value-based inference occurs.
/// `AdvApp2VarTruncationOrigins { coefficients: 0, component_bounds: 3, precision: 4, maximum_error: 4 }`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdvApp2VarTruncationOrigins {
    /// Zero-based origin of the column-major coefficient table.
    pub coefficients: usize,
    /// Zero-based origin of the per-component truncation bounds.
    pub component_bounds: usize,
    /// Scalar precision slot, read after bound and error writes.
    pub precision: usize,
    /// Scalar maximum-error output slot.
    pub maximum_error: usize,
}
impl AdvApp2VarTruncationOrigins {
    fn ordered(self) -> [usize; 4] {
        [
            self.coefficients,
            self.component_bounds,
            self.precision,
            self.maximum_error,
        ]
    }
}
#[derive(Clone, Copy)]
enum TruncationRole {
    Coefficients,
    Bounds,
    Precision,
    Error,
}
impl CallerRealTableRole for TruncationRole {
    const LABEL: &'static str = "mmtrpjj caller real roles";
    const POISON_MESSAGE: &'static str = "mmtrpjj operation-local real owner poisoned";
    fn index(self) -> usize {
        self as usize
    }
}
#[derive(Clone, Copy)]
pub(super) enum ScalarValidation {
    Eager,
    SourceOrdered,
}
pub(super) struct CallerTruncationReals(OwnedCallerTableSlots<TruncationRole, 4>);
impl CallerTruncationReals {
    pub(super) fn new(
        values: Arc<Mutex<Vec<f64>>>,
        origins: AdvApp2VarTruncationOrigins,
    ) -> Result<Self, Failure> {
        Self::with_validation(values, origins, ScalarValidation::Eager)
    }
    pub(super) fn with_validation(
        values: Arc<Mutex<Vec<f64>>>,
        origins: AdvApp2VarTruncationOrigins,
        validation: ScalarValidation,
    ) -> Result<Self, Failure> {
        Self::validate_origins(&values, origins)?;
        let access = Self(OwnedCallerTableSlots::new(values, origins.ordered(), 0));
        if matches!(validation, ScalarValidation::Eager) {
            access.validate_scalar_slots()?;
        }
        Ok(access)
    }
    fn validate_origins(
        values: &Mutex<Vec<f64>>,
        origins: AdvApp2VarTruncationOrigins,
    ) -> Result<(), Failure> {
        let slots = values.lock().expect(TruncationRole::POISON_MESSAGE);
        for origin in origins.ordered() {
            checked_real_range(origin, 0, 0, slots.len())?;
        }
        Ok(())
    }
    fn validate_scalar_slots(&self) -> Result<(), Failure> {
        self.precision()?;
        self.0.value(TruncationRole::Error, 0)?;
        Ok(())
    }
}
impl TruncationRealAccess for CallerTruncationReals {
    fn coefficient(&self, index: i32) -> Result<f64, Failure> {
        self.0.value(TruncationRole::Coefficients, index)
    }
    fn bound(&self, index: i32) -> Result<f64, Failure> {
        self.0.value(TruncationRole::Bounds, index)
    }
    fn set_bound(&mut self, index: i32, value: f64) -> Result<(), Failure> {
        self.0.set_value(TruncationRole::Bounds, index, value)
    }
    fn bound_norm(&mut self, dimensions: i32) -> Result<f64, Failure> {
        self.0
            .value(TruncationRole::Bounds, dimensions.saturating_sub(1).max(0))?;
        self.0.with_values(TruncationRole::Bounds, |bounds| {
            mzsnorm_(dimensions, bounds)
        })
    }
    fn precision(&self) -> Result<f64, Failure> {
        self.0.value(TruncationRole::Precision, 0)
    }
    fn set_error(&mut self, value: f64) -> Result<(), Failure> {
        self.0.set_value(TruncationRole::Error, 0, value)
    }
}

/// Truncates with live coefficient, bound, precision and error identities.
/// Uses one initialized mutable operation-local owner; each scalar read/store
/// releases its guard, and pure norm evaluation does not call another accessor.
/// This API does not promise immutable publication or integer-input aliasing.
/// Invalid accesses return a checked failure; earlier ordered writes remain.
///
/// ```
/// use martensite_tkgeombase::adv_app2var::math_base::{
///     mmtrpjj_with_caller_slots_, AdvApp2VarTruncationOrigins};
/// use std::sync::{Arc, Mutex};
/// let slots = Arc::new(Mutex::new(vec![1., 0., 1e-8, 7., 1e-6]));
/// let origins = AdvApp2VarTruncationOrigins {
///     coefficients: 0, component_bounds: 3, precision: 4, maximum_error: 4 };
/// let mut count = 0;
/// mmtrpjj_with_caller_slots_(3, 1, 3, -1, slots, origins, &mut count).unwrap();
/// assert_eq!(count, 3);
/// ```
pub fn mmtrpjj_with_caller_slots_(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    iordre: i32,
    initialized_reals: Arc<Mutex<Vec<f64>>>,
    origins: AdvApp2VarTruncationOrigins,
    ncfnew: &mut i32,
) -> Result<i32, Failure> {
    let mut access = CallerTruncationReals::new(initialized_reals, origins)?;
    mmtrpjj_with_real_access(ncofmx, ndimen, ncoeff, iordre, &mut access, ncfnew)
}

/// Follows source access order, allowing one-past origins for unused real roles.
/// Invalid consumed slots return failure after earlier ordered writes remain.
/// Existing eager-validation entrypoints retain their original contract.
/// ```
/// use martensite_tkgeombase::adv_app2var::math_base::*;
/// use std::sync::{Arc, Mutex};
/// let slots = Arc::new(Mutex::new(vec![7.; 8]));
/// let origins = AdvApp2VarTruncationOrigins {
///     coefficients: 4, component_bounds: 2, precision: 8, maximum_error: 3 };
/// let mut retained = -7;
/// mmtrpjj_with_ordered_caller_slots_(3, 1, 1, -1, slots, origins, &mut retained).unwrap();
/// assert_eq!(retained, 1);
/// ```
pub fn mmtrpjj_with_ordered_caller_slots_(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    iordre: i32,
    initialized_reals: Arc<Mutex<Vec<f64>>>,
    origins: AdvApp2VarTruncationOrigins,
    ncfnew: &mut i32,
) -> Result<i32, Failure> {
    let mut access = CallerTruncationReals::with_validation(
        initialized_reals,
        origins,
        ScalarValidation::SourceOrdered,
    )?;
    mmtrpjj_with_real_access(ncofmx, ndimen, ncoeff, iordre, &mut access, ncfnew)
}
