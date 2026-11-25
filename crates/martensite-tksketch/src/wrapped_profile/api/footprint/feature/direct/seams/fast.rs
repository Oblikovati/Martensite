#[derive(Clone, Copy, Default)]
struct Interval {
    lower: f64,
    upper: f64,
}

impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            lower: (self.lower + rhs.lower).next_down(),
            upper: (self.upper + rhs.upper).next_up(),
        }
    }

    fn neg(self) -> Self {
        Self {
            lower: -self.upper,
            upper: -self.lower,
        }
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lower * rhs.lower,
            self.lower * rhs.upper,
            self.upper * rhs.lower,
            self.upper * rhs.upper,
        ];
        Self {
            lower: products
                .into_iter()
                .fold(f64::INFINITY, f64::min)
                .next_down(),
            upper: products
                .into_iter()
                .fold(f64::NEG_INFINITY, f64::max)
                .next_up(),
        }
    }

    fn fraction(self, divisor: f64) -> Self {
        debug_assert!(divisor.is_finite() && divisor > 0.0);
        Self {
            lower: (self.lower / divisor).next_down(),
            upper: (self.upper / divisor).next_up(),
        }
    }

    fn magnitude(self) -> f64 {
        self.lower.abs().max(self.upper.abs())
    }
}

const ONE: Interval = Interval {
    lower: 1.0,
    upper: 1.0,
};
const PI: Interval = Interval {
    lower: std::f64::consts::PI,
    upper: std::f64::consts::PI.next_up(),
};
const MAX_DEGREE: usize = 36;

const fn pascal_table() -> [[u64; MAX_DEGREE + 1]; MAX_DEGREE + 1] {
    let mut table = [[0; MAX_DEGREE + 1]; MAX_DEGREE + 1];
    let mut n = 0;
    while n <= MAX_DEGREE {
        table[n][0] = 1;
        table[n][n] = 1;
        let mut k = 1;
        while k < n {
            table[n][k] = table[n - 1][k - 1] + table[n - 1][k];
            k += 1;
        }
        n += 1;
    }
    table
}
static BINOMIAL: [[u64; MAX_DEGREE + 1]; MAX_DEGREE + 1] = pascal_table();

fn product(a: &[Interval], b: &[Interval]) -> Vec<Interval> {
    let (m, n) = (a.len() - 1, b.len() - 1);
    let mut out = vec![Interval::default(); m + n + 1];
    for (i, x) in a.iter().copied().enumerate() {
        for (j, y) in b.iter().copied().enumerate() {
            let numerator = BINOMIAL[m][i] * BINOMIAL[n][j];
            let ratio = Interval::point(numerator as f64).fraction(BINOMIAL[m + n][i + j] as f64);
            out[i + j] = out[i + j].add(x.mul(y).mul(ratio));
        }
    }
    out
}

fn elevate(poles: &[Interval], degree: usize) -> Vec<Interval> {
    if poles.len() == degree + 1 {
        return poles.to_vec();
    }
    product(poles, &vec![ONE; degree + 2 - poles.len()])
}

fn combine(
    a: &[Interval],
    b: &[Interval],
    combine: impl Fn(Interval, Interval) -> Interval,
) -> Vec<Interval> {
    let degree = a.len().max(b.len()) - 1;
    elevate(a, degree)
        .into_iter()
        .zip(elevate(b, degree))
        .map(|(l, r)| combine(l, r))
        .collect()
}

fn widen(value: Interval, error: f64) -> Interval {
    value.add(Interval {
        lower: -error,
        upper: error,
    })
}

fn tail(amplitude: f64, order: u32) -> f64 {
    let amplitude = Interval::point(amplitude);
    (1..=order)
        .fold(ONE, |prior, k| prior.mul(amplitude).fraction(f64::from(k)))
        .upper
}

fn scalar_trig(center: f64) -> [Interval; 2] {
    let mut target = Interval::point(center);
    if center > PI.lower {
        target = target.sub(PI.add(PI));
    } else if center < -PI.lower {
        target = target.add(PI.add(PI));
    }
    let square = target.mul(target);
    let mut terms = [ONE, target];
    let mut sum = terms;
    for order in 1..=20_u32 {
        for axis in 0..2 {
            let k = 2 * order + axis as u32;
            terms[axis] = terms[axis].mul(square).fraction(f64::from(k * (k - 1)));
            sum[axis] = if order % 2 == 0 {
                sum[axis].add(terms[axis])
            } else {
                sum[axis].sub(terms[axis])
            };
        }
    }
    let error = tail(target.magnitude(), 42);
    sum.map(|value| widen(value, error))
}

