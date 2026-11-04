//! CPU controls exercise owned input geometry and actual context storage, without Vulkan.
use super::{source_cylinder, source_object};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

#[test]
fn cylinder_volume_matches_literal_radius_and_height() {
    let shape = source_cylinder().unwrap();
    let mut properties = GProps::new();
    BRepGProp::volume_properties(&shape, &mut properties, false, false, false).unwrap();
    let expected = std::f64::consts::PI * 100. * 100. * 300.;
    assert!((properties.mass() - expected).abs() < expected * 1.0e-12);
}

#[test]
fn own_highlight_mode_does_not_force_shaded_display_mode() {
    let object = source_object(&source_cylinder().unwrap());
    assert_eq!(object.hilight_mode(), 1);
    assert!(!object.has_display_mode());
}

#[test]
fn anonymous_display_is_stored_by_context_without_named_binding() {
    let session = ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)
        .expect("anonymous cylinder control; expected isolated409x409 viewer");
    let object = source_object(&source_cylinder().unwrap());
    session.context().display(&object, true).unwrap();
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 1);
    assert_eq!(
        displayed.iter().next().unwrap().object_address(),
        object.object_address()
    );
    assert!(session.object("c").is_none());
    assert_eq!(object.hilight_mode(), 1);
}

#[path = "../support/selection_frame_probe.rs"]
mod frame_probe;

#[test]
fn submitted_anonymous_wire_and_highlight_are_in_the_actual_backend_frame_and_hook() {
    use martensite_tkernel::quantity::NamedColor;
    let (probe, _) = anonymous_frame_probe();
    let (frame, recorded) = probe.snapshot();
    let wire = frame_probe::uniform_color_draws(
        frame.draws(),
        frame_probe::named_linear_rgba(NamedColor::YELLOW),
    );
    assert!(
        !wire.is_empty(),
        "actual backend omitted anonymous wire geometry"
    );
    assert_eq!(
        wire,
        frame_probe::uniform_color_draws(
            &recorded,
            frame_probe::named_linear_rgba(NamedColor::YELLOW)
        )
    );
    frame_probe::assert_ordered_join_rejects_corruption(frame.draws());
    assert!(frame.draws().iter().any(|draw| draw.line_style().is_none()));
    println!(
        "CYLINDER_COVERAGE frame={} hook={}",
        frame.draws().len(),
        recorded.len()
    );
    frame_probe::write_geometry("buc60876", &frame);
}

fn anonymous_frame_probe() -> (
    frame_probe::SelectionFrameProbe,
    martensite_tkernel::standard::Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
) {
    let mut probe = frame_probe::SelectionFrameProbe::new();
    let object = source_object(&source_cylinder().unwrap());
    probe.session.context().display(&object, true).unwrap();
    probe.session.fit().unwrap();
    probe.session.move_cursor(200, 200).unwrap();
    (probe, object)
}

#[test]
fn object_boundary_control_removes_wire_without_changing_shaded_highlight() {
    let (probe, object) = anonymous_frame_probe();
    let (before, _) = probe.snapshot();
    let bindings = boundary_scope::aspect_bindings(&object);
    let scope = BoundaryPatternScope::new(&object);
    let (without, _) = probe.snapshot();
    assert_only_boundary_removed(&before, &without);
    frame_probe::write_geometry("buc60876-without-wire", &without);
    drop(scope);
    assert_eq!(bindings, boundary_scope::aspect_bindings(&object));
    let (restored, _) = probe.snapshot();
    assert_eq!(before.draws(), restored.draws());
}

fn assert_only_boundary_removed(
    before: &martensite_vulkan::RenderFrame,
    without: &martensite_vulkan::RenderFrame,
) {
    let yellow = frame_probe::named_linear_rgba(martensite_tkernel::quantity::NamedColor::YELLOW);
    let removed = frame_probe::uniform_color_draws(before.draws(), yellow);
    assert_eq!(removed.len(), 1);
    let expected: Vec<_> = before
        .draws()
        .iter()
        .filter(|draw| !removed.contains(draw))
        .collect();
    assert_eq!(expected, without.draws().iter().collect::<Vec<_>>());
    assert_eq!(before.extent(), without.extent());
    assert_eq!(before.clear_color(), without.clear_color());
    let policies = |frame: &martensite_vulkan::RenderFrame| {
        frame
            .layers()
            .iter()
            .map(|layer| (layer.depth_policy(), layer.polygon_offset()))
            .collect::<Vec<_>>()
    };
    assert_eq!(policies(before), policies(without));
}

use super::boundary_scope::{self, BoundaryPatternScope};

#[test]
fn boundary_control_restores_exact_handles_after_error() {
    let (probe, object) = anonymous_frame_probe();
    let (before, _) = probe.snapshot();
    let bindings = boundary_scope::aspect_bindings(&object);
    let result: Result<(), &'static str> = {
        let _scope = BoundaryPatternScope::new(&object);
        assert_eq!(probe.snapshot().0.draws().len() + 1, before.draws().len());
        Err("injected control capture failure")
    };
    assert_eq!(result, Err("injected control capture failure"));
    assert_eq!(bindings, boundary_scope::aspect_bindings(&object));
    assert_eq!(before.draws(), probe.snapshot().0.draws());
}
