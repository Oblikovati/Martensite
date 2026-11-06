//! `StepToTopoDSRoot`: the state common to the translators.
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;

/// `StepToTopoDSRoot`: gka 13.04.99 S4136: the precision and the maximal tolerance.
///
/// ```
/// use martensite_tkdestep::step_to_topo_ds::StepToTopoDSRoot;
/// let mut root = StepToTopoDSRoot::new();
/// root.set_precision(0.5);
/// assert!(!root.is_done() && root.precision() == 0.5);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct StepToTopoDSRoot {
    pub(super) done: bool,
    my_precision: f64,
    my_max_tol: f64,
}

impl Default for StepToTopoDSRoot {
    fn default() -> Self {
        Self {
            done: false,
            my_precision: precision::confusion(),
            my_max_tol: precision::confusion(),
        }
    }
}

impl StepToTopoDSRoot {
    /// `StepToTopoDSRoot()`: not done, both tolerances at `Precision::confusion`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// `precision`.
    pub fn precision(&self) -> f64 {
        self.my_precision
    }
    /// `set_precision`.
    pub fn set_precision(&mut self, preci: f64) {
        self.my_precision = preci;
    }
    /// `max_tol`.
    pub fn max_tol(&self) -> f64 {
        self.my_max_tol
    }
    /// `set_max_tol`.
    pub fn set_max_tol(&mut self, maxpreci: f64) {
        self.my_max_tol = maxpreci;
    }
    /// a not done failure while the translator named `member` has no result.
    pub(super) fn check_done(&self, member: &str) -> Result<(), Failure> {
        if self.done {
            return Ok(());
        }
        Err(martensite_tkernel::std_fail::not_done(&format!(
            "{member} - no result"
        )))
    }
}
