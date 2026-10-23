//! Volume publication and completion checks on owned planar geometry.
mod brep_check_support;
use brep_check_support::cube;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Face, Shape, Wire};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomPlane, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

fn unit_box() -> Shape {
    let planar = cube(&[0, 1, 2, 3, 4, 5]);
    planar.shell(&planar.faces).into_shape()
}

fn empty_wire_face() -> Shape {
    let basis: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(Plane3::default())).cast();
    let surface: Handle<dyn GeomSurface> = Handle::new(
        GeomRectangularTrimmedSurface::new(&basis, (0., 1.), (0., 1.), (true, true)).unwrap(),
    )
    .cast();
    let builder = BRepBuilder::default();
    let mut face = Face::default();
    builder
        .make_face_on_surface(&mut face, &surface, 1e-7)
        .unwrap();
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    builder.add(&mut face, &wire).unwrap();
    face.into_shape()
}

fn compound(parts: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for part in parts {
        builder.add(&mut compound, part).unwrap();
    }
    compound.into_shape()
}

fn sentinel() -> GProps {
    GProps {
        dim: 17.,
        g: Point3::new(2., 3., 4.),
        loc: Point3::new(5., 6., 7.),
        ..GProps::new()
    }
}

fn assert_unit_box(props: &GProps) {
    assert!((props.mass() - 1.).abs() < 1e-12, "mass: {}", props.mass());
    assert!(props.centre_of_mass().distance(&Point3::new(0.5, 0.5, 0.5)) < 1e-12);
    let inertia = props.matrix_of_inertia();
    for row in 1..=3 {
        for column in 1..=3 {
            let expected = if row == column { 1. / 6. } else { 0. };
            assert!((inertia.value(row, column).unwrap() - expected).abs() < 1e-12);
        }
    }
}

#[test]
fn complete_unit_box_publishes_closed_form_properties() {
    let mut props = sentinel();
    BRepGProp::volume_properties(&unit_box(), &mut props, false, false, false).unwrap();
    assert_unit_box(&props);
}

fn assert_incomplete_rollback(parts: &[Shape]) {
    let mut props = sentinel();
    let result = BRepGProp::volume_properties(&compound(parts), &mut props, false, false, false);
    assert!(
        result.is_err(),
        "incomplete integration returned {result:?}, mass {}",
        props.mass()
    );
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "BRepGProp_IncompleteIntegration");
    assert!(failure.message().contains("integration status -1"));
    assert!(failure.message().contains("expected a nonnegative"));
    assert_eq!(props, sentinel());
}

#[test]
fn incomplete_face_alone_preserves_caller_properties() {
    let invalid = empty_wire_face();
    assert_incomplete_rollback(&[invalid]);
}

#[test]
fn incomplete_face_after_box_preserves_caller_properties() {
    let parts = [unit_box(), empty_wire_face()];
    assert_incomplete_rollback(&parts);
}

#[test]
fn incomplete_face_before_box_preserves_caller_properties() {
    let parts = [empty_wire_face(), unit_box()];
    assert_incomplete_rollback(&parts);
}

#[test]
fn empty_and_filtered_shapes_publish_completed_empty_properties() {
    let open = cube(&[0]);
    for shape in [
        compound(&[]),
        empty_wire_face(),
        open.shell(&open.faces).into_shape(),
    ] {
        let mut props = sentinel();
        BRepGProp::volume_properties(&shape, &mut props, true, false, false).unwrap();
        assert_eq!(props.mass(), 0.);
    }
    let mut props = sentinel();
    BRepGProp::volume_properties(&compound(&[]), &mut props, false, false, false).unwrap();
    assert_eq!(props.mass(), 0.);
    let mut face = Face::default();
    BRepBuilder::default().make_face(&mut face);
    BRepGProp::volume_properties(&face, &mut props, false, false, false).unwrap();
    assert_eq!(props.mass(), 0.);
}

#[test]
fn closed_filter_excludes_unrelated_incomplete_face() {
    let shape = compound(&[unit_box(), empty_wire_face()]);
    let mut props = sentinel();
    BRepGProp::volume_properties(&shape, &mut props, true, false, false).unwrap();
    assert_unit_box(&props);
}

#[test]
fn explicit_numeric_status_contract_is_preserved() {
    let mut props = sentinel();
    let flags = VolumeGKFlags::default();
    let status =
        BRepGProp::volume_properties_gk(&empty_wire_face(), &mut props, 1e-5, false, flags)
            .unwrap();
    assert!(status < 0.);
    let status =
        BRepGProp::volume_properties_gk(&compound(&[]), &mut props, 1e-5, true, flags).unwrap();
    assert_eq!(status, -1.);
}

#[test]
fn explicit_plane_numeric_status_contract_is_preserved() {
    let mut props = sentinel();
    let status = BRepGProp::volume_properties_gk_plane(
        &empty_wire_face(),
        &mut props,
        Plane3::default(),
        1e-5,
        false,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(status < 0.);
    let status = BRepGProp::volume_properties_gk_plane(
        &compound(&[]),
        &mut props,
        Plane3::default(),
        1e-5,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert_eq!(status, -1.);
}
