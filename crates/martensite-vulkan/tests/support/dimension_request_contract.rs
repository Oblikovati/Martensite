// Owned command-input and closed-form checks; these never create a renderer or viewer.
use super::*;

type DimensionOperandBuilder = fn(&[Handle<dyn AisInteractiveObject>]) -> Built;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomCircle};
use martensite_tkmath::gp::{Axis2, Direction3};
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Arrow, Prs3dDimensionTextHorizontalPosition as HPos,
    Prs3dDimensionTextVerticalPosition as VPos,
};

#[test]
fn dimension_default_request_keeps_operands_and_leaves_options_unset() {
    let operands = [Measured::Object("second"), Measured::Object("first")];
    use DimensionKind::{Angle, Diameter, Length, Radius};
    for kind in [Length, Angle, Radius, Diameter] {
        let request = DimensionRequest::new(kind, &operands);
        assert_eq!(request.kind, kind);
        assert!(matches!(
            request.shapes.as_slice(),
            [Measured::Object("second"), Measured::Object("first")]
        ));
        assert!(request.plane.is_none() && request.font.is_none());
        assert!(request.text_3d.is_none() && request.style.show_units.is_none());
        assert!(request.value.is_none() && request.flyout.is_none());
        assert!(request.model_units.is_none() && request.display_units.is_none());
    }
}

#[test]
fn dimension_default_aspect_has_fitted_labels_and_six_unit_arrows() {
    let request = DimensionRequest::new(DimensionKind::Length, &[]);
    let aspect = request.aspect().unwrap();
    assert!(!aspect.is_text_3d() && !aspect.is_text_shaded());
    assert!(!aspect.is_arrows_3d() && !aspect.is_units_displayed());
    assert_eq!(aspect.arrow_orientation(), Arrow::Fit);
    assert_eq!(aspect.text_horizontal_position(), HPos::Fit);
    assert_eq!(aspect.text_vertical_position(), VPos::Center);
    assert_eq!(
        aspect.arrow_aspect().get().unwrap().length().to_bits(),
        6_f64.to_bits()
    );
    assert_eq!(aspect.extension_size().to_bits(), 6_f64.to_bits());
    assert_eq!(aspect.arrow_tail_size().to_bits(), 6_f64.to_bits());
    assert_eq!(aspect.value_string_format().as_bytes(), b"%g");
}

#[test]
fn dimension_aspect_flags_are_explicit_and_fresh_between_requests() {
    for enabled in [true, false] {
        let mut request = DimensionRequest::new(DimensionKind::Length, &[]);
        request.text_3d = Some(enabled);
        request.style.show_units = Some(enabled);
        let aspect = request.aspect().unwrap();
        assert_eq!(aspect.is_text_3d(), enabled);
        assert_eq!(aspect.is_units_displayed(), enabled);
        assert!(!aspect.is_text_shaded() && !aspect.is_arrows_3d());
    }
    let fresh = DimensionRequest::new(DimensionKind::Length, &[])
        .aspect()
        .unwrap();
    assert!(!fresh.is_text_3d() && !fresh.is_units_displayed());
}

#[test]
fn dimension_working_planes_match_right_handed_coordinate_frames() {
    let frames = [
        (WorkingPlane::Xoy, [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]),
        (WorkingPlane::Yoz, [0., 1., 0.], [0., 0., 1.], [1., 0., 0.]),
        (WorkingPlane::Zox, [0., 0., 1.], [1., 0., 0.], [0., 1., 0.]),
    ];
    for (plane, x, y, z) in frames {
        let mut request = DimensionRequest::new(DimensionKind::Length, &[]);
        assert!(request.working_plane().is_none());
        request.plane = Some(plane);
        let frame = request.working_plane().unwrap().position();
        let origin = frame.location().xyz().coordinates().to_owned();
        assert_eq!(origin, [0., 0., 0.]);
        assert_eq!(frame.x_direction().xyz().coordinates().to_owned(), x);
        assert_eq!(frame.y_direction().xyz().coordinates().to_owned(), y);
        assert_eq!(frame.direction().xyz().coordinates().to_owned(), z);
    }
}

#[test]
fn dimension_default_parameters_preserve_seeded_value_and_flyout() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_flyout(-23.5);
    value.set_custom_value(17.25);
    DimensionRequest::new(DimensionKind::Length, &[]).apply_parameters(value);
    assert_eq!(value.get_flyout().to_bits(), (-23.5_f64).to_bits());
    assert_eq!(value.get_value().to_bits(), 17.25_f64.to_bits());
}

#[test]
fn dimension_explicit_parameters_replace_seeded_value_and_flyout() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_flyout(-23.5);
    value.set_custom_value(17.25);
    let mut request = DimensionRequest::new(DimensionKind::Length, &[]);
    request.flyout = Some(3.5);
    request.value = Some(-7.25);
    request.apply_parameters(value);
    assert_eq!(value.get_flyout().to_bits(), 3.5_f64.to_bits());
    assert_eq!(value.get_value().to_bits(), (-7.25_f64).to_bits());
}

