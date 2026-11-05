use super::{TopOpeBRepEdgesIntersector, TopOpeBRepPoint2d};
use martensite_tkernel::standard::Failure;
impl TopOpeBRepEdgesIntersector {
    /// Checks the number of selected point records; `state.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.selected_count == 0
    }
    /// Reads all point records, including discarded segment endpoints; `state.points()`.
    pub fn points(&self) -> &[TopOpeBRepPoint2d] {
        &self.points
    }
    /// Starts iteration with optional Keep filtering; `state.init_point(true)`.
    pub fn init_point(&mut self, select_keep: bool) {
        self.select_keep = select_keep;
        self.cursor = 1;
        self.selected_count = self.points.len() as i32;
        self.find();
    }
    /// Reports whether the native point cursor remains in range; `state.more_point()`.
    pub fn more_point(&self) -> bool {
        self.cursor <= self.selected_count
    }
    /// Advances and skips discarded points when selected; `state.next_point()`.
    pub fn next_point(&mut self) {
        self.cursor += 1;
        self.find();
    }
    /// Reads the current point; `state.point()?`.
    pub fn point(&self) -> Result<&TopOpeBRepPoint2d, Failure> {
        self.point_at(self.cursor)
    }
    /// Reads a point at its one-based sequence index; `state.point_at(1)?`.
    pub fn point_at(&self, index: i32) -> Result<&TopOpeBRepPoint2d, Failure> {
        let slot = usize::try_from(index - 1)
            .ok()
            .and_then(|i| self.points.get(i));
        slot.ok_or_else(|| {
            Failure::new(&format!(
                "TopOpeBRepEdgesIntersector::Point: index={index}; expected 1..={}",
                self.points.len()
            ))
        })
    }
    fn find(&mut self) {
        while self.cursor <= self.selected_count && self.select_keep {
            if self.points[(self.cursor - 1) as usize].keep() {
                return;
            }
            self.cursor += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iteration_skips_discarded_records_only_when_requested() {
        let mut state = TopOpeBRepEdgesIntersector::new();
        let mut discarded = TopOpeBRepPoint2d::new();
        discarded.set_keep(false);
        discarded.set_index(1);
        let mut kept = TopOpeBRepPoint2d::new();
        kept.set_index(2);
        state.points = vec![discarded, kept];
        state.init_point(true);
        assert_eq!(state.point().unwrap().index(), 2);
        state.next_point();
        assert!(!state.more_point());
        state.init_point(false);
        assert_eq!(state.point().unwrap().index(), 1);
        state.next_point();
        assert_eq!(state.point().unwrap().index(), 2);
        state.next_point();
        assert!(!state.more_point());
    }
}
