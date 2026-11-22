//! Tests of resource helpers with injected storage.
#[path = "../../tests/resource_api_behavior/fixtures.rs"]
mod fixtures;
#[path = "access_tests/manager.rs"]
mod manager;
#[path = "access_tests/save.rs"]
mod save;
mod support {
    use crate as martensite_tkernel;
    include!("../../tests/resource_manager/support.rs");
}
#[path = "access_tests/unicode.rs"]
mod unicode;
