use super::support::FakeResourceSystem;
use std::{path::PathBuf, sync::Arc};

pub(super) fn system_with(name: &str, content: &[u8]) -> Arc<FakeResourceSystem> {
    let system: Arc<FakeResourceSystem> = Arc::default();
    system.set_environment(&format!("CSF_{name}Defaults"), PathBuf::from("defaults"));
    let path = PathBuf::from(format!("defaults/{name}"));
    system.files.lock().unwrap().insert(path, content.to_vec());
    system
}
pub(super) fn console(system: &FakeResourceSystem) -> String {
    String::from_utf8(system.console.lock().unwrap().clone()).unwrap()
}
