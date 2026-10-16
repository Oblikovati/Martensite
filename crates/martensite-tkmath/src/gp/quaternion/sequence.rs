// Note: implementation is based on free samples from
// http://www.gamedev.ru/code/articles/?id=4215&page=3
// and maths found in Wikipedia and elsewhere

//! Euler sequence enumeration and Shoemake sequence parameters (Graphics Gems IV).
/// Selects an Euler convention; `EulerSequence::YawPitchRoll`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EulerSequence {
    /// EulerAngles convention.
    EulerAngles = 0,
    /// YawPitchRoll convention.
    YawPitchRoll = 1,
    /// ExtrinsicXyz convention.
    ExtrinsicXyz = 2,
    /// ExtrinsicXzy convention.
    ExtrinsicXzy = 3,
    /// ExtrinsicYzx convention.
    ExtrinsicYzx = 4,
    /// ExtrinsicYxz convention.
    ExtrinsicYxz = 5,
    /// ExtrinsicZxy convention.
    ExtrinsicZxy = 6,
    /// ExtrinsicZyx convention.
    ExtrinsicZyx = 7,
    /// IntrinsicXyz convention.
    IntrinsicXyz = 8,
    /// IntrinsicXzy convention.
    IntrinsicXzy = 9,
    /// IntrinsicYzx convention.
    IntrinsicYzx = 10,
    /// IntrinsicYxz convention.
    IntrinsicYxz = 11,
    /// IntrinsicZxy convention.
    IntrinsicZxy = 12,
    /// IntrinsicZyx convention.
    IntrinsicZyx = 13,
    /// ExtrinsicXyx convention.
    ExtrinsicXyx = 14,
    /// ExtrinsicXzx convention.
    ExtrinsicXzx = 15,
    /// ExtrinsicYzy convention.
    ExtrinsicYzy = 16,
    /// ExtrinsicYxy convention.
    ExtrinsicYxy = 17,
    /// ExtrinsicZyz convention.
    ExtrinsicZyz = 18,
    /// ExtrinsicZxz convention.
    ExtrinsicZxz = 19,
    /// IntrinsicXyx convention.
    IntrinsicXyx = 20,
    /// IntrinsicXzx convention.
    IntrinsicXzx = 21,
    /// IntrinsicYzy convention.
    IntrinsicYzy = 22,
    /// IntrinsicYxy convention.
    IntrinsicYxy = 23,
    /// IntrinsicZxz convention.
    IntrinsicZxz = 24,
    /// IntrinsicZyz convention.
    IntrinsicZyz = 25,
}
impl EulerSequence {
    /// All 24 non-alias conventions in enum order; `EulerSequence::ALL`.
    pub const ALL: [Self; 24] = [
        Self::ExtrinsicXyz,
        Self::ExtrinsicXzy,
        Self::ExtrinsicYzx,
        Self::ExtrinsicYxz,
        Self::ExtrinsicZxy,
        Self::ExtrinsicZyx,
        Self::IntrinsicXyz,
        Self::IntrinsicXzy,
        Self::IntrinsicYzx,
        Self::IntrinsicYxz,
        Self::IntrinsicZxy,
        Self::IntrinsicZyx,
        Self::ExtrinsicXyx,
        Self::ExtrinsicXzx,
        Self::ExtrinsicYzy,
        Self::ExtrinsicYxy,
        Self::ExtrinsicZyz,
        Self::ExtrinsicZxz,
        Self::IntrinsicXyx,
        Self::IntrinsicXzx,
        Self::IntrinsicYzy,
        Self::IntrinsicYxy,
        Self::IntrinsicZxz,
        Self::IntrinsicZyz,
    ];
    pub(super) fn parameters(self) -> Parameters {
        let (i, odd, two, extrinsic) = PARAMETERS[self as usize];
        Parameters {
            i,
            j: (i + 1 + usize::from(odd)) % 3,
            k: (i + 1 + usize::from(!odd)) % 3,
            odd,
            two,
            extrinsic,
        }
    }
}
const PARAMETERS: [(usize, bool, bool, bool); 26] = [
    (2, false, true, false),
    (0, false, false, false),
    (0, false, false, true),
    (0, true, false, true),
    (1, false, false, true),
    (1, true, false, true),
    (2, false, false, true),
    (2, true, false, true),
    (2, true, false, false),
    (1, false, false, false),
    (0, true, false, false),
    (2, false, false, false),
    (1, true, false, false),
    (0, false, false, false),
    (0, false, true, true),
    (0, true, true, true),
    (1, false, true, true),
    (1, true, true, true),
    (2, true, true, true),
    (2, false, true, true),
    (0, false, true, false),
    (0, true, true, false),
    (1, false, true, false),
    (1, true, true, false),
    (2, false, true, false),
    (2, true, true, false),
];
#[derive(Clone, Copy)]
pub(super) struct Parameters {
    pub i: usize,
    pub j: usize,
    pub k: usize,
    pub odd: bool,
    pub two: bool,
    pub extrinsic: bool,
}
