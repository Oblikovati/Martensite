use super::*;

#[test]
fn strip_header_skips_only_the_drawable_type_word() {
    assert_eq!(strip_header(b"DBRep_DrawableShape\nCASCADE"), b"\nCASCADE");
    assert_eq!(strip_header(b"CASCADE Topology"), b"CASCADE Topology");
}

#[test]
fn geometry_record_follows_only_a_drawable_geometry_header() {
    assert_eq!(
        geometry_record(b"DrawTrSurf_Surface\n5 1 2"),
        Some(&b"5 1 2"[..])
    );
    assert_eq!(geometry_record(b"DBRep_DrawableShape\n5"), None);
}

#[test]
fn relative_names_are_found_below_a_dataset_folder() {
    let path = locate_dataset_file("occ/Top.brep").unwrap();
    assert!(
        path.ends_with("occt-source-data/occ/Top.brep"),
        "found {}",
        path.display()
    );
    assert!(locate_dataset_file("occ/no-such-input.brep").is_err());
}

#[test]
fn missing_dataset_file_names_the_file() {
    let failure = locate_dataset_file("no-such-input.brep").unwrap_err();
    assert!(format!("{failure:?}").contains("no-such-input.brep"));
}
