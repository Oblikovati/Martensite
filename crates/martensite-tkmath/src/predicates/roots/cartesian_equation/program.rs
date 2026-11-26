// SPDX-License-Identifier: GPL-2.0-only
//! Reduce a bounded typed postfix program to exact normalized power coefficients.
use super::{
    CartesianEquationError as Failure, EquationExpressionError as Error,
    MAX_CARTESIAN_EQUATION_DEGREE, R, net::Net, poll,
};

/// Total instruction budget shared by the three coordinate expressions.
pub const MAX_CARTESIAN_EQUATION_INSTRUCTIONS: usize = 64;

/// One typed postfix Cartesian coordinate instruction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EquationInstruction {
    /// The authored parameter.
    Parameter,
    /// A finite binary64 constant in model units.
    Literal(f64),
    /// Negate the preceding operand.
    Negate,
    /// Add the two preceding operands.
    Add,
    /// Subtract the second operand from the first.
    Subtract,
    /// Multiply the two preceding operands.
    Multiply,
    /// Divide the first operand by the second, which must reduce to a nonzero constant.
    Divide,
}

const COUNT: usize = MAX_CARTESIAN_EQUATION_DEGREE + 1;
const CHOOSE: [[u8; COUNT]; COUNT] = [[1, 0, 0, 0], [1, 1, 0, 0], [1, 2, 1, 0], [1, 3, 3, 1]];

struct Power([R; COUNT]);

pub(super) struct Coordinates([Power; 3]);

impl Coordinates {
    pub(super) fn into_bernstein(self) -> Net {
        let degree = self.0.iter().map(Power::degree).max().unwrap_or(0);
        Net(self.0.map(|polynomial| polynomial.bernstein(degree)))
    }
}

pub(super) fn compile(
    programs: [&[EquationInstruction]; 3],
    first: &R,
    span: &R,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Coordinates, Failure> {
    let count = programs
        .iter()
        .fold(0_usize, |n, program| n.saturating_add(program.len()));
    if count > MAX_CARTESIAN_EQUATION_INSTRUCTIONS {
        return Err(Failure::InstructionLimit {
            count,
            limit: MAX_CARTESIAN_EQUATION_INSTRUCTIONS,
        });
    }
    let x = coordinate(programs[0], 0, first, span, cancelled)?;
    let y = coordinate(programs[1], 1, first, span, cancelled)?;
    let z = coordinate(programs[2], 2, first, span, cancelled)?;
    Ok(Coordinates([x, y, z]))
}

fn coordinate(
    program: &[EquationInstruction],
    axis: usize,
    first: &R,
    span: &R,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Power, Failure> {
    let mut stack = Vec::with_capacity(program.len());
    let indexed = |instruction, cause| Failure::Expression {
        axis,
        instruction,
        cause,
    };
    for (index, &instruction) in program.iter().enumerate() {
        poll(cancelled)?;
        let value =
            apply(instruction, &mut stack, first, span).map_err(|cause| indexed(index, cause))?;
        stack.push(value);
    }
    if stack.len() != 1 {
        return Err(indexed(program.len(), Error::ResultCount(stack.len())));
    }
    stack
        .pop()
        .ok_or_else(|| indexed(program.len(), Error::ResultCount(0)))
}

fn apply(
    instruction: EquationInstruction,
    stack: &mut Vec<Power>,
    first: &R,
    span: &R,
) -> Result<Power, Error> {
    use EquationInstruction::*;
    match instruction {
        Parameter => Ok(Power([first.clone(), span.clone(), zero(), zero()])),
        Literal(value) => {
            if !value.is_finite() {
                return Err(Error::NonFiniteLiteral(value));
            }
            Ok(Power([R::from_finite(value), zero(), zero(), zero()]))
        }
        Negate => Ok(Power(
            pop(stack)?.0.map(|coefficient| coefficient.negated()),
        )),
        _ => binary(instruction, stack),
    }
}

fn binary(instruction: EquationInstruction, stack: &mut Vec<Power>) -> Result<Power, Error> {
    let rhs = pop(stack)?;
    let lhs = pop(stack)?;
    match instruction {
        EquationInstruction::Add => Ok(Power(std::array::from_fn(|i| lhs.0[i].add(&rhs.0[i])))),
        EquationInstruction::Subtract => {
            Ok(Power(std::array::from_fn(|i| lhs.0[i].subtract(&rhs.0[i]))))
        }
        EquationInstruction::Multiply => lhs.multiply(&rhs),
        EquationInstruction::Divide => lhs.divide(&rhs),
        _ => Err(Error::MissingOperand),
    }
}

fn pop(stack: &mut Vec<Power>) -> Result<Power, Error> {
    stack.pop().ok_or(Error::MissingOperand)
}

impl Power {
    fn degree(&self) -> usize {
        self.0
            .iter()
            .rposition(|coefficient| coefficient.sign().is_ne())
            .unwrap_or(0)
    }

    fn multiply(&self, rhs: &Self) -> Result<Self, Error> {
        let [a, b] = [self.degree(), rhs.degree()];
        if a + b > MAX_CARTESIAN_EQUATION_DEGREE {
            return Err(Error::Degree(a + b));
        }
        let mut result = std::array::from_fn(|_| zero());
        for i in 0..=a {
            for j in 0..=b {
                result[i + j] = result[i + j].add(&self.0[i].multiply(&rhs.0[j]));
            }
        }
        Ok(Self(result))
    }

    fn divide(self, rhs: &Self) -> Result<Self, Error> {
        if rhs.degree() != 0 {
            return Err(Error::NonConstantDivisor);
        }
        if rhs.0[0].sign().is_eq() {
            return Err(Error::ZeroDivisor);
        }
        Ok(Self(self.0.map(|value| value.divide(&rhs.0[0]))))
    }

    fn bernstein(&self, degree: usize) -> Vec<R> {
        (0..=degree)
            .map(|k| {
                (0..=k).fold(zero(), |sum, i| {
                    let factor = R::new(CHOOSE[k][i].into(), CHOOSE[degree][i].into());
                    sum.add(&self.0[i].multiply(&factor))
                })
            })
            .collect()
    }
}

pub(super) fn zero() -> R {
    R::from_finite(0.)
}
