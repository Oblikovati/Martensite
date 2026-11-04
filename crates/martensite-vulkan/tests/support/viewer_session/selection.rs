//! Interactive selection: activation of selection modes, cursor detection and picking.
//!
//! Cursor and pick events go through one view controller owned by the session, queued and
//! flushed against the active view as an interactive viewer delivers pointer input.
use super::*;
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkservice::aspect::{ASPECT_VKEY_FLAGS_NONE, ASPECT_VKEY_MOUSE_NONE};
use martensite_tkv3d::ais::{AisSelectionModesConcurrency, AisSelectionScheme};
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorMembers as _;

impl ViewerSession {
    /// Turn selection `mode` on or off for every displayed object, keeping the other active
    /// modes by the global-or-local policy; `session.set_selection_mode(1, true)?`.
    pub fn set_selection_mode(&self, mode: i32, active: bool) -> Result<(), Failure> {
        let mut displayed = Default::default();
        self.context.displayed_objects(&mut displayed)?;
        for object in displayed.iter() {
            self.context.set_selection_mode_active(
                object,
                mode,
                active,
                AisSelectionModesConcurrency::GlobalOrLocal,
                false,
            )?;
        }
        Ok(())
    }

    /// Move the cursor to pixel (x, y) and highlight what it detects; `session.move_cursor(26, 305)?`.
    pub fn move_cursor(&mut self, x: i32, y: i32) -> Result<(), Failure> {
        let point = Vec2::new(x, y);
        self.controller.reset_previous_move_to();
        self.controller.update_mouse_position(
            point,
            ASPECT_VKEY_MOUSE_NONE,
            ASPECT_VKEY_FLAGS_NONE,
            false,
        )?;
        self.flush_pointer_events()
    }

    /// Pick at pixel (x, y), replacing the selection; `session.select_point(26, 305)?`.
    pub fn select_point(&mut self, x: i32, y: i32) -> Result<(), Failure> {
        self.select_polyline(&[(x, y)], AisSelectionScheme::Replace)
    }

    /// Select at the given pixels as the select command does: one pixel picks, two are rubber
    /// band corners (top first, or bottom first with overlap allowed), more form a polygon;
    /// `allow_overlap` turns overlap detection on for this and later selections.
    /// Example: `session.select_pixels(&[(300, 200)], false)?`.
    pub fn select_pixels(
        &mut self,
        pixels: &[(i32, i32)],
        allow_overlap: bool,
    ) -> Result<(), Failure> {
        if allow_overlap {
            self.context.main_selector().allow_overlap_detection(true);
        }
        let mut ordered = pixels.to_vec();
        if let [first, last] = ordered[..]
            && (if allow_overlap {
                first.1 < last.1
            } else {
                first.1 > last.1
            })
        {
            ordered.swap(0, 1);
        }
        self.select_polyline(&ordered, AisSelectionScheme::Replace)
    }

    /// Pick inside the rectangle of two corners with `scheme`; the corners are ordered top
    /// first, as a rubber band is; `session.select_rectangle((204, 409), (0, 0), Xor)?`.
    pub fn select_rectangle(
        &mut self,
        first: (i32, i32),
        second: (i32, i32),
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        let ordered = if first.1 > second.1 {
            [second, first]
        } else {
            [first, second]
        };
        self.select_polyline(&ordered, scheme)
    }

    /// Pick in a rectangle that also takes entities only partly inside: turns overlap
    /// detection on for the selector (it stays on), and orders the corners bottom first.
    /// Example: `session.select_rectangle_overlapping((75, 230), (235, 320), Replace)?`.
    pub fn select_rectangle_overlapping(
        &mut self,
        first: (i32, i32),
        second: (i32, i32),
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        self.context.main_selector().allow_overlap_detection(true);
        let ordered = if first.1 < second.1 {
            [second, first]
        } else {
            [first, second]
        };
        self.select_polyline(&ordered, scheme)
    }

    /// The number of selected owners; `session.selected_count()`.
    pub fn selected_count(&self) -> i32 {
        self.context.nb_selected()
    }

    /// Pick at a point (one pixel), in a rectangle (two) or in a polygon (more) with `scheme`.
    /// Example: `session.select_polyline(&[(1, 1), (9, 1), (5, 9)], AisSelectionScheme::Add)?`.
    pub fn select_polyline(
        &mut self,
        pixels: &[(i32, i32)],
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        let mut points = Sequence::default();
        for &(x, y) in pixels {
            points.append(Vec2::new(x, y));
        }
        self.controller.select_points_in_viewer(&points, scheme)?;
        self.flush_pointer_events()
    }

    /// Apply `scheme` to what the cursor currently detects; `session.select_detected(Replace)?`.
    pub fn select_detected(&mut self, scheme: AisSelectionScheme) -> Result<(), Failure> {
        self.context.select_detected(scheme)?;
        if let Some(viewer) = self.context.current_viewer().get() {
            viewer.invalidate();
        }
        self.flush_pointer_events()
    }

    /// Highlight the next of the overlapping detected entities, wrapping at the end, and redraw.
    /// Example: `session.highlight_next_detected()?`.
    pub fn highlight_next_detected(&self) -> Result<(), Failure> {
        self.context.hilight_next_detected(self.view(), true)?;
        Ok(())
    }

    /// Highlight the previous detected entity, wrapping at the start, and redraw.
    /// Example: `session.highlight_previous_detected()?`.
    pub fn highlight_previous_detected(&self) -> Result<(), Failure> {
        self.context.hilight_previous_detected(self.view(), true)?;
        Ok(())
    }

    fn flush_pointer_events(&mut self) -> Result<(), Failure> {
        let view = self.views[self.active].1.clone();
        self.controller
            .flush_view_events(&self.context, &view, true)
    }
}
