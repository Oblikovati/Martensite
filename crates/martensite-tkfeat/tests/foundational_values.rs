//! Owned value and empty-construction contracts of the local-operation foundation.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Face, Shape, Vertex};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkfeat::loc_ope::{BuildShape, FindEdges, PntFace};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;

#[test]
fn point_face_preserves_authored_coordinates_face_identity_and_parameters() {
    let mut face = Face::default();
    BRepBuilder::default().make_face(&mut face);
    let point = Point3::new(-2., 3., 7.);
    let hit = PntFace::new(point, face.clone(), Orientation::Reversed, -0.5, 2., -3.);
    assert_eq!(hit.pnt(), &point);
    assert!(hit.face().is_same(&face));
    assert_eq!(hit.orientation(), Orientation::Reversed);
    assert_eq!(
        (hit.parameter(), hit.u_parameter(), hit.v_parameter()),
        (-0.5, 2., -3.)
    );
}

#[test]
fn cloned_point_face_has_independent_orientation_and_shared_face_identity() {
    let mut face = Face::default();
    BRepBuilder::default().make_face(&mut face);
    let original = PntFace::new(Point3::default(), face, Orientation::Forward, 4., 5., 6.);
    let mut copied = original.clone();
    *copied.change_orientation() = Orientation::External;
    assert_eq!(original.orientation(), Orientation::Forward);
    assert_eq!(copied.orientation(), Orientation::External);
    assert!(copied.face().is_same(original.face()));
    assert_eq!(
        (
            copied.parameter(),
            copied.u_parameter(),
            copied.v_parameter()
        ),
        (4., 5., 6.)
    );
}

#[test]
fn point_face_parameters_preserve_signed_zero_and_nonfinite_values() {
    let hit = PntFace::new(
        Point3::default(),
        Face::default(),
        Orientation::Internal,
        -0.,
        f64::INFINITY,
        f64::NAN,
    );
    assert_eq!(hit.parameter().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(hit.u_parameter(), f64::INFINITY);
    assert!(hit.v_parameter().is_nan());
    assert_eq!(hit.orientation(), Orientation::Internal);
}

#[test]
fn point_face_default_initializes_only_defined_reference_value_contracts() {
    let hit = PntFace::default();
    assert_eq!(hit.pnt(), &Point3::new(0., 0., 0.));
    assert!(hit.face().is_null());
    assert_eq!(
        (hit.parameter(), hit.u_parameter(), hit.v_parameter()),
        (0., 0., 0.)
    );
    // The default reference orientation is uninitialized and supplies no expected value.
}

#[test]
fn construction_without_faces_keeps_the_result_empty() -> Result<(), Failure> {
    let mut builder = BuildShape::new();
    assert!(builder.shape().is_null());
    let mut vertex = Vertex::default();
    BRepBuilder::default().make_vertex(&mut vertex);
    let mut entries = List::<Shape>::default();
    entries.append(vertex.into_shape());
    builder.perform(&entries)?;
    assert!(builder.shape().is_null());
    assert!(BuildShape::with_faces(&List::default())?.shape().is_null());
    Ok(())
}

#[test]
fn empty_edge_search_has_no_pairs_after_each_iterator_reset() -> Result<(), Failure> {
    let mut search = FindEdges::new();
    search.init_iterator();
    assert!(!search.more());
    search.set(&Shape::default(), &Shape::default())?;
    search.init_iterator();
    assert!(!search.more());
    Ok(())
}
