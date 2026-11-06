//! The default axis of a view is read back as it was set, with every digit.
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::v3d::{V3dView, V3dViewSource, V3dViewer};

fn viewer() -> Handle<V3dViewer> {
    V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap()
}

/// An axis no six significant digits can tell from its neighbours.
fn precise_axis() -> Axis1 {
    Axis1::new(
        Point3::new(1.234_567_890_123, -2.345_678_901_234, 3.456_789_012_345),
        Direction3::new(0.123_456_789_012, 0.234_567_890_123, 0.345_678_901_234).unwrap(),
    )
}

fn set_precise_axis(view: &Handle<dyn V3dViewSource>) {
    let axis = precise_axis();
    let (point, direction) = (axis.location(), axis.direction());
    view.view_base()
        .set_axis(
            point.x(),
            point.y(),
            point.z(),
            direction.x(),
            direction.y(),
            direction.z(),
        )
        .unwrap();
}

#[test]
fn the_default_axis_reads_back_with_every_digit() {
    let view = viewer().create_view().unwrap();
    set_precise_axis(&view);
    assert_eq!(view.view_base().default_view_axis(), precise_axis());
}

#[test]
fn a_copied_view_has_the_default_axis_of_its_source() {
    let viewer = viewer();
    let source = viewer.create_view().unwrap();
    set_precise_axis(&source);
    let copy = V3dView::new_copy(&viewer, &source).unwrap();
    assert_eq!(copy.view_base().default_view_axis(), precise_axis());
}
