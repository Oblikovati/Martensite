//! Detached cursors retain node identity across prepends and remain ended after append.
use super::*;
impl TopOpeBRepBuildAreaBuilder {
    /// Starts at the first area and initializes its loop; `let count = builder.init_area()`.
    pub fn init_area(&mut self) -> i32 {
        self.my_area_iterator = self.my_area.cursor();
        self.bound_area_source = None;
        self.init_loop();
        self.my_area.extent()
    }
    /// Reports whether the area cursor names a node; `builder.more_area()`.
    pub fn more_area(&self) -> bool {
        self.my_area_iterator.more()
    }
    /// Advances area and initializes its loop cursor; `builder.next_area()`.
    pub fn next_area(&mut self) {
        let mut cursor = self.my_area_iterator;
        if let Some(owner) = self.area_cursor_owner() {
            cursor.next(owner);
        }
        self.my_area_iterator = cursor;
        self.init_loop();
    }
    /// Starts at the current area's first loop; `let count = builder.init_loop()`.
    pub fn init_loop(&mut self) -> i32 {
        self.bound_loop_source = None;
        let Some(area) = self
            .area_cursor_owner()
            .and_then(|owner| self.my_area_iterator.value(owner))
        else {
            self.my_loop_iterator = ListCursor::default();
            return 0;
        };
        let (cursor, count) = (area.cursor(), area.extent());
        self.my_loop_iterator = cursor;
        self.loop_area_hint = self.my_area_iterator;
        count
    }
    /// Reports whether the loop cursor names a node; `builder.more_loop()`.
    pub fn more_loop(&self) -> bool {
        self.my_loop_iterator.more()
    }
    /// Advances in the retained loop's list; `builder.next_loop()`.
    pub fn next_loop(&mut self) {
        let mut cursor = self.my_loop_iterator;
        if let Some(area) = self.loop_owner() {
            cursor.next(area);
        }
        self.my_loop_iterator = cursor;
    }
    /// Reads the retained loop, failing after exhaustion; `let current = builder.loop_()?`.
    pub fn loop_(&self) -> Result<&Handle<dyn TopOpeBRepBuildLoop>, Failure> {
        self.loop_owner()
            .and_then(|area| self.my_loop_iterator.value(area))
            .ok_or_else(|| {
                Failure::with_type(
                    "Standard_NoSuchObject",
                    "TopOpeBRepBuildAreaBuilder::Loop: no current loop; expected MoreLoop()",
                    "TopOpeBRepBuildAreaBuilder",
                )
            })
    }
}
