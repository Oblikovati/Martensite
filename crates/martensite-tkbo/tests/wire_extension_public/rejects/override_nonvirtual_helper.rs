use martensite_tkbo::bop_algo::{ProgressCompletion, WireSplitter, WireSplitterHooks};
use martensite_tkernel::message::ProgressRange;

struct InvalidWireHelper(WireSplitter);

impl WireSplitterHooks for InvalidWireHelper {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        let receiver = &mut self.0;
        receiver
    }

    fn make_wires(&mut self, _: &ProgressRange<'_>) -> ProgressCompletion {
        let unsupported_override = ProgressCompletion::Completed;
        unsupported_override
    }
}

fn main() {
    let receiver = WireSplitter::new();
    std::hint::black_box(receiver);
}
