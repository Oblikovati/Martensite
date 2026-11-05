//! Geometry counterparts of the original four prepared Boolean regression cases.
mod prepared_13116;

#[test]
fn bug13116_1_fuse() {
    prepared_13116::checked_result(1);
}
#[test]
fn bug13116_2_common() {
    prepared_13116::checked_result(2);
}
#[test]
fn bug13116_3_cut() {
    prepared_13116::checked_result(3);
}
#[test]
fn bug13116_4_reverse_cut() {
    prepared_13116::checked_result(4);
}
