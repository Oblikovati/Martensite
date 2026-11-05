//! Preserve owned-copy isolation while remapping cursor nodes in every retained source.
use super::owners::owns_cursor;
use super::*;
use crate::top_ope_brep_build::list_cursor_copy::copied_cursor;
impl Clone for TopOpeBRepBuildAreaBuilder {
    fn clone(&self) -> Self {
        let mut copied = Self {
            my_area: self.my_area.clone(),
            area_cursor_sources: self.area_cursor_sources.clone(),
            loop_cursor_sources: self.loop_cursor_sources.clone(),
            my_unknown_raise: self.my_unknown_raise,
            my_area_iterator: ListCursor::default(),
            my_loop_iterator: ListCursor::default(),
            loop_area_hint: ListCursor::default(),
            bound_area_source: self.bound_area_source,
            bound_loop_source: self.bound_loop_source,
        };
        copied.loop_area_hint = copied_cursor(self.loop_area_hint, &self.my_area, &copied.my_area);
        copied.my_area_iterator = self.copy_area_cursor(&copied);
        copied.my_loop_iterator = self.copy_loop_cursor(&copied);
        copied
    }
}
impl TopOpeBRepBuildAreaBuilder {
    fn copy_area_cursor(&self, copied: &Self) -> ListCursor {
        self.area_sources()
            .zip(copied.area_sources())
            .find_map(|(before, after)| {
                owns_cursor(before, self.my_area_iterator)
                    .then(|| copied_cursor(self.my_area_iterator, before, after))
            })
            .unwrap_or_default()
    }
    fn copy_loop_cursor(&self, copied: &Self) -> ListCursor {
        self.loop_sources()
            .zip(copied.loop_sources())
            .find_map(|(before, after)| {
                owns_cursor(before, self.my_loop_iterator)
                    .then(|| copied_cursor(self.my_loop_iterator, before, after))
            })
            .unwrap_or_default()
    }
    fn area_sources(&self) -> impl Iterator<Item = &AreaList> {
        std::iter::once(&self.my_area).chain(self.area_cursor_sources.iter())
    }
    fn loop_sources(&self) -> impl Iterator<Item = &AreaLoopList> {
        self.area_sources()
            .flat_map(List::iter)
            .chain(self.loop_cursor_sources.iter())
    }
}
