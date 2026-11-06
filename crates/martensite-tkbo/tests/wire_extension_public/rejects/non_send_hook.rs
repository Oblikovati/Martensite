use martensite_tkbo::bop_algo::{WireSplitter, WireSplitterHooks};
use std::rc::Rc;

struct ConfinedWireHooks {
    receiver: WireSplitter,
    confined: Rc<()>,
}

impl WireSplitterHooks for ConfinedWireHooks {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        let receiver = &mut self.receiver;
        receiver
    }
}

fn main() {
    let receiver = WireSplitter::new();
    std::hint::black_box(receiver);
}
