//! `checkview -display shape -2d [-with names] [-otherwise names] [-s|-l|-v]`: chooses what to
//! show from the shape and its property, then views and captures it.
use super::{
    properties::{Prop, props},
    session::{Session, ViewStep},
};

/// The image name a script's `checkview` writes unless the case chooses another.
pub const DEFAULT_IMAGE: &str = "photo";

/// The options of one `checkview -2d`.
#[derive(Clone, Copy, Debug)]
pub struct CheckView<'a> {
    /// `-display`: the shape to show.
    pub display: &'a str,
    /// `-with`: shown together with it.
    pub with: &'a [&'a str],
    /// `-otherwise`: shown when the shape is absent or its property is 0.
    pub otherwise: &'a [&'a str],
    /// `-s`, `-l` or `-v`: the property that decides whether the shape is shown.
    pub property: Option<Prop>,
    /// `-path`: the image name.
    pub image: &'a str,
}

impl<'a> CheckView<'a> {
    /// `checkview -display name -2d`; `CheckView::of("result")`.
    pub const fn of(display: &'a str) -> Self {
        Self {
            display,
            with: &[],
            otherwise: &[],
            property: None,
            image: DEFAULT_IMAGE,
        }
    }

    /// Adds `-path image`; `CheckView::of("result").image_named("bug1")`.
    pub const fn image_named(self, image: &'a str) -> Self {
        Self { image, ..self }
    }

    /// Adds `-otherwise names`, decided by `-s`, `-l` or `-v`.
    /// Example: `CheckView::of("result").otherwise(&["b1", "b2"], Prop::area)`.
    pub const fn otherwise(self, names: &'a [&'a str], property: Prop) -> Self {
        Self {
            otherwise: names,
            property: Some(property),
            ..self
        }
    }

    /// Adds `-s`, `-l` or `-v` alone: it decides whether the shape is shown, with no fallback.
    /// Example: `CheckView::of("result").deciding(Prop::area)`.
    pub const fn deciding(self, property: Prop) -> Self {
        Self {
            property: Some(property),
            ..self
        }
    }

    /// Adds `-with names`; `CheckView::of("result").with(&["v1"])`.
    pub const fn with(self, names: &'a [&'a str]) -> Self {
        Self {
            with: names,
            ..self
        }
    }

    fn command_text(&self) -> String {
        let mut text = format!("checkview -display {} -2d", self.display);
        if !self.with.is_empty() {
            text += &format!(" -with {{ {} }}", self.with.join(" "));
        }
        if !self.otherwise.is_empty() {
            text += &format!(" -otherwise {{ {} }}", self.otherwise.join(" "));
        }
        if let Some(property) = self.property {
            text += &format!(" {}", property.flag());
        }
        text + " -path " + self.image
    }
}

/// Whether the displayed shape is shown: bound, and with a nonzero property when one is asked.
/// The property is measured even when it is going to be 0: its report is part of the stream.
fn displays_shape(session: &dyn Session, view: &CheckView<'_>) -> bool {
    let Some(shape) = session.shape(view.display) else {
        return false;
    };
    match view.property {
        Some(property) => props(property, shape, None) != 0.,
        None => true,
    }
}

/// Runs the check: the property, `smallview`, `clear`, the choice of `donly`, `fit`, `xwd`.
/// Example: `checkview(s, &CheckView::of("result").otherwise(&["b1", "b2"], Prop::area))`.
pub fn checkview(session: &mut dyn Session, view: &CheckView<'_>) {
    println!("{}", view.command_text());
    let shown = displays_shape(session, view);
    session.view(ViewStep::SmallView);
    session.view(ViewStep::Clear);
    if shown {
        let mut names = view.with.to_vec();
        names.push(view.display);
        session.view(ViewStep::Donly(&names));
    } else if !view.otherwise.is_empty() {
        session.view(ViewStep::Donly(view.otherwise));
    }
    session.view(ViewStep::Fit);
    session.view(ViewStep::Xwd(view.image));
}

/// `checkview -screenshot -2d`: captures the view as it is; `screenshot(s, "photo")`.
pub fn screenshot(session: &mut dyn Session, image: &str) {
    println!("checkview -screenshot -2d -path {image}");
    session.view(ViewStep::Xwd(image));
}
