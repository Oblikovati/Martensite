//! Actual context owners and topological identities are the selected-state reference.
use super::{cpu_scene::*, prelude::*, selected_state::*};
use crate::primitive_shapes::{compound_of, translated};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::std_select::StdSelectBRepOwner;

#[test]
fn clipping_selected_face_rank_uses_real_owner_and_refuses_another_face_or_global_owner() {
    let (mut session, _) = session();
    let shape = box_sized([1.; 3]).unwrap();
    session
        .display("b", &shape, &DisplayRequest::default())
        .unwrap();
    let faces: Vec<_> = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let object = session.object("b").unwrap();
    assert_selected_line(&session, &faces[0], "Face #1");
    assert_selected_line(&session, &faces[1], "Face #2");
    session
        .context()
        .set_selected_owner(&object.global_sel_owner(), false)
        .unwrap();
    assert!(local_selected_faces(session.context()).unwrap().is_empty());
}

#[test]
fn clipping_explorer_duplicate_identity_and_location_have_native_rank_semantics() {
    let shape = box_sized([1.; 3]).unwrap();
    let faces: Vec<_> = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let repeated = compound_of(&[shape.clone(), shape]).unwrap();
    assert_eq!(occurrence_rank(&repeated, &faces[1]).unwrap(), Some(2));
    let moved = translated(&faces[0], [100.; 3]).unwrap();
    assert_eq!(occurrence_rank(&repeated, &moved).unwrap(), None);
    let parent = translated(&repeated, [100.; 3]).unwrap();
    assert_eq!(occurrence_rank(&parent, &moved).unwrap(), None);
    let actual = Explorer::new(&parent, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .current()
        .clone();
    assert_eq!(occurrence_rank(&parent, &actual).unwrap(), Some(1));
}

fn select_face(session: &ViewerSession, face: &martensite_tkbrep::topods::Shape) {
    let object = session.object("b").unwrap();
    let owner = StdSelectBRepOwner::with_selectable(face, &object.cast(), 7, true);
    session
        .context()
        .set_selected_owner(&owner.cast(), false)
        .unwrap();
}

fn assert_selected_line(
    session: &ViewerSession,
    face: &martensite_tkbrep::topods::Shape,
    expected: &str,
) {
    select_face(session, face);
    let actual = local_selected_faces(session.context()).unwrap();
    assert_eq!(actual, [expected]);
}
