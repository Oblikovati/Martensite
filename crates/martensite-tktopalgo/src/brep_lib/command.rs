//! `BRepLib_Command`: the done flag at the root of every BRepLib maker.
use super::errors::not_done;
use martensite_tkernel::standard::Failure;

/// `BRepLib_Command`: the done flag of a
/// construction. The native constructor and completion transitions are protected;
/// they are public here so makers in other crates can compose the same command.
///
/// ```
/// use martensite_tktopalgo::brep_lib::BRepLibMakeEdge;
/// let maker = BRepLibMakeEdge::new();
/// assert!(!maker.is_done());
/// assert_eq!(maker.check().unwrap_err().exception_type(), "StdFail_NotDone");
/// ```
#[derive(Clone, Debug, Default)]
pub struct BRepLibCommand {
    my_done: bool,
}

impl BRepLibCommand {
    /// Creates a not-done command for a composed maker.
    /// `assert!(!BRepLibCommand::new().is_done());`
    pub const fn new() -> Self {
        Self { my_done: false }
    }

    /// `IsDone()`; `maker.is_done()`.
    pub const fn is_done(&self) -> bool {
        self.my_done
    }

    /// `Check()`: StdFail_NotDone when the command is not done; `maker.check()?`.
    pub fn check(&self) -> Result<(), Failure> {
        if !self.my_done {
            return Err(not_done("BRep_API: command not done"));
        }
        Ok(())
    }

    /// Marks a composed maker's command complete, as the protected `Done()` does.
    /// `let mut command = BRepLibCommand::new(); command.set_done();`
    pub fn set_done(&mut self) {
        self.my_done = true;
    }

    /// Clears completion for a composed maker, as the protected `NotDone()` does.
    /// `let mut command = BRepLibCommand::new(); command.set_not_done();`
    pub fn set_not_done(&mut self) {
        self.my_done = false;
    }
}
