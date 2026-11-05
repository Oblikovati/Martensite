use martensite_tkbo::bop_algo::{WireEdgeSet, WireOperationContext, WireSplitter};
use martensite_tkernel::message::ProgressRange;

fn main() {
    let mut receiver = WireSplitter::new();
    receiver.set_wes(WireEdgeSet::new());
    let mut operation = WireOperationContext::new();
    let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
    std::hint::black_box(&operation);
    std::hint::black_box(prepared.make_wires(&ProgressRange::default()));
}
