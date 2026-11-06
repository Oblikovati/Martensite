//! Stable discriminants and text names of intersection result categories.
use std::io::{self, Write};

/// The status of a 2D intersection point; `TopOpeBRepP2DStatus::Intersection as i32 == 1`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TopOpeBRepP2DStatus {
    /// No assigned status.
    #[default]
    Unknown = 0,
    /// An isolated intersection point.
    Intersection = 1,
    /// The first endpoint of an intersection segment.
    SegmentFirst = 2,
    /// The last endpoint of an intersection segment.
    SegmentLast = 3,
    /// A point introduced during intersection processing.
    New = 4,
}

/// The geometric kind of an intersection line; `TopOpeBRepTypeLineCurve::Circle as i32 == 4`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TopOpeBRepTypeLineCurve {
    /// A general analytic line.
    #[default]
    Analytic = 0,
    /// A surface restriction.
    Restriction = 1,
    /// A sampled walking line.
    Walking = 2,
    /// A straight line.
    Line = 3,
    /// A circle.
    Circle = 4,
    /// An ellipse.
    Ellipse = 5,
    /// A parabola.
    Parabola = 6,
    /// A hyperbola.
    Hyperbola = 7,
    /// Another curve kind.
    OtherType = 8,
}

impl From<TopOpeBRepTypeLineCurve> for i32 {
    fn from(kind: TopOpeBRepTypeLineCurve) -> Self {
        kind as Self
    }
}

/// Text output for intersection categories; `TopOpeBRep::print(kind, &mut output)?`.
pub struct TopOpeBRep;

impl TopOpeBRep {
    /// Writes the exact category name without a newline; `Self::print(TopOpeBRepTypeLineCurve::Line, &mut output)?`.
    pub fn print(kind: impl Into<i32>, output: &mut impl Write) -> io::Result<()> {
        const NAMES: [&str; 9] = [
            "ANALYTIC",
            "RESTRICTION",
            "WALKING",
            "LINE",
            "CIRCLE",
            "ELLIPSE",
            "PARABOLA",
            "HYPERBOLA",
            "OTHERTYPE",
        ];
        let index = usize::try_from(kind.into()).unwrap_or(usize::MAX);
        let name = NAMES.get(index).copied().unwrap_or("UNKNOWN");
        output.write_all(name.as_bytes())
    }
}
