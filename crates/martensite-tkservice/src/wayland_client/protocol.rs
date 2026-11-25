//! Static Wayland interface descriptors for `xdg-shell` and `xdg-decoration-unstable-v1`.
//!
//! Layout and signatures match `wayland-scanner private-code` for the stable `xdg-shell`
//! and `xdg-decoration-unstable-v1` XML specifications.
#![allow(unsafe_code)]

use super::{CoreInterfaces, WlInterface, WlMessage};
use std::ptr;
use std::sync::OnceLock;

/// Owned process-lifetime descriptor tables for `xdg-shell` and `xdg-decoration-unstable-v1`.
pub(crate) struct XdgProtocol {
    pub(crate) wm_base: *const WlInterface,
    pub(crate) surface: *const WlInterface,
    pub(crate) toplevel: *const WlInterface,
    pub(crate) decoration_manager: *const WlInterface,
    pub(crate) toplevel_decoration: *const WlInterface,
}

// SAFETY: All pointers point into a leaked immutable table initialized once per process.
unsafe impl Send for XdgProtocol {}
// SAFETY: All pointers point into a leaked immutable table initialized once per process.
unsafe impl Sync for XdgProtocol {}

/// Returns the initialized `xdg-shell` and `xdg-decoration` protocol descriptors.
pub(crate) fn xdg_protocol(core: &CoreInterfaces) -> &'static XdgProtocol {
    static PROTO: OnceLock<XdgProtocol> = OnceLock::new();
    PROTO.get_or_init(|| build_xdg_protocol(core))
}

struct XdgTables {
    types: [*const WlInterface; 26],
    wm_base_reqs: [WlMessage; 4],
    wm_base_evts: [WlMessage; 1],
    positioner_reqs: [WlMessage; 10],
    surface_reqs: [WlMessage; 5],
    surface_evts: [WlMessage; 1],
    toplevel_reqs: [WlMessage; 14],
    toplevel_evts: [WlMessage; 4],
    popup_reqs: [WlMessage; 3],
    popup_evts: [WlMessage; 3],
    wm_base: WlInterface,
    positioner: WlInterface,
    surface: WlInterface,
    toplevel: WlInterface,
    popup: WlInterface,
    decor_types: [*const WlInterface; 3],
    decor_mgr_reqs: [WlMessage; 2],
    decor_reqs: [WlMessage; 3],
    decor_evts: [WlMessage; 1],
    decor_mgr: WlInterface,
    decor: WlInterface,
}

fn msg(
    name: &'static std::ffi::CStr,
    sig: &'static std::ffi::CStr,
    types: *const *const WlInterface,
) -> WlMessage {
    WlMessage {
        name: name.as_ptr(),
        signature: sig.as_ptr(),
        types,
    }
}

fn empty_iface(name: &'static std::ffi::CStr, version: i32) -> WlInterface {
    WlInterface {
        name: name.as_ptr(),
        version,
        method_count: 0,
        methods: ptr::null(),
        event_count: 0,
        events: ptr::null(),
    }
}

fn build_xdg_protocol(core: &CoreInterfaces) -> XdgProtocol {
    let tables = Box::leak(Box::new(allocate_tables()));
    wire_types(tables, core);
    wire_messages(tables);
    wire_interfaces(tables);
    XdgProtocol {
        wm_base: ptr::addr_of!(tables.wm_base),
        surface: ptr::addr_of!(tables.surface),
        toplevel: ptr::addr_of!(tables.toplevel),
        decoration_manager: ptr::addr_of!(tables.decor_mgr),
        toplevel_decoration: ptr::addr_of!(tables.decor),
    }
}

