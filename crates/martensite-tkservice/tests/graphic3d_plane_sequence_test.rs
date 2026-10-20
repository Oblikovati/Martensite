//! Graphic3d_SequenceOfHClipPlane, Graphic3d_ViewAffinity and the Graphic3d exceptions.
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkservice::graphic3d::{
    Graphic3dClipPlane, Graphic3dSequenceOfHClipPlane, Graphic3dSequenceOfHClipPlaneIterator,
    Graphic3dViewAffinity, group_definition_error, priority_definition_error,
    structure_definition_error,
};

fn planes(count: usize) -> Vec<Handle<Graphic3dClipPlane>> {
    (0..count)
        .map(|_| Handle::new(Graphic3dClipPlane::default()))
        .collect()
}

#[test]
fn append_keeps_each_plane_once() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    let [a, b] = <[_; 2]>::try_from(planes(2)).unwrap();
    assert!(sequence.append(a.clone()));
    assert!(!sequence.append(a.clone()));
    assert!(sequence.append(b.clone()));
    assert_eq!(sequence.size(), 2);
    assert_eq!(sequence.first().unwrap(), a);
    assert!(sequence.remove(&a));
    assert!(!sequence.remove(&a));
    assert_eq!(sequence.first().unwrap(), b);
}

#[test]
fn first_of_empty_sequence_raises() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    assert!(sequence.is_empty());
    let error = sequence.first().unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NoSuchObject");
}

#[test]
fn iterator_remove_moves_to_the_next_plane() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    let all = planes(3);
    all.iter().for_each(|plane| {
        sequence.append(plane.clone());
    });
    let mut it = Graphic3dSequenceOfHClipPlaneIterator::new(&sequence);
    it.next();
    sequence.remove_at(&mut it).unwrap();
    assert_eq!(it.value(), all[2]);
    assert_eq!(sequence.items(), vec![all[0].clone(), all[2].clone()]);
    it.next();
    assert!(!it.more());
    sequence.remove_at(&mut it).unwrap();
    let null = Graphic3dSequenceOfHClipPlaneIterator::from_handle(&Handle::<
        Graphic3dSequenceOfHClipPlane,
    >::default());
    assert!(!null.more());
    sequence.clear();
    assert!(sequence.is_empty());
}

/// `Iterator::Init` restarts at the sequence's current first node, not at a copy taken when
/// the iterator was made; the handle form with a null handle empties the iterator.
#[test]
fn init_restarts_at_the_current_first_plane() {
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    let all = planes(3);
    sequence.append(all[0].clone());
    let mut it = Graphic3dSequenceOfHClipPlaneIterator::new(&sequence);
    it.next();
    assert!(!it.more());
    sequence.append(all[1].clone());
    sequence.append(all[2].clone());
    assert!(sequence.remove(&all[0]));
    it.init(&sequence);
    assert_eq!(it.value(), all[1]);
    let before = all[1].ref_count();
    it.init_handle(&sequence);
    assert_eq!(it.value(), all[1]);
    assert_eq!(all[1].ref_count(), before);
    it.next();
    assert_eq!(it.value(), all[2]);
    it.init_handle(&Handle::<Graphic3dSequenceOfHClipPlane>::default());
    assert!(!it.more());
}

#[test]
fn removing_a_default_or_null_initialized_iterator_is_a_native_noop() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    let plane = planes(1).pop().unwrap();
    sequence.append(plane.clone());
    let mut iterator = Graphic3dSequenceOfHClipPlaneIterator::default();
    sequence.remove_at(&mut iterator).unwrap();
    iterator.init_handle(&Handle::<Graphic3dSequenceOfHClipPlane>::default());
    sequence.remove_at(&mut iterator).unwrap();
    assert_eq!(sequence.size(), 1);
    assert_eq!(plane.ref_count(), 2);
    assert!(!iterator.more());
    assert_native_end_cursor(&sequence, &iterator);
}

fn assert_native_end_cursor(
    sequence: &Graphic3dSequenceOfHClipPlane,
    iterator: &Graphic3dSequenceOfHClipPlaneIterator,
) {
    let actual = format!(
        "remove-end {} {}",
        i32::from(iterator.more()),
        sequence.size()
    );
    let expected = include_str!("fixtures/clip-collection-references-native.txt");
    assert_eq!(actual, expected.lines().last().unwrap());
}

#[test]
fn dump_lists_the_flag_then_the_planes() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    sequence.set_override_global(true);
    assert!(sequence.to_override_global());
    sequence.append(planes(1).remove(0));
    let mut stream = DumpStream::default();
    sequence.dump_json(&mut stream, -1).unwrap();
    let text = String::from_utf8(stream.bytes().to_vec()).unwrap();
    assert!(text.starts_with("\"ToOverrideGlobal\": 1"), "{text}");
    assert!(
        text.contains("\"className\": \"Graphic3d_ClipPlane\""),
        "{text}"
    );
}

#[test]
fn view_affinity_masks_views() {
    let affinity = Graphic3dViewAffinity::default();
    assert!(affinity.is_visible(0) && affinity.is_visible(31));
    affinity.set_view_visible(3, false);
    assert!(!affinity.is_visible(3) && affinity.is_visible(2));
    affinity.set_visible(false);
    assert!(!affinity.is_visible(0));
    affinity.set_view_visible(31, true);
    assert!(affinity.is_visible(31));
    let mut stream = DumpStream::default();
    affinity.dump_json(&mut stream, -1).unwrap();
    let text = String::from_utf8(stream.bytes().to_vec()).unwrap();
    assert!(text.ends_with("\"Mask\": 2147483648"), "{text}");
}

#[test]
fn exceptions_keep_their_class_names() {
    assert_eq!(
        group_definition_error("x").exception_type(),
        "Graphic3d_GroupDefinitionError"
    );
    assert_eq!(
        priority_definition_error("x").exception_type(),
        "Graphic3d_PriorityDefinitionError"
    );
    assert_eq!(
        structure_definition_error("x").exception_type(),
        "Graphic3d_StructureDefinitionError"
    );
}
