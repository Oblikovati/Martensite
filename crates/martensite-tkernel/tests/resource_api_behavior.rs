//! Behavior of every resource member the legacy suites do not reach.
#[path = "resource_api_behavior/fixtures.rs"]
mod fixtures;
#[path = "resource_api_behavior/manager.rs"]
mod manager;
#[path = "resource_manager/support.rs"]
#[allow(dead_code)]
mod support;

/// The ResourceDebug flag is process-wide, so manager tests run one at a time.
fn serial() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL.lock().unwrap_or_else(|poison| poison.into_inner())
}
