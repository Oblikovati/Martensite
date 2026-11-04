//! Read actual buffers and computed sensitives after the source construction/removal sequence.
use super::*;

#[test]
fn segment_array_keeps_source_vertices_and_explicit_red_colors() {
    let array = objects::segment_array(&[[76., -249., 0.], [274., -249., 0.]]).unwrap();
    assert_eq!(array.vertex_number(), 2);
    assert_eq!(array.vertice(1).unwrap().x(), 76.);
    assert_eq!(array.vertice(2).unwrap().x(), 274.);
    assert!(array.has_vertex_colors());
    assert!(!array.has_vertex_normals());
    assert_eq!(array.vertex_color_rgb(1).unwrap(), [1., 0., 0.]);
    assert_eq!(array.vertex_color_rgb(2).unwrap(), [1., 0., 0.]);
}

#[test]
fn quad_array_preserves_four_corners_and_blue_vertex_channels() {
    let array = objects::quad_array().unwrap();
    assert_eq!(array.vertex_number(), 4);
    assert!(array.has_vertex_colors());
    assert!(!array.has_vertex_normals());
    assert_eq!(
        array.vertice(1).unwrap(),
        martensite_tkmath::gp::Point3::new(175., -300., 0.)
    );
    assert_eq!(
        array.vertice(3).unwrap(),
        martensite_tkmath::gp::Point3::new(375., -100., 0.)
    );
    assert_eq!(array.vertex_color_rgb(4).unwrap(), [0., 0., 1.]);
}

#[test]
fn triangle_sensitive_retains_source_vertices_after_point_objects_are_removed() {
    let mut session = cpu_session();
    arrays::triangle_scene(&mut session).unwrap();
    assert!(session.object("pt1").is_none());
    assert!(session.object("pt4").is_none());
    let entity = sensitive(&session, "tri1").cast::<Select3DSensitivePrimitiveArray>();
    assert!(!entity.is_null());
    assert_eq!(
        entity.get_vertex(0),
        [
            Vec3::new(75., -250., 0.),
            Vec3::new(275., -250., 0.),
            Vec3::new(275., -50., 0.)
        ]
    );
    assert_eq!(entity.nb_sub_elements(), 1);
}

#[test]
fn segment_and_quad_selection_use_real_segment_and_point_array_entities() {
    let mut session = cpu_session();
    objects::display_array(
        &mut session,
        "seg",
        objects::segment_array(&objects::SEGMENTS[0]).unwrap(),
    )
    .unwrap();
    objects::display_array(&mut session, "quad", objects::quad_array().unwrap()).unwrap();
    let segment = sensitive(&session, "seg");
    let quad = sensitive(&session, "quad");
    assert_eq!(
        segment.get().unwrap().dynamic_type().name(),
        "Select3DSensitiveSegment"
    );
    assert_eq!(segment.sensitivity_factor(), 4);
    assert!(!quad.cast::<Select3DSensitivePrimitiveArray>().is_null());
    assert_eq!(quad.sensitivity_factor(), 8);
    assert_eq!(quad.nb_sub_elements(), 4);
}

#[test]
fn circle_keeps_real_filled_sensitive_after_temporary_point_removal() {
    let mut session = cpu_session();
    arrays::circle_scene(&mut session).unwrap();
    assert!(session.object("pr1").is_none());
    let circle = sensitive(&session, "circle1");
    assert_eq!(
        circle.get().unwrap().dynamic_type().name(),
        "Select3DSensitiveCircle"
    );
    assert!(
        !named(&session, "circle1")
            .unwrap()
            .global_sel_owner()
            .is_null()
    );
}

#[test]
fn point_operand_count_mismatch_is_refused_before_partial_display() {
    let mut session = cpu_session();
    let error = objects::points(&mut session, &["pt1", "pt2"], &[[0.; 3]]).unwrap_err();
    assert!(error.to_string().contains("point names=2 coordinates=1"));
    assert!(error.to_string().contains("expected equal counts"));
    assert!(session.object("pt1").is_none());
}
