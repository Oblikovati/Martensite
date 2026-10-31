//! Concurrent field traffic straddles owner retirement with deterministic handoffs.
use martensite_tkservice::graphic3d::{CaptureBufferKindRef, Graphic3dBufferType};
use std::sync::{Arc, Barrier};
use std::thread;

#[derive(Clone)]
struct BufferKindTraffic {
    field: CaptureBufferKindRef,
    epochs: Arc<Barrier>,
}
impl BufferKindTraffic {
    fn new(field: CaptureBufferKindRef) -> Self {
        Self {
            field,
            epochs: Arc::new(Barrier::new(3)),
        }
    }
    fn worker(&self, assigned: Graphic3dBufferType) {
        let before = self.contend(assigned, 512);
        self.epochs.wait();
        self.epochs.wait();
        let after = self.contend(assigned, 4096);
        self.epochs.wait();
        self.epochs.wait();
        assert_eq!(
            before, None,
            "pre-retirement expected only active writer values"
        );
        assert_eq!(
            after, None,
            "post-retirement expected only active writer values"
        );
        assert_eq!(self.field.read(), Graphic3dBufferType::Depth);
    }
    fn contend(
        &self,
        assigned: Graphic3dBufferType,
        rounds: usize,
    ) -> Option<(usize, Graphic3dBufferType)> {
        // Report after every handoff so a failed observation cannot strand a peer.
        let mut failure = None;
        for round in 0..rounds {
            self.field.set(assigned);
            let observed = self.field.read();
            if !matches!(
                observed,
                Graphic3dBufferType::Rgba | Graphic3dBufferType::Red
            ) {
                failure = failure.or(Some((round, observed)));
            }
        }
        failure
    }
    fn retire_and_handoff(&self, original: CaptureBufferKindRef) {
        // Both workers have used the same field and remain alive across retirement.
        self.epochs.wait();
        drop(original);
        self.epochs.wait();
        self.epochs.wait();
        self.field.set(Graphic3dBufferType::Depth);
        self.epochs.wait();
    }
}
#[test]
fn contended_aliases_survive_owner_retirement_and_final_handoff() {
    let owner = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);
    let copied = CaptureBufferKindRef::new(owner.read());
    let traffic = BufferKindTraffic::new(owner.clone());
    let left = traffic.clone();
    let right = traffic.clone();
    let first = thread::spawn(move || left.worker(Graphic3dBufferType::Rgba));
    let second = thread::spawn(move || right.worker(Graphic3dBufferType::Red));
    traffic.retire_and_handoff(owner);
    first.join().unwrap();
    second.join().unwrap();
    assert_eq!(traffic.field.read(), Graphic3dBufferType::Depth);
    assert_eq!(copied.read(), Graphic3dBufferType::Rgb);
    assert!(!copied.same_slot(&traffic.field));
}
