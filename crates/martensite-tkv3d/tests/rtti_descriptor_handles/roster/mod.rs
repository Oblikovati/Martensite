//! One fixed metadata-query order shared with the native producer.
mod ais;
mod prs3d;
mod prs_dim;
mod prs_mgr;
mod select3d;
mod select_mgr;
mod std_prs;
mod std_select;
mod v3d;

use super::DescriptorQuery;

pub(super) const GROUPS: &[&[DescriptorQuery]] = &[
    ais::QUERIES,
    prs3d::QUERIES,
    prs_dim::QUERIES,
    prs_mgr::QUERIES,
    select3d::QUERIES,
    select_mgr::QUERIES,
    std_prs::QUERIES,
    std_select::QUERIES,
    v3d::QUERIES,
];
