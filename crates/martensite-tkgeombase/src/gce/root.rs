//! The construction status shared by the gce and GC algorithms.
use martensite_tkernel::standard::Failure;

/// ErrorType: why a construction algorithm failed, or `Done`.
///
/// The discriminants are the retained enumerator values.
/// `assert_eq!(ErrorType::NullAxis as i32, 5)`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum ErrorType {
    /// Done: the construction succeeded. It is also the value that an
    /// algorithm whose retained constructor never assigns the status reads (see
    /// [`Root`]).
    #[default]
    Done = 0,
    /// ConfusedPoints: two points coincide.
    ConfusedPoints = 1,
    /// NegativeRadius: a radius is negative.
    NegativeRadius = 2,
    /// ColinearPoints: three points are collinear.
    ColinearPoints = 3,
    /// IntersectionError: an intersection has no result.
    IntersectionError = 4,
    /// NullAxis: an axis has a null length.
    NullAxis = 5,
    /// NullAngle: an angle is null.
    NullAngle = 6,
    /// NullRadius: a radius is null.
    NullRadius = 7,
    /// InvertAxis: the axes are inverted.
    InvertAxis = 8,
    /// BadAngle: an angle is out of range.
    BadAngle = 9,
    /// InvertRadius: the major radius is smaller than the minor one.
    InvertRadius = 10,
    /// NullFocusLength: the focal length is negative.
    NullFocusLength = 11,
    /// NullVector: a vector has a null length.
    NullVector = 12,
    /// BadEquation: the plane equation has no normal.
    BadEquation = 13,
}

impl ErrorType {
    /// Every enumerator in retained order; `ErrorType::ALL[13] == ErrorType::BadEquation`.
    pub const ALL: [Self; 14] = [
        Self::Done,
        Self::ConfusedPoints,
        Self::NegativeRadius,
        Self::ColinearPoints,
        Self::IntersectionError,
        Self::NullAxis,
        Self::NullAngle,
        Self::NullRadius,
        Self::InvertAxis,
        Self::BadAngle,
        Self::InvertRadius,
        Self::NullFocusLength,
        Self::NullVector,
        Self::BadEquation,
    ];

    /// The retained enumerator name; `ErrorType::Done.name() == "Done"`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Done => "gce_Done",
            Self::ConfusedPoints => "gce_ConfusedPoints",
            Self::NegativeRadius => "gce_NegativeRadius",
            Self::ColinearPoints => "gce_ColinearPoints",
            Self::IntersectionError => "gce_IntersectionError",
            Self::NullAxis => "gce_NullAxis",
            Self::NullAngle => "gce_NullAngle",
            Self::NullRadius => "gce_NullRadius",
            Self::InvertAxis => "gce_InvertAxis",
            Self::BadAngle => "gce_BadAngle",
            Self::InvertRadius => "gce_InvertRadius",
            Self::NullFocusLength => "gce_NullFocusLength",
            Self::NullVector => "gce_NullVector",
            Self::BadEquation => "gce_BadEquation",
        }
    }
}

/// Root: the status of a gce construction algorithm.
///
/// Construction status starts as [`ErrorType::Done`]. Constructors that detect
/// a failure replace it with the corresponding typed status.
/// `assert!(MakeDir::from_coordinates(1.0, 0.0, 0.0)?.is_done())`.
pub trait Root {
    /// Root::Status: `Done`, or the reason why the construction failed.
    fn status(&self) -> ErrorType;
    /// Root::IsDone: true when the construction succeeded.
    fn is_done(&self) -> bool {
        self.status() == ErrorType::Done
    }
    /// Root::IsError: true when the construction failed.
    fn is_error(&self) -> bool {
        self.status() != ErrorType::Done
    }
}

/// The not-done failure check of every `Value()`: `member` is the retained member
/// name, such as `MakeCirc::Value()`, and `status` the construction status.
pub(crate) fn check_done(status: ErrorType, member: &str) -> Result<(), Failure> {
    if status == ErrorType::Done {
        return Ok(());
    }
    let message = format!(
        "{member} - no result (status {}; expected gce_Done)",
        status.name()
    );
    Err(Failure::with_type("StdFail_NotDone", &message, ""))
}
