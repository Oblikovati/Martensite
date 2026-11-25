//! Propose cubic traces, then refine only under the unchanged sewing allowance.
use super::{
    enclosure::{halve, rounded},
    *,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalProfileFailure, certify_conical_trim,
};
const MAX_LEVEL: usize = 8;

pub(super) struct Fit<'a> {
    pub(super) carrier: &'a Carrier,
    pub(super) allowance: f64,
    pub(super) endpoints: [[f64; 3]; 2],
}

impl Fit<'_> {
    pub(super) fn fit(
        &self,
        attempt: Attempt,
        exact: Homogeneous,
        range: [f64; 2],
        level: usize,
        out: &mut Vec<Span>,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<()> {
        let (poles, weights, roundoff) =
            rounded(&exact).ok_or_else(|| refused(attempt, "bounded seam restriction"))?;
        let pcurve = propose(self.carrier.datum, &poles, &weights, range, self.endpoints);
        let limit = (self.allowance - roundoff).next_down();
        if limit <= 0. || !in_sheet(pcurve, self.carrier.sheet) {
            return Err(refused(
                attempt,
                "seam fit inside original strict metric and sheet limits",
            ));
        }
        let fit = SpanFit {
            range,
            poles,
            weights,
            pcurve,
            limit,
            roundoff,
        };
        if cancelled() {
            return Err(attempt.fail(ASSEMBLY, CylinderFeatureCause::Cancelled));
        }
        let query = fit.query(self.carrier.datum);
        if let Some(proof) = TrimProof::filtered(query) {
            if (proof.residual_bounds()[2] + roundoff).next_up() < self.allowance {
                out.push(Span { fit, proof });
                return Ok(());
            }
            if level < MAX_LEVEL {
                let [lo, hi] = range;
                let mid = lo.midpoint(hi);
                let (left, right) = halve(exact);
                self.fit(attempt, left, [lo, mid], level + 1, out, cancelled)?;
                return self.fit(attempt, right, [mid, hi], level + 1, out, cancelled);
            }
        }
        let mut interrupted = false;
        let result = certify_conical_trim(query, || {
            interrupted |= cancelled();
            interrupted
        });
        if interrupted {
            return Err(attempt.fail(ASSEMBLY, CylinderFeatureCause::Cancelled));
        }
        match result {
            Ok(proof) if (proof.residual_bounds()[2] + roundoff).next_up() < self.allowance => {
                out.push(Span {
                    fit,
                    proof: TrimProof::dyadic(proof),
                });
                Ok(())
            }
            Err(cause) if !matches!(cause.cause, CylindricalProfileFailure::Uncertified { .. }) => {
                Err(attempt.fail(
                    ASSEMBLY,
                    CylinderFeatureCause::Native {
                        query: "direct strict same-parameter trim",
                        cause: Box::new(Failure::new(&cause.to_string())),
                    },
                ))
            }
            _ if level < MAX_LEVEL => {
                let [lo, hi] = range;
                let mid = (lo + hi) * 0.5;
                let (left, right) = halve(exact);
                self.fit(attempt, left, [lo, mid], level + 1, out, cancelled)?;
                self.fit(attempt, right, [mid, hi], level + 1, out, cancelled)
            }
            _ => Err(refused(
                attempt,
                "strict correspondence before the dyadic fit budget",
            )),
        }
    }
}

fn in_sheet(poles: [[f64; 2]; 4], sheet: [[f64; 2]; 2]) -> bool {
    poles.iter().all(|point| {
        point
            .iter()
            .zip(sheet)
            .all(|(p, [lo, hi])| *p > lo && *p < hi)
    })
}

fn propose(
    carrier: ConicalTrimCarrier,
    poles: &[[f64; 3]],
    weights: &[f64],
    range: [f64; 2],
    ends: [[f64; 3]; 2],
) -> [[f64; 2]; 4] {
    let n = poles.len() - 1;
    let cosine = martensite_cmath::cos(carrier.semi_angle);
    let side = |end: bool| {
        let (i, j) = if end { (n, n - 1) } else { (0, 1) };
        let p = match (end, range[usize::from(end)]) {
            (false, 0.) => ends[0],
            (true, 1.) => ends[1],
            _ => poles[i],
        };
        let [x, y, z] = p;
        let rate = n as f64 * weights[j] / weights[i];
        let [dx, dy, dz] = std::array::from_fn(|axis| rate * (poles[j][axis] - poles[i][axis]));
        let u = martensite_cmath::atan2(y, x);
        let u = if u < 0. { u + std::f64::consts::TAU } else { u };
        let uv = [u, (z - carrier.axial_origin) / cosine];
        let d = [(x * dy - y * dx) / (x * x + y * y), dz / cosine];
        (uv, [uv[0] + d[0] / 3., uv[1] + d[1] / 3.])
    };
    let (first, first_inner) = side(false);
    let (last, last_inner) = side(true);
    [first, first_inner, last_inner, last]
}
