//! Independent closed-form and actual query controls; no GPU/original execution is implied.
use super::{cylinder_detected, geometry::source_inputs, source_scene};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods::Edge};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_gprop::BRepGProp};

#[test]
fn tangent_profile_preserves_case_start_direction_and_single_closed_edge() {
    let inputs = source_inputs().unwrap();
    let mut edges = Explorer::new(
        &inputs.profile.into_shape(),
        ShapeEnum::Edge,
        ShapeEnum::Shape,
    )
    .unwrap();
    let edge = Edge::unchecked(edges.current().clone());
    let (curve, first, last) = BRepTool::curve(&edge).unwrap();
    assert_eq!([first, last], [0., 2. * std::f64::consts::PI]);
    let (point, tangent) = curve.d1(first).unwrap();
    assert_eq!([point.x(), point.y(), point.z()], [0.; 3]);
    assert_eq!([tangent.x(), tangent.y(), tangent.z()], [1., 0., 0.]);
    let opposite = curve.value(std::f64::consts::PI).unwrap();
    assert!(opposite.x().abs() < 1.0e-14);
    assert_eq!([opposite.y(), opposite.z()], [2., 0.]);
    edges.next().unwrap();
    assert!(!edges.more());
}

#[test]
fn both_signed_prisms_have_closed_form_volume_and_case_translated_bounds() {
    let inputs = source_inputs().unwrap();
    for (shape, bounds) in inputs
        .prisms
        .iter()
        .zip([[0., 1., 1., 2., 3., 3.], [-2., -1., -3., 0., 1., -1.]])
    {
        let mut properties = GProps::new();
        BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
        assert!((properties.mass() - 2. * std::f64::consts::PI).abs() < 1.0e-11);
        let mut actual = martensite_tkmath::bnd::BoundingBox3::default();
        BRepBndLib::add(shape, &mut actual, false).unwrap();
        for (actual, expected) in actual.get().unwrap().iter().zip(bounds) {
            assert!((actual - expected).abs() < 1.0e-6);
        }
    }
}

#[test]
fn generated_face_is_the_actual_unit_disk_before_both_extrusions() {
    let inputs = source_inputs().unwrap();
    let mut properties = GProps::new();
    BRepGProp::surface_properties(&inputs.face.into_shape(), &mut properties, false, false)
        .unwrap();
    assert!((properties.mass() - std::f64::consts::PI).abs() < 1.0e-12);
    let centre = properties.centre_of_mass();
    assert!(centre.x().abs() < 1.0e-14);
    assert!((centre.y() - 1.).abs() < 1.0e-14);
    assert_eq!(centre.z(), 0.);
}

#[test]
fn both_case_cursor_positions_pick_actual_sensitive_cylinders() {
    let inputs = source_inputs().unwrap();
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)
            .expect("signed prism control; expected isolated409x409 viewer");
    source_scene(&mut session, &inputs).unwrap();
    assert!(cylinder_detected(&mut session, 130, 330).unwrap());
    assert!(cylinder_detected(&mut session, 270, 130).unwrap());
    assert!(!cylinder_detected(&mut session, 0, 0).unwrap());
    super::source_probes(&mut session).unwrap();
}

#[path = "../support/selection_frame_probe.rs"]
mod frame_probe;

#[test]
fn submitted_sensitive_geometry_is_included_in_the_actual_backend_frame_and_hook() {
    use martensite_tkernel::quantity::NamedColor;
    let mut probe = frame_probe::SelectionFrameProbe::new();
    source_scene(&mut probe.session, &source_inputs().unwrap()).unwrap();
    let (frame, recorded) = probe.snapshot();
    let overlay = frame_probe::uniform_color_draws(
        frame.draws(),
        frame_probe::named_linear_rgba(NamedColor::AQUAMARINE1),
    );
    assert!(
        !overlay.is_empty(),
        "actual backend omitted the sensitive overlay"
    );
    assert_eq!(
        overlay,
        frame_probe::uniform_color_draws(
            &recorded,
            frame_probe::named_linear_rgba(NamedColor::AQUAMARINE1)
        )
    );
    let actual_overlay: Vec<_> = overlay.iter().map(|draw| (*draw).clone()).collect();
    frame_probe::assert_ordered_join_rejects_corruption(&actual_overlay);
    for draw in &overlay {
        assert_eq!(draw.line_style().unwrap().parameters(), (1., 0xffc0, 1));
    }
    println!(
        "SENSITIVE_COVERAGE draws={} segments={}",
        overlay.len(),
        overlay
            .iter()
            .map(|draw| draw.indices().len() / 2)
            .sum::<usize>()
    );
    frame_probe::write_geometry("bug33084", &frame);
}
