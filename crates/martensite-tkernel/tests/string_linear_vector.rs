//! Heap-backed string moves during linear-vector erasure.
#![allow(non_snake_case)]
use crate::ncollection::LinearVector;
use crate::tcollection::AsciiString;

#[test]
fn NCollection_LinearVectorTest__EraseSingle_NonTrivial_HeapBuffers() {
    let mut vector = LinearVector::default();
    for prefix in ["alpha", "beta", "gamma", "delta"] {
        vector.append(AsciiString::new(&format!(
            "{prefix}-string-forces-heap-allocation"
        )));
    }
    vector.erase_with_move(1, AsciiString::move_assign).unwrap();
    assert_eq!(vector.len(), 3);
    for (index, prefix) in ["alpha", "gamma", "delta"].into_iter().enumerate() {
        assert_eq!(
            vector[index].as_bytes(),
            format!("{prefix}-string-forces-heap-allocation").as_bytes()
        );
    }
    vector.erase_with_move(0, AsciiString::move_assign).unwrap();
    assert_eq!(vector.len(), 2);
    for (index, prefix) in ["gamma", "delta"].into_iter().enumerate() {
        assert_eq!(
            vector[index].as_bytes(),
            format!("{prefix}-string-forces-heap-allocation").as_bytes()
        );
    }
}

#[test]
fn NCollection_LinearVectorTest__EraseRange_NonTrivial_HeapBuffers() {
    let mut vector = LinearVector::default();
    for index in 0..6 {
        let mut entry = AsciiString::new("payload-with-heap-allocation-");
        entry.append(&AsciiString::from_integer(index));
        vector.append(entry);
    }
    vector
        .erase_range_with_move(1, 4, AsciiString::move_assign)
        .unwrap();
    assert_eq!(vector.len(), 3);
    for (index, value) in [0, 4, 5].into_iter().enumerate() {
        assert_eq!(
            vector[index].as_bytes(),
            format!("payload-with-heap-allocation-{value}").as_bytes()
        );
    }
}
