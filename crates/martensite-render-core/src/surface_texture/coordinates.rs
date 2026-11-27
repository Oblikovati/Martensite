//! Validation follows an immutable UV stream across draw snapshots.
use crate::draw::DrawStream;
use std::{fmt, num::NonZeroUsize, ops::Deref, sync::Weak};

#[derive(Clone, PartialEq)]
pub struct SurfaceUvStream {
    values: DrawStream<[f32; 4]>,
    invalid: Option<NonZeroUsize>,
}

impl From<Vec<[f32; 4]>> for SurfaceUvStream {
    fn from(values: Vec<[f32; 4]>) -> Self {
        let invalid = values
            .iter()
            .position(|uv| uv.iter().any(|value| !value.is_finite()))
            .and_then(|index| NonZeroUsize::new(index + 1));
        Self {
            values: values.into(),
            invalid,
        }
    }
}

impl Deref for SurfaceUvStream {
    type Target = [[f32; 4]];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl fmt::Debug for SurfaceUvStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.values.fmt(formatter)
    }
}

impl SurfaceUvStream {
    pub fn invalid(&self) -> Option<&[f32; 4]> {
        self.invalid.map(|index| &self.values[index.get() - 1])
    }

    pub fn identity(&self) -> Option<NonZeroUsize> {
        self.values.identity()
    }

    pub fn witness(&self) -> Option<Weak<Vec<[f32; 4]>>> {
        self.values.witness()
    }
}
