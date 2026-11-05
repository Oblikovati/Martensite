//! The view commands the scripts call by name, on the memory raster session.
use crate::support::session::{Session, ViewStep};

/// `smallview`; `smallview(s)`.
pub fn smallview(s: &mut dyn Session) {
    s.view(ViewStep::SmallView);
}

/// `donly names`; `donly(s, &["result"])`.
pub fn donly(s: &mut dyn Session, names: &[&str]) {
    s.view(ViewStep::Donly(names));
}

/// `don names`: displays them as well; `don(s, &["b", "result"])`.
pub fn don(s: &mut dyn Session, names: &[&str]) {
    s.view(ViewStep::Disp(names));
}

/// `fit`; `fit(s)`.
pub fn fit(s: &mut dyn Session) {
    s.view(ViewStep::Fit);
}

/// `xwd image`; `xwd(s, "rfu")`.
pub fn xwd(s: &mut dyn Session, image: &str) {
    s.view(ViewStep::Xwd(image));
}
