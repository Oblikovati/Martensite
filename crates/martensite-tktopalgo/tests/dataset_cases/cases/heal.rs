//! `heal_checkshape`, same_parameter, same_parameter_locked and update_tolerance_locked
//! case bodies whose inputs are in the public
//! dataset. Each grid's `end` script runs after the case: same_parameter runs
//! `sameparameter result 0.001`, the locked grids `setflags s locked` and
//! `sameparameter result s 0.001` / `updatetolerance result s 1`, all then
//! `puts [checkshape result]`; `heal_end` shows `result` with `checkview -display
//! result -2d` (heal/checkshape/begin sets `no_result`, which skips it).
use crate::support::{
    checkshape::{CheckOptions, checkshape, checkshape_report},
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    topology::{
        sameparameter_in_place, sameparameter_reshaped, setflags_locked, updatetolerance_reshaped,
    },
};

/// `heal_checkshape`.
pub mod checkshape {
    use super::*;

    /// heal/checkshape/begin `CheckPerform path`: `checkshape c` and `checkshape c
    /// -parallel` (both logged) must give the same text; the timings are only printed.
    fn check_perform(s: &mut dyn Session, file: &str) {
        let c = restore_as(s, file, "c").shape();
        let cres = checkshape_report(&c, CheckOptions::default());
        let parallel = CheckOptions {
            parallel: true,
            ..CheckOptions::default()
        };
        let pres = checkshape_report(&c, parallel);
        if cres.text != pres.text {
            log::puts(&format!(
                "Error: different result between single-thread and parallel on {file}"
            ));
        }
    }

    /// 0027814: Parallelize BRepCheck_Analyzer.
    pub fn bug27814_1(s: &mut dyn Session) {
        check_perform(s, "bug27814.brep");
    }

    /// 0032448: `checkshape b -exact -parallel` must print
    /// `Faulty shapes in variables faulty_1 to faulty_88`.
    pub fn bug32448_1(s: &mut dyn Session) {
        log::required("Faulty shapes in variables faulty_1 to faulty_88");
        let b = restore_as(s, "bug27814.brep", "b").shape();
        let options = CheckOptions {
            parallel: true,
            exact: true,
            ..CheckOptions::default()
        };
        checkshape_report(&b, options);
    }
}

/// `heal_same_parameter`.
pub mod same_parameter {
    use super::*;

    /// `restore file result`, then the grid end.
    fn same_parameter(s: &mut dyn Session, file: &str, image: &str) {
        let result = restore_as(s, file, "result").shape();
        sameparameter_in_place(&result, 0.001, false);
        set(s, "result", result.clone());
        checkshape(&result);
        checkview_display(s, Some("result"), image);
    }

    pub fn a1(s: &mut dyn Session) {
        same_parameter(s, "CFE903_ger60ggw.rle", "A1");
    }

    pub fn a2(s: &mut dyn Session) {
        same_parameter(s, "CNP001_projoien.rle", "A2");
    }

    pub fn a5(s: &mut dyn Session) {
        same_parameter(s, "BUC50070-#53168-good.rle", "A5");
    }

    pub fn a6(s: &mut dyn Session) {
        same_parameter(s, "self-int.rle", "A6");
    }

    /// `TODO ALL: Faulty shapes in variables faulty_1 to faulty_1`.
    pub fn a9(s: &mut dyn Session) {
        log::todo("Faulty shapes in variables faulty_1 to faulty_1");
        same_parameter(s, "revface.rle", "A9");
    }
}

/// `heal_same_parameter_locked`.
pub mod same_parameter_locked {
    use super::*;

    /// `restore file s`, then the grid end.
    fn same_parameter_locked(s: &mut dyn Session, file: &str, image: &str) {
        let shape = restore_as(s, file, "s").shape();
        setflags_locked(&shape);
        let result = set(s, "result", sameparameter_reshaped(&shape, 0.001, false));
        checkshape(&result);
        checkview_display(s, Some("result"), image);
    }

    pub fn a1(s: &mut dyn Session) {
        same_parameter_locked(s, "CFE903_ger60ggw.rle", "A1");
    }

    pub fn a2(s: &mut dyn Session) {
        same_parameter_locked(s, "CNP001_projoien.rle", "A2");
    }

    pub fn a5(s: &mut dyn Session) {
        same_parameter_locked(s, "BUC50070-#53168-good.rle", "A5");
    }

    pub fn a6(s: &mut dyn Session) {
        same_parameter_locked(s, "self-int.rle", "A6");
    }
}

/// `heal_update_tolerance_locked`.
pub mod update_tolerance_locked {
    use super::*;

    /// `restore file s`, then the grid end.
    fn update_tolerance_locked(s: &mut dyn Session, file: &str, image: &str) {
        let shape = restore_as(s, file, "s").shape();
        setflags_locked(&shape);
        let result = set(s, "result", updatetolerance_reshaped(&shape));
        checkshape(&result);
        checkview_display(s, Some("result"), image);
    }

    pub fn a1(s: &mut dyn Session) {
        update_tolerance_locked(s, "CFE903_ger60ggw.rle", "A1");
    }

    pub fn a2(s: &mut dyn Session) {
        update_tolerance_locked(s, "CNP001_projoien.rle", "A2");
    }

    pub fn a3(s: &mut dyn Session) {
        update_tolerance_locked(s, "BUC50070-#53168-good.rle", "A3");
    }

    pub fn a4(s: &mut dyn Session) {
        update_tolerance_locked(s, "self-int.rle", "A4");
    }
}
