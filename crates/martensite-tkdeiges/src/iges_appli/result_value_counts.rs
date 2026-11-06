//! The count of real values a result entity holds, by its form.
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCheck;

/// Adds a failure to the check when `form_num` is not a result form, or when `nv` values do not
/// suit it; `check_result_value_count(0, 1, &check)`.
pub(super) fn check_result_value_count(form_num: i32, nv: i32, ach: &Handle<InterfaceCheck>) {
    let mut ok: bool = true;
    match form_num {
        0 => {
            if nv < 0 {
                ok = false;
            }
        }
        1 => {
            if nv != 1 {
                ok = false;
            }
        }
        2 => {
            if nv != 1 {
                ok = false;
            }
        }
        3 => {
            if nv != 3 {
                ok = false;
            }
        }
        4 => {
            if nv != 6 {
                ok = false;
            }
        }
        5 => {
            if nv != 3 {
                ok = false;
            }
        }
        6 => {
            if nv != 3 {
                ok = false;
            }
        }
        7 => {
            if nv != 3 {
                ok = false;
            }
        }
        8 => {
            if nv != 3 {
                ok = false;
            }
        }
        9 => {
            if nv != 3 {
                ok = false;
            }
        }
        10 => {
            if nv != 1 {
                ok = false;
            }
        }
        11 => {
            if nv != 1 {
                ok = false;
            }
        }
        12 => {
            if nv != 3 {
                ok = false;
            }
        }
        13 => {
            if nv != 1 {
                ok = false;
            }
        }
        14 => {
            if nv != 1 {
                ok = false;
            }
        }
        15 => {
            if nv != 3 {
                ok = false;
            }
        }
        16 => {
            if nv != 1 {
                ok = false;
            }
        }
        17 => {
            if nv != 3 {
                ok = false;
            }
        }
        18 => {
            if nv != 3 {
                ok = false;
            }
        }
        19 => {
            if nv != 3 {
                ok = false;
            }
        }
        20 => {
            if nv != 3 {
                ok = false;
            }
        }
        21 => {
            if nv != 3 {
                ok = false;
            }
        }
        22 => {
            if nv != 3 {
                ok = false;
            }
        }
        23 => {
            if nv != 6 {
                ok = false;
            }
        }
        24 => {
            if nv != 6 {
                ok = false;
            }
        }
        25 => {
            if nv != 6 {
                ok = false;
            }
        }
        26 => {
            if nv != 6 {
                ok = false;
            }
        }
        27 => {
            if nv != 6 {
                ok = false;
            }
        }
        28 => {
            if nv != 6 {
                ok = false;
            }
        }
        29 => {
            if nv != 9 {
                ok = false;
            }
        }
        30 => {
            if nv != 9 {
                ok = false;
            }
        }
        31 => {
            if nv != 9 {
                ok = false;
            }
        }
        32 => {
            if nv != 9 {
                ok = false;
            }
        }
        33 => {
            if nv != 9 {
                ok = false;
            }
        }
        34 => {
            if nv != 9 {
                ok = false;
            }
        }
        _ => {
            ach.add_fail_text("Incorrect Form Number", "");
        }
    }
    if !ok {
        ach.add_fail_text("Incorrect count of real values in array V for FEM node", "");
    }
}
