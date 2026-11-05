//! Indexed traversal of a line's intersection points, with optional keep filtering.
use super::{TopOpeBRepLineInter, TopOpeBRepVPointInter};
use martensite_tkernel::standard::Failure;

/// A borrowing traversal; initialize with `TopOpeBRepVPointInterIterator::with_line(&mut line)`.
#[derive(Default)]
pub struct TopOpeBRepVPointInterIterator<'line> {
    line: Option<&'line mut TopOpeBRepLineInter>,
    index: i32,
    count: i32,
    check_keep: bool,
}

impl<'line> TopOpeBRepVPointInterIterator<'line> {
    /// Creates the native unbound state; `Self::new().more()` is true until initialization.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts at the first point without filtering; `Self::with_line(&mut line)?`.
    pub fn with_line(line: &'line mut TopOpeBRepLineInter) -> Result<Self, Failure> {
        let mut iterator = Self::new();
        iterator.init(line, false)?;
        Ok(iterator)
    }

    /// Binds a line and restarts its traversal; `iterator.init(&mut line, true)?`.
    pub fn init(
        &mut self,
        line: &'line mut TopOpeBRepLineInter,
        check_keep: bool,
    ) -> Result<(), Failure> {
        self.line = Some(line);
        self.check_keep = check_keep;
        self.restart()
    }

    /// Restarts using the original filter; `iterator.restart()?`.
    pub fn restart(&mut self) -> Result<(), Failure> {
        self.count = self
            .line
            .as_ref()
            .ok_or_else(|| unbound("Init"))?
            .nb_vpoint();
        self.index = 1;
        self.skip_rejected()
    }

    /// Tests the native index/count relation; `iterator.more()`.
    pub fn more(&self) -> bool {
        self.index <= self.count
    }

    /// Advances to the next eligible point; `iterator.next()?`.
    // Native Next advances a cursor and reports failure; it does not yield an item.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<(), Failure> {
        self.index += 1;
        self.skip_rejected()
    }

    /// Reads the current point or raises the native program error at end; `iterator.current_vp()?`.
    pub fn current_vp(&self) -> Result<&TopOpeBRepVPointInter<'static>, Failure> {
        self.require_current("CurrentVP")?;
        self.line
            .as_ref()
            .ok_or_else(|| unbound("CurrentVP"))?
            .vpoint(self.index)
    }

    /// Mutates the current record through the exclusive line borrow; `iterator.change_current_vp()?`.
    pub fn change_current_vp(&mut self) -> Result<&mut TopOpeBRepVPointInter<'static>, Failure> {
        self.require_current("ChangeCurrentVP")?;
        self.line
            .as_mut()
            .ok_or_else(|| unbound("ChangeCurrentVP"))?
            .change_vpoint(self.index)
    }

    /// Reads the one-based index or raises at end; `iterator.current_vp_index()?`.
    pub fn current_vp_index(&self) -> Result<i32, Failure> {
        self.require_current("CurrentVPIndex")?;
        Ok(self.index)
    }

    /// Exposes the safely borrowed line identity; `iterator.line_inter()`.
    pub fn line_inter(&self) -> Option<&TopOpeBRepLineInter> {
        self.line.as_deref()
    }

    pub(crate) fn line_inter_mut(&mut self) -> Result<&mut TopOpeBRepLineInter, Failure> {
        self.line.as_deref_mut().ok_or_else(|| unbound("LineInter"))
    }

    fn require_current(&self, operation: &str) -> Result<(), Failure> {
        if self.more() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_ProgramError",
            &format!("TopOpeBRepVPointInterIterator::{operation}"),
        ))
    }

    fn skip_rejected(&mut self) -> Result<(), Failure> {
        while self.check_keep && self.more() {
            if self.current_vp()?.keep() {
                break;
            }
            self.index += 1;
        }
        Ok(())
    }
}

fn unbound(operation: &str) -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        &format!("TopOpeBRepVPointInterIterator::{operation}: line=None; expected Init(line)"),
    )
}
