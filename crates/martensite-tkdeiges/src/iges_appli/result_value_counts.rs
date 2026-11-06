//! The count of real values a result entity holds, by its form.
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCheck;

/// The count of real values of each form from 1. Form 0 takes any count that is not negative.
const COUNT_BY_FORM: [i32; 34] = [
    1, 1, 3, 6, 3, 3, 3, 3, 3, 1, 1, 3, 1, 1, 3, 1, 3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 9, 9, 9, 9,
    9, 9,
];

/// Adds a failure to the check when `form_num` is not a result form, or when `nv` values do not
/// suit it; `check_result_value_count(0, 1, &check)`.
pub(super) fn check_result_value_count(form_num: i32, nv: i32, ach: &Handle<InterfaceCheck>) {
    let suits = match usize::try_from(form_num) {
        Ok(0) => nv >= 0,
        Ok(form) if form <= COUNT_BY_FORM.len() => nv == COUNT_BY_FORM[form - 1],
        _ => {
            ach.add_fail_text("Incorrect Form Number", "");
            return;
        }
    };
    if !suits {
        ach.add_fail_text("Incorrect count of real values in array V for FEM node", "");
    }
}

#[cfg(test)]
mod tests {
    use super::{COUNT_BY_FORM, check_result_value_count};
    use martensite_tkernel::standard::Handle;
    use martensite_tkxsbase::interface::InterfaceCheck;

    fn fails_of(form: i32, count: i32) -> usize {
        let check = Handle::new(InterfaceCheck::new());
        check_result_value_count(form, count, &check);
        check.get().unwrap().fails(true).len()
    }

    #[test]
    fn each_form_takes_its_own_count() {
        assert_eq!(fails_of(0, 0), 0);
        assert_eq!(fails_of(0, 7), 0);
        assert_eq!(fails_of(0, -1), 1);
        for (rank, count) in COUNT_BY_FORM.iter().enumerate() {
            let form = rank as i32 + 1;
            assert_eq!(fails_of(form, *count), 0, "form {form}");
            assert_eq!(fails_of(form, *count + 1), 1, "form {form}");
        }
    }

    #[test]
    fn a_form_outside_the_table_is_a_failure() {
        assert_eq!(fails_of(-1, 1), 1);
        assert_eq!(fails_of(COUNT_BY_FORM.len() as i32 + 1, 1), 1);
    }
}
