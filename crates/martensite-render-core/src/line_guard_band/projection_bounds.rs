//! Independent homogeneous coordinate ranges enclose the per-endpoint f32 expressions.
use crate::RenderVertex;

#[derive(Clone, Copy)]
pub(super) struct PositionBounds([Range; 4]);

impl PositionBounds {
    pub(super) fn indexed(vertices: &[RenderVertex], indices: &[u32]) -> Option<Self> {
        let mut endpoints = indices.as_chunks::<2>().0.iter().flatten();
        let first = components(vertices.get(*endpoints.next()? as usize)?);
        let mut bounds = first.map(Range::point);
        for &index in endpoints {
            let values = components(vertices.get(index as usize)?);
            if !values.iter().all(|value| value.is_finite()) {
                return None;
            }
            for (range, value) in bounds.iter_mut().zip(values) {
                range.include(value);
            }
        }
        first
            .iter()
            .all(|value| value.is_finite())
            .then_some(Self(bounds))
    }

    pub(super) fn spans_less_than(self, matrix: &[f32; 16], extent: [f32; 2], limit: f32) -> bool {
        let clip: [Range; 4] = std::array::from_fn(|row| self.clip_axis(matrix, row));
        if !clip.iter().all(Range::finite) || clip[3].minimum <= 0. {
            return false;
        }
        (0..2).all(|axis| clip[axis].window_span(clip[3], extent[axis]) < limit)
    }

    fn clip_axis(self, matrix: &[f32; 16], row: usize) -> Range {
        // Rounded IEEE operations are monotone in each operand with the selected signs.
        // Use the endpoint operation order so the actual f32 results stay inside the range.
        (0..4).fold(Range::point(0.), |sum, column| {
            sum.plus(self.0[column].scale(matrix[column * 4 + row]))
        })
    }
}

fn components(vertex: &RenderVertex) -> [f32; 4] {
    let [x, y, z] = vertex.position;
    [x, y, z, vertex.weight]
}

#[derive(Clone, Copy)]
struct Range {
    minimum: f32,
    maximum: f32,
}

impl Range {
    fn point(value: f32) -> Self {
        Self {
            minimum: value,
            maximum: value,
        }
    }

    fn include(&mut self, value: f32) {
        self.minimum = self.minimum.min(value);
        self.maximum = self.maximum.max(value);
    }

    fn finite(&self) -> bool {
        self.minimum.is_finite() && self.maximum.is_finite()
    }

    fn scale(self, value: f32) -> Self {
        let [minimum, maximum] = if value < 0. {
            [self.maximum * value, self.minimum * value]
        } else {
            [self.minimum * value, self.maximum * value]
        };
        Self { minimum, maximum }
    }

    fn plus(self, other: Self) -> Self {
        Self {
            minimum: self.minimum + other.minimum,
            maximum: self.maximum + other.maximum,
        }
    }

    fn window_span(self, weight: Self, extent: f32) -> f32 {
        let denominator_low = if self.minimum < 0. {
            weight.minimum
        } else {
            weight.maximum
        };
        let denominator_high = if self.maximum < 0. {
            weight.maximum
        } else {
            weight.minimum
        };
        let minimum = self.minimum / denominator_low * extent / 2.;
        let maximum = self.maximum / denominator_high * extent / 2.;
        maximum - minimum
    }
}
