//! Owned geometry, aliasing and actual context controls; none replace the original image.
use super::*;
use crate::hatch_surface::scene::SourceViewOutput as _;
use martensite_tkbrep::{brep::BRepTool, brep_tools};
use martensite_tkernel::quantity::Color;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{aspect::AspectTypeOfLine, null_driver::NullGraphicDriver};
use martensite_tkv3d::{
    ais::AisTypeOfIso,
    std_prs::{StdPrsIsolineDestination, StdPrsIsolinePolylines, StdPrsIsolines},
};

fn cpu_face_session() -> ViewerSession {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
    let face = infinite_cylinder_face().unwrap();
    session
        .display(
            "f",
            &face.into_shape(),
            &DisplayRequest::default().with_display_mode(0),
        )
        .unwrap();
    session
}

fn seeded_face_iso() -> Prs3dIsoAspect {
    let color = Color::from_name("CORAL").unwrap();
    let kind = AspectTypeOfLine::Dash;
    Prs3dIsoAspect::new(color, kind, 2.5, 3).unwrap()
}

fn face_iso_counts(drawer: &Prs3dDrawer) -> [i32; 2] {
    let u = drawer.u_iso_aspect().number();
    let v = drawer.v_iso_aspect().number();
    [u, v]
}

fn assert_distinct_face_isos(drawer: &Prs3dDrawer, defaults: &Prs3dDrawer) {
    let own = [drawer.u_iso_aspect(), drawer.v_iso_aspect()];
    let linked = [defaults.u_iso_aspect(), defaults.v_iso_aspect()];
    for (copy, default) in own.iter().zip(linked) {
        assert_ne!(copy.object_address(), default.object_address());
    }
}

#[test]
fn natural_face_retains_infinite_domain_and_closed_form_radius() {
    let face = infinite_cylinder_face().unwrap();
    let (_, _, first, last) = brep_tools::uv_bounds(&face).unwrap();
    assert!(precision::is_infinite(first) && first < 0.);
    assert!(precision::is_infinite(last) && last > 0.);
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let point = surface.value(0., 123.).unwrap();
    assert!(point.distance(&Point3::new(10., 0., 123.)) < 1e-12);
}

#[test]
fn copied_iso_preserves_appearance_without_shared_line_ownership() {
    let original = seeded_face_iso();
    let copy = independent_face_iso(&original, 10).unwrap();
    assert_eq!(copy.number(), 10);
    let expected = original.aspect().interior_color();
    assert_eq!(copy.aspect().interior_color(), expected);
    assert_eq!(copy.aspect().line_kind(), AspectTypeOfLine::Dash);
    assert_eq!(copy.aspect().width(), 2.5);
    copy.set_width(7.).unwrap();
    assert_eq!(original.aspect().width(), 2.5);
    assert_eq!(original.number(), 3);
}

#[test]
fn object_iso_counts_leave_linked_defaults_unchanged() {
    let defaults = Handle::new(Prs3dDrawer::default());
    defaults.setup_own_defaults().unwrap();
    let drawer = Prs3dDrawer::default();
    drawer.set_link(&defaults);
    let counts = face_iso_counts(&defaults);
    own_face_isolines(&drawer).unwrap();
    assert_eq!(face_iso_counts(&drawer), [10, 4]);
    assert_eq!(face_iso_counts(&defaults), counts);
    assert_distinct_face_isos(&drawer, &defaults);
}

#[test]
fn body_view_retains_first_view_and_context_with_second_active() {
    let mut session =
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
    let first = session.view().clone();
    let context = session.context().object_address();
    face_body_view(&mut session).unwrap();
    assert_ne!(session.view().object_address(), first.object_address());
    assert_eq!(
        session.view().viewer().object_address(),
        first.viewer().object_address()
    );
    assert_eq!(session.context().object_address(), context);
    assert_eq!(session.view().viewer().defined_views().len(), 2);
    assert_eq!(session.source_window_size(), (409, 409));
}

#[test]
fn displayed_face_commands_recompute_actual_object_and_context_policy() {
    let session = cpu_face_session();
    let defaults = session.context().default_drawer();
    let counts = face_iso_counts(&defaults);
    displayed_face_limit(&session).unwrap();
    displayed_face_isolines(&session).unwrap();
    let drawer = session.object("f").unwrap().attributes();
    assert_eq!(defaults.maximal_parameter_value(), 10.);
    assert!(session.context().iso_on_plane());
    assert_eq!(face_iso_counts(&drawer), [10, 4]);
    assert_eq!(session.context().iso_number(AisTypeOfIso::U), counts[0]);
    assert_eq!(session.context().iso_number(AisTypeOfIso::V), counts[1]);
}

fn generated_face_isolines(face: &Face) -> [StdPrsIsolinePolylines; 2] {
    let drawer = Handle::new(Prs3dDrawer::default());
    drawer.setup_own_defaults().unwrap();
    drawer.set_maximal_parameter_value(10.);
    own_face_isolines(&drawer).unwrap();
    let [mut u, mut v] = [
        StdPrsIsolinePolylines::default(),
        StdPrsIsolinePolylines::default(),
    ];
    StdPrsIsolines::add_on_surface_polylines(
        face,
        &drawer,
        0.01,
        StdPrsIsolineDestination::distinct(&mut u, &mut v),
    )
    .unwrap();
    [u, v]
}

fn assert_display_point(point: &Point3) {
    assert!(point.x().is_finite() && point.y().is_finite() && point.z().is_finite());
    assert!((martensite_cmath::hypot(point.x(), point.y()) - 10.).abs() < 1e-10);
    assert!(point.z().abs() <= 10. + 1e-12);
}

#[test]
fn actual_isoline_polylines_are_finite_without_trimming_input_geometry() {
    let face = infinite_cylinder_face().unwrap();
    let before = brep_tools::uv_bounds(&face).unwrap();
    let lines = generated_face_isolines(&face);
    assert_eq!([lines[0].length(), lines[1].length()], [10, 4]);
    for direction in lines {
        for line in direction.iter() {
            line.sequence(|points| {
                assert!(points.length() >= 2);
                points.iter().for_each(assert_display_point);
            });
        }
    }
    assert_eq!(brep_tools::uv_bounds(&face).unwrap(), before);
}
