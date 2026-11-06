use martensite_tkbo::bop_algo::WireSplitter;
use martensite_tkernel::message::ProgressRange;

fn main() {
    let mut receiver = WireSplitter::new();
    let _ = receiver.make_wires(&ProgressRange::default(), &());
}
