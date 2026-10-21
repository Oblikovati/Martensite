//! Independent message and callback-state contracts of whole-shape conversion.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Builder, Compound, Shape},
};
use martensite_tkernel::{
    impl_transient,
    message::{Gravity, Msg},
    standard::{Handle, SharedState},
};
use martensite_tkg3d::{geom::GeomBezierSurface, top_abs::ShapeEnum};
use martensite_tkmath::gp::Plane3;
use martensite_tkshhealing::{
    shape_extend::{BasicMsgRegistrator, ShapeExtendBasicMsgRegistrator},
    shape_upgrade::{ShapeDivider, ShapeUpgradeShapeConvertToBezier},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::{Arc, Weak};

#[derive(Default)]
struct ConversionMessages {
    parent: ShapeExtendBasicMsgRegistrator,
    received: SharedState<Vec<String>>,
    converter: Weak<ShapeUpgradeShapeConvertToBezier>,
}
impl_transient!(ConversionMessages, "ConversionMessages", ShapeExtendBasicMsgRegistrator,
    parent, interfaces: [dyn BasicMsgRegistrator]);
impl BasicMsgRegistrator for ConversionMessages {
    fn send_shape(&self, _shape: &Shape, message: &Msg, gravity: Gravity) {
        assert_eq!(gravity, Gravity::Info);
        self.received.write().push(message.original().to_utf8());
        if let Some(converter) = self.converter.upgrade() {
            converter.set_3d_line_conversion(false);
            converter.set_surface_conversion(false);
        }
    }
}

fn plane() -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn assert_face_conversion_message(messages: &[String]) {
    assert_eq!(messages.len(), 1);
    // The process catalog may be initialized by another independent suite test.
    let expected = [
        "Face converted to Bezier",
        "Unknown message invoked with the keyword ShapeDivide.FaceConvertToBezier.MSG0",
    ];
    assert!(expected.contains(&messages[0].as_str()), "{messages:?}");
}

fn converted_face_messages(source: Shape) -> Vec<String> {
    let converter = ShapeUpgradeShapeConvertToBezier::new(source);
    let messages = Handle::new(ConversionMessages::default());
    converter.set_msg_registrator(&messages.cast());
    converter.set_surface_conversion(true);
    assert!(converter.perform(true).unwrap());
    messages.received.read().clone()
}

#[test]
fn surface_phase_uses_conversion_face_message() {
    let messages = converted_face_messages(plane());
    assert_face_conversion_message(&messages);
}

fn face_compound(count: usize) -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for _ in 0..count {
        Builder.add(&mut compound, &plane()).unwrap();
    }
    compound.into_shape()
}

fn converter_with_callback(
    source: Shape,
) -> (
    Arc<ShapeUpgradeShapeConvertToBezier>,
    Handle<ConversionMessages>,
) {
    let converter = Arc::new(ShapeUpgradeShapeConvertToBezier::new(source));
    let messages = Handle::new(ConversionMessages {
        converter: Arc::downgrade(&converter),
        ..Default::default()
    });
    converter.set_msg_registrator(&messages.cast());
    converter.set_surface_conversion(true);
    (converter, messages)
}

#[test]
fn nested_compound_preserves_conversion_virtual_messages() {
    let messages = converted_face_messages(face_compound(1));
    assert_face_conversion_message(&messages);
}

#[test]
fn callback_changes_to_unmasked_modes_survive_surface_phase() {
    let (converter, messages) = converter_with_callback(plane());
    assert!(converter.perform(true).unwrap());
    assert!(!converter.get_3d_line_conversion());
    assert!(!converter.get_surface_conversion());
    assert_eq!(messages.received.read().len(), 1);
}

#[test]
fn callback_policy_change_applies_to_next_compound_face() {
    let (converter, messages) = converter_with_callback(face_compound(2));
    assert!(converter.perform(true).unwrap());
    assert_eq!(bezier_face_count(&converter.result()), 1);
    assert_eq!(messages.received.read().len(), 1);
}

#[test]
fn curve_phase_retains_callback_line_mode_change() {
    let (converter, messages) = converter_with_callback(plane());
    converter.set_surface_conversion(false);
    converter.set_3d_conversion(true);
    assert!(converter.perform(true).unwrap());
    assert!(!converter.get_3d_line_conversion());
    assert!(!converter.get_surface_conversion());
    assert_eq!(messages.received.read().len(), 1);
}

fn bezier_face_count(shape: &Shape) -> usize {
    Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .filter(|shape| {
            let face = topods::face(shape).unwrap();
            BRepTool::surface(&face)
                .unwrap()
                .cast::<GeomBezierSurface>()
                .get()
                .is_some()
        })
        .count()
}
