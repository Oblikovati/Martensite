// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const DEPTH_COEFFICIENTS: [(usize, usize); 3] = [(0, 0), (0, 1), (1, 1)];

pub(super) fn side(
    side: DecodedRuledSide<'_>,
    query: CarrierSeparationQuery<'_>,
    trig: &Trig,
    arithmetic: &mut Arithmetic<'_>,
    exact_sides: &mut usize,
) -> Result<SideSeparation, SeparationCause> {
    let mut hull = Hull::default();
    let rows = side
        .rows
        .map(|row| lift(row, side.location, query.stock, trig, &mut hull));
    let [a, b] = rows;
    let denominator = hull.denominator(trig.cosine)?;
    let axial = rim_gaps(hull.axial, query.rims)?;
    let mut layers = DEPTH_COEFFICIENTS.map(|(i, j)| {
        let rows = [&a, &b];
        tensor_layer(rows[i], rows[j], denominator, arithmetic)
    });
    if layers
        .iter()
        .any(|layer| !layer.finite() || layer.directed_lower(query.sense) <= 0.)
    {
        *exact_sides += 1;
        layers = dyadic::side_layers(side, query.stock, trig, denominator, arithmetic)?;
    }
    if layers.iter().any(|layer| !layer.finite()) {
        return Err(SeparationCause::UnrepresentableEnclosure);
    }
    let lower = layers.map(|bound| bound.directed_lower(query.sense));
    for (layer, lower) in lower.into_iter().enumerate() {
        positive_sign(query.sense, layer, lower)?;
    }
    Ok(SideSeparation {
        id: side.id,
        signed_normal_lower: lower,
        zero_normal_upper: layers[0].magnitude(),
        rim_lower: axial,
    })
}

fn positive_sign(sense: SeparationSense, layer: usize, lower: f64) -> Result<(), SeparationCause> {
    if lower > 0. && lower.is_finite() {
        Ok(())
    } else {
        Err(SeparationCause::NormalSign {
            sense,
            layer,
            lower,
        })
    }
}

fn rim_gaps(axial: Enclosure, rims: [DecodedRimPlane; 2]) -> Result<[f64; 2], SeparationCause> {
    let gaps = [
        (axial.lo - rims[0].elevation).next_down(),
        (rims[1].elevation - axial.hi).next_down(),
    ];
    if gaps.into_iter().all(|gap| gap.is_finite() && gap > 0.) {
        Ok(gaps)
    } else {
        Err(SeparationCause::RimSeparation { gaps })
    }
}

fn lift(
    row: &[[f64; 4]],
    location: DecodedCoaxialLocation,
    stock: DecodedConeFrame,
    trig: &Trig,
    hull: &mut Hull,
) -> Vec<[Enclosure; 3]> {
    let [[a, b, _], [c, d, _], _] = location.matrix;
    let origin = Enclosure::point(location.translation[2]);
    let delta = origin.subtract(Enclosure::point(stock.origin[2]));
    let rc = Enclosure::point(stock.radius).multiply(trig.cosine);
    row.iter()
        .map(|&p| {
            let [x, y, z, weight] = p.map(Enclosure::point);
            let wx = Enclosure::point(a)
                .multiply(x)
                .add(Enclosure::point(b).multiply(y));
            let wy = Enclosure::point(c)
                .multiply(x)
                .add(Enclosure::point(d).multiply(y));
            let threshold = rc.add(z.add(delta).multiply(trig.sine));
            hull.add([wx, wy, z.add(origin), weight], threshold);
            [
                wx.multiply(trig.cosine),
                wy.multiply(trig.cosine),
                threshold,
            ]
            .map(|v| v.multiply(weight))
        })
        .collect()
}

fn tensor_layer(
    a: &[[Enclosure; 3]],
    b: &[[Enclosure; 3]],
    denominator: Enclosure,
    arithmetic: &Arithmetic<'_>,
) -> Enclosure {
    let mut coefficients = [Enclosure::ZERO; 2 * MAX_POLES - 1];
    let n = a.len() - 1;
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let [xx, yy, rr] = std::array::from_fn(|axis| x[axis].multiply(y[axis]));
            let weight = arithmetic.binomial(n, i) * arithmetic.binomial(n, j);
            let term = xx
                .add(yy)
                .subtract(rr)
                .multiply(Enclosure::point(weight as f64));
            coefficients[i + j] = coefficients[i + j].add(term);
        }
    }
    let mut hull = Enclosure {
        lo: f64::INFINITY,
        hi: f64::NEG_INFINITY,
    };
    for (k, term) in coefficients[..=2 * n].iter().enumerate() {
        let coefficient =
            term.divide_positive(Enclosure::point(arithmetic.binomial(2 * n, k) as f64));
        let signed = coefficient.divide_positive(denominator);
        hull.lo = hull.lo.min(signed.lo);
        hull.hi = hull.hi.max(signed.hi);
    }
    hull
}

