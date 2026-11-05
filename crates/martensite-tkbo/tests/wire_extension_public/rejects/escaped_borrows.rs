use martensite_tkbo::bop_algo::{
    WireEdgeSet, WireOperationContext, WirePreparedExtension, WireSplitter,
};

fn escape_prepared_wire() -> WirePreparedExtension<'static, 'static> {
    let mut receiver = WireSplitter::new();
    receiver.set_wes(WireEdgeSet::new());
    let mut operation = WireOperationContext::new();
    receiver.prepare_wires_extension(&mut operation).unwrap()
}

fn main() {
    let escaped = escape_prepared_wire();
    std::hint::black_box(escaped);
}
