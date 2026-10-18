//! Integer access is independent of the callback buffer representation.
use super::{AdvApp2VarBufferEvaluation, AdvApp2VarEvaluationSlots, AdvApp2VarIntegerField};

pub(super) trait IntegerArguments {
    fn argument_value(&self, field: AdvApp2VarIntegerField) -> i32;
    fn set_argument_value(&mut self, field: AdvApp2VarIntegerField, value: i32);
    fn shared_orders(&self) -> bool;
}

// Both callback views carry the same references; only their buffer access differs.
macro_rules! implement_integer_arguments {
    ($view:ident) => {
        impl IntegerArguments for $view<'_> {
            fn argument_value(&self, field: AdvApp2VarIntegerField) -> i32 {
                match field {
                    AdvApp2VarIntegerField::Dimension => *self.dimension,
                    AdvApp2VarIntegerField::FavoriteIso => *self.favor_iso,
                    AdvApp2VarIntegerField::ParameterCount => *self.nb_params,
                    AdvApp2VarIntegerField::UOrder => self.orders.u(),
                    AdvApp2VarIntegerField::VOrder => self.orders.v(),
                }
            }
            fn set_argument_value(&mut self, field: AdvApp2VarIntegerField, value: i32) {
                match field {
                    AdvApp2VarIntegerField::Dimension => *self.dimension = value,
                    AdvApp2VarIntegerField::FavoriteIso => *self.favor_iso = value,
                    AdvApp2VarIntegerField::ParameterCount => *self.nb_params = value,
                    AdvApp2VarIntegerField::UOrder => self.orders.set_u(value),
                    AdvApp2VarIntegerField::VOrder => self.orders.set_v(value),
                }
            }
            fn shared_orders(&self) -> bool {
                matches!(self.orders, super::AdvApp2VarDerivativeOrders::Shared(_))
            }
        }
    };
}
implement_integer_arguments!(AdvApp2VarEvaluationSlots);
implement_integer_arguments!(AdvApp2VarBufferEvaluation);

#[cfg(test)]
#[path = "integer_arguments_tests.rs"]
mod tests;
