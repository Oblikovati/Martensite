//! `Status` and `ParametrizationType`: the flags of the multi-line
//! approximation drivers.

/// `Status`: what the line can do for a range of points that does not fit
/// (`LineTool::WhatStatus`); `Status::NoPointsAdded.code() == 1`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Status {
    /// `PointsAdded`: the line can give more points (`MakeMLBetween`).
    PointsAdded = 0,
    /// `NoPointsAdded`: no point can be added.
    #[default]
    NoPointsAdded = 1,
    /// `NoApproximation`: the range must not be approximated.
    NoApproximation = 2,
}

impl Status {
    /// The retained enumeration value; `Status::NoApproximation.code() == 2`.
    pub fn code(self) -> i32 {
        self as i32
    }

    /// The status of a retained value (0 to 2), None otherwise;
    /// `Status::from_code(0) == Some(Status::PointsAdded)`.
    pub fn from_code(code: i32) -> Option<Self> {
        [
            Self::PointsAdded,
            Self::NoPointsAdded,
            Self::NoApproximation,
        ]
        .get(usize::try_from(code).ok()?)
        .copied()
    }
}

/// `ParametrizationType`: the parameter of point `i` is the cumulated chord
/// length, the cumulated square root of the chords (centripetal) or `i` itself
/// (iso-parametric), scaled to `[0, 1]`; `ParametrizationType::Centripetal.code() == 1`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ParametrizationType {
    /// `ChordLength`.
    #[default]
    ChordLength = 0,
    /// `Centripetal`.
    Centripetal = 1,
    /// `IsoParametric`.
    IsoParametric = 2,
}

impl ParametrizationType {
    /// The retained enumeration value; `ParametrizationType::IsoParametric.code() == 2`.
    pub fn code(self) -> i32 {
        self as i32
    }

    /// The type of a retained value (0 to 2), None otherwise;
    /// `ParametrizationType::from_code(0) == Some(ParametrizationType::ChordLength)`.
    pub fn from_code(code: i32) -> Option<Self> {
        [Self::ChordLength, Self::Centripetal, Self::IsoParametric]
            .get(usize::try_from(code).ok()?)
            .copied()
    }
}
