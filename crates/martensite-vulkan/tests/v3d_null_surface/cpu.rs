//! Actual fixture properties and Null presentations cannot be satisfied by a replacement box.
use super::commands;
use crate::{
    case_view::{SourceViewOutput, viewer_session::ViewerSession},
    dataset_shape::read_dataset_shape,
    property_queries::{Kind, properties},
};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
#[test]
fn null_surface_fixture_queries_and_presentation_do_not_crash() {
    let shape = read_dataset_shape("bug27821_nullsurf.brep").unwrap();
    let faces = crate::boolean_typed_explode::all_distinct(&shape, ShapeEnum::Face);
    assert!(faces.iter().any(|f| {
        BRepTool::surface_with_location(&topods::face(f).unwrap())
            .unwrap()
            .0
            .is_null()
    }));
    properties(&shape, Kind::Area, false).unwrap();
    properties(&shape, Kind::Volume, false).unwrap();
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    commands::presentation(&mut session, &shape, &faces).unwrap();
    session.close();
}
