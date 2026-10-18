//! Typed integer identities share the existing checked caller table allocation.
use super::AdvApp2VarTruncationOrigins;
use super::truncation::mmtrpjj_with_caller_access;
use super::truncation_caller::{CallerTruncationReals, ScalarValidation};
use super::truncation_integers::TruncationIntegerAccess;
use crate::adv_app2var::real_slots::{CallerRealTableRole, OwnedCallerTableSlots};
use martensite_tkernel::standard::Failure;
use std::sync::{Arc, Mutex};

/// Five integer argument identities in initialized operation-local caller storage.
/// `AdvApp2VarTruncationIntegerOrigins { coefficient_stride:0, dimensions:1,
/// coefficient_count:2, continuity_order:3, retained_count:2 }` shares the count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdvApp2VarTruncationIntegerOrigins {
    /// Coefficient-table leading dimension, captured before output initialization.
    pub coefficient_stride: usize,
    /// Active component count, read after output initialization.
    pub dimensions: usize,
    /// Active coefficient count, read after output initialization.
    pub coefficient_count: usize,
    /// Continuity order, captured for dispatch before output initialization.
    pub continuity_order: usize,
    /// Retained coefficient count; may share any input slot.
    pub retained_count: usize,
}
impl AdvApp2VarTruncationIntegerOrigins {
    fn ordered(self) -> [usize; 5] {
        [
            self.coefficient_stride,
            self.dimensions,
            self.coefficient_count,
            self.continuity_order,
            self.retained_count,
        ]
    }
}
#[derive(Clone, Copy)]
enum IntegerRole {
    Stride,
    Dimension,
    Count,
    Order,
    Retained,
}
impl CallerRealTableRole for IntegerRole {
    const LABEL: &'static str = "mmtrpjj caller integer roles";
    const POISON_MESSAGE: &'static str = "mmtrpjj operation-local integer owner poisoned";
    fn index(self) -> usize {
        self as usize
    }
    fn slot_failure(_: Failure, origin: usize, index: i32, capacity: usize) -> Failure {
        Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "mmtrpjj caller integer roles: origin={origin}, index={index}; expected initialized i32 slot in 0..{capacity}"
            ),
        )
    }
}
struct CallerTruncationIntegers(OwnedCallerTableSlots<IntegerRole, 5, i32>);
impl CallerTruncationIntegers {
    fn new(
        values: Arc<Mutex<Vec<i32>>>,
        origins: AdvApp2VarTruncationIntegerOrigins,
    ) -> Result<Self, Failure> {
        let access = Self(OwnedCallerTableSlots::new(values, origins.ordered(), 0));
        for role in [
            IntegerRole::Stride,
            IntegerRole::Dimension,
            IntegerRole::Count,
            IntegerRole::Order,
            IntegerRole::Retained,
        ] {
            access.0.value(role, 0)?;
        }
        Ok(access)
    }
}
impl TruncationIntegerAccess for CallerTruncationIntegers {
    fn stride(&self) -> Result<i32, Failure> {
        self.0.value(IntegerRole::Stride, 0)
    }
    fn dimension(&self) -> Result<i32, Failure> {
        self.0.value(IntegerRole::Dimension, 0)
    }
    fn count(&self) -> Result<i32, Failure> {
        self.0.value(IntegerRole::Count, 0)
    }
    fn order(&self) -> Result<i32, Failure> {
        self.0.value(IntegerRole::Order, 0)
    }
    fn retained(&self) -> Result<i32, Failure> {
        self.0.value(IntegerRole::Retained, 0)
    }
    fn set_retained(&mut self, value: i32) -> Result<(), Failure> {
        self.0.set_value(IntegerRole::Retained, 0, value)
    }
}

/// Truncates through explicit integer and real identities in initialized owners.
/// Uses the same arithmetic as the borrowed entrypoint. Dispatch and coefficient
/// stride are captured before retained-count stores; dimension/count remain live.
/// These owners are mutable operation state, not immutable published results.
///
/// ```
/// use martensite_tkgeombase::adv_app2var::math_base::{mmtrpjj_with_indexed_slots_,
///     AdvApp2VarTruncationIntegerOrigins, AdvApp2VarTruncationOrigins};
/// use std::sync::{Arc, Mutex};
/// let integers=Arc::new(Mutex::new(vec![3,1,3,-1,-7]));
/// let origins=AdvApp2VarTruncationIntegerOrigins { coefficient_stride:0, dimensions:1,
///     coefficient_count:2, continuity_order:3, retained_count:2 };
/// let reals=Arc::new(Mutex::new(vec![1.,0.,1e-8,7.,1e-6,7.]));
/// let real_origins=AdvApp2VarTruncationOrigins { coefficients:0, component_bounds:3,
///     precision:4, maximum_error:5 };
/// mmtrpjj_with_indexed_slots_(Arc::clone(&integers),origins,reals,real_origins).unwrap();
/// assert_eq!(integers.lock().unwrap()[2],1);
/// ```
pub fn mmtrpjj_with_indexed_slots_(
    initialized_integers: Arc<Mutex<Vec<i32>>>,
    integer_origins: AdvApp2VarTruncationIntegerOrigins,
    initialized_reals: Arc<Mutex<Vec<f64>>>,
    real_origins: AdvApp2VarTruncationOrigins,
) -> Result<i32, Failure> {
    indexed_slots_with_validation(
        initialized_integers,
        integer_origins,
        initialized_reals,
        real_origins,
        ScalarValidation::Eager,
    )
}

fn indexed_slots_with_validation(
    initialized_integers: Arc<Mutex<Vec<i32>>>,
    integer_origins: AdvApp2VarTruncationIntegerOrigins,
    initialized_reals: Arc<Mutex<Vec<f64>>>,
    real_origins: AdvApp2VarTruncationOrigins,
    validation: ScalarValidation,
) -> Result<i32, Failure> {
    let mut integers = CallerTruncationIntegers::new(initialized_integers, integer_origins)?;
    let mut reals =
        CallerTruncationReals::with_validation(initialized_reals, real_origins, validation)?;
    mmtrpjj_with_caller_access(&mut integers, &mut reals)
}

/// Keeps live integer identities and validates each real slot when consumed.
/// An origin at owner length is permitted when its role is never read/written.
/// Mutable owners remain operation-local; prior writes survive checked failure.
/// ```
/// use martensite_tkgeombase::adv_app2var::math_base::*;
/// use std::sync::{Arc, Mutex};
/// let integers = Arc::new(Mutex::new(vec![3, 0, 1, -1, -7]));
/// let origins = AdvApp2VarTruncationIntegerOrigins { coefficient_stride: 0,
///     dimensions: 1, coefficient_count: 2, continuity_order: 3, retained_count: 4 };
/// let reals = Arc::new(Mutex::new(vec![7.; 8]));
/// let outputs = AdvApp2VarTruncationOrigins { coefficients: 8,
///     component_bounds: 8, precision: 8, maximum_error: 3 };
/// mmtrpjj_with_ordered_indexed_slots_(integers, origins, reals, outputs).unwrap();
/// ```
pub fn mmtrpjj_with_ordered_indexed_slots_(
    initialized_integers: Arc<Mutex<Vec<i32>>>,
    integer_origins: AdvApp2VarTruncationIntegerOrigins,
    initialized_reals: Arc<Mutex<Vec<f64>>>,
    real_origins: AdvApp2VarTruncationOrigins,
) -> Result<i32, Failure> {
    indexed_slots_with_validation(
        initialized_integers,
        integer_origins,
        initialized_reals,
        real_origins,
        ScalarValidation::SourceOrdered,
    )
}
