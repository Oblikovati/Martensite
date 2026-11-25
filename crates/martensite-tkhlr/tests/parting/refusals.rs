// SPDX-License-Identifier: GPL-2.0-only
use super::fixtures::{self, Model, SCOPE};
use martensite_tkhlr::parting::{
    self, PartingBindingProblem, PartingCylinderProblem, PartingEntity, PartingFaceId,
    PartingFailureKind, PartingOptions,
};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Direction3, Point3},
};
use std::num::NonZeroU64;

#[test]
fn unsupported_curved_wall_and_partial_cylinder_fail_as_typed_results() {
    let cone = Model::new(fixtures::cone());
    let kind = refuse(&cone, Direction3::X);
    assert!(
        matches!(
            kind,
            PartingFailureKind::UnsupportedFace {
                surface: SurfaceType::Cone,
                ..
            }
        ),
        "{kind:?}"
    );
    let partial = Model::new(fixtures::incomplete_wall());
    let kind = refuse(&partial, Direction3::X);
    assert!(
        matches!(kind, PartingFailureKind::Cylinder { problem: PartingCylinderProblem::Bounds([0., last, 0., 10.]), .. } if last == std::f64::consts::FRAC_PI_2),
        "{kind:?}"
    );
}

#[test]
fn skew_wall_and_unproven_cavity_are_incomplete() {
    let cylinder = Model::new(fixtures::cylinder(5., 10.));
    let skew = Direction3::new(1., 0., 1.).unwrap();
    let kind = refuse(&cylinder, skew);
    assert!(
        matches!(
            kind,
            PartingFailureKind::Cylinder {
                problem: PartingCylinderProblem::SkewPull,
                ..
            }
        ),
        "{kind:?}"
    );
    let touching = Model::new(fixtures::cavity_box(
        Point3::new(0., 3., 2.),
        Point3::new(8., 7., 8.),
    ));
    let kind = refuse(&touching, Direction3::Z);
    assert!(
        matches!(kind, PartingFailureKind::ShellScope { shells: 2 }),
        "{kind:?}"
    );
}

#[test]
fn foreign_face_bindings_duplicate_edges_and_exclusions_refuse_in_scope() {
    let model = Model::new(fixtures::block());
    let other_model = Model::new(fixtures::block());
    let (mut faces, mut edges) = model.bindings();
    let mut foreign = faces[0];
    foreign.occurrence = &other_model.faces[0];
    faces[0] = foreign;
    let err = parting::query(
        model.source(&faces, &edges),
        Direction3::Z,
        PartingOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.source, SCOPE);
    assert!(matches!(
        err.kind,
        PartingFailureKind::Binding {
            problem: PartingBindingProblem::ForeignOccurrence,
            ..
        }
    ));
    let (faces, _) = model.bindings();
    edges[1].id = edges[0].id;
    let err = parting::query(
        model.source(&faces, &edges),
        Direction3::Z,
        PartingOptions::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err.kind,
        PartingFailureKind::Binding {
            problem: PartingBindingProblem::DuplicateId,
            ..
        }
    ));
    let (_, edges) = model.bindings();
    let id = PartingFaceId(NonZeroU64::MAX);
    let options = PartingOptions {
        exclude_faces: &[id],
        ..Default::default()
    };
    let err = parting::query(model.source(&faces, &edges), Direction3::Z, options).unwrap_err();
    assert!(
        matches!(err.kind, PartingFailureKind::Binding { entity: PartingEntity::Face(face), problem: PartingBindingProblem::UnknownExclusion } if face == id)
    );
}

#[test]
fn overflowed_native_direction_does_not_publish_an_empty_result() {
    let model = Model::new(fixtures::block());
    let pull = Direction3::new(1e300, 2e300, 3e300).unwrap();
    let kind = refuse(&model, pull);
    assert!(
        matches!(kind, PartingFailureKind::Pull([0., 0., 0.])),
        "{kind:?}"
    );
}

fn refuse(model: &Model, pull: Direction3) -> PartingFailureKind {
    let (faces, edges) = model.bindings();
    let err = parting::query(
        model.source(&faces, &edges),
        pull,
        PartingOptions::default(),
    )
    .unwrap_err();
    assert_eq!(err.source, SCOPE);
    err.kind
}

#[test]
fn complete_wall_bounds_do_not_admit_an_altered_trim() {
    use martensite_tkbrep::{brep::BRepTool, brep_adaptor::BRepAdaptorSurface};
    use martensite_tkg2d::geom2d::Geom2dLine;
    use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
    use martensite_tkmath::gp::Direction2;
    let model = Model::new(fixtures::cylinder(5., 10.));
    let wall = model
        .faces
        .iter()
        .find(|face| {
            BRepAdaptorSurface::with_face(face, false)
                .unwrap()
                .get_type()
                .unwrap()
                == SurfaceType::Cylinder
        })
        .unwrap();
    let pcurve = model
        .edges
        .iter()
        .find_map(|edge| {
            let curve = BRepTool::curve_on_face(edge, wall).unwrap();
            let line = curve.pcurve.cast::<Geom2dLine>();
            (line.get()?.direction().y() == 0. && line.get()?.location().y() == 0.).then_some(line)
        })
        .unwrap();
    pcurve
        .get()
        .unwrap()
        .set_direction(Direction2::new(1., 0.1).unwrap());
    let kind = refuse(&model, Direction3::X);
    assert!(
        matches!(
            kind,
            PartingFailureKind::Cylinder {
                problem: PartingCylinderProblem::Trim,
                ..
            }
        ),
        "{kind:?}"
    );
}

#[test]
fn unit_direct_placement_is_an_explicit_admission_boundary() {
    use martensite_tkbrep::topods::Solid;
    use martensite_tkmath::{gp::Transform3, toploc::Location};
    let body = fixtures::block();
    let scale = Transform3::scaling(Point3::default(), 2.).unwrap();
    let scaled = Solid::unchecked(body.moved(&Location::new(scale), false).unwrap());
    let kind = refuse(&Model::new(scaled), Direction3::X);
    assert!(
        matches!(
            kind,
            PartingFailureKind::Placement {
                entity: PartingEntity::Solid,
                scale: 2.,
                mirrored: false
            }
        ),
        "{kind:?}"
    );
}
