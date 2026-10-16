use crate::math::{Matrix, Vector};
/// Solver outcome; `Status::Ok` denotes a successful computation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum Status {
    /// Outcome Ok; `Status::Ok`.
    Ok = 0,
    /// Outcome NotConverged; `Status::NotConverged`.
    #[default]
    NotConverged = 1,
    /// Outcome MaxIterations; `Status::MaxIterations`.
    MaxIterations = 2,
    /// Outcome NumericalError; `Status::NumericalError`.
    NumericalError = 3,
    /// Outcome InvalidInput; `Status::InvalidInput`.
    InvalidInput = 4,
    /// Outcome InfiniteSolutions; `Status::InfiniteSolutions`.
    InfiniteSolutions = 5,
    /// Outcome NoSolution; `Status::NoSolution`.
    NoSolution = 6,
    /// Outcome NotPositiveDefinite; `Status::NotPositiveDefinite`.
    NotPositiveDefinite = 7,
    /// Outcome Singular; `Status::Singular`.
    Singular = 8,
    /// Outcome NonDescentDirection; `Status::NonDescentDirection`.
    NonDescentDirection = 9,
    /// Outcome CallbackError; `Status::CallbackError`.
    CallbackError = 10,
}
/// Result with optional computed fields; `ScalarResult::default()`.
#[derive(Default)]
pub struct ScalarResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Iterations field; `result.iterations`.
    pub iterations: u32,
    /// Root field; `result.root`.
    pub root: Option<f64>,
    /// Value field; `result.value`.
    pub value: Option<f64>,
    /// Derivative field; `result.derivative`.
    pub derivative: Option<f64>,
}
impl ScalarResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&ScalarResult> for bool {
    fn from(result: &ScalarResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `PolyResult::default()`.
#[derive(Default)]
pub struct PolyResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Number of roots field; `result.number_of_roots`.
    pub number_of_roots: usize,
    /// Roots field; `result.roots`.
    pub roots: [f64; 4],
}
impl PolyResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&PolyResult> for bool {
    fn from(result: &PolyResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `VectorResult::default()`.
#[derive(Default)]
pub struct VectorResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Iterations field; `result.iterations`.
    pub iterations: u32,
    /// Solution field; `result.solution`.
    pub solution: Option<Vector<'static>>,
    /// Value field; `result.value`.
    pub value: Option<f64>,
    /// Gradient field; `result.gradient`.
    pub gradient: Option<Vector<'static>>,
    /// Jacobian field; `result.jacobian`.
    pub jacobian: Option<Matrix<'static>>,
}
impl VectorResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&VectorResult> for bool {
    fn from(result: &VectorResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `LinearResult::default()`.
#[derive(Default)]
pub struct LinearResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Solution field; `result.solution`.
    pub solution: Option<Vector<'static>>,
    /// Determinant field; `result.determinant`.
    pub determinant: Option<f64>,
}
impl LinearResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&LinearResult> for bool {
    fn from(result: &LinearResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `LinearMultipleResult::default()`.
#[derive(Default)]
pub struct LinearMultipleResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Solutions field; `result.solutions`.
    pub solutions: Option<Matrix<'static>>,
    /// Determinant field; `result.determinant`.
    pub determinant: Option<f64>,
}
impl LinearMultipleResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&LinearMultipleResult> for bool {
    fn from(result: &LinearMultipleResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `EigenResult::default()`.
#[derive(Default)]
pub struct EigenResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Iterations field; `result.iterations`.
    pub iterations: u32,
    /// Eigen values field; `result.eigen_values`.
    pub eigen_values: Option<Vector<'static>>,
    /// Eigen vectors field; `result.eigen_vectors`.
    pub eigen_vectors: Option<Matrix<'static>>,
}
impl EigenResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&EigenResult> for bool {
    fn from(result: &EigenResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `DecompResult::default()`.
#[derive(Default)]
pub struct DecompResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Lower field; `result.lower`.
    pub lower: Option<Matrix<'static>>,
    /// Upper field; `result.upper`.
    pub upper: Option<Matrix<'static>>,
    /// Diagonal field; `result.diagonal`.
    pub diagonal: Option<Vector<'static>>,
    /// Determinant field; `result.determinant`.
    pub determinant: Option<f64>,
}
impl DecompResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&DecompResult> for bool {
    fn from(result: &DecompResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `IntegResult::default()`.
#[derive(Default)]
pub struct IntegResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Iterations field; `result.iterations`.
    pub iterations: u32,
    /// Number of points field; `result.number_of_points`.
    pub number_of_points: usize,
    /// Value field; `result.value`.
    pub value: Option<f64>,
    /// Absolute error field; `result.absolute_error`.
    pub absolute_error: Option<f64>,
    /// Relative error field; `result.relative_error`.
    pub relative_error: Option<f64>,
}
impl IntegResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&IntegResult> for bool {
    fn from(result: &IntegResult) -> Self {
        result.is_done()
    }
}
/// Result with optional computed fields; `InverseResult::default()`.
#[derive(Default)]
pub struct InverseResult {
    /// Status field; `result.status`.
    pub status: Status,
    /// Inverse field; `result.inverse`.
    pub inverse: Option<Matrix<'static>>,
    /// Determinant field; `result.determinant`.
    pub determinant: Option<f64>,
}
impl InverseResult {
    /// Reports Status::Ok; `result.is_done()`.
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}
impl From<&InverseResult> for bool {
    fn from(result: &InverseResult) -> Self {
        result.is_done()
    }
}
impl std::ops::Index<usize> for PolyResult {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.roots[index]
    }
}
