//! `bcut_simple` bodies written by hand: they keep checks an earlier test made during the script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::make_box,
    construction::{Command, profile, revol},
    one_step::bcut,
    properties::Prop,
    session::Session,
    sorted_explode::nexplode_faces,
};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::{geom::GeomPlane, top_abs::ShapeEnum};
use martensite_tkmath::gp::Plane3;

/// `profile ... S face` works on the plane stored in the face, which has no location here.
fn stored_plane(s: &dyn Session, face: &str) -> Plane3 {
    let shape = s.shape(face).expect("the selected face");
    let (surface, location) =
        BRepTool::surface_with_location(&topods::face(shape).unwrap()).unwrap();
    assert!(location.is_identity(), "{face}: expected no location");
    surface.cast::<GeomPlane>().get().unwrap().pln()
}

/// Beyond the script: the frame a native run reads from the face.
fn frame_of(plane: Plane3, origin: [f64; 3], normal: [f64; 3], x_axis: [f64; 3]) {
    let (at, n, x) = (
        plane.location(),
        plane.axis().direction(),
        plane.x_axis().direction(),
    );
    assert_eq!([at.x(), at.y(), at.z()], origin, "G6 plane origin");
    assert_eq!([n.x(), n.y(), n.z()], normal, "G6 plane normal");
    assert_eq!([x.x(), x.y(), x.z()], x_axis, "G6 plane x axis");
}

pub fn G6(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [100., 100., 40.]);
    let order = nexplode_faces(s, "b");
    // Beyond the script: the sorted faces are these of the ones explored, as a native run has it.
    assert_eq!(order, [0, 2, 4, 5, 3, 1], "G6 sorted-to-explored order");
    let plane = stored_plane(s, "b_4");
    frame_of(plane, [0., 0., 40.], [0., 0., 1.], [1., 0., 0.]);
    profile(
        s,
        "rev",
        &[
            Command::Plane([0., 0., 1.], [1., 0., 0.]),
            Command::Origin([0., 0., 40.]),
            Command::First(50., 20.),
            Command::Y(50.),
            Command::Arc(10., 180.),
            Command::Y(-50.),
            Command::Arc(10., 180.),
        ],
    );
    // Beyond the script: the arcs end where the profile started, so nothing closes it.
    part_count(s, "rev", Some(ShapeEnum::Edge), 4);
    printed_mass(s, "rev", Prop::Area, "1314.16");
    printed_mass(s, "rev", Prop::Length, "162.832");
    revol(s, "rev2", "rev", [0., 0., 50.], [0., 1., 0.], 360.);
    bcut(s, "result", "b", "rev2");
    checkprops(s, "result", Prop::Area, Expected::Mass(41187.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "rev2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b", Prop::Area, "36000");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "rev2", Prop::Area, "42000.2");
    kind_is(s, "rev2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
