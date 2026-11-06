//! Independent continuity checks on a cylinder's analytic periodic seam.
mod support;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkfillet::chfi_ds::TypeOfConcavity;
use martensite_tkfillet::chfi3d::{define_connect_type, is_tangent_faces};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::toploc::Location;
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Transform3, Vector3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn cylinder_shape() -> Shape {
    BRepPrimAPIMakeCylinder::new(2., 5.)
        .shape()
        .unwrap()
        .clone()
}

fn seam_pair(shape: &Shape) -> (Edge, Face) {
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let face = topods::face(&face).unwrap();
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            let edge = topods::edge(&edge).unwrap();
            if BRepTool::is_closed_on_face(&edge, &face).unwrap() {
                return (edge, face);
            }
        }
    }
    panic!("cylinder has no seam; expected a periodic lateral face with two edge occurrences");
}

fn assert_seam_continuity(shape: &Shape) {
    let (edge, face) = seam_pair(shape);
    for order in [Continuity::G1, Continuity::C2, Continuity::G2] {
        assert!(
            is_tangent_faces(&edge, &face, &face, order).unwrap(),
            "{order:?}"
        );
    }
    assert_eq!(
        define_connect_type(&edge, &face, &face, 1e-5, true).unwrap(),
        TypeOfConcavity::Tangential
    );
}

#[test]
fn analytic_cylinder_seam_is_tangent_through_second_order() {
    let cylinder = cylinder_shape();
    assert_seam_continuity(&cylinder);
}

#[test]
fn translated_cylinder_seam_preserves_continuity_and_concavity() {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(7., -11., 13.));
    let located = cylinder_shape()
        .moved(&Location::new(transform), false)
        .unwrap();
    assert_seam_continuity(&located);
}

#[test]
fn stored_regularity_shortcut_is_specific_to_first_geometric_order() {
    let edge = support::fixture_edge(0);
    let first = support::fixture_face(&edge, (0., 0., 1.), false, false);
    let second = support::fixture_face(&edge, (0., 0., 1.), false, true);
    assert!(!is_tangent_faces(&edge, &first, &second, Continuity::G1).unwrap());
    for continuity in [Continuity::G1, Continuity::C2] {
        BRepBuilder::default()
            .continuity_on_faces(&edge, &first, &second, continuity)
            .unwrap();
        assert!(is_tangent_faces(&edge, &first, &second, Continuity::G1).unwrap());
        assert!(!is_tangent_faces(&edge, &first, &second, Continuity::G2).unwrap());
    }
}
