//! The retained native const-list is live and callbacks borrow each actual handle slot.
#[path = "stored_filters_live/lifetime.rs"]
mod lifetime;
#[path = "stored_filters_live/observations.rs"]
mod observations;
#[path = "stored_filters_live/probe.rs"]
mod probe;

#[test]
fn live_alias_and_callbacks_match_defined_native_list_records() {
    let actual = observations::native_records();
    println!("{actual}");
    assert_eq!(actual, native_content_projection());
}

fn native_content_projection() -> String {
    // Native list-address equality has no borrowed Rust pointer equivalent; compare actual contents/counts.
    let native = include_str!("fixtures/composition-filter-live/native.txt");
    native
        .lines()
        .map(|line| {
            if line.starts_with("alias ") {
                line.rsplit_once(' ').unwrap().0
            } else {
                line
            }
        })
        .map(|line| format!("{line}\n"))
        .collect()
}

#[path = "stored_filters_live/context.rs"]
mod context;
#[allow(dead_code)] // Reuse the named null driver; this host observes filter forwarding only.
#[path = "support/sensitive_display_driver.rs"]
mod display;
#[path = "stored_filters_live/failure_concurrency.rs"]
mod failure_concurrency;

#[path = "stored_filters_live/clearing/mod.rs"]
mod clearing;

#[path = "stored_filters_live/mode_changes.rs"]
mod mode_changes;
