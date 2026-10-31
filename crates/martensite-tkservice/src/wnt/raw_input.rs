//! Owned HID reports and bounded delivery diagnostics for WM_INPUT (#334).
use martensite_tkernel::standard::Failure;

/// One native HID report; `report.bytes` can be borrowed by `WntHidSpaceMouse`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WntRawHidReport {
    /// Native device identifier, meaningful while Windows keeps that device connected.
    pub device: usize,
    /// USB vendor identifier.
    pub vendor_id: u32,
    /// USB product identifier.
    pub product_id: u32,
    /// One complete HID report, including its report id.
    pub bytes: Vec<u8>,
}

/// Raw input drained from an owned window; inspect reports, errors and dropped packets.
#[derive(Default, Debug)]
pub struct WntRawInputBatch {
    /// Captured reports in native message order.
    pub reports: Vec<WntRawHidReport>,
    /// Native acquisition or packet-validation failures.
    pub errors: Vec<Failure>,
    /// Packets refused because the undrained queue reached its limit.
    pub dropped_packets: u64,
}

#[cfg(windows)]
impl WntRawInputBatch {
    pub(super) fn push(&mut self, packet: Result<Vec<WntRawHidReport>, Failure>) {
        let bytes: usize = self.reports.iter().map(|report| report.bytes.len()).sum();
        let incoming = packet.as_ref().map_or(0, |reports| {
            reports.iter().map(|report| report.bytes.len()).sum()
        });
        let count = packet.as_ref().map_or(1, Vec::len);
        if self.reports.len() + self.errors.len() + count > 4096
            || bytes + incoming > 16 * 1024 * 1024
        {
            self.dropped_packets = self.dropped_packets.saturating_add(1);
            return;
        }
        match packet {
            Ok(reports) => self.reports.extend(reports),
            Err(error) => self.errors.push(error),
        }
    }
}

#[cfg(windows)]
#[path = "raw_input_native.rs"]
mod native;
#[cfg(windows)]
pub(super) use native::{read, register};

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn report_queue_reports_overflow_and_can_be_drained() {
        let mut batch = WntRawInputBatch::default();
        for _ in 0..4097 {
            batch.push(Ok(vec![WntRawHidReport {
                device: 1,
                vendor_id: 2,
                product_id: 3,
                bytes: vec![1],
            }]));
        }
        assert_eq!(batch.reports.len(), 4096);
        assert_eq!(batch.dropped_packets, 1);
        let taken = std::mem::take(&mut batch);
        assert_eq!(taken.reports.len(), 4096);
        assert!(batch.reports.is_empty());
    }
    #[test]
    fn owned_window_registers_raw_input_and_drains_empty_queue() {
        let window = crate::wnt::WntWindow::new("Input", 0, 0, 100, 100).unwrap();
        window.enable_raw_input().unwrap();
        assert!(window.take_raw_input().reports.is_empty());
    }
}
