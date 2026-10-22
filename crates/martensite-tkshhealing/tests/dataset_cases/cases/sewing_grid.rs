//! The body every tests/sewing case runs at its grid's tolerance (3d097a0328e71b826377d4814ab05ec3c3d23871):
//! ```text
//! restore [locate_data_file file] a
//! sewing result $tol a
//! checkmaxtol result -ref max_tolerance
//! checknbshapes result -shell shells
//! checkfreebounds result free_edges
//! checkfaults result a faults
//! ```
//! then tests/sewing/end: `checkview -display result -2d`, or `Error : The sewing cannot
//! be built.` when `result` is not drawn.
use crate::support::{
    logged_checks::checknbshapes_some,
    restore::restore_as,
    session::{Session, checkview_display, set},
    sewing::{checkfaults, checkfreebounds, checkmaxtol, sewing},
};

/// The reference values of one case.
pub struct Expected {
    pub max_tolerance: f64,
    pub shells: usize,
    pub free_edges: i64,
    pub faults: i64,
}

/// The case body at the grid tolerance `tol`.
pub fn sewing_grid_case(
    s: &mut dyn Session,
    tol: f64,
    file: &str,
    expected: Expected,
    image: &str,
) {
    let a = restore_as(s, file, "a").shape();
    let result = sewing(tol, &[&a])
        .map(|shape| set(s, "result", shape))
        .expect("checkmaxtol: result is not a shape (sewing built nothing)");
    checkmaxtol(&result, "result", expected.max_tolerance);
    let mut counts = [None; 9];
    counts[4] = Some(expected.shells);
    checknbshapes_some(&result, counts);
    checkfreebounds(&result, "result", expected.free_edges);
    checkfaults(&result, "result", &a, "a", expected.faults);
    checkview_display(s, Some("result"), image);
}

/// The case bodies of one sewing grid from its table: `grid_tol; case "image" [todo
/// "pattern"] => (file, max_tolerance, shells, free_edges, faults),` per case.
macro_rules! sewing_cases {
    ($tol:expr; $($case:ident $image:literal $(todo $todo:literal)*
        => ($file:literal, $max:expr, $shells:expr, $free:expr, $faults:expr),)*) => {$(
        #[doc = concat!("The `", $image, "` case of the grid: ", $file, ".")]
        pub fn $case(s: &mut dyn crate::support::session::Session) {
            $(crate::support::log::todo($todo);)*
            let expected = super::sewing_grid::Expected {
                max_tolerance: $max,
                shells: $shells,
                free_edges: $free,
                faults: $faults,
            };
            super::sewing_grid::sewing_grid_case(s, $tol, $file, expected, $image);
        }
    )*};
}
pub(crate) use sewing_cases;
