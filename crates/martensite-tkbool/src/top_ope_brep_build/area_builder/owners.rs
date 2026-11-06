//! Foreign cursor bindings retain owned lists; lookup uses stable node identities.
use super::*;
#[derive(Clone, Copy)]
enum LoopOwner {
    CurrentArea,
    Area(usize),
    ForeignArea(usize, usize),
    ForeignLoop(usize),
}
pub(super) fn owns_cursor<T>(owner: &List<T>, cursor: ListCursor) -> bool {
    if cursor.more() {
        return cursor.value(owner).is_some();
    }
    let Some(previous) = cursor.previous_node() else {
        return false;
    };
    let mut item = owner.cursor();
    while item.more() {
        if item.current_node() == Some(previous) {
            return true;
        }
        item.next(owner);
    }
    false
}
impl TopOpeBRepBuildAreaBuilder {
    pub(super) fn area_cursor_owner(&self) -> Option<&AreaList> {
        if !self.my_area_iterator.more()
            && let Some(index) = self.bound_area_source
        {
            return self.area_cursor_sources.get(index);
        }
        if owns_cursor(&self.my_area, self.my_area_iterator) {
            return Some(&self.my_area);
        }
        self.area_cursor_sources
            .iter()
            .find(|owner| owns_cursor(owner, self.my_area_iterator))
    }
    pub(super) fn area_cursor_owner_mut(&mut self) -> Option<&mut AreaList> {
        if !self.my_area_iterator.more()
            && let Some(index) = self.bound_area_source
        {
            return self.area_cursor_sources.get_mut(index);
        }
        if owns_cursor(&self.my_area, self.my_area_iterator) {
            return Some(&mut self.my_area);
        }
        self.area_cursor_sources
            .iter_mut()
            .find(|owner| owns_cursor(owner, self.my_area_iterator))
    }
    fn loop_owner_location(&self) -> Option<LoopOwner> {
        let cursor = self.my_loop_iterator;
        if !cursor.more()
            && let Some(index) = self.bound_loop_source
        {
            return Some(LoopOwner::ForeignLoop(index));
        }
        if self
            .loop_area_hint
            .value(&self.my_area)
            .is_some_and(|area| owns_cursor(area, cursor))
        {
            return Some(LoopOwner::CurrentArea);
        }
        if let Some(index) = self
            .my_area
            .iter()
            .position(|owner| owns_cursor(owner, cursor))
        {
            return Some(LoopOwner::Area(index));
        }
        for (outer, areas) in self.area_cursor_sources.iter().enumerate() {
            if let Some(inner) = areas.iter().position(|owner| owns_cursor(owner, cursor)) {
                return Some(LoopOwner::ForeignArea(outer, inner));
            }
        }
        self.loop_cursor_sources
            .iter()
            .position(|owner| owns_cursor(owner, cursor))
            .map(LoopOwner::ForeignLoop)
    }
    pub(super) fn loop_owner(&self) -> Option<&AreaLoopList> {
        match self.loop_owner_location()? {
            LoopOwner::CurrentArea => self.loop_area_hint.value(&self.my_area),
            LoopOwner::Area(index) => self.my_area.iter().nth(index),
            LoopOwner::ForeignArea(outer, inner) => {
                self.area_cursor_sources.get(outer)?.iter().nth(inner)
            }
            LoopOwner::ForeignLoop(index) => self.loop_cursor_sources.get(index),
        }
    }
    pub(super) fn loop_owner_mut(&mut self) -> Option<&mut AreaLoopList> {
        match self.loop_owner_location()? {
            LoopOwner::CurrentArea => self.loop_area_hint.change_value(&mut self.my_area),
            LoopOwner::Area(index) => self.my_area.iter_mut().nth(index),
            LoopOwner::ForeignArea(outer, inner) => self
                .area_cursor_sources
                .get_mut(outer)?
                .iter_mut()
                .nth(inner),
            LoopOwner::ForeignLoop(index) => self.loop_cursor_sources.get_mut(index),
        }
    }
    pub(super) fn retain_area_cursor_source(
        &mut self,
        owner: AreaList,
        cursor: ListCursor,
    ) -> Result<(), Failure> {
        validate_cursor(&owner, cursor)?;
        self.bound_area_source = Some(self.area_cursor_sources.len());
        self.area_cursor_sources.push(owner);
        self.my_area_iterator = cursor;
        Ok(())
    }
    pub(super) fn retain_loop_cursor_source(
        &mut self,
        owner: AreaLoopList,
        cursor: ListCursor,
    ) -> Result<(), Failure> {
        validate_cursor(&owner, cursor)?;
        self.bound_loop_source = Some(self.loop_cursor_sources.len());
        self.loop_cursor_sources.push(owner);
        self.my_loop_iterator = cursor;
        Ok(())
    }
}
fn validate_cursor<T>(owner: &List<T>, cursor: ListCursor) -> Result<(), Failure> {
    if (cursor.current_node().is_none() && cursor.previous_node().is_none())
        || owns_cursor(owner, cursor)
    {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_NoSuchObject",
        &format!(
            "AreaBuilder cursor {:?}; expected a live node in its supplied owner",
            cursor.current_node()
        ),
    ))
}
