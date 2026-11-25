use super::*;

#[derive(Clone, Copy)]
struct Hull([[f64; 2]; 2]);

type Cubic = [[I; 2]; 4];

#[derive(Clone, Copy)]
struct Cell {
    curve: Cubic,
    hull: Hull,
}
impl Cell {
    fn new(curve: Cubic) -> Self {
        let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
        for point in &curve {
            for axis in 0..2 {
                bounds[axis][0] = bounds[axis][0].min(point[axis].0);
                bounds[axis][1] = bounds[axis][1].max(point[axis].1);
            }
        }
        Self {
            curve,
            hull: Hull(bounds),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Bounds {
    pub nonincident: f64,
    pub foreign: f64,
    pub cross_loop: f64,
    pub angle_scale: f64,
    pub pairs: usize,
    pub split_visits: usize,
}

pub(super) fn census(
    input: CensusInput<'_>,
    charts: &BTreeMap<PieceId, Piece>,
    incidents: &BTreeSet<Incidence>,
    ranges: Ranges,
    loops: usize,
) -> Result<Census, Failure> {
    let c = input.cone;
    if !positive(c.radius)
        || !(0. < c.sin[0] && c.sin[0] <= c.sin[1] && c.sin[1] < 1.)
        || !positive(input.endpoint_window_threshold)
        || !positive(input.native_rim_lower)
        || !positive(input.native_periodic_lower)
    {
        return Err(Failure::InvalidScalar);
    }
    let weight = angle_scale(c, ranges);
    if !positive(weight) {
        return Err(Failure::Pad);
    }
    let cut_sq = I::p(input.endpoint_window_threshold).square().1;
    let cells: Vec<_> = charts
        .values()
        .map(|p| (p, Cell::new(p.uv.map(|uv| uv.map(I::p)))))
        .collect();
    let mut bounds = empty_bounds(weight);
    prove_pairs(&cells, cut_sq, &mut bounds)?;
    let mut windows = Vec::new();
    for &j in incidents {
        let (w, n) = choose_window(j, charts, cut_sq, weight)?;
        windows.push(w);
        bounds.split_visits += n;
    }
    let lower = distance_lower(bounds.nonincident);
    Ok(Census {
        separation: Separation {
            all_nonincident_lower: lower,
            all_cross_contour_lower: distance_lower(bounds.cross_loop).min(f64::MAX),
            all_foreign_face_lower: distance_lower(bounds.foreign).min(f64::MAX),
            native_rim_lower: input.native_rim_lower,
            native_periodic_lower: input.native_periodic_lower,
            excluded_loop_pairs: loops
                .checked_mul(loops.saturating_sub(1))
                .ok_or(Failure::Census)?
                / 2,
            endpoint_window_threshold: input.endpoint_window_threshold,
            weighted_angle_scale: weight,
        },
        windows,
        pairs: bounds.pairs,
        split_visits: bounds.split_visits,
    })
}

fn empty_bounds(weight: f64) -> Bounds {
    Bounds {
        nonincident: f64::INFINITY,
        foreign: f64::INFINITY,
        cross_loop: f64::INFINITY,
        angle_scale: weight,
        pairs: 0,
        split_visits: 0,
    }
}

fn angle_scale(cone: Cone, ranges: Ranges) -> f64 {
    let rho = (I::p(cone.radius) + I::p(ranges.lo[1]) * I::p(cone.sin[0])).0;
    (I::p(rho) * (I::p(7.) / I::p(11.))).0
}

fn choose_window(
    pair: Incidence,
    charts: &BTreeMap<PieceId, Piece>,
    cutoff: f64,
    scale: f64,
) -> Result<(Window, usize), Failure> {
    let Incidence([a, b], v) = pair;
    let a = oriented(&charts[&a], v)?;
    let b = oriented(&charts[&b], v)?;
    for level in (1..=13).rev() {
        let fraction = 1. / f64::from(1u32 << level);
        if let Ok(n) = prove_window_rectangle(a, b, fraction, cutoff, scale) {
            return Ok((Window { pair, fraction }, n));
        }
    }
    Err(Failure::Separation)
}

fn prove_window_rectangle(
    a: Cubic,
    b: Cubic,
    fraction: f64,
    cutoff: f64,
    scale: f64,
) -> Result<usize, Failure> {
    let in_direction = |a, b| {
        let [an, af] = split(a, fraction);
        let [_, bf] = split(b, fraction);
        let (_, left) = exclude(af, Cell::new(b), cutoff, scale)?;
        let (_, right) = exclude(an, bf, cutoff, scale)?;
        Ok(left + right)
    };
    in_direction(a, b).or_else(|_: Failure| in_direction(b, a))
}

pub(super) fn prove(input: &Input<'_>, geo: &Geometry) -> Result<Bounds, Failure> {
    let rho = (I::p(input.cone.radius) + I::p(geo.ranges.lo[1]) * I::p(input.cone.sin[0])).0;
    // All points examined below are on seam pieces, with a chart difference < pi.
    let scale = (I::p(rho) * (I::p(7.) / I::p(11.)))
        .0
        .min(input.separation.weighted_angle_scale);
    let cut = input.separation.endpoint_window_threshold;
    if !positive(scale) || !positive(cut) {
        return Err(Failure::InvalidScalar);
    }
    let cut_sq = I::p(cut).square().1;
    let shapes: Vec<_> = geo
        .charts
        .values()
        .map(|p| (p, Cell::new(p.uv.map(|pt| pt.map(I::p)))))
        .collect();
    let mut bounds = Bounds {
        nonincident: f64::INFINITY,
        foreign: f64::INFINITY,
        cross_loop: f64::INFINITY,
        angle_scale: scale,
        pairs: 0,
        split_visits: 0,
    };
    prove_pairs(&shapes, cut_sq, &mut bounds)?;
    prove_windows(input, geo, cut_sq, &mut bounds)?;
    bounds.nonincident = distance_lower(bounds.nonincident);
    bounds.cross_loop = distance_lower(bounds.cross_loop);
    bounds.foreign = distance_lower(bounds.foreign);
    Ok(bounds)
}

fn distance_lower(square: f64) -> f64 {
    // Rust f64::sqrt is the correctly rounded IEEE 754 square-root operation.
    square.sqrt().next_down()
}

fn prove_pairs(shapes: &[(&Piece, Cell)], cut_sq: f64, bounds: &mut Bounds) -> Result<(), Failure> {
    let mut loops = BTreeSet::new();
    for (i, (a, ac)) in shapes.iter().enumerate() {
        for (b, bc) in shapes.iter().skip(i + 1) {
            bounds.pairs += 1;
            if a.id.0 == b.id.0 || shared(a, b) {
                continue;
            }
            let gap = gap_sq(ac.hull, bc.hull, bounds.angle_scale);
            let margin = if gap > cut_sq {
                gap
            } else {
                let (lower, visits) = exclude(*ac, *bc, cut_sq, bounds.angle_scale)?;
                bounds.split_visits += visits;
                lower
            };
            bounds.nonincident = bounds.nonincident.min(margin);
            if a.contour != b.contour {
                loops.insert([a.contour.min(b.contour), a.contour.max(b.contour)]);
                bounds.cross_loop = bounds.cross_loop.min(margin);
                if a.contour.0 != b.contour.0 {
                    bounds.foreign = bounds.foreign.min(margin);
                }
            }
        }
    }
    let mut distinct = BTreeSet::new();
    distinct.extend(shapes.iter().map(|(p, _)| p.contour));
    if loops.len()
        != distinct
            .len()
            .saturating_mul(distinct.len().saturating_sub(1))
            / 2
    {
        return Err(Failure::Census);
    }
    Ok(())
}

fn prove_windows(
    input: &Input<'_>,
    geo: &Geometry,
    cut_sq: f64,
    bounds: &mut Bounds,
) -> Result<(), Failure> {
    validate_windows(input, geo)?;
    for window in input.windows {
        let Incidence([a, b], tip) = window.pair;
        let a = oriented(&geo.charts[&a], tip)?;
        let b = oriented(&geo.charts[&b], tip)?;
        bounds.split_visits +=
            prove_window_rectangle(a, b, window.fraction, cut_sq, bounds.angle_scale)?;
    }
    Ok(())
}

fn oriented(piece: &Piece, tip: Vertex) -> Result<Cubic, Failure> {
    let raw = piece.uv.map(|pt| pt.map(I::p));
    if piece.endpoints[0] == Some(tip) {
        return Ok(raw);
    }
    if piece.endpoints[1] == Some(tip) {
        return Ok([raw[3], raw[2], raw[1], raw[0]]);
    }
    Err(Failure::Identity)
}

fn shared(a: &Piece, b: &Piece) -> bool {
    a.endpoints
        .iter()
        .flatten()
        .any(|v| b.endpoints.contains(&Some(*v)))
}

fn gap_sq(a: Hull, b: Hull, scale: f64) -> f64 {
    let gap = |i: usize| {
        (a.0[i][0] - b.0[i][1])
            .next_down()
            .max((b.0[i][0] - a.0[i][1]).next_down())
            .max(0.)
    };
    let u = (I::p(scale) * I::p(gap(0))).0.max(0.);
    (I::p(u).square() + I::p(gap(1)).square()).0.max(0.)
}

fn size(h: Hull, scale: f64) -> f64 {
    // This heuristic only chooses which exact partition to refine.
    ((h.0[0][1] - h.0[0][0]) * scale).max(h.0[1][1] - h.0[1][0])
}

fn split(curve: Cubic, t: f64) -> [Cell; 2] {
    let near = I::p(t);
    let far = I::p(1.) - near;
    let interp = |a: [I; 2], b: [I; 2]| [0, 1].map(|i| far * a[i] + near * b[i]);
    let [a, b, c, d] = curve;
    let (ab, bc, cd) = (interp(a, b), interp(b, c), interp(c, d));
    let (abc, bcd) = (interp(ab, bc), interp(bc, cd));
    let p = interp(abc, bcd);
    [Cell::new([a, ab, abc, p]), Cell::new([p, bcd, cd, d])]
}

fn exclude(a: Cell, b: Cell, cut_sq: f64, scale: f64) -> Result<(f64, usize), Failure> {
    const MAX_NODES: usize = 2048;
    const MAX_DEPTH: usize = 24;
    let mut pending = vec![(a, b, 0)];
    let mut bound = f64::INFINITY;
    let mut visits = 0;
    while let Some((a, b, depth)) = pending.pop() {
        visits += 1;
        let gap = gap_sq(a.hull, b.hull, scale);
        if gap > cut_sq {
            bound = bound.min(gap);
            continue;
        }
        let am = split(a.curve, 0.5);
        let bm = split(b.curve, 0.5);
        let d = [0, 1].map(|i| am[0].curve[3][i] - bm[0].curve[3][i]);
        let close = ((d[0] * I::p(scale)).square() + d[1].square()).1 <= cut_sq;
        if close {
            return Err(Failure::Separation);
        }
        if visits >= MAX_NODES || depth >= MAX_DEPTH {
            return Err(Failure::Separation);
        }
        if size(a.hull, scale) >= size(b.hull, scale) {
            pending.extend(split(a.curve, 0.5).map(|a| (a, b, depth + 1)));
        } else {
            pending.extend(split(b.curve, 0.5).map(|b| (a, b, depth + 1)));
        }
    }
    Ok((bound, visits))
}