fn trig_polynomial(u: &[f64; 4]) -> [Vec<Interval>; 2] {
    let low = u.iter().copied().fold(f64::INFINITY, f64::min);
    let high = u.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let center = low.midpoint(high);
    let values = scalar_trig(center);
    if low == high {
        return values.map(|value| vec![value]);
    }
    let delta = u.map(|value| Interval::point(value).sub(Interval::point(center)));
    let amplitude = delta
        .into_iter()
        .map(Interval::magnitude)
        .fold(0.0, f64::max);
    let mut sum = [vec![values[0]], vec![values[1]]];
    let mut power = vec![ONE];
    let mut factorial = 1_u32;
    for order in 1..=8_u32 {
        power = product(&power, &delta);
        factorial *= order;
        for (axis, aggregate) in sum.iter_mut().enumerate() {
            let cycle = (order as usize + axis) % 4;
            let sign = if cycle == 1 || cycle == 2 { -1.0 } else { 1.0 };
            // cos derivatives are cos, -sin, -cos, sin. sin derivatives are sin, cos, -sin, -cos.
            let sign = if axis == 0 { sign } else { -sign };
            let coefficient = if sign < 0.0 {
                values[cycle % 2].neg()
            } else {
                values[cycle % 2]
            };
            let factor = coefficient.fraction(f64::from(factorial));
            let term = power
                .iter()
                .map(|term| term.mul(factor))
                .collect::<Vec<_>>();
            *aggregate = combine(aggregate, &term, Interval::add);
        }
    }
    let error = tail(amplitude, 9);
    sum.map(|poles| poles.into_iter().map(|value| widen(value, error)).collect())
}

fn bisect(poles: &[Interval]) -> [Vec<Interval>; 2] {
    let mut work = poles.to_vec();
    let mut left = vec![work[0]];
    let mut right = vec![work[work.len() - 1]];
    for active in (1..poles.len()).rev() {
        for i in 0..active {
            work[i] = work[i].add(work[i + 1]).fraction(2.0);
        }
        left.push(work[0]);
        right.push(work[active - 1]);
    }
    right.reverse();
    [left, right]
}

fn magnitude(poles: &[Interval]) -> f64 {
    poles
        .iter()
        .copied()
        .map(Interval::magnitude)
        .fold(0.0, f64::max)
}

fn residual(
    coordinate: &[Interval],
    factor: &[Interval],
    target: &[Interval],
    weight: f64,
    limit: f64,
) -> f64 {
    let target = product(factor, target);
    let difference = combine(coordinate, &target, Interval::sub);
    let hull = magnitude(&difference);
    let cutoff = Interval::point(limit)
        .mul(Interval::point(weight))
        .fraction(4.0)
        .lower;
    let upper = if hull <= cutoff {
        hull
    } else {
        bisect(&difference)
            .into_iter()
            .flat_map(|cell| bisect(&cell))
            .map(|cell| magnitude(&cell))
            .fold(0.0, f64::max)
    };
    Interval::point(upper).fraction(weight).upper
}

pub(super) struct Query {
    pub(super) angle: f64,
    pub(super) radius: f64,
    pub(super) origin: f64,
    pub(super) limit: f64,
    pub(super) curve: [[f64; 4]; 10],
    pub(super) uv: [[f64; 2]; 4],
}

pub(super) fn certify(query: &Query) -> Option<[f64; 3]> {
    // Degree and magnitude bounds keep the directed expansions inside their certified range.
    if !(0.0..=1.0).contains(&query.angle)
        || !(1.0..=100.0).contains(&query.radius)
        || !(-128.0..=128.0).contains(&query.origin)
        || !query.uv.iter().all(|&[u, v]| {
            (-std::f64::consts::TAU..=std::f64::consts::TAU).contains(&u) && v.abs() <= 256.0
        })
        || !query
            .curve
            .iter()
            .all(|p| p[..3].iter().all(|x| x.abs() <= 256.0) && (0.1..=16.0).contains(&p[3]))
        || !query.limit.is_finite()
        || query.limit <= 0.0
    {
        return None;
    }
    let [cos, sin] = scalar_trig(query.angle);
    let phase = query.uv.map(|p| p[0]);
    let station = query.uv.map(|p| Interval::point(p[1]));
    let radius = station.map(|v| v.mul(sin).add(Interval::point(query.radius)));
    if radius.iter().any(|r| r.lower <= 0.0) {
        return None;
    }
    let height = station.map(|v| v.mul(cos).add(Interval::point(query.origin)));
    let weight = query.curve.map(|p| Interval::point(p[3]));
    let minimum = weight.iter().map(|w| w.lower).fold(f64::INFINITY, f64::min);
    let stored = |axis: usize| {
        query
            .curve
            .map(|p| Interval::point(p[axis]).mul(Interval::point(p[3])))
    };
    let scaled = product(&weight, &radius);
    let [cosine, sine] = trig_polynomial(&phase);
    let x = residual(&stored(0), &scaled, &cosine, minimum, query.limit);
    let y = residual(&stored(1), &scaled, &sine, minimum, query.limit);
    let z = residual(&stored(2), &weight, &height, minimum, query.limit);
    Some([
        Interval::point(x).add(Interval::point(y)).upper,
        z,
        Interval::point(x)
            .add(Interval::point(y))
            .add(Interval::point(z))
            .upper,
    ])
}
