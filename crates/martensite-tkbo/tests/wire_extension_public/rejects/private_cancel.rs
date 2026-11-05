use martensite_tkbo::bop_algo::ProgressCancellation;

fn main() {
    let cancellation = ProgressCancellation::default();
    std::hint::black_box(cancellation);
}
