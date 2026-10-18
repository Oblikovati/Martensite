//! Fixed continuity-matrix coefficients.
//!
//! Frozen literal values are grouped in fragments of at most 1920 entries.
#![allow(clippy::excessive_precision)]

/// One flat retained `static const double` array, stored as fixed-size fragments.
///
/// ```
/// use martensite_tkgeombase::app_cont::cont_matrices::THE_BBMATRIX_DATA;
/// assert_eq!(THE_BBMATRIX_DATA.get(0), Some(0.3333333333333333333333333));
/// assert_eq!(THE_BBMATRIX_DATA.get(THE_BBMATRIX_DATA.len()), None);
/// ```
pub struct ContMatrixTable {
    chunks: &'static [&'static [f64]],
}
impl ContMatrixTable {
    /// The flattened entry `index`, or `None` past the retained array end;
    /// `THE_BBMATRIX_DATA.get(0)`.
    pub fn get(&self, index: usize) -> Option<f64> {
        self.chunks.get(index / CHUNK)?.get(index % CHUNK).copied()
    }
    /// The number of entries of the retained array; `THE_BBMATRIX_DATA.len()`.
    pub fn len(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.len()).sum()
    }
    /// Whether the array has no entry (never, for the retained tables);
    /// `THE_BBMATRIX_DATA.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
const CHUNK: usize = 1920;

/// `ContMatrices::THE_BBMATRIX_DATA` (6200 values).
pub const THE_BBMATRIX_DATA: ContMatrixTable = ContMatrixTable {
    chunks: &[
        include!("cont_tables/bbmatrix_0.inc"),
        include!("cont_tables/bbmatrix_1.inc"),
        include!("cont_tables/bbmatrix_2.inc"),
        include!("cont_tables/bbmatrix_3.inc"),
    ],
};

/// `ContMatrices::THE_INV_MMATRIX_DATA` (6200 values).
pub const THE_INV_MMATRIX_DATA: ContMatrixTable = ContMatrixTable {
    chunks: &[
        include!("cont_tables/inv_mmatrix_0.inc"),
        include!("cont_tables/inv_mmatrix_1.inc"),
        include!("cont_tables/inv_mmatrix_2.inc"),
        include!("cont_tables/inv_mmatrix_3.inc"),
    ],
};

/// `ContMatrices::THE_IBPMATRIX_DATA` (4900 values).
pub const THE_IBPMATRIX_DATA: ContMatrixTable = ContMatrixTable {
    chunks: &[
        include!("cont_tables/ibpmatrix_0.inc"),
        include!("cont_tables/ibpmatrix_1.inc"),
        include!("cont_tables/ibpmatrix_2.inc"),
    ],
};

/// `ContMatrices::THE_IBTMATRIX_DATA` (3795 values).
pub const THE_IBTMATRIX_DATA: ContMatrixTable = ContMatrixTable {
    chunks: &[
        include!("cont_tables/ibtmatrix_0.inc"),
        include!("cont_tables/ibtmatrix_1.inc"),
    ],
};

/// `ContMatrices::THE_VBMATRIX_DATA` (105000 values).
pub const THE_VBMATRIX_DATA: ContMatrixTable = ContMatrixTable {
    chunks: &[
        include!("cont_tables/vbmatrix_0.inc"),
        include!("cont_tables/vbmatrix_1.inc"),
        include!("cont_tables/vbmatrix_2.inc"),
        include!("cont_tables/vbmatrix_3.inc"),
        include!("cont_tables/vbmatrix_4.inc"),
        include!("cont_tables/vbmatrix_5.inc"),
        include!("cont_tables/vbmatrix_6.inc"),
        include!("cont_tables/vbmatrix_7.inc"),
        include!("cont_tables/vbmatrix_8.inc"),
        include!("cont_tables/vbmatrix_9.inc"),
        include!("cont_tables/vbmatrix_10.inc"),
        include!("cont_tables/vbmatrix_11.inc"),
        include!("cont_tables/vbmatrix_12.inc"),
        include!("cont_tables/vbmatrix_13.inc"),
        include!("cont_tables/vbmatrix_14.inc"),
        include!("cont_tables/vbmatrix_15.inc"),
        include!("cont_tables/vbmatrix_16.inc"),
        include!("cont_tables/vbmatrix_17.inc"),
        include!("cont_tables/vbmatrix_18.inc"),
        include!("cont_tables/vbmatrix_19.inc"),
        include!("cont_tables/vbmatrix_20.inc"),
        include!("cont_tables/vbmatrix_21.inc"),
        include!("cont_tables/vbmatrix_22.inc"),
        include!("cont_tables/vbmatrix_23.inc"),
        include!("cont_tables/vbmatrix_24.inc"),
        include!("cont_tables/vbmatrix_25.inc"),
        include!("cont_tables/vbmatrix_26.inc"),
        include!("cont_tables/vbmatrix_27.inc"),
        include!("cont_tables/vbmatrix_28.inc"),
        include!("cont_tables/vbmatrix_29.inc"),
        include!("cont_tables/vbmatrix_30.inc"),
        include!("cont_tables/vbmatrix_31.inc"),
        include!("cont_tables/vbmatrix_32.inc"),
        include!("cont_tables/vbmatrix_33.inc"),
        include!("cont_tables/vbmatrix_34.inc"),
        include!("cont_tables/vbmatrix_35.inc"),
        include!("cont_tables/vbmatrix_36.inc"),
        include!("cont_tables/vbmatrix_37.inc"),
        include!("cont_tables/vbmatrix_38.inc"),
        include!("cont_tables/vbmatrix_39.inc"),
        include!("cont_tables/vbmatrix_40.inc"),
        include!("cont_tables/vbmatrix_41.inc"),
        include!("cont_tables/vbmatrix_42.inc"),
        include!("cont_tables/vbmatrix_43.inc"),
        include!("cont_tables/vbmatrix_44.inc"),
        include!("cont_tables/vbmatrix_45.inc"),
        include!("cont_tables/vbmatrix_46.inc"),
        include!("cont_tables/vbmatrix_47.inc"),
        include!("cont_tables/vbmatrix_48.inc"),
        include!("cont_tables/vbmatrix_49.inc"),
        include!("cont_tables/vbmatrix_50.inc"),
        include!("cont_tables/vbmatrix_51.inc"),
        include!("cont_tables/vbmatrix_52.inc"),
        include!("cont_tables/vbmatrix_53.inc"),
        include!("cont_tables/vbmatrix_54.inc"),
    ],
};
