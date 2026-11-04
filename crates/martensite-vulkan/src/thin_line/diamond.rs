//! Exact half-pixel-grid reference for the selected open-diamond line contract.
pub(super) fn exits_diamond(
    start: [i64; 2],
    end: [i64; 2],
    pixel_center: [i64; 2],
    pixel_scale: i64,
) -> bool {
    let pixel_radius = i128::from(pixel_scale) / 2;
    let [u0, v0] = transformed(start);
    let [u1, v1] = transformed(end);
    let [uc, vc] = transformed(pixel_center);
    if endpoint_inside([u1, v1], [uc, vc], pixel_radius) {
        return false;
    }
    intersects_open_square([u0, v0], [u1, v1], [uc, vc], pixel_radius)
}

fn transformed(point: [i64; 2]) -> [i128; 2] {
    let [x, y] = point.map(i128::from);
    [x + y, x - y]
}

fn endpoint_inside(point: [i128; 2], center: [i128; 2], radius: i128) -> bool {
    (0..2).all(|axis| {
        let lower = center[axis] - radius;
        let upper = center[axis] + radius;
        let basis = [if axis == 0 { 1 } else { 0 }, if axis == 1 { 1 } else { 0 }];
        perturbed_cmp(point[axis], lower, basis[0], basis[1]).is_gt()
            && perturbed_cmp(point[axis], upper, basis[0], basis[1]).is_lt()
    })
}

fn intersects_open_square(a: [i128; 2], b: [i128; 2], center: [i128; 2], radius: i128) -> bool {
    let low = center.map(|v| v - radius);
    let high = center.map(|v| v + radius);
    axis_overlap(a, b, low, high, 1, 0)
        && axis_overlap(a, b, low, high, 0, 1)
        && direction_overlap(a, b, low, high)
}

fn axis_overlap(
    a: [i128; 2],
    b: [i128; 2],
    low: [i128; 2],
    high: [i128; 2],
    x: i128,
    y: i128,
) -> bool {
    projection_overlap(a, b, low, high, x, y)
}

fn direction_overlap(a: [i128; 2], b: [i128; 2], low: [i128; 2], high: [i128; 2]) -> bool {
    let direction = [b[0] - a[0], b[1] - a[1]];
    if direction == [0, 0] {
        return false;
    }
    let normal = [-direction[1], direction[0]];
    projection_overlap(a, b, low, high, normal[0], normal[1])
        && projection_overlap(a, b, low, high, direction[0], direction[1])
}

fn projection_overlap(
    a: [i128; 2],
    b: [i128; 2],
    low: [i128; 2],
    high: [i128; 2],
    x: i128,
    y: i128,
) -> bool {
    let segment = [dot(a, x, y), dot(b, x, y)];
    let square = corners(low, high).map(|point| dot(point, x, y));
    strict_overlap(segment, square, x, y)
}

fn corners(low: [i128; 2], high: [i128; 2]) -> [[i128; 2]; 4] {
    [
        [low[0], low[1]],
        [low[0], high[1]],
        [high[0], low[1]],
        [high[0], high[1]],
    ]
}

fn dot(point: [i128; 2], x: i128, y: i128) -> i128 {
    point[0] * x + point[1] * y
}

fn strict_overlap(segment: [i128; 2], square: [i128; 4], x: i128, y: i128) -> bool {
    let (seg_low, seg_high) = ordered(segment[0], segment[1]);
    let (square_low, square_high) = range(square);
    perturbed_cmp(seg_low, square_high, x, y).is_lt()
        && perturbed_cmp(seg_high, square_low, x, y).is_gt()
}

fn perturbed_cmp(value: i128, bound: i128, x: i128, y: i128) -> std::cmp::Ordering {
    let first_order = (value - bound).cmp(&0);
    if first_order.is_ne() {
        return first_order;
    }
    // GL's symbolic endpoint move is (-eps,-eps^2), so U=x+y and V=x-y
    // receive exact lexicographic shifts rather than a chosen finite epsilon.
    let epsilon = (-(x + y)).cmp(&0);
    if epsilon.is_ne() {
        return epsilon;
    }
    (-x + y).cmp(&0)
}

fn ordered(a: i128, b: i128) -> (i128, i128) {
    (a.min(b), a.max(b))
}

fn range(values: [i128; 4]) -> (i128, i128) {
    values
        .into_iter()
        .fold((i128::MAX, i128::MIN), |(lo, hi), value| {
            (lo.min(value), hi.max(value))
        })
}
