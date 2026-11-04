use super::*;
use crate::case_view::SourceViewOutput;
use martensite_tkernel::osd::{SignalHost, SignalMode, SignalState};
use std::cell::Cell;
struct NamedSignalSetupHost {
    floating: Cell<bool>,
    mode: Cell<SignalMode>,
}
impl SignalHost for NamedSignalSetupHost {
    fn set_floating_signal(&self, enabled: bool) {
        self.floating.set(enabled);
    }
    fn to_catch_floating_signals(&self) -> bool {
        self.floating.get()
    }
    fn set_signal_handlers(&self, mode: SignalMode) {
        self.mode.set(mode);
    }
    fn debug_mode_requested(&self) -> bool {
        false
    }
}
#[test]
fn tiled_dump_retains_exact_requests_and_global_scene_mode() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    prepare(&mut session, &box_sized([1., 2., 3.]).unwrap()).unwrap();
    assert_eq!(session.context().display_mode(), 1);
    for (size, tile) in [(409, 0), (412, 409), (500, 300)] {
        let request = options(size, tile);
        assert_eq!(
            (request.width, request.height, request.tile_size),
            (size, size, tile)
        );
        assert_eq!(
            request.stereo_options,
            martensite_tkv3d::v3d::V3dStereoDumpOptions::Mono
        );
    }
    session.close();
}
#[test]
fn signal_command_disables_host_fpe_and_preserves_stack_length() {
    let state = SignalState::new();
    state.set_signal_stack_trace_length(7);
    let host = NamedSignalSetupHost {
        floating: Cell::new(true),
        mode: Cell::new(SignalMode::AsIs),
    };
    signals::disable_fpe(&state, &host);
    assert!(!host.floating.get());
    assert_eq!(host.mode.get(), SignalMode::Set);
    assert_eq!(state.signal_mode(), SignalMode::Set);
    assert_eq!(state.signal_stack_trace_length(), 7);
}
