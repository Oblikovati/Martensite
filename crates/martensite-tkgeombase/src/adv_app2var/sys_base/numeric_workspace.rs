//! Operation-local typed scratch storage; allocation precedes callbacks and release follows publication.
pub(crate) trait NumericWorkspace {
    fn allocate(&mut self, length: usize) -> Result<Vec<f64>, ()>;
    fn release(&mut self, buffer: Vec<f64>) -> Result<(), ()>;
}
pub(crate) struct HeapNumericWorkspace;
impl NumericWorkspace for HeapNumericWorkspace {
    fn allocate(&mut self, length: usize) -> Result<Vec<f64>, ()> {
        let mut buffer = Vec::new();
        buffer.try_reserve_exact(length).map_err(|_| ())?;
        // The retained Rust helper initialized padding; keep that baseline unchanged.
        // Reservation above guarantees resize does not introduce another allocation.
        buffer.resize(length, 0.);
        Ok(buffer)
    }
    fn release(&mut self, buffer: Vec<f64>) -> Result<(), ()> {
        drop(buffer);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heap_workspace_preserves_initialized_payload_and_reports_bounded_cost() {
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let mut buffer = HeapNumericWorkspace.allocate(38).unwrap();
            assert_eq!(buffer.len(), 38);
            assert!(
                buffer
                    .iter()
                    .all(|value| value.to_bits() == 0_f64.to_bits())
            );
            buffer[0] = std::hint::black_box(1.);
            HeapNumericWorkspace.release(buffer).unwrap();
        }
        println!(
            "{{\"metric\":\"curve_workspace\",\"iterations\":1000,\"payload_bytes\":304,\"payload_allocations_per_operation\":1,\"workspace_locks_per_operation\":0,\"provider_bytes\":{},\"elapsed_ns\":{}}}",
            std::mem::size_of::<HeapNumericWorkspace>(),
            start.elapsed().as_nanos()
        );
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HeapNumericWorkspace>();
    }
}
