//! Preserve native warning-before-error text and one message per actual alert type.
use martensite_tkernel::message::{Gravity, Msg, Report};

/// Formats one gravity without suppressing alerts; `gravity_alert_text(report, Gravity::Warning, "Warning: ")`.
pub fn gravity_alert_text(report: &Report, gravity: Gravity, prefix: &str) -> String {
    let mut seen = Vec::new();
    let mut output = String::new();
    for alert in report.get_alerts(gravity) {
        let kind = alert.dynamic_type().name();
        if seen.contains(&kind) {
            continue;
        }
        seen.push(kind);
        let mut message = Msg::from_key(&alert.get_message_key());
        output.push_str(&format!("{prefix}{}\n", message.get().unwrap().to_utf8()));
    }
    output
}
