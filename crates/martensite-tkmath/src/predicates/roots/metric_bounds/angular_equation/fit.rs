// SPDX-License-Identifier: GPL-2.0-only
//! Piecewise cubic Hermite with independent derivative remainder and actual control error.
use super::{
    AngularEquationCurve, AngularEquationError as Error, Arithmetic, MAX_DEPTH, Net, POLICY, R,
    interval::nominal,
    law,
    refusal::{Checked, check_cancelled},
    series::WorldJet,
};
use crate::predicates::roots::{
    cartesian_equation::net, certificate_arithmetic::bounded_dyadic::Interval,
};
use num_bigint::BigInt;

/// One checked native cubic over a dyadic span of the normalized authored domain.
#[derive(Clone, Debug, PartialEq)]
pub struct AngularBezierSpan {
    interval: [f64; 2],
    poles: [[f64; 3]; 4],
    remainder: f64,
    control_error: f64,
    error: f64,
    speed_lower: f64,
}

impl AngularBezierSpan {
    /// Span in the global normalized authored domain, increasing on [0, 1].
    /// Example: `span.normalized_interval()`.
    pub fn normalized_interval(&self) -> [f64; 2] {
        self.interval
    }

    /// Actual rounded native coefficients on this local span. Example: `span.poles()`.
    pub fn poles(&self) -> &[[f64; 3]; 4] {
        &self.poles
    }

    /// Ideal Hermite positional remainder upper bound. Example: `span.remainder_bound()`.
    pub fn remainder_bound(&self) -> f64 {
        self.remainder
    }

    /// Additional discrepancy from coefficient materialization. Example: `span.control_error_bound()`.
    pub fn control_error_bound(&self) -> f64 {
        self.control_error
    }

    /// Inclusive positional error against the exact polynomial of the retained native poles.
    /// The comparison uses the same mapped parameter and precedes native evaluation rounding.
    /// Example: `span.distance_bound()`.
    pub fn distance_bound(&self) -> f64 {
        self.error
    }

    /// Strict lower bound on retained native speed in the local [0, 1] parameter.
    /// Example: `span.local_speed_lower()`.
    pub fn local_speed_lower(&self) -> f64 {
        self.speed_lower
    }
}

/// Complete ordered cubic cover of one angular law with a certified geometric discrepancy.
#[derive(Clone, Debug, PartialEq)]
pub struct AngularBezierCertificate {
    domain: [f64; 2],
    spans: Vec<AngularBezierSpan>,
    distance_bound: f64,
}

impl AngularBezierCertificate {
    /// Authored interval corresponding to global normalized [0, 1].
    /// Example: `fit.authored_domain()`.
    pub fn authored_domain(&self) -> [f64; 2] {
        self.domain
    }

    /// All cubic spans in deterministic increasing order. Example: `fit.spans()`.
    pub fn spans(&self) -> &[AngularBezierSpan] {
        &self.spans
    }

    /// Worst certified geometric discrepancy over the complete cover.
    /// Example: `fit.distance_bound()`.
    pub fn distance_bound(&self) -> f64 {
        self.distance_bound
    }
}

impl AngularEquationCurve {
    /// Fit and certify all ordered native cubics before publication.
    /// Each local curve is on [0, 1]; map it through the span and original authored interval.
    /// Example: `curve.certify_bezier(1e-6, || false)?.spans()`.
    pub fn certify_bezier(
        &self,
        limit: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<AngularBezierCertificate> {
        check_cancelled(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(Error::DistanceLimit(limit));
        }
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let mut fit = Fitter {
            curve: self,
            limit,
            spans: Vec::new(),
            arithmetic: &mut arithmetic,
        };
        fit.cover(&self.rows, Cell { index: 0, depth: 0 })?;
        let spans = fit.spans;
        check_cancelled(&mut cancelled)?;
        let distance_bound = spans
            .iter()
            .fold(0.0_f64, |worst, span| worst.max(span.error));
        Ok(AngularBezierCertificate {
            domain: self.domain,
            spans,
            distance_bound,
        })
    }
}

#[derive(Clone, Copy)]
struct Cell {
    index: u32,
    depth: u8,
}

impl Cell {
    fn limits(self) -> [f64; 2] {
        let width = f64::from(1u32 << self.depth);
        [
            f64::from(self.index) / width,
            f64::from(self.index + 1) / width,
        ]
    }

    fn halves(self) -> [Self; 2] {
        let depth = self.depth + 1;
        [
            Self {
                index: self.index * 2,
                depth,
            },
            Self {
                index: self.index * 2 + 1,
                depth,
            },
        ]
    }
}

struct Fitter<'a, 'b, 'c> {
    curve: &'a AngularEquationCurve,
    limit: f64,
    spans: Vec<AngularBezierSpan>,
    arithmetic: &'b mut Arithmetic<'c>,
}

