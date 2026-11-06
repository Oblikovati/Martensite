use martensite_tkbo::bop_algo::{WireOperationContext, WireSplitter};

fn main() {
    let receiver = WireSplitter::new();
    let operation = WireOperationContext::new();
    let _ = operation.bind_direct(&receiver);
}
