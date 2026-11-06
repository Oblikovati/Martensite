//! Pure completion marker keeps independent drawing hosts free of viewer state.
use std::error::Error;
/// Completion of a regression that opens no view; `finish_without_view()`.
pub(crate) fn finish_without_view() -> Result<(), Box<dyn Error>> {
    println!("TEST COMPLETED");
    Ok(())
}
