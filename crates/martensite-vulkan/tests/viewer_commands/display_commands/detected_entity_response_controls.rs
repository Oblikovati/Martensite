use super::*;
use martensite_tkv3d::{select_mgr::SelectMgrEntityOwner, select3d::Select3DSensitivePoint};

fn point_pick(normal: Vec3<f32>) -> RankedPick {
    let owner = SelectMgrEntityOwner::new(0).cast();
    RankedPick {
        entity: Handle::new(Select3DSensitivePoint::new(&owner, Point3::new(1., 2., 3.))).cast(),
        depth: 10.,
        min_dist: 0.,
        point: Point3::new(1., 2., 3.),
        normal,
    }
}

#[test]
fn missing_selector_returns_actual_empty_response_header() {
    assert_eq!(
        detected_entity_response(None, &|_| String::new()).unwrap(),
        "Detected entities:\n"
    );
}

#[test]
fn full_point_response_preserves_name_padding_and_general_six_fields() {
    let pick = point_pick(Vec3::new(0., 0., 1.));
    assert_eq!(
        entity_row(&pick, &|_| "b".into()),
        "  b                    Depth: 10 Distance: 0 Point: 1 2 3 Normal: 0 0 1 (Select3DSensitivePoint)\n"
    );
}

#[test]
fn long_object_name_is_never_truncated() {
    let pick = point_pick(Vec3::new(0., 0., 0.));
    assert!(
        entity_row(&pick, &|_| "an_object_name_longer_than_twenty".into())
            .starts_with("  an_object_name_longer_than_twenty Depth:")
    );
}

#[test]
fn normal_threshold_uses_float_square_and_strict_epsilon() {
    let component = martensite_cmath::powi(f64::from(2.0_f32), -12) as f32;
    assert_eq!(normal_metrics(Vec3::new(component, component, 0.)), "");
    let above = f32::from_bits(component.to_bits() + 1);
    assert!(normal_metrics(Vec3::new(above, component, 0.)).starts_with(" Normal:"));
}

#[test]
fn metrics_preserve_general_six_exponents_and_negative_zero() {
    let mut pick = point_pick(Vec3::new(0., 0., 0.));
    pick.depth = 1_234_567.;
    pick.min_dist = -0.;
    assert_eq!(
        pick_metrics(&pick),
        " Depth: 1.23457e+06 Distance: -0 Point: 1 2 3"
    );
}

#[test]
fn shape_row_reads_the_actual_vertex_implementation_type() {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    use martensite_tkv3d::std_select::StdSelectBRepOwner;
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap();
    let owner = StdSelectBRepOwner::with_shape(&vertex, 0, true);
    assert_eq!(
        shape_row(&owner.cast()),
        "                       Detected Shape: BRep_TVertex\n"
    );
}

fn matched_wire() -> Handle<Select3DSensitiveWire> {
    use martensite_tkmath::gp::{Axis2, Circle3};
    use martensite_tkv3d::select3d::Select3DSensitiveCircle;
    let owner = SelectMgrEntityOwner::new(0).cast();
    let circle = Circle3::new(Axis2::default(), 1.).unwrap();
    let child = Handle::new(Select3DSensitiveCircle::new(&owner, &circle, true)).cast();
    let wire = Handle::new(Select3DSensitiveWire::new(&owner));
    wire.add(&child).unwrap();
    require_match(wire.get().unwrap());
    wire
}

#[test]
fn wire_row_reads_actual_last_detected_child_after_axis_match() {
    assert_eq!(
        subtype_row(&matched_wire().cast()),
        "                       Detected Child: Select3DSensitiveCircle\n"
    );
}

fn require_match(entity: &dyn Select3DSensitiveEntity) {
    use martensite_tkv3d::select_basics::SelectBasicsPickResult;
    assert!(
        entity
            .matches(&axis_volume(), &mut SelectBasicsPickResult::default())
            .unwrap()
    );
}

fn axis_volume() -> martensite_tkv3d::select_mgr::SelectMgrSelectingVolumeManager {
    use martensite_tkmath::gp::{Axis1, Direction3};
    let mut volume = martensite_tkv3d::select_mgr::SelectMgrSelectingVolumeManager::new();
    volume.init_axis_selecting_volume(Axis1::new(Point3::new(0.2, 0.7, 2.), -Direction3::Z));
    volume.build_selecting_volume().unwrap();
    volume
}

fn two_triangles() -> Handle<martensite_tkmath::poly::Triangulation> {
    use martensite_tkmath::poly::{Triangle, Triangulation};
    let triangles = Triangulation::with_sizes(4, 2, false, false).unwrap();
    for (index, (x, y)) in [(0., 0.), (1., 0.), (1., 1.), (0., 1.)]
        .into_iter()
        .enumerate()
    {
        triangles
            .set_node(index as i32 + 1, Point3::new(x, y, 0.))
            .unwrap();
    }
    triangles.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    triangles.set_triangle(2, Triangle::new(1, 3, 4)).unwrap();
    Handle::new(triangles)
}

#[test]
fn triangle_row_reads_actual_second_triangle_after_axis_match() {
    use martensite_tkmath::toploc::Location;
    let owner = SelectMgrEntityOwner::new(0).cast();
    let triangle =
        Select3DSensitiveTriangulation::new(&owner, &two_triangles(), &Location::default(), true)
            .unwrap();
    require_match(&triangle);
    assert_eq!(triangle.last_detected_triangle_index(), 2);
    assert_eq!(
        subtype_row(&Handle::new(triangle).cast()),
        "                       Detected Triangle: 2\n"
    );
}

fn primitive_triangles() -> martensite_tkservice::graphic3d::Graphic3dArrayOfTriangles {
    use martensite_tkservice::graphic3d::Graphic3dArrayOfTriangles;
    let triangles = Graphic3dArrayOfTriangles::new(6, 0, 0).unwrap();
    for (x, y) in [(0., 0.), (1., 0.), (1., 1.), (0., 0.), (1., 1.), (0., 1.)] {
        triangles.add_vertex_f64(x, y, 0.).unwrap();
    }
    triangles
}

#[test]
fn array_row_reads_actual_second_element_after_axis_match() {
    use martensite_tkmath::toploc::Location;
    use martensite_tkv3d::select3d::Select3DPrimArrayInit;
    let triangles = primitive_triangles();
    let array = Select3DSensitivePrimitiveArray::new(&SelectMgrEntityOwner::new(0).cast()).unwrap();
    let init = Select3DPrimArrayInit::whole(
        &triangles.attributes(),
        &triangles.indices(),
        &Location::default(),
        true,
        1,
    );
    assert!(array.init_triangulation(&init).unwrap());
    require_match(&array);
    assert_eq!(array.last_detected_element(), 1);
    assert_eq!(
        subtype_row(&Handle::new(array).cast()),
        "                       Detected Element: 1\n"
    );
}
