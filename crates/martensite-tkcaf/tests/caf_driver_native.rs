// Complete driver originals use the checked graph, drawing on a memory raster, and inherited policy.
// Included at the root of the toolkit host: the fixture's own tests must run once, and a case's
// child process finds its test by this top-level name.
use caf_driver::{CafDriverCase, run};

macro_rules! case_driver {
    ($name:ident, $case:ident) => {
        #[test]
        fn $name() {
            let case = CafDriverCase::$case;
            run(stringify!($name), case);
        }
    };
}

case_driver!(caf_driver_a1, A1);
case_driver!(caf_driver_a3, A3);
case_driver!(caf_driver_a4, A4);
case_driver!(caf_driver_a5, A5);
case_driver!(caf_driver_a6, A6);
case_driver!(caf_driver_a7, A7);
case_driver!(caf_driver_a8, A8);
case_driver!(caf_driver_b1, B1);
case_driver!(caf_driver_b4, B4);
case_driver!(caf_driver_b5, B5);
case_driver!(caf_driver_b6, B6);
case_driver!(caf_driver_b7, B7);

#[test]
fn bugs_caf_bug21707() {
    let identity = "bugs_caf_bug21707";
    caf_bug::run(identity);
}
