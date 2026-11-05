//! Sequential walking-point traversal with the source's one-based indexing.
use super::{TopOpeBRepLineInter, TopOpeBRepWPointInter};
use martensite_tkernel::standard::Failure;

/// Iterates sampled points while borrowing their intersection line exclusively.
///
/// `let cursor = TopOpeBRepWPointInterIterator::new(); assert!(cursor.more());`
#[derive(Default)]
pub struct TopOpeBRepWPointInterIterator<'a> {
    line: Option<&'a mut TopOpeBRepLineInter>,
    index: i32,
    count: i32,
}

impl<'a> TopOpeBRepWPointInterIterator<'a> {
    /// Retains the reference default zero index/count; `Self::new().more()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Starts at the first walking point; `Self::with_line(&mut line)?`.
    pub fn with_line(line: &'a mut TopOpeBRepLineInter) -> Result<Self, Failure> {
        let mut iterator = Self::new();
        iterator.init(line)?;
        Ok(iterator)
    }
    /// Rebinds the traversal source and restarts it; `cursor.init(&mut line)?`.
    pub fn init(&mut self, line: &'a mut TopOpeBRepLineInter) -> Result<(), Failure> {
        self.line = Some(line);
        self.restart()
    }
    /// Restarts against the retained source; `cursor.restart()?`.
    pub fn restart(&mut self) -> Result<(), Failure> {
        self.index = 1;
        self.count = self.source()?.nb_wpoint()?;
        Ok(())
    }
    /// Preserves the inclusive source comparison; `cursor.more()`.
    pub fn more(&self) -> bool {
        self.index <= self.count
    }
    /// Advances even beyond the final sample; `cursor.next()`.
    pub fn next(&mut self) {
        self.index += 1;
    }
    /// Borrows the current sampled point; `cursor.current_wp()?.value()?`.
    pub fn current_wp(&mut self) -> Result<TopOpeBRepWPointInter<'_>, Failure> {
        if !self.more() {
            return Err(Failure::with_message(
                "Standard_ProgramError",
                "TopOpeBRepWPointInterIterator::Current",
            ));
        }
        let index = self.index;
        self.line
            .as_deref_mut()
            .ok_or_else(unbound_line)?
            .wpoint(index)
    }
    /// Exposes the retained source without casting away constness; `cursor.line_inter()`.
    pub fn line_inter(&self) -> Option<&TopOpeBRepLineInter> {
        self.line.as_deref()
    }
    fn source(&self) -> Result<&TopOpeBRepLineInter, Failure> {
        self.line_inter().ok_or_else(unbound_line)
    }
}

fn unbound_line() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "TopOpeBRepWPointInterIterator: source line=None; expected Init with an intersection line",
    )
}
