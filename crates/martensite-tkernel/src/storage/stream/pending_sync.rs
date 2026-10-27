//! The ordinary stream keeps its existing synchronization policy.
use super::super::StreamDevice;
use std::io::SeekFrom;

/// Preserve ordinary stream synchronization, including its historical error policy; `synchronize_legacy(bytes, cursor, device)`.
pub(super) fn synchronize_legacy(
    bytes: &mut Vec<u8>,
    cursor: &mut usize,
    device: &mut dyn StreamDevice,
) {
    let unread = (bytes.len() - *cursor) as i64;
    if unread > 0 {
        let _ = device.seek(SeekFrom::Current(-unread));
    }
    bytes.clear();
    *cursor = 0;
}

/// Preserve unread input on a borrowed-operation failure; `synchronize_operation(bytes, cursor, device, session, state)`.
pub(super) fn synchronize_operation(
    bytes: &mut Vec<u8>,
    cursor: &mut usize,
    device: &mut dyn StreamDevice,
    session: Option<&super::borrowed_session::BorrowedSession>,
    state: &mut u8,
) -> bool {
    let Some(session) = session else {
        synchronize_legacy(bytes, cursor, device);
        return true;
    };
    if !rewind_borrowed(bytes.len() - *cursor, device, session) {
        *state |= super::state::BAD;
        return false;
    }
    bytes.clear();
    *cursor = 0;
    true
}

fn rewind_borrowed(
    unread: usize,
    device: &mut dyn StreamDevice,
    session: &super::borrowed_session::BorrowedSession,
) -> bool {
    if unread == 0 {
        return true;
    }
    let requested = SeekFrom::Current(-(unread as i64));
    attempt_rewind(device, requested, session)
}

fn attempt_rewind(
    device: &mut dyn StreamDevice,
    requested: SeekFrom,
    session: &super::borrowed_session::BorrowedSession,
) -> bool {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| device.seek(requested))) {
        Ok(Ok(_)) => true,
        Ok(Err(error)) => {
            session.record(super::borrowed_failure::StreamRejoinCause::Seek { requested, error });
            false
        }
        Err(payload) => {
            session
                .record(super::borrowed_failure::StreamRejoinCause::SeekInterrupted { requested });
            std::panic::resume_unwind(payload)
        }
    }
}
