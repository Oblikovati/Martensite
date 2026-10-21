//! `bugs_modalg_4` case bodies whose
//! inputs are in the public dataset.
use super::bugs_modalg_1::planar_face;
use crate::support::session::Session;

/// `bug906`, `bug889` and `bug1013`: mkplane on the restored wire, its area and checkshape.
pub fn bug906(s: &mut dyn Session) {
    planar_face(s, ("DXF906.rle", "a"), false, 92131.6, "bug906");
}
