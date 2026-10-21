//! `bugs_fclasses::bug25558` body: "It is
//! impossible to restore attached shape".
use crate::support::{
    restore::restore_as,
    session::{Session, ViewStep},
};

/// `smallview`, the restore (outside any catch) must read the shape, `fit` and
/// the screenshot.
pub fn bug25558(s: &mut dyn Session) {
    s.step(ViewStep::SmallView("AXON"));
    let a1 = restore_as(s, "OCC25558_faulty.brep", "a1").shape();
    assert!(
        !a1.is_null(),
        "restore of OCC25558_faulty.brep gave a null shape"
    );
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug25558"));
}