impl Fitter<'_, '_, '_> {
    fn cover(&mut self, rows: &[Net; 4], cell: Cell) -> Checked<()> {
        let candidate = self.candidate(rows, cell)?;
        if candidate.error <= self.limit {
            let span = certify_native(candidate, self.arithmetic)?;
            self.spans.push(span);
            return Ok(());
        }
        if cell.depth == MAX_DEPTH {
            return Err(Error::FitLimit {
                depth: cell.depth,
                bound: candidate.error,
                limit: self.limit,
            });
        }
        let [left, right] = law::bisect(rows);
        let [first, last] = cell.halves();
        self.cover(&left, first)?;
        self.cover(&right, last)
    }

    fn candidate(&mut self, rows: &[Net; 4], cell: Cell) -> Checked<SpanCandidate> {
        let arithmetic = &mut *self.arithmetic;
        let interval = cell.limits();
        let span_length = self
            .curve
            .span
            .divide(&R::new((1u32 << cell.depth).into(), 1.into()));
        let h = arithmetic.rational(&span_length)?;
        let first = self.endpoint(interval[0])?;
        let last = self.endpoint(interval[1])?;
        let arithmetic = &mut *self.arithmetic;
        let chart = law::range_jet(self.curve.mode, rows, arithmetic)?;
        let remainder = remainder(&chart, &h, arithmetic)?;
        let (poles, rounding) = controls(&first, &last, &h, arithmetic)?;
        let error = arithmetic.upper_f64(&remainder + &rounding)?;
        Ok(SpanCandidate {
            interval,
            poles,
            error,
            remainder: arithmetic.upper_f64(remainder)?,
            control_error: arithmetic.upper_f64(rounding)?,
        })
    }

    fn endpoint(&mut self, normalized: f64) -> Checked<WorldJet> {
        law::point_jet(
            self.curve.mode,
            &self.curve.rows,
            &R::from_finite(normalized),
            self.arithmetic,
        )
    }
}

struct SpanCandidate {
    interval: [f64; 2],
    poles: [[f64; 3]; 4],
    remainder: f64,
    control_error: f64,
    error: f64,
}

fn remainder(jet: &WorldJet, h: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<BigInt> {
    // Scalar Hermite interpolation has error f''''(xi)(t-a)^2(t-b)^2 / 4!.
    // The product is at most h^4/16. Summing axis bounds encloses the Euclidean norm.
    let h2 = arithmetic.multiply(h, h)?;
    let h4 = arithmetic.multiply(&h2, &h2)?;
    let h4_over_16 = arithmetic.fraction(&h4, 1, 16)?;
    let mut total = BigInt::from(0);
    for axis in jet {
        total += arithmetic.multiply(&h4_over_16, &axis[4])?.magnitude();
    }
    Ok(total)
}

fn controls(
    first: &WorldJet,
    last: &WorldJet,
    h: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([[f64; 3]; 4], BigInt)> {
    let mut poles = [[0.; 3]; 4];
    let mut error = [0, 0, 0, 0].map(BigInt::from);
    let h_over_3 = arithmetic.fraction(h, 1, 3)?;
    for (axis, (a, b)) in first.iter().zip(last).enumerate() {
        let outgoing = arithmetic.multiply(&a[1], &h_over_3)?;
        let incoming = arithmetic.multiply(&b[1], &h_over_3)?;
        let controls = [
            a[0].clone(),
            arithmetic.add(&a[0], &outgoing)?,
            arithmetic.subtract(&b[0], &incoming)?,
            b[0].clone(),
        ];
        for (i, exact) in controls.iter().enumerate() {
            poles[i][axis] = nominal(exact, arithmetic)?;
            let retained = arithmetic.scalar(poles[i][axis])?;
            error[i] += arithmetic.subtract(exact, &retained)?.magnitude();
        }
    }
    // A Bézier difference lies in the convex hull of its difference controls.
    Ok((poles, error.into_iter().max().unwrap_or_default()))
}

fn certify_native(
    candidate: SpanCandidate,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<AngularBezierSpan> {
    let rounded = Net(std::array::from_fn(|axis| {
        candidate
            .poles
            .iter()
            .map(|p| R::from_finite(p[axis]))
            .collect()
    }));
    let rows = net::jet(rounded, &R::from_finite(1.));
    let mut stop = None;
    let native = net::certify(&rows, &mut || {
        stop = arithmetic.charge().err();
        stop.is_some()
    });
    if let Some(cause) = stop {
        return Err(cause.into());
    }
    let native = native.map_err(Error::RoundedCarrier)?;
    Ok(AngularBezierSpan {
        interval: candidate.interval,
        poles: candidate.poles,
        remainder: candidate.remainder,
        control_error: candidate.control_error,
        error: candidate.error,
        speed_lower: native.speed_lower,
    })
}
