//! Fixture-backed Boolean fuse regression cases.

#[path = "bfuse_first_face_case.rs"]
mod bfuse_first_face_case;
#[test]
fn BFuseComplex__O1() {
    let result = bfuse_first_face_case::checked_result("O1");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__O2() {
    let result = bfuse_first_face_case::checked_result("O2");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__O3() {
    let result = bfuse_first_face_case::checked_result("O3");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__O4() {
    let result = bfuse_first_face_case::checked_result("O4");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__P2() {
    let result = bfuse_first_face_case::checked_result("P2");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__P3() {
    let result = bfuse_first_face_case::checked_result("P3");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__S3() {
    let result = bfuse_first_face_case::checked_result("S3");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__S4() {
    let result = bfuse_first_face_case::checked_result("S4");
    bfuse_first_face_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_first_face_case::boolean_unconditional_case::check_end(&result);
}

#[path = "bfuse_chained_case.rs"]
mod bfuse_chained_case;

#[test]
fn BFuseComplex__H5() {
    let result = bfuse_chained_case::checked_h5();
    bfuse_chained_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_chained_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BFuseComplex__J2() {
    let result = bfuse_chained_case::checked_j2();
    bfuse_chained_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_chained_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BFuseComplex__J3() {
    let result = bfuse_chained_case::checked_j3();
    bfuse_chained_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_chained_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BFuseComplex__M5() {
    let result = bfuse_chained_case::checked_m5();
    bfuse_chained_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_chained_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BFuseComplex__N3() {
    let result = bfuse_chained_case::checked_n3();
    bfuse_chained_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_chained_case::boolean_unconditional_case::check_end(&result);
}
