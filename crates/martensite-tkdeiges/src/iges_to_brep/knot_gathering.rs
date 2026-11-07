//! The knot sequences of IGES as knots with multiplicities, for curves and surfaces.
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::real::epsilon;

/// Which of two neighbours gives the spacing of reals that makes them one knot.
#[derive(Clone, Copy)]
pub(super) enum KnotCompare {
    /// The knot being read.
    AtKnot,
    /// The knot before it.
    AtPrevious,
}

/// The knots of an entity with multiplicities corrected, and the poles the correction takes out.
pub(super) struct GatheredKnots {
    pub(super) knots: Vec<f64>,
    pub(super) multiplicities: Vec<i32>,
    /// Ranks from 1 of the poles to take out.
    pub(super) removed_poles: Vec<i32>,
    pub(super) sum_of_multiplicities: i32,
}

/// Gathers neighbours that are one knot, raising the multiplicity, from `-degree` up.
pub(super) fn gather_knots(
    nb_knots: i32,
    degree: i32,
    knot_at: impl Fn(i32) -> Result<f64, Failure>,
    compare: KnotCompare,
) -> Result<(Vec<f64>, Vec<i32>), Failure> {
    let mut knots = vec![knot_at(-degree)?];
    let mut multiplicities = vec![1];
    for rank in (1 - degree)..(nb_knots - degree) {
        let knot = knot_at(rank)?;
        let previous = knot_at(rank - 1)?;
        let spacing = match compare {
            KnotCompare::AtKnot => epsilon(knot),
            KnotCompare::AtPrevious => epsilon(previous),
        };
        if (knot - previous).abs() <= spacing {
            if let Some(last) = multiplicities.last_mut() {
                *last += 1;
            }
        } else {
            knots.push(knot);
            multiplicities.push(1);
        }
    }
    Ok((knots, multiplicities))
}

/// Interior multiplicities over the degree, or end ones over the degree plus one, are corrected;
/// each knot corrected calls `on_excess`, which gives its warning.
pub(super) fn correct_multiplicities(
    (knots, read): (Vec<f64>, Vec<i32>),
    degree: i32,
    mut on_excess: impl FnMut() -> Result<(), Failure>,
) -> Result<GatheredKnots, Failure> {
    let last = read.len() - 1;
    let mut gathered = GatheredKnots {
        knots,
        multiplicities: Vec::with_capacity(read.len()),
        removed_poles: Vec::new(),
        sum_of_multiplicities: 0,
    };
    let mut passed = 0;
    for (rank, &multiplicity) in read.iter().enumerate() {
        let limit = if rank == 0 || rank == last {
            degree + 1
        } else {
            degree
        };
        let kept = multiplicity.min(limit);
        if multiplicity > limit {
            on_excess()?;
            gathered
                .removed_poles
                .extend((passed + 1)..=(passed + multiplicity - limit));
        }
        passed += multiplicity;
        gathered.multiplicities.push(kept);
        gathered.sum_of_multiplicities += kept;
    }
    Ok(gathered)
}

/// Values without the items at the given ranks from 1.
pub(super) fn without_ranks<T: Clone>(values: &[T], removed: &[i32]) -> Vec<T> {
    values
        .iter()
        .zip(1..)
        .filter(|(_, rank)| !removed.contains(rank))
        .map(|(value, _)| value.clone())
        .collect()
}
