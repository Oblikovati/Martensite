// SPDX-License-Identifier: GPL-2.0-only
//! Retain literal unit-scale rows only when point shortcuts agree with them.
use super::{Transform3, TransformForm};
use std::{error::Error, fmt};

/// A transform cannot supply one consistent native unit-scale affine map.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum UnitPlacementError {
    /// Stored scale must be exactly one.
    Scale(f64),
    /// The form's native point shortcut contradicts the stored affine rows.
    Shortcut(TransformForm),
    /// A retained matrix or translation entry is nonfinite.
    NonFinite {
        /// Zero-based matrix row.
        row: usize,
        /// Zero-based column, with column 3 holding translation.
        column: usize,
        /// Offending stored entry.
        value: f64,
    },
}

impl fmt::Display for UnitPlacementError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scale(value) => write!(output, "native placement scale {value} must equal one"),
            Self::Shortcut(form) => {
                write!(
                    output,
                    "native {form:?} shortcut must agree with the stored placement"
                )
            }
            Self::NonFinite { row, column, value } => write!(
                output,
                "native placement entry [{row}, {column}] is {value}, expected finite"
            ),
        }
    }
}

impl Error for UnitPlacementError {}

impl Transform3 {
    /// Read the original unit-scale affine rows after validating their native shortcuts.
    /// Example: `let rows = transform.checked_unit_placement_rows()?;`.
    pub fn checked_unit_placement_rows(self) -> Result<[[f64; 4]; 3], UnitPlacementError> {
        let scale = self.scale_factor();
        if scale != 1. {
            return Err(UnitPlacementError::Scale(scale));
        }
        let matrix = self.matrix().coefficients();
        let translation = *self.translation_part().coordinates();
        let form = self.form();
        if !shortcut_agrees(form, matrix, translation) {
            return Err(UnitPlacementError::Shortcut(form));
        }
        let rows = std::array::from_fn(|i| {
            let [x, y, z] = matrix[i];
            [x, y, z, translation[i]]
        });
        finite_rows(rows)
    }
}

fn shortcut_agrees(form: TransformForm, matrix: [[f64; 3]; 3], translation: [f64; 3]) -> bool {
    let identity = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    match form {
        TransformForm::Identity => matrix == identity && translation == [0.; 3],
        TransformForm::Translation | TransformForm::Scale => matrix == identity,
        TransformForm::PointMirror => false,
        _ => true,
    }
}

fn finite_rows(rows: [[f64; 4]; 3]) -> Result<[[f64; 4]; 3], UnitPlacementError> {
    for (row, entries) in rows.iter().enumerate() {
        for (column, &value) in entries.iter().enumerate() {
            if !value.is_finite() {
                return Err(UnitPlacementError::NonFinite { row, column, value });
            }
        }
    }
    Ok(rows)
}
