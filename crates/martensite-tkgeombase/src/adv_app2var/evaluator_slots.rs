//! Alias-aware derivative slots for sequential evaluator callbacks.
use super::evaluator::{AdvApp2VarEvaluationMut, AdvApp2VarEvaluatorFunc2Var};

/// Derivative arguments may name two locals or one shared local. No duplicate
/// mutable reference is created; `orders.set_u(2); assert_eq!(orders.v(), 2)` for
/// a shared slot.
pub enum AdvApp2VarDerivativeOrders<'a> {
    /// Distinct U and V derivative locals, as in iso approximation.
    Independent {
        /// U derivative local.
        u: &'a mut i32,
        /// V derivative local.
        v: &'a mut i32,
    },
    /// Both arguments name the same local, as in square sampling.
    Shared(&'a mut i32),
}

impl AdvApp2VarDerivativeOrders<'_> {
    pub(super) fn reborrow(&mut self) -> AdvApp2VarDerivativeOrders<'_> {
        match self {
            Self::Independent { u, v } => AdvApp2VarDerivativeOrders::Independent { u, v },
            Self::Shared(order) => AdvApp2VarDerivativeOrders::Shared(order),
        }
    }
    /// Read the U slot; `assert_eq!(orders.u(), 0)` before differentiation.
    pub fn u(&self) -> i32 {
        match self {
            Self::Independent { u, .. } => **u,
            Self::Shared(order) => **order,
        }
    }
    /// Read the V slot, including a preceding write through U; `orders.v()`.
    pub fn v(&self) -> i32 {
        match self {
            Self::Independent { v, .. } => **v,
            Self::Shared(order) => **order,
        }
    }
    /// Write through the U argument; `orders.set_u(1)`.
    pub fn set_u(&mut self, value: i32) {
        match self {
            Self::Independent { u, .. } => **u = value,
            Self::Shared(order) => **order = value,
        }
    }
    /// Write through the V argument; `orders.set_v(2)`.
    pub fn set_v(&mut self, value: i32) {
        match self {
            Self::Independent { v, .. } => **v = value,
            Self::Shared(order) => **order = value,
        }
    }
}

/// One evaluator call with explicit derivative-slot identity. Scalar and array
/// borrows are operation-local; shared orders remain visible during the callback.
/// Implement `AdvApp2VarEvaluatorFunc2Var::evaluate_slots` to observe that alias.
pub struct AdvApp2VarEvaluationSlots<'a> {
    /// Number of reals per sample.
    pub dimension: &'a mut i32,
    /// Mutable local U bounds.
    pub u_start_end: &'a mut [f64; 2],
    /// Mutable local V bounds.
    pub v_start_end: &'a mut [f64; 2],
    /// Preferred constant-parameter direction.
    pub favor_iso: &'a mut i32,
    /// Constant parameter for this call.
    pub const_param: &'a mut f64,
    /// Number of sample parameters.
    pub nb_params: &'a mut i32,
    /// Mutable sample-parameter storage.
    pub parameters: &'a mut [f64],
    /// Independent or aliased derivative-order arguments.
    pub orders: AdvApp2VarDerivativeOrders<'a>,
}

impl AdvApp2VarEvaluationSlots<'_> {
    pub(super) fn evaluate_legacy(
        &mut self,
        evaluator: &(impl AdvApp2VarEvaluatorFunc2Var + ?Sized),
        result: &mut [f64],
        error: &mut i32,
    ) {
        let previous = self.orders.u();
        let (mut u_order, mut v_order) = (previous, self.orders.v());
        evaluator.evaluate_native(
            &mut self.legacy_request(&mut u_order, &mut v_order),
            result,
            error,
        );
        self.restore_legacy_orders(previous, u_order, v_order);
    }

    fn legacy_request<'a>(
        &'a mut self,
        u: &'a mut i32,
        v: &'a mut i32,
    ) -> AdvApp2VarEvaluationMut<'a> {
        AdvApp2VarEvaluationMut {
            dimension: self.dimension,
            u_start_end: self.u_start_end,
            v_start_end: self.v_start_end,
            favor_iso: self.favor_iso,
            const_param: self.const_param,
            nb_params: self.nb_params,
            parameters: self.parameters,
            u_order: u,
            v_order: v,
        }
    }

    fn restore_legacy_orders(&mut self, previous: i32, u_order: i32, v_order: i32) {
        // Preserve the earlier independent-reference convenience's conflict rule.
        if matches!(self.orders, AdvApp2VarDerivativeOrders::Shared(_)) {
            self.orders.set_u(if u_order != previous {
                u_order
            } else {
                v_order
            });
            return;
        }
        self.orders.set_u(u_order);
        self.orders.set_v(v_order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivative_order_writes_preserve_declared_slot_identity() {
        let (mut u, mut v) = (1, 2);
        let mut distinct = AdvApp2VarDerivativeOrders::Independent {
            u: &mut u,
            v: &mut v,
        };
        distinct.set_u(3);
        assert_eq!((distinct.u(), distinct.v()), (3, 2));
        distinct.set_v(4);
        assert_eq!((distinct.u(), distinct.v()), (3, 4));
        let mut shared = AdvApp2VarDerivativeOrders::Shared(&mut u);
        shared.set_u(5);
        assert_eq!(shared.v(), 5);
        shared.set_v(6);
        assert_eq!(shared.u(), 6);
    }

    #[test]
    fn read_only_adapter_reads_shared_derivative_values() {
        let (mut dimension, mut iso, mut count, mut order) = (1, 2, 1, 4);
        let (mut u, mut v, mut constant, mut parameters) = ([0., 1.], [0., 1.], 0.5, [0.25]);
        let mut call = AdvApp2VarEvaluationSlots {
            dimension: &mut dimension,
            u_start_end: &mut u,
            v_start_end: &mut v,
            favor_iso: &mut iso,
            const_param: &mut constant,
            nb_params: &mut count,
            parameters: &mut parameters,
            orders: AdvApp2VarDerivativeOrders::Shared(&mut order),
        };
        let evaluator = |request: &super::super::evaluator::AdvApp2VarEvaluation<'_>,
                         values: &mut [f64],
                         error: &mut i32| {
            values[0] = f64::from(request.u_order + request.v_order);
            *error = 0;
        };
        let (mut values, mut error) = ([0.], -1);
        evaluator.evaluate_slots(&mut call, &mut values, &mut error);
        assert_eq!((values, error, call.orders.u()), ([8.], 0, 4));
    }
    #[test]
    fn reborrow_preserves_shared_and_distinct_derivative_identity() {
        let (mut u, mut v) = (0, 0);
        let mut separate = AdvApp2VarDerivativeOrders::Independent {
            u: &mut u,
            v: &mut v,
        };
        separate.reborrow().set_u(2);
        assert_eq!((separate.u(), separate.v()), (2, 0));
        let mut shared = AdvApp2VarDerivativeOrders::Shared(&mut u);
        let mut borrowed = shared.reborrow();
        borrowed.set_u(3);
        assert_eq!(borrowed.v(), 3);
        borrowed.set_v(4);
        assert_eq!(shared.u(), 4);
    }
}
