pub(super) type Homogeneous = Vec<[Enclosure; 4]>;

pub(super) fn world_homogeneous(point: [f64; 4], location: &[f64; 13]) -> Option<[Enclosure; 4]> {
    let w = point[3];
    if !point.iter().all(|p| p.is_finite()) || w <= 0. {
        return None;
    }
    let input = point.map(Enclosure::point);
    let mut result = input;
    for i in 0..3 {
        let mut value = Enclosure::point(location[i + 1]);
        for j in 0..3 {
            value = value.add(Enclosure::point(location[4 + 3 * i + j]).multiply(input[j]));
        }
        result[i] = value.multiply(input[3]);
    }
    result.iter().all(|cell| cell.finite()).then_some(result)
}

pub(super) fn halve(mut row: Homogeneous) -> (Homogeneous, Homogeneous) {
    let mut left = Vec::with_capacity(row.len());
    let mut right = Vec::with_capacity(row.len());
    while !row.is_empty() {
        left.push(row[0]);
        right.push(row[row.len() - 1]);
        for i in 0..row.len() - 1 {
            row[i] = std::array::from_fn(|j| row[i][j].add(row[i + 1][j]).half());
        }
        row.pop();
    }
    right.reverse();
    (left, right)
}

pub(super) fn rounded(exact: &[[Enclosure; 4]]) -> Option<(Vec<[f64; 3]>, Vec<f64>, f64)> {
    let wlo = exact.iter().map(|x| x[3].lo).fold(f64::INFINITY, f64::min);
    if wlo <= 0. || !wlo.is_finite() || exact.iter().flatten().any(|cell| !cell.finite()) {
        return None;
    }
    let poles: Vec<_> = exact
        .iter()
        .map(|p| std::array::from_fn(|i| p[i].over_positive(p[3]).midpoint()))
        .collect();
    let weights: Vec<_> = exact.iter().map(|p| p[3].midpoint()).collect();
    if poles.iter().flatten().any(|x| !x.is_finite())
        || weights.iter().any(|x| *x <= 0. || !x.is_finite())
    {
        return None;
    }
    let mut rounding = 0.;
    let w_error = exact
        .iter()
        .zip(&weights)
        .map(|(cell, w)| cell[3].sub(Enclosure::point(*w)).abs_upper())
        .fold(0., f64::max);
    for i in 0..3 {
        let maximum_pole = poles.iter().map(|p| p[i].abs()).fold(0., f64::max);
        let numerator_error = exact
            .iter()
            .zip(poles.iter().zip(&weights))
            .map(|(cell, (p, w))| {
                cell[i]
                    .sub(Enclosure::point(p[i]).multiply(Enclosure::point(*w)))
                    .abs_upper()
            })
            .fold(0., f64::max);
        // For exact N/W and rounded N'/W', bound N-N' and W-W' in Bernstein
        // form, bound N'/W' by its pole hull, and divide by the positive lower W.
        let axis = up(up(numerator_error + up(maximum_pole * w_error)) / wlo);
        rounding = up(rounding + axis);
    }
    rounding.is_finite().then_some((poles, weights, rounding))
}

#[derive(Clone, Copy)]
pub(super) struct Enclosure {
    lo: f64,
    hi: f64,
}

impl Enclosure {
    fn point(x: f64) -> Self {
        Self { lo: x, hi: x }
    }
    fn finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }
    fn add(self, rhs: Self) -> Self {
        Self {
            lo: down(self.lo + rhs.lo),
            hi: up(self.hi + rhs.hi),
        }
    }
    fn sub(self, rhs: Self) -> Self {
        Self {
            lo: down(self.lo - rhs.hi),
            hi: up(self.hi - rhs.lo),
        }
    }
    fn half(self) -> Self {
        Self {
            lo: down(self.lo * 0.5),
            hi: up(self.hi * 0.5),
        }
    }
    fn midpoint(self) -> f64 {
        (self.lo * 0.5) + (self.hi * 0.5)
    }
    fn abs_upper(self) -> f64 {
        self.lo.abs().max(self.hi.abs())
    }
    fn multiply(self, rhs: Self) -> Self {
        limits([
            self.lo * rhs.lo,
            self.hi * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.hi,
        ])
    }
    fn over_positive(self, rhs: Self) -> Self {
        limits([
            self.lo / rhs.lo,
            self.hi / rhs.lo,
            self.lo / rhs.hi,
            self.hi / rhs.hi,
        ])
    }
}

fn limits(values: [f64; 4]) -> Enclosure {
    Enclosure {
        lo: down(values.into_iter().fold(f64::INFINITY, f64::min)),
        hi: up(values.into_iter().fold(f64::NEG_INFINITY, f64::max)),
    }
}

fn up(x: f64) -> f64 {
    x.next_up()
}
fn down(x: f64) -> f64 {
    x.next_down()
}
