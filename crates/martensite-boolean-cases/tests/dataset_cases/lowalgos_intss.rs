//! The `lowalgos_intss` cases.
use crate::support::{groups, hooks, pinned_problem::pinned_problem_case, process::dataset_cases};

dataset_cases!(groups::LOWALGOS, lowalgos_intss, hooks::no_grid_end;
    bug21494_1, bug21494_2, bug23176, bug23948_1, bug23948_2, bug24429, bug24676, bug25818, bug25820_1, bug25820_2, bug25820_3, bug25890, bug25952_1, bug26008, bug26509_1, bug26509_2, bug26509_3, bug26509_4, bug26509_5, bug26509_6, bug26576_4, bug26684_1, bug26684_2, bug27227, bug27252_1, bug27431, bug27780, bug28544_2, bug29673, bug29807_i1001, bug29807_i1002, bug29807_i1003, bug29807_i1004, bug29807_i1005, bug29807_i1006, bug29807_i2001, bug29807_i2002, bug29807_i2003, bug29807_i2004, bug29807_i2005, bug29807_i2006, bug29807_i3001, bug29807_i3002, bug29807_i3003, bug29807_i3004, bug29807_i3005, bug29807_i4001, bug29807_i4002, bug29807_i4003, bug29807_i4004, bug29807_i4005, bug29807_i5001, bug29807_i5002, bug29972_1, bug29972_2, bug29972_3, bug29994, bug30082_1, bug30082_2, bug30202_2, bug30354, bug32701, bug32849, bug567_2);

// Correctly rounded math gives this script 8 curves and 8 edges, which it declares as known problems
// on Linux. The call-by-call attribution is under `docs/features/martensite-tkbool/corpus-attribution`.
pinned_problem_case!(
    groups::LOWALGOS,
    lowalgos_intss,
    hooks::no_grid_end,
    bug567_1,
    "Error :  is WRONG because number of EDGE entities in shape \"rs\" is 8"
);
