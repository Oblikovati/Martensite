//! Argument selection: sequential (`%d`, `*`) or numbered (`%2$d`, `*3$`) positions.
use super::{CArg, Spec};

/// The typed argument list and the next sequential position.
pub(crate) struct Arguments<'s, 'a> {
    values: &'s [CArg<'a>],
    next: usize,
}

impl<'s, 'a> Arguments<'s, 'a> {
    /// Starts at the first argument; `Arguments::new(&[CArg::Int(1)])`.
    pub(crate) fn new(values: &'s [CArg<'a>]) -> Self {
        Self { values, next: 0 }
    }

    /// The argument at the 1-based `position`, or the next sequential one when `None`.
    pub(crate) fn take(&mut self, position: Option<usize>) -> Option<&'s CArg<'a>> {
        let Some(position) = position else {
            let value = self.values.get(self.next);
            self.next += 1;
            return value;
        };
        position
            .checked_sub(1)
            .and_then(|index| self.values.get(index))
    }

    /// The conversion's own argument; none for `%%` and `%m`.
    pub(crate) fn value(&mut self, spec: &Spec) -> Option<&'s CArg<'a>> {
        if spec.takes_no_argument() {
            return None;
        }
        self.take(spec.position)
    }

    /// Reads the `*` width and precision (width first, as C does) into the spec.
    pub(crate) fn resolve_dimensions(&mut self, mut spec: Spec) -> Spec {
        if spec.dynamic_width {
            let width = int_value(self.take(spec.width_position));
            spec.dynamic_width = false;
            spec.width = width.unsigned_abs() as usize;
            spec.left |= width < 0;
        }
        if spec.dynamic_precision {
            let precision = int_value(self.take(spec.precision_position));
            spec.dynamic_precision = false;
            spec.precision = (precision >= 0).then_some(precision as usize);
        }
        spec
    }
}

fn int_value(argument: Option<&CArg<'_>>) -> i32 {
    match argument {
        Some(CArg::Int(value)) => *value,
        _ => 0,
    }
}
