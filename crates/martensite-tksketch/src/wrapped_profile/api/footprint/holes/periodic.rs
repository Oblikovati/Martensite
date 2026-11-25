//! Rounded comparisons use an enclosure of the physical angular period.
use super::*;
use std::f64::consts::TAU;

pub(super) fn images(bounds: TrimBox) -> Checked<Vec<BvhBox<BvhVec2d>>> {
    if !bounds.finite() {
        return Err(unsupported("finite ordered inner-clearance query envelope"));
    }
    let [angular, meridian] = bounds.0;
    Ok(windows(angular)
        .into_iter()
        .map(|[lo, hi]| {
            BvhBox::from_corners(
                BvhVec2d::new(lo, meridian[0]),
                BvhVec2d::new(hi, meridian[1]),
            )
        })
        .collect())
}

fn windows(angular: [f64; 2]) -> Vec<[f64; 2]> {
    let turns = (angular[0] / TAU).floor();
    // The binary64 TAU constant and its successor enclose the mathematical period.
    let full = [0., TAU.next_up()];
    if !turns.is_finite() || turns.abs() >= 4_503_599_627_370_496. {
        return vec![full];
    }
    let [lower, upper] = shifted(angular, turns);
    if lower <= -TAU || upper >= 2. * TAU {
        return vec![full];
    }
    // The central copy stays within three periods, so only its immediate neighbors can enter one.
    let images: Vec<_> = [-1., 0., 1.]
        .into_iter()
        .filter_map(|offset| intersect_domain(shifted(angular, turns + offset)))
        .collect();
    if images.is_empty() {
        vec![full]
    } else {
        images
    }
}

fn shifted(angular: [f64; 2], turns: f64) -> [f64; 2] {
    if turns == 0. {
        return angular;
    }
    let a = turns * TAU;
    let b = turns * TAU.next_up();
    [
        (angular[0] - a.max(b).next_up()).next_down(),
        (angular[1] - a.min(b).next_down()).next_up(),
    ]
}

fn intersect_domain([lo, hi]: [f64; 2]) -> Option<[f64; 2]> {
    let interval = [lo.max(0.), hi.min(TAU.next_up())];
    (interval[0] <= interval[1]).then_some(interval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seam_neighbors_and_distant_lifts_preserve_equal_physical_angles() {
        let hole = TrimBox([[0.02, 0.05], [1., 2.]]);
        let mut index = Exclusions::new(&[hole]).ok().unwrap();
        for angular in [
            [-0.1, 0.03],
            [TAU + 0.02, TAU + 0.05],
            [TAU * 7. + 0.02, TAU * 7. + 0.05],
        ] {
            assert_eq!(
                index.overlap(TrimBox([angular, [1.5, 2.]])).ok().unwrap(),
                Some(1)
            );
        }
        assert_eq!(
            index
                .overlap(TrimBox([[TAU - 0.1, TAU + 0.01], [1.5, 2.]]))
                .ok()
                .unwrap(),
            None
        );
        assert_eq!(
            index
                .overlap(TrimBox([[0.02, 0.03], [2.01, 3.]]))
                .ok()
                .unwrap(),
            None
        );
        assert_eq!(
            index
                .overlap(TrimBox([[0.05, 0.06], [1., 2.]]))
                .ok()
                .unwrap(),
            Some(1)
        );
    }
}
