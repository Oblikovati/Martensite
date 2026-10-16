//! Compound operators return the modified matrix; Rust assignment traits return unit.
use super::{Mat3, Mat4};

macro_rules! matrix_returning_contracts {
    ($matrix:ident) => {
        impl<T: Copy + From<u8> + std::ops::Mul<Output = T> + std::ops::AddAssign> $matrix<T> {
            /// Multiplies in place and returns the matrix; `matrix.multiply_returning(&rhs).transpose()`.
            pub fn multiply_returning(&mut self, right: &Self) -> &mut Self {
                self.multiply(right);
                self
            }

            /// Squares in place when both operands alias; `matrix.multiply_self()`.
            pub fn multiply_self(&mut self) -> &mut Self {
                *self = Self::product(self, self);
                self
            }
        }
        impl<T: Copy + std::ops::AddAssign> $matrix<T> {
            /// Adds in place and returns the matrix; `matrix.add_returning(&rhs).transpose()`.
            pub fn add_returning(&mut self, right: &Self) -> &mut Self {
                self.add(right);
                self
            }

            /// Adds each element to itself using its assignment operator; `matrix.add_self()`.
            pub fn add_self(&mut self) -> &mut Self {
                let right = *self;
                self.add_returning(&right)
            }
        }
        impl<T: Copy + std::ops::SubAssign> $matrix<T> {
            /// Subtracts in place and returns the matrix; `matrix.subtract_returning(&rhs).transpose()`.
            pub fn subtract_returning(&mut self, right: &Self) -> &mut Self {
                self.subtract(right);
                self
            }

            /// Subtracts each element from itself, including NaNs; `matrix.subtract_self()`.
            pub fn subtract_self(&mut self) -> &mut Self {
                let right = *self;
                self.subtract_returning(&right)
            }
        }
        impl<T: Copy + std::ops::MulAssign> $matrix<T> {
            /// Scales in place and returns the matrix; `matrix.multiply_scalar_returning(2)`.
            pub fn multiply_scalar_returning(&mut self, factor: T) -> &mut Self {
                self.multiply_scalar(factor);
                self
            }
        }
        impl<T: Copy + std::ops::DivAssign> $matrix<T> {
            /// Divides in place and returns the matrix; `matrix.divide_returning(2)`.
            pub fn divide_returning(&mut self, divisor: T) -> &mut Self {
                self.divide(divisor);
                self
            }
        }
    };
}

matrix_returning_contracts!(Mat3);
matrix_returning_contracts!(Mat4);
