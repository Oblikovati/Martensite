//! StdSelectBRepOwner::DumpJson and StdSelectShape::DumpJson of a null shape at depths -1, 1, 0.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObject;
use martensite_tkv3d::select_mgr::SelectMgrEntityOwnerSource;
use martensite_tkv3d::std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers, StdSelectShape};

fn owner_dump(depth: i32) -> String {
    let owner = StdSelectBRepOwner::with_shape(&Shape::default(), 5, false);
    owner.set_hilight_mode(3);
    let mut stream = DumpStream::default();
    owner.dump_json(&mut stream, depth).unwrap();
    String::from_utf8(stream.bytes().to_vec()).unwrap()
}

#[test]
fn brep_owner_dump_matches_native_text() {
    assert_eq!(
        owner_dump(-1),
        "\"className\": \"StdSelectBRepOwner\", \"Shape\": {\"className\": \"TopoDS_Shape\", \
         \"Location\": {\"className\": \"TopLoc_Location\", \"Transformation\": {\"Location\": \
         [0, 0, 0], \"Matrix\": [1, 0, 0, 0, 1, 0, 0, 0, 1], \"shape\": 0, \"scale\": 1}, \
         \"IsIdentity\": 1}, \"Orient\": 3}, \"CurMode\": 3"
    );
    assert_eq!(
        owner_dump(1),
        "\"className\": \"StdSelectBRepOwner\", \"Shape\": {\"className\": \"TopoDS_Shape\", \
         \"Orient\": 3}, \"CurMode\": 3"
    );
    assert_eq!(
        owner_dump(0),
        "\"className\": \"StdSelectBRepOwner\", \"CurMode\": 3"
    );
}

#[test]
fn shape_dump_matches_native_text() {
    let shape = StdSelectShape::new(&Shape::default(), &Handle::default());
    let mut stream = DumpStream::default();
    shape.dump_json(&mut stream, 1).unwrap();
    assert_eq!(
        String::from_utf8(stream.bytes().to_vec()).unwrap(),
        "\"className\": \"StdSelectShape\", \"className\": \"PrsMgrPresentableObject\", \
         \"Parent\": \"\", \"TypeOfPresentation3d\": 0, \"CurrentFacingModel\": 0, \"OwnWidth\": 0, \
         \"hasOwnColor\": 0, \"hasOwnMaterial\": 0, \"InfiniteState\": 0, \"IsMutable\": 0, \
         \"HasOwnPresentations\": 1, \"ToPropagateVisualState\": 1, \"sh\": {\"className\": \
         \"TopoDS_Shape\", \"Orient\": 3}"
    );
}
