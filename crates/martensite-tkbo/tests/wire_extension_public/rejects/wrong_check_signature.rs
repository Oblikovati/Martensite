use martensite_tkbo::bop_algo::{WireOperationContext, WireSplitter, WireSplitterHooks};

struct InvalidWireCheck(WireSplitter);

impl WireSplitterHooks for InvalidWireCheck {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        let receiver = &mut self.0;
        receiver
    }

    fn check_data(&mut self, _: &WireOperationContext) -> bool {
        let incorrect = true;
        incorrect
    }
}

fn main() {
    let receiver = WireSplitter::new();
    std::hint::black_box(receiver);
}