fn allocate_tables() -> XdgTables {
    let m = msg(c"", c"", ptr::null());
    XdgTables {
        types: [ptr::null(); 26],
        wm_base_reqs: [m, m, m, m],
        wm_base_evts: [m],
        positioner_reqs: [m; 10],
        surface_reqs: [m; 5],
        surface_evts: [m],
        toplevel_reqs: [m; 14],
        toplevel_evts: [m; 4],
        popup_reqs: [m; 3],
        popup_evts: [m; 3],
        wm_base: empty_iface(c"xdg_wm_base", 6),
        positioner: empty_iface(c"xdg_positioner", 6),
        surface: empty_iface(c"xdg_surface", 6),
        toplevel: empty_iface(c"xdg_toplevel", 6),
        popup: empty_iface(c"xdg_popup", 6),
        decor_types: [ptr::null(); 3],
        decor_mgr_reqs: [m; 2],
        decor_reqs: [m; 3],
        decor_evts: [m],
        decor_mgr: empty_iface(c"zxdg_decoration_manager_v1", 1),
        decor: empty_iface(c"zxdg_toplevel_decoration_v1", 1),
    }
}

fn wire_types(t: &mut XdgTables, core: &CoreInterfaces) {
    let pos = ptr::addr_of!(t.positioner);
    let surf = ptr::addr_of!(t.surface);
    let top = ptr::addr_of!(t.toplevel);
    let pop = ptr::addr_of!(t.popup);
    t.types[4] = pos;
    t.types[5] = surf;
    t.types[6] = core.surface;
    t.types[7] = top;
    t.types[8] = pop;
    t.types[9] = surf;
    t.types[10] = pos;
    t.types[11] = top;
    t.types[12] = core.seat;
    t.types[16] = core.seat;
    t.types[18] = core.seat;
    t.types[21] = core.output;
    t.types[22] = core.seat;
    t.types[24] = pos;
    t.decor_types[1] = ptr::addr_of!(t.decor);
    t.decor_types[2] = top;
}

fn wire_messages(t: &mut XdgTables) {
    let base = t.types.as_ptr();
    // SAFETY: Offsets 0..=24 are strictly within `t.types` (length 26).
    let at = |offset: usize| unsafe { base.add(offset) };
    t.wm_base_reqs = [
        msg(c"destroy", c"", at(0)),
        msg(c"create_positioner", c"n", at(4)),
        msg(c"get_xdg_surface", c"no", at(5)),
        msg(c"pong", c"u", at(0)),
    ];
    t.wm_base_evts = [msg(c"ping", c"u", at(0))];
    t.positioner_reqs = [
        msg(c"destroy", c"", at(0)),
        msg(c"set_size", c"ii", at(0)),
        msg(c"set_anchor_rect", c"iiii", at(0)),
        msg(c"set_anchor", c"u", at(0)),
        msg(c"set_gravity", c"u", at(0)),
        msg(c"set_constraint_adjustment", c"u", at(0)),
        msg(c"set_offset", c"ii", at(0)),
        msg(c"set_reactive", c"3", at(0)),
        msg(c"set_parent_size", c"3ii", at(0)),
        msg(c"set_parent_configure", c"3u", at(0)),
    ];
    t.surface_reqs = [
        msg(c"destroy", c"", at(0)),
        msg(c"get_toplevel", c"n", at(7)),
        msg(c"get_popup", c"n?oo", at(8)),
        msg(c"set_window_geometry", c"iiii", at(0)),
        msg(c"ack_configure", c"u", at(0)),
    ];
    t.surface_evts = [msg(c"configure", c"u", at(0))];
    t.toplevel_reqs = [
        msg(c"destroy", c"", at(0)),
        msg(c"set_parent", c"?o", at(11)),
        msg(c"set_title", c"s", at(0)),
        msg(c"set_app_id", c"s", at(0)),
        msg(c"show_window_menu", c"ouii", at(12)),
        msg(c"move", c"ou", at(16)),
        msg(c"resize", c"ouu", at(18)),
        msg(c"set_max_size", c"ii", at(0)),
        msg(c"set_min_size", c"ii", at(0)),
        msg(c"set_maximized", c"", at(0)),
        msg(c"unset_maximized", c"", at(0)),
        msg(c"set_fullscreen", c"?o", at(21)),
        msg(c"unset_fullscreen", c"", at(0)),
        msg(c"set_minimized", c"", at(0)),
    ];
    t.toplevel_evts = [
        msg(c"configure", c"iia", at(0)),
        msg(c"close", c"", at(0)),
        msg(c"configure_bounds", c"4ii", at(0)),
        msg(c"wm_capabilities", c"5a", at(0)),
    ];
    t.popup_reqs = [
        msg(c"destroy", c"", at(0)),
        msg(c"grab", c"ou", at(22)),
        msg(c"reposition", c"3ou", at(24)),
    ];
    t.popup_evts = [
        msg(c"configure", c"iiii", at(0)),
        msg(c"popup_done", c"", at(0)),
        msg(c"repositioned", c"3u", at(0)),
    ];
    let dbase = t.decor_types.as_ptr();
    // SAFETY: Offsets 0 and 1 are within `t.decor_types` (length 3).
    let dat = |offset: usize| unsafe { dbase.add(offset) };
    t.decor_mgr_reqs = [
        msg(c"destroy", c"", dat(0)),
        msg(c"get_toplevel_decoration", c"no", dat(1)),
    ];
    t.decor_reqs = [
        msg(c"destroy", c"", dat(0)),
        msg(c"set_mode", c"u", dat(0)),
        msg(c"unset_mode", c"", dat(0)),
    ];
    t.decor_evts = [msg(c"configure", c"u", dat(0))];
}

