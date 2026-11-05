//! Preserve reachable input geometry/topology bytes; an unchanged handle alone is insufficient.
use martensite_tkbrep::{brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};

/// Capture each authored input without altering it; `shapes(builder.shapes().iter())`.
pub(super) fn shapes<'a>(inputs: impl Iterator<Item = &'a Shape>) -> Vec<Vec<u8>> {
    // Serialize reachable topology and geometry rather than relying on handle identity.
    inputs.map(serialized_shape).collect()
}

fn serialized_shape(shape: &Shape) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    brep_tools::write(shape, &mut stream, &ProgressRange::default()).unwrap();
    let mut bytes = vec![0; stream.tellp() as usize];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

/// Identify the first reachable byte change; `assert_unchanged("two", &before, &after)`.
pub(super) fn assert_unchanged(recipe: &str, before: &[Vec<u8>], after: &[Vec<u8>]) {
    assert_eq!(
        before.len(),
        after.len(),
        "recipe={recipe}; input count changed"
    );
    for (index, (left, right)) in before.iter().zip(after).enumerate() {
        if left == right {
            continue;
        }
        let first = left.iter().zip(right).position(|(a, b)| a != b);
        panic!(
            "recipe={recipe}; input={index}; first_changed_byte={first:?}; before={}; after={}",
            String::from_utf8_lossy(left),
            String::from_utf8_lossy(right)
        );
    }
}