fn point_operand([x, y, z]: [f64; 3]) -> Handle<dyn AisInteractiveObject> {
    let point = Handle::new(GeomCartesianPoint::new(x, y, z));
    let geometry = point.cast();
    AisPoint::new(&geometry).unwrap().cast()
}

#[test]
fn dimension_point_operand_preserves_all_coordinate_bits() {
    let object = point_operand([-2.5, 3.25, 7.5]);
    let point = point_of(&object).unwrap();
    assert_eq!(point.xyz().coordinates().to_owned(), [-2.5, 3.25, 7.5]);
    let vertex = shape_of(&object).unwrap();
    assert_eq!(
        BRepTool::pnt(&topods::vertex(&vertex).unwrap()).unwrap(),
        point
    );
}

#[test]
fn dimension_point_pair_length_is_the_three_four_five_hypotenuse() {
    let objects = [point_operand([0., 0., 0.]), point_operand([3., 4., 0.])];
    let plane = Plane3::from_position(Axis3::from_axis2(xoy()));
    let dimension = length(&objects, Some(plane)).unwrap();
    let value = dimension.get().unwrap();
    assert!(value.is_valid());
    assert_eq!(value.compute_value().to_bits(), 5_f64.to_bits());
}

#[test]
fn dimension_three_points_form_a_right_angle_at_the_middle_operand() {
    let objects = [
        point_operand([2., 0., 0.]),
        point_operand([0., 0., 0.]),
        point_operand([0., 3., 0.]),
    ];
    let dimension = angle(&objects).unwrap();
    let value = dimension.get().unwrap();
    assert!(value.is_valid());
    assert_eq!(
        value.compute_value().to_bits(),
        std::f64::consts::FRAC_PI_2.to_bits()
    );
}

#[test]
fn dimension_circle_measures_the_authored_radius_and_diameter() {
    let axis = Axis2::new(
        Point3::new(7., -3., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let geometry = Handle::new(GeomCircle::new(axis, 2.5).unwrap()).cast();
    let objects = [AisCircle::new(&geometry).cast()];
    let radial = radius(&objects).unwrap();
    let diametric = diameter(&objects).unwrap();
    assert!(radial.get().unwrap().is_valid() && diametric.get().unwrap().is_valid());
    let radial_value = radial.get().unwrap().compute_value();
    let diameter_value = diametric.get().unwrap().compute_value();
    assert_eq!(radial_value.to_bits(), 2.5_f64.to_bits());
    assert_eq!(diameter_value.to_bits(), 5_f64.to_bits());
}

#[test]
fn dimension_invalid_operand_counts_keep_the_current_command_errors() {
    let operations: [DimensionOperandBuilder; 2] = [angle, diameter];
    for operation in operations {
        let error = operation(&[]).expect_err("zero operands must be refused");
        assert_eq!(
            error.to_string(),
            "Error: wrong number of shapes to build dimension"
        );
    }
    assert_eq!(
        length(&[], None).err().unwrap().to_string(),
        "Error: wrong number of shapes to build dimension"
    );
    assert_eq!(
        radius(&[]).err().unwrap().to_string(),
        "Syntax error: wrong number of shapes to build dimension"
    );
}

#[test]
fn dimension_two_points_require_an_explicit_working_plane() {
    let objects = [point_operand([0., 0., 0.]), point_operand([3., 4., 0.])];
    let error = length(&objects, None).expect_err("point pair needs a working plane");
    assert_eq!(
        error.to_string(),
        "Error: can not build dimension without working plane"
    );
}

#[test]
fn dimension_default_parameters_preserve_seeded_unit_labels() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_model_units(&AsciiString::from("mm"));
    value.set_display_units(&AsciiString::from("in"));
    DimensionRequest::new(DimensionKind::Length, &[]).apply_parameters(value);
    assert_eq!(value.get_model_units().as_bytes(), b"mm");
    assert_eq!(value.get_display_units().as_bytes(), b"in");
}

#[test]
fn dimension_explicit_parameters_replace_seeded_unit_labels() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_model_units(&AsciiString::from("mm"));
    value.set_display_units(&AsciiString::from("in"));
    let mut request = DimensionRequest::new(DimensionKind::Length, &[]);
    request.model_units = Some("cm");
    request.display_units = Some("m");
    request.apply_parameters(value);
    assert_eq!(value.get_model_units().as_bytes(), b"cm");
    assert_eq!(value.get_display_units().as_bytes(), b"m");
}

#[test]
fn dimension_explicit_font_is_attached_to_its_own_text_aspect() {
    let mut request = DimensionRequest::new(DimensionKind::Length, &[]);
    request.font = Some("OwnedDimensionFont");
    let aspect = request.aspect().unwrap();
    let text = aspect.text_aspect();
    let style = text.get().unwrap().aspect();
    assert_eq!(
        style.get().unwrap().font().as_bytes(),
        b"OwnedDimensionFont"
    );
}
