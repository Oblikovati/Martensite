// SPDX-License-Identifier: GPL-2.0-only
//! A fixed seed reference with a basis differentiated at every trial.
use super::{ResidualError, Vector, constant_vector, unit};
use crate::LineGeometry;

const TRIAL_LIMIT: f64 = 0.9;
const REFRESH_MARGIN: f64 = 0.75;

#[derive(Clone, Copy)]
pub(in crate::constraints) enum FrameReference {
    Axis(usize),
    Normal([f64; 3]),
}

impl FrameReference {
    pub(in crate::constraints) fn axis(line: LineGeometry) -> Self {
        Self::Axis(axis(*line.carrier.direction().coordinates()))
    }

    pub(in crate::constraints) fn pair([first, second]: [LineGeometry; 2]) -> Self {
        let a = *first.carrier.direction().coordinates();
        let b = *second.carrier.direction().coordinates();
        let cross = std::array::from_fn(|i| {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            a[j] * b[k] - a[k] * b[j]
        });
        // Near parallelism, the cross product may be dominated by cancellation.
        let length = martensite_cmath::hypot(martensite_cmath::hypot(cross[0], cross[1]), cross[2]);
        if length > f64::EPSILON.sqrt() {
            Self::Normal(cross.map(|component| component / length))
        } else {
            Self::axis(first)
        }
    }

    pub(in crate::constraints) fn needs_refresh(self, line: LineGeometry) -> bool {
        let direction = *line.carrier.direction().coordinates();
        let projection: f64 = direction
            .into_iter()
            .zip(self.coordinates())
            .map(|(a, b)| a * b)
            .sum();
        projection.abs() >= REFRESH_MARGIN
    }

    fn coordinates(self) -> [f64; 3] {
        match self {
            Self::Axis(i) => std::array::from_fn(|j| f64::from(i == j)),
            Self::Normal(normal) => normal,
        }
    }

    pub(super) fn basis(
        self,
        direction: &Vector,
        relation: &str,
    ) -> Result<[Vector; 2], ResidualError> {
        let reference = constant_vector(self.coordinates())?;
        let projection = match self {
            Self::Axis(i) => direction.components()[i].clone(),
            Self::Normal(_) => direction.dot(&reference)?,
        };
        if projection.value().abs() > TRIAL_LIMIT {
            return Err(ResidualError::Domain(format!(
                "{relation} frame component {}: expected magnitude at most {TRIAL_LIMIT}",
                projection.value()
            )));
        }
        let first = unit(&reference.minus(&direction.product(&projection)?)?)?;
        let second = direction.cross(&first)?;
        Ok([first, second])
    }
}

fn axis([x, y, z]: [f64; 3]) -> usize {
    let [x, y, z] = [x.abs(), y.abs(), z.abs()];
    if x <= y && x <= z {
        0
    } else if y <= z {
        1
    } else {
        2
    }
}
