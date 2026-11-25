// SPDX-License-Identifier: GPL-2.0-only
//! Native knot intervals for a spline that already has full Bernstein multiplicities.

/// One original Bernstein span, borrowed in increasing spline parameter.
#[derive(Clone, Copy, Debug)]
pub struct NativeBernsteinSpan<'a> {
    /// Unchanged native knot interval.
    pub interval: [f64; 2],
    /// Original Cartesian poles, sharing the junction with the next span.
    pub poles: &'a [[f64; 2]],
    /// Original weights, sharing the junction with the next span.
    pub weights: Option<&'a [f64]>,
}

impl NativeBernsteinSpan<'_> {
    /// Whether the legacy homogeneous extraction also preserves these source poles.
    /// Example: `let exact = span.matches_homogeneous_roundtrip();`.
    pub fn matches_homogeneous_roundtrip(&self) -> bool {
        self.weights.is_none_or(|weights| {
            self.poles.iter().zip(weights).all(|(point, &weight)| {
                point.iter().all(|&coordinate| {
                    (coordinate * weight / weight).to_bits() == coordinate.to_bits()
                })
            })
        })
    }
}

/// Borrow an already partitioned clamped spline with finite positive weights.
/// Each interior knot must have multiplicity `degree`. No knot insertion or
/// Cartesian conversion is needed to form these spans.
/// Example: `let spans = exact_bernstein_partition(degree, &poles, None, &knots)?;`.
pub fn exact_bernstein_partition<'a>(
    degree: usize,
    poles: &'a [[f64; 2]],
    weights: Option<&'a [f64]>,
    knots: &'a [f64],
) -> Option<impl ExactSizeIterator<Item = NativeBernsteinSpan<'a>>> {
    let count = partition_count(degree, poles, weights, knots)?;
    if !(0..count).all(|index| exact_span_knots(degree, count, index, knots)) {
        return None;
    }
    Some((0..count).map(move |index| {
        let first = degree * index;
        let last = first + degree;
        NativeBernsteinSpan {
            interval: [knots[first + degree], knots[last + 1]],
            poles: &poles[first..=last],
            weights: weights.map(|values| &values[first..=last]),
        }
    }))
}

fn partition_count(
    degree: usize,
    poles: &[[f64; 2]],
    weights: Option<&[f64]>,
    knots: &[f64],
) -> Option<usize> {
    let len = poles.len();
    if !(1..=16).contains(&degree)
        || len <= degree
        || !(len - 1).is_multiple_of(degree)
        || len.checked_add(degree + 1)? != knots.len()
        || !poles.iter().flatten().all(|value| value.is_finite())
        || !weights.is_none_or(|values| {
            values.len() == len && values.iter().all(|&w| w.is_finite() && w > 0.)
        })
        || knots[..=degree].iter().any(|&knot| knot != knots[degree])
    {
        return None;
    }
    Some((len - 1) / degree)
}

fn exact_span_knots(degree: usize, count: usize, index: usize, knots: &[f64]) -> bool {
    let offset = 1 + degree * (index + 1);
    let repeated = degree + usize::from(index + 1 == count);
    let [first, last] = [knots[degree * (index + 1)], knots[offset]];
    first.is_finite()
        && last.is_finite()
        && first < last
        && knots[offset..offset + repeated]
            .iter()
            .all(|&knot| knot == last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_spans_borrow_exact_original_poles_and_native_knots() {
        let poles = [[0., 0.], [1., 0.], [2., 1.], [3., 0.], [4., 0.]];
        let weights = [1., 2., 4., 4., 6.];
        let knots = [-2., -2., -2., 0.75, 0.75, 1., 1., 1.];
        let spans: Vec<_> = exact_bernstein_partition(2, &poles, Some(&weights), &knots)
            .unwrap()
            .collect();
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].interval, [-2., 0.75]);
        assert_eq!(spans[1].interval, [0.75, 1.]);
        assert_eq!(spans[0].poles, &poles[..3]);
        assert_eq!(spans[1].poles, &poles[2..]);
        assert_eq!(spans[1].weights, Some(&weights[2..]));
        assert!(spans[1].matches_homogeneous_roundtrip());
    }

    #[test]
    fn generic_knot_insertion_and_a_rounded_homogeneous_roundtrip_are_distinct() {
        let poles = [[0.1, 0.], [0.1, 0.1], [0.2, 0.1], [0.2, 0.]];
        assert!(
            exact_bernstein_partition(2, &poles, None, &[0., 0., 0., 0.5, 1., 1., 1.]).is_none()
        );
        let span = exact_bernstein_partition(
            2,
            &poles[..3],
            Some(&[3., 3., 3.]),
            &[0., 0., 0., 1., 1., 1.],
        )
        .unwrap()
        .next()
        .unwrap();
        assert!(!span.matches_homogeneous_roundtrip());
    }
}
