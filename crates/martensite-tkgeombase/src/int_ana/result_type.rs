//! ResultType: the kind of a geometric quadric-quadric intersection.

/// ResultType: the kind of result of [`super::QuadQuadGeo`]. The
/// discriminants are the retained enumerator values.
/// `assert_eq!(ResultType::Empty as i32, 7)`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum ResultType {
    /// Point: one or two points.
    Point = 0,
    /// Line: one or two lines.
    Line = 1,
    /// Circle: one to four circles.
    Circle = 2,
    /// PointAndCircle: a point and a circle.
    PointAndCircle = 3,
    /// Ellipse: one or two ellipses.
    Ellipse = 4,
    /// Parabola: one parabola.
    Parabola = 5,
    /// Hyperbola: the two branches of a hyperbola.
    Hyperbola = 6,
    /// Empty: no intersection (the value of a new QuadQuadGeo).
    #[default]
    Empty = 7,
    /// Same: the quadrics are identical.
    Same = 8,
    /// NoGeometricSolution: an analytic algorithm must compute the result.
    NoGeometricSolution = 9,
}

impl ResultType {
    /// Every enumerator in retained order; `ResultType::ALL[9] == ResultType::NoGeometricSolution`.
    pub const ALL: [Self; 10] = [
        Self::Point,
        Self::Line,
        Self::Circle,
        Self::PointAndCircle,
        Self::Ellipse,
        Self::Parabola,
        Self::Hyperbola,
        Self::Empty,
        Self::Same,
        Self::NoGeometricSolution,
    ];

    /// The retained enumerator name; `ResultType::Same.name() == "Same"`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Point => "IntAna_Point",
            Self::Line => "IntAna_Line",
            Self::Circle => "IntAna_Circle",
            Self::PointAndCircle => "IntAna_PointAndCircle",
            Self::Ellipse => "IntAna_Ellipse",
            Self::Parabola => "IntAna_Parabola",
            Self::Hyperbola => "IntAna_Hyperbola",
            Self::Empty => "IntAna_Empty",
            Self::Same => "IntAna_Same",
            Self::NoGeometricSolution => "IntAna_NoGeometricSolution",
        }
    }
}
