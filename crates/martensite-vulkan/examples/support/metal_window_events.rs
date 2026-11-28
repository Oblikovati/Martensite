#![allow(unsafe_code)] // Reads one AppKit constant; see the SAFETY comment.
//! Example-owned AppKit event pump: the application, activation and one event batch.
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSEventMask};
use objc2_foundation::{NSDate, NSDefaultRunLoopMode};
use std::time::Duration;

/// The shared application, activated as a regular app so its window comes to the front.
pub fn start_application() -> Retained<NSApplication> {
    let mtm = MainThreadMarker::new().expect("an example's main runs on the main thread");
    let application = NSApplication::sharedApplication(mtm);
    application.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    application.finishLaunching();
    #[allow(deprecated)] // activate() needs macOS 14; this example also runs on older hosts.
    application.activateIgnoringOtherApps(true);
    application
}

/// Dispatches the pending events, waiting at most `budget` for the first one.
pub fn pump_events(budget: Duration) {
    let mtm = MainThreadMarker::new().expect("events are pumped on the main thread");
    let application = NSApplication::sharedApplication(mtm);
    let mut until = NSDate::dateWithTimeIntervalSinceNow(budget.as_secs_f64());
    loop {
        // SAFETY: NSDefaultRunLoopMode is an immutable AppKit constant string.
        let mode = unsafe { NSDefaultRunLoopMode };
        let event = application.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        );
        let Some(event) = event else { break };
        application.sendEvent(&event);
        until = NSDate::distantPast();
    }
    application.updateWindows();
}
