use super::*;
#[test]
fn source_inventory_rejects_repeated_authored_span_before_gamma_generation() {
    let request = crate::surface_sketch::wrapped_profile::operation::cone::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = Work::new(request.proof.arithmetic);
    let mut selection =
        prepare::select(&request, &inputs.originals, &mut work, &mut || false).unwrap();
    let duplicate = selection.prepared[0].source.loops[0].clone();
    selection.prepared[0].source.loops.push(duplicate);
    let consumed = work.consumed;
    let failure = Inventory::capture(&selection.prepared).err().unwrap();
    assert_eq!(failure.region, Some(inputs.originals[0].region));
    assert!(failure.span.is_some());
    assert!(
        matches!(failure.cause, ConeCause::Binding { field: "unique directed side source", actual, expected } if actual == [2] && expected == [1])
    );
    assert_eq!(work.consumed, consumed);
}
