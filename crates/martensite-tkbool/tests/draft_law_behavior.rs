//! Independent straight-path placement and draft junction contracts.
use martensite_tkbool::brep_fill::{BRepFillDraftLaw, BRepFillLocationLaw};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::geom_fill::{GeomFillLocationDraft, GeomFillLocationLaw, LocationD0};
use martensite_tkmath::gp::{Direction3, Matrix3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;

fn straight_draft() -> Result<(BRepFillDraftLaw, Handle<GeomFillLocationDraft>), Failure> {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for x in [0., 1., 3.] {
        polygon.add_point(Point3::new(x, 0., 0.))?;
    }
    let model = Handle::new(GeomFillLocationDraft::new(Direction3::Z, 0.2)?);
    Ok((BRepFillDraftLaw::new(&polygon.wire()?, &model)?, model))
}

fn placement(law: &BRepFillDraftLaw, index: i32) -> Result<LocationD0, Failure> {
    let copy = law.law(index)?;
    let copy = copy.get().expect("built law");
    let mut out = LocationD0::default();
    assert!(copy.d0(copy.get_domain()?.0, &mut out)?);
    Ok(out)
}

fn assert_matrix(actual: &Matrix3, expected: &Matrix3) -> Result<(), Failure> {
    for row in 1..=3 {
        for column in 1..=3 {
            assert!((actual.value(row, column)? - expected.value(row, column)?).abs() < 1e-12);
        }
    }
    Ok(())
}

#[test]
fn constructor_copies_each_edge_without_loading_the_model() -> Result<(), Failure> {
    let (law, model) = straight_draft()?;
    assert_eq!(law.nb_law()?, 2);
    assert!(model.get().expect("model").get_curve().is_null());
    assert_ne!(law.law(1)?.object_address(), law.law(2)?.object_address());
    assert_eq!(law.law(1)?.get().expect("first").get_domain()?, (0., 1.));
    assert_eq!(law.law(2)?.get().expect("second").get_domain()?, (0., 2.));
    assert_eq!(placement(&law, 1)?.v.x(), 0.);
    assert_eq!(placement(&law, 2)?.v.x(), 1.);
    Ok(())
}

#[test]
fn straight_draft_frame_has_closed_form_columns() -> Result<(), Failure> {
    let (law, _) = straight_draft()?;
    let out = placement(&law, 1)?;
    let (s, c) = martensite_cmath::sincos(0.2_f64);
    let expected = Matrix3::new([[0., 0., -1.], [-c, -s, 0.], [-s, c, 0.]]);
    assert_matrix(&out.m, &expected)
}

fn clean_transformed_second(transform: Matrix3) -> Result<(), Failure> {
    let (law, _) = straight_draft()?;
    law.law(2)?.get().expect("second").set_trsf(&transform)?;
    let before = placement(&law, 2)?;
    law.clean_law(1e-12)?;
    assert_matrix(&placement(&law, 2)?.m, &before.m)?;
    assert_eq!(placement(&law, 2)?.v.x(), 1.);
    Ok(())
}

#[test]
fn cleanup_replaces_the_transform_for_parallel_normals() -> Result<(), Failure> {
    clean_transformed_second(Matrix3::new([[1., 0., 0.], [0., -1., 0.], [0., 0., -1.]]))
}

#[test]
fn cleanup_accepts_antiparallel_normals() -> Result<(), Failure> {
    clean_transformed_second(Matrix3::new([[-1., 0., 0.], [0., -1., 0.], [0., 0., 1.]]))
}

#[test]
fn cleanup_preserves_nonparallel_normal_and_leaves_first_law_unchanged() -> Result<(), Failure> {
    let (law, _) = straight_draft()?;
    let first = placement(&law, 1)?;
    let transform = Matrix3::new([[0., 0., 1.], [0., 1., 0.], [-1., 0., 0.]]);
    law.law(2)?.get().expect("second").set_trsf(&transform)?;
    let before = placement(&law, 2)?;
    law.clean_law(1e-12)?;
    assert_matrix(&placement(&law, 2)?.m, &before.m)?;
    assert_matrix(&placement(&law, 1)?.m, &first.m)
}

#[test]
fn cleanup_replaces_instead_of_composing_an_existing_transform() -> Result<(), Failure> {
    let (law, _) = straight_draft()?;
    let scale = Matrix3::new([[2., 0., 0.], [0., 3., 0.], [0., 0., 4.]]);
    law.law(2)?.get().expect("second").set_trsf(&scale)?;
    law.clean_law(1e-12)?;
    let (s, c) = martensite_cmath::sincos(0.2_f64);
    let expected = Matrix3::new([
        [0., 0., -0.25],
        [-c / 2., -s / 3., 0.],
        [-s / 2., c / 3., 0.],
    ]);
    assert_matrix(&placement(&law, 2)?.m, &expected)
}

#[test]
fn reversed_path_uses_reversed_copies_without_mutating_source_curves() -> Result<(), Failure> {
    let (forward, model) = straight_draft()?;
    let reversed = martensite_tkbrep::topods::wire(&forward.wire().as_ref().reversed())?;
    let backward = BRepFillDraftLaw::new(&reversed, &model)?;
    assert_eq!(backward.nb_law()?, 2);
    assert_eq!(placement(&backward, 1)?.v.x(), 3.);
    assert_eq!(placement(&backward, 2)?.v.x(), 1.);
    assert_eq!(placement(&forward, 1)?.v.x(), 0.);
    assert_eq!(placement(&forward, 2)?.v.x(), 1.);
    Ok(())
}

#[test]
fn degenerate_path_edges_do_not_allocate_placement_laws() -> Result<(), Failure> {
    let (forward, model) = straight_draft()?;
    let second = forward.edge(2)?;
    martensite_tkbrep::brep::BRepBuilder::default().degenerated(&second, true)?;
    let rebuilt = BRepFillDraftLaw::new(forward.wire(), &model)?;
    assert_eq!(rebuilt.nb_law()?, 1);
    assert!(rebuilt.edge(1)?.is_same(forward.edge(1)?.as_ref()));
    assert_eq!(placement(&rebuilt, 1)?.v.x(), 0.);
    Ok(())
}
