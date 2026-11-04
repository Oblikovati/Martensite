//! Lengths and angles between points, vertices, edges and faces.
use super::prelude::*;
use crate::primitive_shapes::box_sized;

const CR26035: &str = "============";

/// `angle180`: a straight (180 degree) angle between three aligned points.
pub(crate) fn straight_angle() -> Outcome {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    for (name, x) in [("p0", -100.), ("p1", 0.), ("p2", 100.)] {
        display_point(&mut run, name, [x, 0., 0.])?;
    }
    let shapes = [
        Measured::Object("p0"),
        Measured::Object("p1"),
        Measured::Object("p2"),
    ];
    display_dimension(
        run.session(),
        "angle",
        &DimensionRequest::new(DimensionKind::Angle, &shapes),
    )?;
    run.session().fit()?;
    run.dump("angle180.png")?;
    run.finish()
}

/// `bug25403`: custom values keep their model units (degrees, millimetres).
pub(crate) fn valued_dimensions() -> Outcome {
    banner(&[
        CR26035,
        "CR25403: Wrong storage of model units in AisAngleDimension",
        CR26035,
        "",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    for (name, p) in [
        ("angleP1", [0., 0., 0.]),
        ("angleP2", [50., 50., 50.]),
        ("angleP3", [50., 50., 100.]),
    ] {
        display_point(&mut run, name, p)?;
    }
    let three = [
        Measured::Object("angleP1"),
        Measured::Object("angleP2"),
        Measured::Object("angleP3"),
    ];
    let angle = DimensionRequest {
        value: Some(300.),
        model_units: Some("deg"),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Angle, &three)
    };
    display_dimension(run.session(), "dim1", &angle)?;
    let two = [Measured::Object("angleP1"), Measured::Object("angleP3")];
    let length = DimensionRequest {
        value: Some(10.),
        model_units: Some("mm"),
        style: DimensionStyle {
            show_units: Some(true),
            ..DimensionStyle::default()
        },
        plane: Some(WorkingPlane::Yoz),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &two)
    };
    display_dimension(run.session(), "dim2", &length)?;
    run.session().fit()?;
    run.dump("bug25403.png")?;
    run.finish()
}

/// `bug26317`: the length between two opposite box faces checks its plane.
pub(crate) fn face_pair_length() -> Outcome {
    banner(&[
        CR26035,
        "CR26317: Visualization, AisLengthDimension::CheckPlane is incorrect",
        CR26035,
        "",
    ]);
    let b = box_sized([100., 200., 300.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let (b1, b2) = (face_of(&b, 1), face_of(&b, 2));
    run.session()
        .display("b_1", &b1, &DisplayRequest::default())?;
    run.session()
        .display("b_2", &b2, &DisplayRequest::default())?;
    let shapes = [Measured::Shape(&b1), Measured::Shape(&b2)];
    display_dimension(
        run.session(),
        "len",
        &DimensionRequest::new(DimensionKind::Length, &shapes),
    )?;
    run.session().fit()?;
    run.dump("bug26317.png")?;
    run.finish()
}

/// `bug26035_1`: the length between two vertices on the YOZ plane.
pub(crate) fn vertex_pair_length() -> Outcome {
    banner(&[
        CR26035,
        "CR26035: Dimension drawing between two vertices",
        CR26035,
        "",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let (v1, v2) = (vertex([10., 0., 0.])?, vertex([10., 0., 10.])?);
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Yoz),
        font: Some("SansFont"),
        ..DimensionRequest::new(
            DimensionKind::Length,
            &[Measured::Shape(&v1), Measured::Shape(&v2)],
        )
    };
    display_dimension(run.session(), "len", &request)?;
    run.session().fit()?;
    run.dump("bug26035_1.png")?;
    run.finish()
}

/// How the point operand of an edge-to-point length is given and ordered.
#[derive(Clone, Copy, Debug)]
pub(crate) struct EdgeOperand {
    /// A displayed vertex shape instead of a point marker.
    pub vertex_shape: bool,
    /// The point is the first operand.
    pub point_first: bool,
}

/// `bug26035_3`..`_6`: the length between an edge and a vertex or point on the YOZ plane.
pub(crate) fn edge_vertex_length(
    operand: EdgeOperand,
    title: &str,
    image: &'static str,
) -> Outcome {
    banner(&[CR26035, title, CR26035, ""]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let e = edge([10., -10., 0.], [10., 10., 0.])?;
    run.session().display("e", &e, &DisplayRequest::default())?;
    let v3 = vertex([10., 0., 10.])?;
    let point = if operand.vertex_shape {
        run.session()
            .display("v3", &v3, &DisplayRequest::default())?;
        Measured::Shape(&v3)
    } else {
        display_point(&mut run, "p", [10., 0., 10.])?;
        Measured::Object("p")
    };
    let shapes = if operand.point_first {
        [point, Measured::Shape(&e)]
    } else {
        [Measured::Shape(&e), point]
    };
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Yoz),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &shapes)
    };
    display_dimension(run.session(), "len", &request)?;
    run.session().fit()?;
    dump_coincident(&mut run, image, "len", &["e"])?;
    run.finish()
}

/// `bug24133_1`: a length dimension highlights under the cursor.
pub(crate) fn highlighted_length() -> Outcome {
    banner(&[
        CR26035,
        "CR24133: Development of improvement of dimensions implementation - Length dimension",
        CR26035,
        "",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    display_point(&mut run, "lengthP1", [0., 0., 0.])?;
    display_point(&mut run, "lengthP2", [50., 50., 50.])?;
    let shapes = [Measured::Object("lengthP1"), Measured::Object("lengthP2")];
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Xoy),
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Length, &shapes)
    };
    display_dimension(run.session(), "dim1", &request)?;
    run.session().fit()?;
    run.session().move_cursor(82, 254)?;
    if !run.session().check_color(362, 102, [0., 1., 1.])? {
        println!("Error : Highlighting of length dimension is wrong.");
    }
    run.dump("bug24133_1.png")?;
    run.finish()
}
