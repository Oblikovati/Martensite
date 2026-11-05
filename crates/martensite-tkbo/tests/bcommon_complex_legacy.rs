//! Native Rust translations of the complete-input `bcommon_complex` command-host regressions.
//! Each case embeds its original BRep fixtures and checks the documented surface area.
#![allow(non_snake_case)]

#[test]
fn BCommonComplex__B2() {
    let result = bcommon_b2_b9_case::checked_b2();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B3() {
    let result = bcommon_b2_b9_case::checked_b3();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B4() {
    let result = bcommon_b2_b9_case::checked_b4();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B5() {
    let result = bcommon_b2_b9_case::checked_b5();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B6() {
    let result = bcommon_b2_b9_case::checked_b6();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B7() {
    let result = bcommon_b2_b9_case::checked_b7();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B8() {
    let result = bcommon_b2_b9_case::checked_b8();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCommonComplex__B9() {
    let result = bcommon_b2_b9_case::checked_b9();
    bcommon_b2_b9_case::boolean_unconditional_case::check_view_result(&result);
    bcommon_b2_b9_case::boolean_unconditional_case::check_end(&result);
}

#[path = "bcommon_b2_b9_case.rs"]
mod bcommon_b2_b9_case;
