//! The `bugs_modalg_7` cases.
use crate::support::{
    groups, hooks,
    process::{RustImprovement, dataset_cases, rust_improvement_case, unstable_dataset_cases},
};

dataset_cases!(groups::BUGS, bugs_modalg_7, hooks::no_grid_end;
    bug21264, bug22886, bug23386, bug24365, bug24424, bug24632_3, bug24692, bug24905, bug25023, bug25104, bug25462, bug25542, bug25966, bug25968, bug25994, bug26077_1, bug26563, bug26795, bug26933, bug27224, bug27928, bug27936, bug27948, bug28157, bug28245, bug28485_1, bug28786_2, bug28786_3, bug28786_4, bug28786_5, bug29038, bug29322_2, bug29322_3, bug29322_4, bug29322_5, bug29333_1, bug29333_2, bug29400, bug29484, bug29511, bug29580_2, bug29586, bug29807_svm01, bug29807_svm02, bug29843_2, bug29910_1, bug29968, bug29973_1, bug29973_2, bug30092, bug30154_1, bug30202_1, bug30206, bug30354, bug30386_4, bug30386_5, bug30396_1, bug30760, bug30788, bug31404, bug31460, bug31470, bug31835_1, bug31835_2, bug31836, bug31850_1, bug31850_2, bug31858_1, bug31858_2, bug31974, bug32470, bug32502, bug32578, bug83);

unstable_dataset_cases!(groups::BUGS, bugs_modalg_7, hooks::no_grid_end, "the 0.1 s limits, which the release build misses by a recorded timing exception: the profile is flat across the geometry accessors"; bug83_elapsed_limits);

rust_improvement_case!(
    groups::BUGS,
    bugs_modalg_7,
    hooks::no_grid_end,
    bug27378,
    RustImprovement {
        todo: "Error: BRepLib_MakeFace produces invalid faces on periodic surfaces",
        cause: "Martensite 5db73af3 preserves the seam on shifted periodic faces, the reference bug OCC27378 names",
        decision: "the maintainer accepted this improvement on 2026-11-05: the seam fix is understood and harmless"
    }
);
