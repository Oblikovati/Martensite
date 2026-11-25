// SPDX-License-Identifier: GPL-2.0-only
//! Exact clipping against the open interior of a strictly convex verified ring.
use super::{Error, Loop, Ordering, R, Uv, determinant, poll, segments};

impl Loop {
    pub(in super::super) fn intersects_open_interior(
        &self,
        path: [&Uv; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, Error> {
        let mut span = [R::from_finite(0.), R::from_finite(1.)];
        for (a, b) in segments(&self.points) {
            poll(cancelled)?;
            let signs = path.map(|point| {
                let turn = determinant(a, b, point);
                if self.sense == Ordering::Less {
                    turn.negated()
                } else {
                    turn
                }
            });
            if !clip(&mut span, signs) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn clip([lower, upper]: &mut [R; 2], [start, end]: [R; 2]) -> bool {
    let start_inside = start.sign() == Ordering::Greater;
    let end_inside = end.sign() == Ordering::Greater;
    match (start_inside, end_inside) {
        (false, false) => return false,
        (true, true) => {}
        (false, true) => {
            let zero = start.divide(&start.subtract(&end));
            if zero > *lower {
                *lower = zero;
            }
        }
        (true, false) => {
            let zero = start.divide(&start.subtract(&end));
            if zero < *upper {
                *upper = zero;
            }
        }
    }
    // Two open half-spaces can meet at a boundary vertex only. A strict solution
    // in [0, 1] necessarily contains an interval with positive length.
    lower < upper
}
