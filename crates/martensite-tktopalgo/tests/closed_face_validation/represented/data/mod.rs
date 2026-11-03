//! Five owned inputs with explicit alias graphs.
use super::types::GraphInput;
mod case_0;
mod case_1;
mod case_2;
mod case_3;
mod case_4;
pub(super) fn cases() -> [&'static GraphInput; 5] {
    [
        &case_0::INPUT,
        &case_1::INPUT,
        &case_2::INPUT,
        &case_3::INPUT,
        &case_4::INPUT,
    ]
}
