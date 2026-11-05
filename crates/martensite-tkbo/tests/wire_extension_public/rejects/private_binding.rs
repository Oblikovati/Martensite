use martensite_tkbo::bop_algo::WireOperationContext;

fn main() {
    let operation = WireOperationContext::new();
    std::hint::black_box(operation.receiver_address);
}
