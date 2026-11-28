//! macOS AppKit: as in OCCT's native macOS build (no Xlib), the connection only stores the
//! given display; AppKit windows reach the window server without a socket. A zero display
//! becomes the non-null marker 1, as on the browser, so the connection reads as connected.
use super::{AspectDisplayBackend, ConnectionState, NativeDisplayCalls};
use martensite_tkernel::standard::Failure;

/// Stores `display` (or the marker) under the AppKit backend; never fails.
pub(super) fn init(
    state: &mut ConnectionState,
    display: usize,
    _native: &dyn NativeDisplayCalls,
) -> Result<(), Failure> {
    state.display = if display != 0 { display } else { 1 };
    state.backend = AspectDisplayBackend::AppKit;
    state.is_own_display = false;
    state.atoms.clear();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::{AspectDisplayBackend, AspectDisplayConnection, AspectXAtom};

    #[test]
    fn appkit_connection_stores_a_marker_without_x_atoms() {
        let connection = AspectDisplayConnection::new().expect("AppKit needs no socket");
        assert_eq!(connection.backend(), AspectDisplayBackend::AppKit);
        assert_eq!(connection.get_display_aspect(), 1);
        assert!(!connection.is_own_display());
        assert!(connection.get_atom(AspectXAtom::DeleteWindow).is_err());
    }

    #[test]
    fn appkit_connection_keeps_an_explicit_display_borrowed() {
        let connection =
            AspectDisplayConnection::with_display(0x5000).expect("any address is kept");
        assert_eq!(connection.get_display_aspect(), 0x5000);
        assert!(!connection.is_own_display());
    }
}
