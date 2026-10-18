//! Fixed polynomial and quadrature tables.
//!
//! Frozen literal values are grouped in fragments of 1920 entries.
// The literals are the retained ones, even where one equals a std constant.
#![allow(clippy::excessive_precision, clippy::approx_constant)]

/// Verbatim `mlgdrtl_` initialiser (1891 values).
pub(super) const MLGDRTL: &[&[f64]] = &[include!("tables/mlgdrtl_0.inc")];

/// Verbatim `mmjcobi_` initialiser (7936 values).
pub(super) const MMJCOBI: &[&[f64]] = &[
    include!("tables/mmjcobi_0.inc"),
    include!("tables/mmjcobi_1.inc"),
    include!("tables/mmjcobi_2.inc"),
    include!("tables/mmjcobi_3.inc"),
    include!("tables/mmjcobi_4.inc"),
];

/// Verbatim `mmcmcnp_` initialiser (3721 values).
pub(super) const MMCMCNP: &[&[f64]] = &[
    include!("tables/mmcmcnp_0.inc"),
    include!("tables/mmcmcnp_1.inc"),
];

/// Verbatim `mmapgss_` initialiser (5069 values).
pub(super) const MMAPGSS: &[&[f64]] = &[
    include!("tables/mmapgss_0.inc"),
    include!("tables/mmapgss_1.inc"),
    include!("tables/mmapgss_2.inc"),
];

/// Verbatim `mmapgs0_` initialiser (4810 values).
pub(super) const MMAPGS0: &[&[f64]] = &[
    include!("tables/mmapgs0_0.inc"),
    include!("tables/mmapgs0_1.inc"),
    include!("tables/mmapgs0_2.inc"),
];

/// Verbatim `mmapgs1_` initialiser (4551 values).
pub(super) const MMAPGS1: &[&[f64]] = &[
    include!("tables/mmapgs1_0.inc"),
    include!("tables/mmapgs1_1.inc"),
    include!("tables/mmapgs1_2.inc"),
];

/// Verbatim `mmapgs2_` initialiser (4292 values).
pub(super) const MMAPGS2: &[&[f64]] = &[
    include!("tables/mmapgs2_0.inc"),
    include!("tables/mmapgs2_1.inc"),
    include!("tables/mmapgs2_2.inc"),
];
