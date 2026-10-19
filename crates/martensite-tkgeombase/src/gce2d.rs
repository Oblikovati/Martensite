//! Deprecated planar construction aliases.
//!
//! Prefer the corresponding `crate::gc` maker, whose name explicitly identifies two dimensions.
//! The aliases remain available for existing callers.

/// Shared construction status; use [`crate::gc::Root`] in new code.
pub use crate::gc::Root;

/// Declares the deprecated aliases, each with the retained deprecation message.
macro_rules! deprecated_aliases {
    ($($alias:ident = $target:ident, $native:literal, $message:literal;)*) => {
        $(
            #[doc = concat!("`", $native, "`: ", $message)]
            #[deprecated(note = $message)]
            pub type $alias = crate::gc::$target;
        )*
    };
}

deprecated_aliases! {
    MakeArcOfCircle = MakeArcOfCircle2d,
        "MakeArcOfCircle",
        "Use crate::gc::MakeArcOfCircle2d instead.";
    MakeArcOfEllipse = MakeArcOfEllipse2d,
        "MakeArcOfEllipse",
        "Use crate::gc::MakeArcOfEllipse2d instead.";
    MakeArcOfHyperbola = MakeArcOfHyperbola2d,
        "MakeArcOfHyperbola",
        "Use crate::gc::MakeArcOfHyperbola2d instead.";
    MakeArcOfParabola = MakeArcOfParabola2d,
        "MakeArcOfParabola",
        "Use crate::gc::MakeArcOfParabola2d instead.";
    MakeCircle = MakeCircle2d,
        "MakeCircle",
        "Use crate::gc::MakeCircle2d instead.";
    MakeEllipse = MakeEllipse2d,
        "MakeEllipse",
        "Use crate::gc::MakeEllipse2d instead.";
    MakeHyperbola = MakeHyperbola2d,
        "MakeHyperbola",
        "Use crate::gc::MakeHyperbola2d instead.";
    MakeLine = MakeLine2d,
        "MakeLine",
        "Use crate::gc::MakeLine2d instead.";
    MakeMirror = MakeMirror2d,
        "MakeMirror",
        "Use crate::gc::MakeMirror2d instead.";
    MakeParabola = MakeParabola2d,
        "MakeParabola",
        "Use crate::gc::MakeParabola2d instead.";
    MakeRotation = MakeRotation2d,
        "MakeRotation",
        "Use crate::gc::MakeRotation2d instead.";
    MakeScale = MakeScale2d,
        "MakeScale",
        "Use crate::gc::MakeScale2d instead.";
    MakeSegment = MakeSegment2d,
        "MakeSegment",
        "Use crate::gc::MakeSegment2d instead.";
    MakeTranslation = MakeTranslation2d,
        "MakeTranslation",
        "Use crate::gc::MakeTranslation2d instead.";
}
