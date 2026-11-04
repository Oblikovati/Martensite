use super::*;
#[path = "../support/recording_device.rs"]
mod recording_device;

#[test]
fn grouped_display_updates_once_before_the_separate_fit_command() {
    use crate::case_view::SourceViewOutput;
    let device = recording_device::RecordingDevice::default();
    let driver =
        martensite_vulkan::Graphic3dRenderDriver::with_backend(device.clone(), 512).unwrap();
    let mut session = ViewerSession::with_graphic_driver(&driver.cast(), 64, 64).unwrap();
    session.set_auto_triangulation(false);
    let inputs = geometry::located_inputs().unwrap();
    let before = device.frames.read().len();
    show_vertices(&mut session, &inputs).unwrap();
    assert_eq!(device.frames.read().len() - before, 2);
    assert_eq!(device.last_frame().extent(), [64, 64]);
    session.close();
}

#[test]
fn case_named_vertices_and_shallow_locations_are_preserved() {
    let inputs = geometry::located_inputs().unwrap();
    assert!(inputs.original.is_same(&inputs.located_copies[0]));
    assert!(!inputs.original.is_same(&inputs.independent_copy));
    assert!(inputs.original.is_partner(&inputs.located_copies[1]));
    assert!(inputs.original.is_partner(&inputs.located_copies[2]));
    for (vertex, expected) in
        inputs
            .vertices
            .iter()
            .zip([[0., 0., 0.], [10., 0., 0.], [10., 10., 0.]])
    {
        let point = martensite_tkbrep::brep::BRepTool::pnt(vertex).unwrap();
        assert_eq!([point.x(), point.y(), point.z()], expected);
    }
}

#[test]
fn shared_compound_meshing_preserves_authored_unlocated_bounds() {
    let inputs = geometry::located_inputs().unwrap();
    mesh_shape(&inputs.compound, &MeshRequest::new(1.)).unwrap();
    let bounds = wire_bounds(&inputs.located_copies[0]).unwrap();
    for (actual, expected) in bounds.into_iter().zip([-1., -1., -1., 11., 11., 1.]) {
        assert!(
            (actual - expected).abs() <= 0.0001,
            "bound={actual}; expected {expected} within0.0001"
        );
    }
}