fn wire_interfaces(t: &mut XdgTables) {
    t.wm_base.method_count = t.wm_base_reqs.len() as i32;
    t.wm_base.methods = t.wm_base_reqs.as_ptr();
    t.wm_base.event_count = t.wm_base_evts.len() as i32;
    t.wm_base.events = t.wm_base_evts.as_ptr();

    t.positioner.method_count = t.positioner_reqs.len() as i32;
    t.positioner.methods = t.positioner_reqs.as_ptr();

    t.surface.method_count = t.surface_reqs.len() as i32;
    t.surface.methods = t.surface_reqs.as_ptr();
    t.surface.event_count = t.surface_evts.len() as i32;
    t.surface.events = t.surface_evts.as_ptr();

    t.toplevel.method_count = t.toplevel_reqs.len() as i32;
    t.toplevel.methods = t.toplevel_reqs.as_ptr();
    t.toplevel.event_count = t.toplevel_evts.len() as i32;
    t.toplevel.events = t.toplevel_evts.as_ptr();

    t.popup.method_count = t.popup_reqs.len() as i32;
    t.popup.methods = t.popup_reqs.as_ptr();
    t.popup.event_count = t.popup_evts.len() as i32;
    t.popup.events = t.popup_evts.as_ptr();

    t.decor_mgr.method_count = t.decor_mgr_reqs.len() as i32;
    t.decor_mgr.methods = t.decor_mgr_reqs.as_ptr();

    t.decor.method_count = t.decor_reqs.len() as i32;
    t.decor.methods = t.decor_reqs.as_ptr();
    t.decor.event_count = t.decor_evts.len() as i32;
    t.decor.events = t.decor_evts.as_ptr();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wayland_client::wayland;
    use std::ffi::CStr;

    #[test]
    fn xdg_protocol_descriptors_report_expected_names_and_counts() {
        let Some(api) = wayland() else {
            return;
        };
        let proto = xdg_protocol(&api.core);
        // SAFETY: `proto` points to leaked static `WlInterface` tables.
        unsafe {
            assert_eq!(CStr::from_ptr((*proto.wm_base).name), c"xdg_wm_base");
            assert_eq!((*proto.wm_base).method_count, 4);
            assert_eq!((*proto.surface).event_count, 1);
            assert_eq!((*proto.toplevel).method_count, 14);
            assert_eq!((*proto.toplevel).event_count, 4);
            assert_eq!(
                CStr::from_ptr((*proto.decoration_manager).name),
                c"zxdg_decoration_manager_v1"
            );
        }
    }
}