struct Hull {
    axial: Enclosure,
    threshold: Enclosure,
    weight: Enclosure,
    radial_upper: f64,
}
impl Default for Hull {
    fn default() -> Self {
        let empty = Enclosure {
            lo: f64::INFINITY,
            hi: f64::NEG_INFINITY,
        };
        Self {
            axial: empty,
            threshold: empty,
            weight: empty,
            radial_upper: 0.,
        }
    }
}
impl Hull {
    fn add(&mut self, [x, y, z, w]: [Enclosure; 4], threshold: Enclosure) {
        for (hull, value) in [
            (&mut self.axial, z),
            (&mut self.weight, w),
            (&mut self.threshold, threshold),
        ] {
            hull.lo = hull.lo.min(value.lo);
            hull.hi = hull.hi.max(value.hi);
        }
        self.radial_upper = self
            .radial_upper
            .max((x.magnitude() + y.magnitude()).next_up());
    }

    fn denominator(&self, cosine: Enclosure) -> Result<Enclosure, SeparationCause> {
        // H = W² (rho*cos(alpha) + h) * D. The threshold h must be
        // positive throughout; otherwise the opposite cone sheet is ambiguous.
        let lower = Enclosure::point(self.weight.lo)
            .square()
            .multiply(Enclosure::point(self.threshold.lo))
            .lo;
        if lower < f64::MIN_POSITIVE {
            return Err(SeparationCause::PositiveRadius {
                lower: self.threshold.lo,
            });
        }
        let plus = Enclosure::point(self.radial_upper)
            .multiply(cosine)
            .add(Enclosure::point(self.threshold.hi));
        let upper = Enclosure::point(self.weight.hi).square().multiply(plus).hi;
        Ok(Enclosure {
            lo: lower,
            hi: upper,
        })
    }
}

pub(super) fn cap(
    cap: DecodedCapTrim,
    query: CarrierSeparationQuery<'_>,
    trig: &Trig,
) -> Result<CapSeparation, SeparationCause> {
    let r = Enclosure::point(cap.frame.radius);
    let z = Enclosure::point(cap.frame.origin[2])
        .add(Enclosure::point(cap.frame.location.translation[2]));
    let [a, b, _] = cap.frame.location.matrix[0].map(Enclosure::point);
    let stretch = a
        .square()
        .add(b.square())
        .positive_sqrt()
        .ok_or(SeparationCause::PositiveRadius { lower: 0. })?;
    let [lo, hi] = cap.axial_parameters;
    let axial = Enclosure { lo, hi }.multiply(trig.cosine).add(z);
    let rim_lower = rim_gaps(axial, query.rims)?;
    let lower = cap
        .axial_parameters
        .into_iter()
        .try_fold(f64::INFINITY, |bound, v| {
            let signed = cap_endpoint(Enclosure::point(v), r, z, stretch, query, trig)?;
            Ok::<_, SeparationCause>(bound.min(signed))
        })?;
    positive_sign(query.sense, 0, lower)?;
    Ok(CapSeparation {
        id: cap.id,
        signed_normal_lower: lower,
        rim_lower,
    })
}

fn cap_endpoint(
    v: Enclosure,
    r: Enclosure,
    z: Enclosure,
    stretch: Enclosure,
    query: CarrierSeparationQuery<'_>,
    trig: &Trig,
) -> Result<f64, SeparationCause> {
    // With positive radius and the stored conformal XY block, D is affine in V.
    let radius = r.add(v.multiply(trig.sine));
    if radius.lo <= 0. {
        return Err(SeparationCause::PositiveRadius { lower: radius.lo });
    }
    let radial = radius
        .multiply(stretch)
        .subtract(Enclosure::point(query.stock.radius))
        .multiply(trig.cosine);
    let axial = z
        .add(v.multiply(trig.cosine))
        .subtract(Enclosure::point(query.stock.origin[2]))
        .multiply(trig.sine);
    Ok(radial.subtract(axial).directed_lower(query.sense))
}
