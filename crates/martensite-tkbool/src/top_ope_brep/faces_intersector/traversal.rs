//! One-based traversal skips intersection lines explicitly marked unusable.
use super::TopOpeBRepFacesIntersector;
use martensite_tkernel::standard::Failure;

impl TopOpeBRepFacesIntersector {
    /// Reports emptiness after traversal; an unperformed pair returns false; `pair.is_empty()?`.
    pub fn is_empty(&mut self) -> Result<bool, Failure> {
        if !self.done {
            return Ok(false);
        }
        if !self.intersection.is_done() || self.intersection.is_empty()? {
            return Ok(true);
        }
        self.init_line()?;
        while self.more_line() {
            if self.current_line()?.nb_vpoint() != 0 {
                return Ok(false);
            }
            self.next_line()?;
        }
        Ok(true)
    }

    /// Counts prepared lines, including unusable lines; `pair.nb_lines()`.
    pub fn nb_lines(&self) -> i32 {
        self.line_count
    }

    /// Starts traversal at the first usable line; `pair.init_line()`.
    pub fn init_line(&mut self) -> Result<(), Failure> {
        self.line_index = 1;
        self.find_line()
    }

    /// Reports whether traversal found a usable line; `pair.more_line()`.
    pub fn more_line(&self) -> bool {
        self.line_found
    }

    /// Advances and skips unusable records; `pair.next_line()`.
    pub fn next_line(&mut self) -> Result<(), Failure> {
        self.line_index += 1;
        self.find_line()
    }

    /// Returns the current one-based index, even at the end; `pair.current_line_index()`.
    pub fn current_line_index(&self) -> i32 {
        self.line_index
    }

    pub(super) fn find_line(&mut self) -> Result<(), Failure> {
        self.line_found = false;
        if !self.done {
            return Ok(());
        }
        while self.line_index <= self.line_count {
            self.line_found = self
                .lines
                .array1()
                .value(self.line_index)
                .map_err(|error| {
                    Failure::with_message(
                        "Standard_OutOfRange",
                        &format!(
                            "FacesIntersector::FindLine: index={}; expected prepared line: {error}",
                            self.line_index
                        ),
                    )
                })?
                .ok();
            if self.line_found {
                return Ok(());
            }
            self.line_index += 1;
        }
        Ok(())
    }
}
