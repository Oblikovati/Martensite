use martensite_tkbo::bop_algo::WireOperationContext;

fn main() {
    let operation = WireOperationContext::from_parts();
    std::hint::black_box(operation);
}
